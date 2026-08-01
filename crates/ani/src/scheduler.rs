//! Restartable, process-wide bounded scheduler shared by asynchronous ANI facilities.

use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::error::{Error, Result, Status};

type Job = Box<dyn FnOnce() + Send + 'static>;
type RunningSnapshot = (
    mpsc::SyncSender<Job>,
    mpsc::Sender<TimerCommand>,
    Arc<AtomicBool>,
    Arc<CancellationRegistry>,
);

struct TimerJob {
    due: Instant,
    job: Job,
}

enum TimerCommand {
    Schedule(TimerJob),
    Shutdown,
}

/// A runtime-owned operation that must be cancelled before scheduler shutdown.
///
/// Implementations must be non-blocking. Shutdown invokes every registered
/// target before draining accepted jobs and joining worker threads.
pub(crate) trait RuntimeCancellable: Send + Sync {
    fn cancel_for_runtime_shutdown(&self);
}

struct CancellationRegistry {
    next_id: AtomicU64,
    closed: AtomicBool,
    entries: Mutex<HashMap<u64, Weak<dyn RuntimeCancellable>>>,
}

impl CancellationRegistry {
    fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            closed: AtomicBool::new(false),
            entries: Mutex::new(HashMap::new()),
        }
    }

    fn register<T>(self: &Arc<Self>, target: &Arc<T>) -> Result<RuntimeRegistration>
    where
        T: RuntimeCancellable + 'static,
    {
        if self.closed.load(Ordering::Acquire) {
            target.cancel_for_runtime_shutdown();
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }

        let target: Arc<dyn RuntimeCancellable> = target.clone();
        let id = self.next_id.fetch_add(1, Ordering::AcqRel);
        let mut entries = self.entries.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "runtime cancellation registry lock poisoned",
            )
        })?;
        if self.closed.load(Ordering::Acquire) {
            drop(entries);
            target.cancel_for_runtime_shutdown();
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }
        entries.retain(|_, target| target.strong_count() > 0);
        entries.insert(id, Arc::downgrade(&target));
        Ok(RuntimeRegistration {
            id,
            registry: Arc::downgrade(self),
        })
    }

    fn cancel_all(&self) {
        self.closed.store(true, Ordering::Release);
        let targets = self
            .entries
            .lock()
            .map(|mut entries| {
                entries
                    .drain()
                    .filter_map(|(_, target)| target.upgrade())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        for target in targets {
            target.cancel_for_runtime_shutdown();
        }
    }

    fn remove(&self, id: u64) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.remove(&id);
        }
    }

    fn live_count(&self) -> usize {
        self.entries
            .lock()
            .map(|mut entries| {
                entries.retain(|_, target| target.strong_count() > 0);
                entries.len()
            })
            .unwrap_or(0)
    }
}

/// RAII registration for an operation participating in runtime shutdown.
pub(crate) struct RuntimeRegistration {
    id: u64,
    registry: Weak<CancellationRegistry>,
}

impl Drop for RuntimeRegistration {
    fn drop(&mut self) {
        if let Some(registry) = self.registry.upgrade() {
            registry.remove(self.id);
        }
    }
}

/// Live scheduler counters used by leak and stress gates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SchedulerMetrics {
    /// Current scheduler generation. It increases after every restart.
    pub generation: u64,
    /// Number of fixed worker threads in the current generation.
    pub workers: usize,
    /// Whether the current generation is shutting down.
    pub closing: bool,
    /// Jobs accepted but not yet started.
    pub queued: usize,
    /// Jobs currently executing.
    pub active: usize,
    /// Delayed jobs waiting for their deadline.
    pub timers: usize,
    /// Runtime-owned cancellable operations.
    pub cancellables: usize,
    /// Jobs completed across all scheduler generations.
    pub completed: usize,
    /// Jobs that panicked and were isolated instead of terminating a worker.
    pub panicked: usize,
}

struct SchedulerGeneration {
    id: u64,
    sender: mpsc::SyncSender<Job>,
    timers: mpsc::Sender<TimerCommand>,
    worker_handles: Vec<JoinHandle<()>>,
    timer_handle: JoinHandle<()>,
    closing: Arc<AtomicBool>,
    cancellations: Arc<CancellationRegistry>,
    workers: usize,
}

enum SchedulerPhase {
    Dormant,
    Running(SchedulerGeneration),
    Closing,
}

struct SchedulerState {
    phase: SchedulerPhase,
    next_generation: u64,
}

/// Shared bounded runtime kernel.
///
/// The object itself is process-wide, while worker generations are created
/// lazily. [`shutdown`](Self::shutdown) cancels registered operations, drains
/// accepted work, joins every thread, and leaves the kernel restartable.
pub struct RuntimeScheduler {
    state: Mutex<SchedulerState>,
    shutdown_finished: Condvar,
    generation: AtomicU64,
    workers: AtomicUsize,
    queued: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    timer_count: Arc<AtomicUsize>,
    completed: Arc<AtomicUsize>,
    panicked: Arc<AtomicUsize>,
}

/// Terminal name for the shared asynchronous runtime lifecycle owner.
///
/// `RuntimeScheduler` remains available as a compatibility name.
pub type RuntimeKernel = RuntimeScheduler;

impl RuntimeScheduler {
    fn new() -> Self {
        Self {
            state: Mutex::new(SchedulerState {
                phase: SchedulerPhase::Dormant,
                next_generation: 1,
            }),
            shutdown_finished: Condvar::new(),
            generation: AtomicU64::new(0),
            workers: AtomicUsize::new(0),
            queued: Arc::new(AtomicUsize::new(0)),
            active: Arc::new(AtomicUsize::new(0)),
            timer_count: Arc::new(AtomicUsize::new(0)),
            completed: Arc::new(AtomicUsize::new(0)),
            panicked: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn configured_workers() -> usize {
        std::env::var("ANI_SCHEDULER_WORKERS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(usize::from)
                    .unwrap_or(2)
                    .clamp(2, 8)
            })
    }

    fn configured_capacity() -> usize {
        std::env::var("ANI_SCHEDULER_QUEUE_CAPACITY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(1024)
    }

    fn start_generation(&self, id: u64) -> SchedulerGeneration {
        let workers = Self::configured_workers();
        let capacity = Self::configured_capacity();
        self.queued.store(0, Ordering::Release);
        self.active.store(0, Ordering::Release);
        self.timer_count.store(0, Ordering::Release);

        let closing = Arc::new(AtomicBool::new(false));
        let cancellations = Arc::new(CancellationRegistry::new());
        let (sender, receiver) = mpsc::sync_channel::<Job>(capacity);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut worker_handles = Vec::with_capacity(workers);
        for index in 0..workers {
            let receiver = Arc::clone(&receiver);
            let queued = Arc::clone(&self.queued);
            let active = Arc::clone(&self.active);
            let completed = Arc::clone(&self.completed);
            let panicked = Arc::clone(&self.panicked);
            worker_handles.push(
                std::thread::Builder::new()
                    .name(format!("ani-scheduler-{id}-{index}"))
                    .spawn(move || worker_loop(receiver, queued, active, completed, panicked))
                    .expect("failed to create ani-rs runtime worker"),
            );
        }

        let (timer_sender, timer_receiver) = mpsc::channel::<TimerCommand>();
        let timer_count = Arc::clone(&self.timer_count);
        let queued = Arc::clone(&self.queued);
        let work_sender = sender.clone();
        let timer_handle = std::thread::Builder::new()
            .name(format!("ani-scheduler-{id}-timer"))
            .spawn(move || timer_loop(timer_receiver, work_sender, queued, timer_count))
            .expect("failed to create ani-rs runtime timer");

        self.generation.store(id, Ordering::Release);
        self.workers.store(workers, Ordering::Release);
        SchedulerGeneration {
            id,
            sender,
            timers: timer_sender,
            worker_handles,
            timer_handle,
            closing,
            cancellations,
            workers,
        }
    }

    fn running_snapshot(&self) -> Result<RunningSnapshot> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "runtime scheduler lock poisoned"))?;
        if matches!(state.phase, SchedulerPhase::Dormant) {
            let id = state.next_generation;
            state.next_generation = state.next_generation.saturating_add(1);
            state.phase = SchedulerPhase::Running(self.start_generation(id));
        }
        match &state.phase {
            SchedulerPhase::Running(generation) => Ok((
                generation.sender.clone(),
                generation.timers.clone(),
                Arc::clone(&generation.closing),
                Arc::clone(&generation.cancellations),
            )),
            SchedulerPhase::Dormant => unreachable!("generation started above"),
            SchedulerPhase::Closing => {
                Err(Error::new(Status::Closing, "shared runtime is closing"))
            }
        }
    }

    /// Register an operation that must be cancelled during runtime shutdown.
    pub(crate) fn register_cancellable<T>(&self, target: &Arc<T>) -> Result<RuntimeRegistration>
    where
        T: RuntimeCancellable + 'static,
    {
        let (_, _, closing, cancellations) = self.running_snapshot()?;
        if closing.load(Ordering::Acquire) {
            target.cancel_for_runtime_shutdown();
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }
        cancellations.register(target)
    }

    /// Submit without blocking. A full queue returns [`Status::QueueFull`].
    pub fn schedule(&self, job: impl FnOnce() + Send + 'static) -> Result<()> {
        let (sender, _, closing, _) = self.running_snapshot()?;
        if closing.load(Ordering::Acquire) {
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }
        self.queued.fetch_add(1, Ordering::AcqRel);
        match sender.try_send(Box::new(job)) {
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
                Err(Error::new(Status::Closing, "shared runtime is closing"))
            }
        }
    }

    /// Submit with bounded-queue backpressure.
    pub fn schedule_blocking(&self, job: impl FnOnce() + Send + 'static) -> Result<()> {
        let (sender, _, closing, _) = self.running_snapshot()?;
        let mut job: Job = Box::new(job);
        loop {
            if closing.load(Ordering::Acquire) {
                return Err(Error::new(Status::Closing, "shared runtime is closing"));
            }
            self.queued.fetch_add(1, Ordering::AcqRel);
            match sender.try_send(job) {
                Ok(()) => return Ok(()),
                Err(mpsc::TrySendError::Full(returned)) => {
                    self.queued.fetch_sub(1, Ordering::AcqRel);
                    job = returned;
                    std::thread::park_timeout(Duration::from_millis(1));
                }
                Err(mpsc::TrySendError::Disconnected(_)) => {
                    self.queued.fetch_sub(1, Ordering::AcqRel);
                    return Err(Error::new(Status::Closing, "shared runtime is closing"));
                }
            }
        }
    }

    /// Submit a job after a delay without occupying a worker while waiting.
    pub fn schedule_after(
        &self,
        delay: Duration,
        job: impl FnOnce() + Send + 'static,
    ) -> Result<()> {
        let (_, timers, closing, _) = self.running_snapshot()?;
        if closing.load(Ordering::Acquire) {
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }
        self.timer_count.fetch_add(1, Ordering::AcqRel);
        if timers
            .send(TimerCommand::Schedule(TimerJob {
                due: Instant::now() + delay,
                job: Box::new(job),
            }))
            .is_err()
        {
            self.timer_count.fetch_sub(1, Ordering::AcqRel);
            return Err(Error::new(Status::Closing, "shared runtime is closing"));
        }
        Ok(())
    }

    /// Capture counters for diagnostics and quiescence tests.
    pub fn metrics(&self) -> SchedulerMetrics {
        let (closing, cancellables, workers, generation) = self
            .state
            .lock()
            .map(|state| match &state.phase {
                SchedulerPhase::Running(current) => (
                    current.closing.load(Ordering::Acquire),
                    current.cancellations.live_count(),
                    current.workers,
                    current.id,
                ),
                SchedulerPhase::Closing => (
                    true,
                    0,
                    self.workers.load(Ordering::Acquire),
                    self.generation.load(Ordering::Acquire),
                ),
                SchedulerPhase::Dormant => (false, 0, 0, self.generation.load(Ordering::Acquire)),
            })
            .unwrap_or((true, 0, 0, self.generation.load(Ordering::Acquire)));
        SchedulerMetrics {
            generation,
            workers,
            closing,
            queued: self.queued.load(Ordering::Acquire),
            active: self.active.load(Ordering::Acquire),
            timers: self.timer_count.load(Ordering::Acquire),
            cancellables,
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

    /// Cancel runtime-owned operations, drain accepted jobs, and join all
    /// scheduler threads. The next submission starts a fresh generation.
    pub fn shutdown(&self) -> Result<()> {
        if std::thread::current()
            .name()
            .is_some_and(|name| name.starts_with("ani-scheduler-"))
        {
            return Err(Error::new(
                Status::InvalidArgs,
                "runtime shutdown cannot run on a scheduler thread",
            ));
        }

        let generation = {
            let mut state = self.state.lock().map_err(|_| {
                Error::new(Status::GenericFailure, "runtime scheduler lock poisoned")
            })?;
            loop {
                match std::mem::replace(&mut state.phase, SchedulerPhase::Closing) {
                    SchedulerPhase::Dormant => {
                        state.phase = SchedulerPhase::Dormant;
                        return Ok(());
                    }
                    SchedulerPhase::Running(generation) => break generation,
                    SchedulerPhase::Closing => {
                        state.phase = SchedulerPhase::Closing;
                        state = self.shutdown_finished.wait(state).map_err(|_| {
                            Error::new(
                                Status::GenericFailure,
                                "runtime scheduler lock poisoned while waiting for shutdown",
                            )
                        })?;
                    }
                }
            }
        };

        generation.closing.store(true, Ordering::Release);
        generation.cancellations.cancel_all();
        let _ = generation.timers.send(TimerCommand::Shutdown);

        let mut join_failed = generation.timer_handle.join().is_err();
        drop(generation.timers);
        drop(generation.sender);
        for handle in generation.worker_handles {
            join_failed |= handle.join().is_err();
        }

        self.queued.store(0, Ordering::Release);
        self.active.store(0, Ordering::Release);
        self.timer_count.store(0, Ordering::Release);
        self.workers.store(0, Ordering::Release);
        if let Ok(mut state) = self.state.lock() {
            state.phase = SchedulerPhase::Dormant;
            self.shutdown_finished.notify_all();
        }

        if join_failed {
            Err(Error::new(
                Status::GenericFailure,
                "one or more runtime scheduler threads panicked during shutdown",
            ))
        } else {
            Ok(())
        }
    }
}

fn worker_loop(
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    queued: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    completed: Arc<AtomicUsize>,
    panicked: Arc<AtomicUsize>,
) {
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
}

fn timer_loop(
    receiver: mpsc::Receiver<TimerCommand>,
    sender: mpsc::SyncSender<Job>,
    queued: Arc<AtomicUsize>,
    timer_count: Arc<AtomicUsize>,
) {
    let mut waiting = Vec::<TimerJob>::new();
    loop {
        let now = Instant::now();
        let mut index = 0;
        while index < waiting.len() {
            if waiting[index].due > now {
                index += 1;
                continue;
            }
            let timer = waiting.swap_remove(index);
            queued.fetch_add(1, Ordering::AcqRel);
            match sender.try_send(timer.job) {
                Ok(()) => {
                    timer_count.fetch_sub(1, Ordering::AcqRel);
                }
                Err(mpsc::TrySendError::Full(job)) => {
                    queued.fetch_sub(1, Ordering::AcqRel);
                    waiting.push(TimerJob {
                        due: Instant::now() + Duration::from_millis(1),
                        job,
                    });
                }
                Err(mpsc::TrySendError::Disconnected(_)) => {
                    queued.fetch_sub(1, Ordering::AcqRel);
                    timer_count.store(0, Ordering::Release);
                    return;
                }
            }
        }

        let timeout = waiting
            .iter()
            .map(|timer| timer.due.saturating_duration_since(Instant::now()))
            .min()
            .unwrap_or(Duration::from_millis(100))
            .min(Duration::from_millis(100));
        match receiver.recv_timeout(timeout) {
            Ok(TimerCommand::Schedule(timer)) => waiting.push(timer),
            Ok(TimerCommand::Shutdown) => {
                timer_count.fetch_sub(waiting.len(), Ordering::AcqRel);
                return;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                timer_count.fetch_sub(waiting.len(), Ordering::AcqRel);
                return;
            }
        }
    }
}

/// Returns the lazily initialized process-wide runtime kernel.
pub fn shared() -> &'static RuntimeScheduler {
    static SCHEDULER: OnceLock<RuntimeScheduler> = OnceLock::new();
    SCHEDULER.get_or_init(RuntimeScheduler::new)
}

/// Returns the process-wide restartable runtime kernel.
pub fn runtime_kernel() -> &'static RuntimeKernel {
    shared()
}

/// Shut down the current runtime generation, if one exists.
pub fn shutdown_runtime() -> Result<()> {
    shared().shutdown()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestCancellation(AtomicBool);

    impl RuntimeCancellable for TestCancellation {
        fn cancel_for_runtime_shutdown(&self) {
            self.0.store(true, Ordering::Release);
        }
    }

    #[test]
    fn concurrent_submission_has_no_lost_jobs_and_becomes_idle() {
        let scheduler = Arc::new(RuntimeScheduler::new());
        let total = 2_000;
        let completed = Arc::new(AtomicUsize::new(0));
        let mut producers = Vec::new();
        for _ in 0..8 {
            let completed = Arc::clone(&completed);
            let scheduler = Arc::clone(&scheduler);
            producers.push(std::thread::spawn(move || {
                for _ in 0..(total / 8) {
                    let completed = Arc::clone(&completed);
                    scheduler
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
        assert!(scheduler.wait_idle(Duration::from_secs(5)));
        assert_eq!(completed.load(Ordering::Acquire), total);
        scheduler.shutdown().unwrap();
    }

    #[test]
    fn shutdown_cancels_joins_and_restarts_with_a_new_generation() {
        let scheduler = RuntimeScheduler::new();
        let target = Arc::new(TestCancellation::default());
        let registration = scheduler.register_cancellable(&target).unwrap();
        let first = scheduler.metrics().generation;
        scheduler.shutdown().unwrap();
        assert!(target.0.load(Ordering::Acquire));
        assert_eq!(scheduler.metrics().workers, 0);
        drop(registration);

        scheduler.schedule(|| {}).unwrap();
        assert!(scheduler.wait_idle(Duration::from_secs(1)));
        let second = scheduler.metrics().generation;
        assert!(second > first);
        scheduler.shutdown().unwrap();
    }

    #[test]
    fn delayed_jobs_do_not_occupy_worker_threads() {
        let scheduler = RuntimeScheduler::new();
        let done = Arc::new(AtomicUsize::new(0));
        let done_job = Arc::clone(&done);
        scheduler
            .schedule_after(Duration::from_millis(5), move || {
                done_job.store(1, Ordering::Release);
            })
            .unwrap();
        assert!(scheduler.wait_idle(Duration::from_secs(1)));
        assert_eq!(done.load(Ordering::Acquire), 1);
        scheduler.shutdown().unwrap();
    }

    #[test]
    fn worker_survives_a_panicking_job() {
        let scheduler = RuntimeScheduler::new();
        let panicked_before = scheduler.metrics().panicked;
        scheduler
            .schedule_blocking(|| panic!("isolated scheduler panic"))
            .unwrap();
        let recovered = Arc::new(AtomicUsize::new(0));
        let recovered_job = Arc::clone(&recovered);
        scheduler
            .schedule_blocking(move || {
                recovered_job.store(1, Ordering::Release);
            })
            .unwrap();
        assert!(scheduler.wait_idle(Duration::from_secs(2)));
        assert_eq!(recovered.load(Ordering::Acquire), 1);
        assert!(scheduler.metrics().panicked > panicked_before);
        scheduler.shutdown().unwrap();
    }
}
