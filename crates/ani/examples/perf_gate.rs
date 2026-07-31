use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use ani::conversions::TypedArray;

fn threshold(name: &str, fallback: f64) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(fallback)
}

fn main() {
    let values = (0..1_000_000_u32).collect::<Vec<_>>();
    let array = TypedArray::new(values);
    let started = Instant::now();
    let bytes = array.to_le_bytes();
    let elapsed = started.elapsed().as_secs_f64();
    let typed_array_mib_s = bytes.len() as f64 / (1024.0 * 1024.0) / elapsed;
    let typed_array_min = threshold("ANI_PERF_TYPED_ARRAY_MIN_MIB_S", 20.0);

    let jobs = 20_000_usize;
    let completed = Arc::new(AtomicUsize::new(0));
    let started = Instant::now();
    for _ in 0..jobs {
        let completed = Arc::clone(&completed);
        ani::scheduler::shared()
            .schedule_blocking(move || {
                completed.fetch_add(1, Ordering::AcqRel);
            })
            .expect("scheduler rejected performance-gate work");
    }
    assert!(
        ani::scheduler::shared().wait_idle(Duration::from_secs(10)),
        "scheduler did not become idle"
    );
    assert_eq!(completed.load(Ordering::Acquire), jobs);
    let scheduler_jobs_s = jobs as f64 / started.elapsed().as_secs_f64();
    let scheduler_min = threshold("ANI_PERF_SCHEDULER_MIN_JOBS_S", 10_000.0);

    println!("typed_array_mib_s={typed_array_mib_s:.1} scheduler_jobs_s={scheduler_jobs_s:.0}");
    assert!(
        typed_array_mib_s >= typed_array_min,
        "typed-array throughput {typed_array_mib_s:.1} MiB/s is below gate {typed_array_min:.1} MiB/s"
    );
    assert!(
        scheduler_jobs_s >= scheduler_min,
        "scheduler throughput {scheduler_jobs_s:.0} jobs/s is below gate {scheduler_min:.0} jobs/s"
    );
}
