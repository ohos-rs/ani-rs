//! Process-wide bounded scheduler shared by asynchronous ANI facilities.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, mpsc};
use std::time::{Duration, Instant};

use crate::error::{Error, Result, Status};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct TimerJob {
    due: Instant,
    job: Job,
}

/// Live scheduler counters used by leak and stress gates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SchedulerMetrics {
    /// Number of fixed worker threads.
    pub workers: usize,
    /// Jobs accepted but not yet started.
    pub queued: usize,
    /// Jobs currently executing.
    pub active: usize,
    /// Delayed jobs waiting for their deadline.
    pub timers: usize,
    /// Jobs completed since scheduler initialization.
    pub completed: usize,
    /// Jobs that panicked and were isolated instead of terminating a worker.
    pub panicked: usize,
}

/// Shared bounded scheduler.
pub struct RuntimeScheduler {
    sender: mpsc::SyncSender<Job>,
    timers: mpsc::Sender<TimerJob>,
    workers: usize,
    queued: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    timer_count: Arc<AtomicUsize>,
    completed: Arc<AtomicUsize>,
    panicked: Arc<AtomicUsize>,
}

impl RuntimeScheduler {
    fn new() -> Self {
        let workers = std::env::var("ANI_SCHEDULER_WORKERS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(usize::from)
                    .unwrap_or(2)
                    .clamp(2, 8)
            });
        let capacity = std::env::var("ANI_SCHEDULER_QUEUE_CAPACITY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(1024);
        let (sender, receiver) = mpsc::sync_channel::<Job>(capacity);
        let receiver = Arc::new(Mutex::new(receiver));
        let queued = Arc::new(AtomicUsize::new(0));
        let active = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicUsize::new(0));
        let panicked = Arc::new(AtomicUsize::new(0));
        for index in 0..workers {
            let receiver = Arc::clone(&receiver);
            let queued = Arc::clone(&queued);
            let active = Arc::clone(&active);
            let completed = Arc::clone(&completed);
            let panicked = Arc::clone(&panicked);
            std::thread::Builder::new()
                .name(format!("ani-scheduler-{index}"))
                .spawn(move || {
                    loop {
                        let job = match receiver.lock() {
                            Ok(receiver) => receiver.recv(),
                            Err(_) => return,
                        };
                        let Ok(job) = job else { return };
                        queued.fetch_sub(1, Ordering::AcqRel);
                        active.fetch_add(1, Ordering::AcqRel);
                        if catch_unwind(AssertUnwindSafe(job)).is_err() {
                            panicked.fetch_add(1, Ordering::AcqRel);
                        }
                        active.fetch_sub(1, Ordering::AcqRel);
                        completed.fetch_add(1, Ordering::AcqRel);
                    }
                })
                .expect("failed to create ani-rs shared scheduler worker");
        }

        let (timer_sender, timer_receiver) = mpsc::channel::<TimerJob>();
        let timer_count = Arc::new(AtomicUsize::new(0));
        let timer_count_thread = Arc::clone(&timer_count);
        let work_sender = sender.clone();
        let queued_for_timer = Arc::clone(&queued);
        std::thread::Builder::new()
            .name("ani-scheduler-timer".to_string())
            .spawn(move || {
                timer_loop(
                    timer_receiver,
                    work_sender,
                    queued_for_timer,
                    timer_count_thread,
                )
            })
            .expect("failed to create ani-rs shared scheduler timer");

        Self {
            sender,
            timers: timer_sender,
            workers,
            queued,
            active,
            timer_count,
            completed,
            panicked,
        }
    }

    /// Submit without blocking. A full queue returns [`Status::QueueFull`].
    pub fn schedule(&self, job: impl FnOnce() + Send + 'static) -> Result<()> {
        self.queued.fetch_add(1, Ordering::AcqRel);
        match self.sender.try_send(Box::new(job)) {
            Ok(()) => Ok(()),
            Err(mpsc::TrySendError::Full(_)) => {
                self.queued.fetch_sub(1, Ordering::AcqRel);
                Err(Error::new(
                    Status::QueueFull,
                    "shared scheduler queue is full",
                ))
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                self.queued.fetch_sub(1, Ordering::AcqRel);
                Err(Error::new(Status::Closing, "shared scheduler is closed"))
            }
        }
    }

    /// Submit with bounded-queue backpressure.
    pub fn schedule_blocking(&self, job: impl FnOnce() + Send + 'static) -> Result<()> {
        self.queued.fetch_add(1, Ordering::AcqRel);
        if self.sender.send(Box::new(job)).is_err() {
            self.queued.fetch_sub(1, Ordering::AcqRel);
            return Err(Error::new(Status::Closing, "shared scheduler is closed"));
        }
        Ok(())
    }

    /// Submit a job after a delay without occupying a worker while waiting.
    pub fn schedule_after(
        &self,
        delay: Duration,
        job: impl FnOnce() + Send + 'static,
    ) -> Result<()> {
        self.timer_count.fetch_add(1, Ordering::AcqRel);
        if self
            .timers
            .send(TimerJob {
                due: Instant::now() + delay,
                job: Box::new(job),
            })
            .is_err()
        {
            self.timer_count.fetch_sub(1, Ordering::AcqRel);
            return Err(Error::new(
                Status::Closing,
                "shared scheduler timer is closed",
            ));
        }
        Ok(())
    }

    /// Capture counters atomically enough for diagnostics and quiescence tests.
    pub fn metrics(&self) -> SchedulerMetrics {
        SchedulerMetrics {
            workers: self.workers,
            queued: self.queued.load(Ordering::Acquire),
            active: self.active.load(Ordering::Acquire),
            timers: self.timer_count.load(Ordering::Acquire),
            completed: self.completed.load(Ordering::Acquire),
            panicked: self.panicked.load(Ordering::Acquire),
        }
    }

    /// Wait until no queued, active, or delayed work remains.
    pub fn wait_idle(&self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let metrics = self.metrics();
            if metrics.queued == 0 && metrics.active == 0 && metrics.timers == 0 {
                return true;
            }
            std::thread::yield_now();
            std::thread::sleep(Duration::from_millis(1));
        }
        false
    }
}

fn timer_loop(
    receiver: mpsc::Receiver<TimerJob>,
    sender: mpsc::SyncSender<Job>,
    queued: Arc<AtomicUsize>,
    timer_count: Arc<AtomicUsize>,
) {
    let mut waiting = Vec::<TimerJob>::new();
    loop {
        let now = Instant::now();
        let mut index = 0;
        while index < waiting.len() {
            if waiting[index].due <= now {
                let timer = waiting.swap_remove(index);
                timer_count.fetch_sub(1, Ordering::AcqRel);
                queued.fetch_add(1, Ordering::AcqRel);
                if sender.send(timer.job).is_err() {
                    queued.fetch_sub(1, Ordering::AcqRel);
                    return;
                }
            } else {
                index += 1;
            }
        }
        let timeout = waiting
            .iter()
            .map(|timer| timer.due.saturating_duration_since(Instant::now()))
            .min()
            .unwrap_or(Duration::from_secs(3600));
        match receiver.recv_timeout(timeout) {
            Ok(timer) => waiting.push(timer),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

/// Returns the lazily initialized process-wide scheduler.
pub fn shared() -> &'static RuntimeScheduler {
    static SCHEDULER: OnceLock<RuntimeScheduler> = OnceLock::new();
    SCHEDULER.get_or_init(RuntimeScheduler::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_submission_has_no_lost_jobs_and_becomes_idle() {
        let total = 2_000;
        let completed = Arc::new(AtomicUsize::new(0));
        let mut producers = Vec::new();
        for _ in 0..8 {
            let completed = Arc::clone(&completed);
            producers.push(std::thread::spawn(move || {
                for _ in 0..(total / 8) {
                    let completed = Arc::clone(&completed);
                    shared()
                        .schedule_blocking(move || {
                            completed.fetch_add(1, Ordering::AcqRel);
                        })
                        .unwrap();
                }
            }));
        }
        for producer in producers {
            producer.join().unwrap();
        }
        assert!(shared().wait_idle(Duration::from_secs(5)));
        assert_eq!(completed.load(Ordering::Acquire), total);
    }

    #[test]
    fn delayed_jobs_do_not_occupy_worker_threads() {
        let done = Arc::new(AtomicUsize::new(0));
        let done_job = Arc::clone(&done);
        shared()
            .schedule_after(Duration::from_millis(5), move || {
                done_job.store(1, Ordering::Release);
            })
            .unwrap();
        assert!(shared().wait_idle(Duration::from_secs(1)));
        assert_eq!(done.load(Ordering::Acquire), 1);
    }

    #[test]
    fn worker_survives_a_panicking_job() {
        let panicked_before = shared().metrics().panicked;
        shared()
            .schedule_blocking(|| panic!("isolated scheduler panic"))
            .unwrap();
        let recovered = Arc::new(AtomicUsize::new(0));
        let recovered_job = Arc::clone(&recovered);
        shared()
            .schedule_blocking(move || {
                recovered_job.store(1, Ordering::Release);
            })
            .unwrap();
        assert!(shared().wait_idle(Duration::from_secs(2)));
        assert_eq!(recovered.load(Ordering::Acquire), 1);
        assert!(shared().metrics().panicked > panicked_before);
    }
}
