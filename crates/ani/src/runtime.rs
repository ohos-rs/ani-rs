//! Runtime diagnostics and leak-gate helpers.

use std::time::Duration;

use crate::async_runtime::{AsyncRuntimeMetrics, async_runtime_metrics};
use crate::conversions::{
    live_async_stream_count, live_deferred_count, live_managed_resource_count,
    live_promise_observer_count, pending_async_stream_waiter_count,
    pending_threadsafe_function_call_count,
};
use crate::env::{ReferenceMetrics, reference_metrics};
use crate::error::{Error, Result, Status};
use crate::scheduler::{SchedulerMetrics, shared};

/// One process-wide snapshot of resources owned by ani-rs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuntimeMetrics {
    /// Safe ANI global/weak reference counts.
    pub references: ReferenceMetrics,
    /// Values retained by [`crate::conversions::ManagedResource`].
    pub managed_resources: usize,
    /// Live bounded async-stream receivers.
    pub async_streams: usize,
    /// Promise resolvers retained by Rust.
    pub deferreds: usize,
    /// Generated ETS continuation observers.
    pub promise_observers: usize,
    /// TSFN calls waiting in bounded queues.
    pub threadsafe_pending_calls: usize,
    /// Async iterator pulls waiting for data.
    pub stream_waiters: usize,
    /// Task handles registered with the ETS cancellation bridge.
    pub cancel_tokens: usize,
    /// Executor-independent task/settlement state.
    pub async_runtime: AsyncRuntimeMetrics,
    /// Shared scheduler state.
    pub scheduler: SchedulerMetrics,
}

/// Capture current ownership and scheduler counters.
pub fn runtime_metrics() -> Result<RuntimeMetrics> {
    Ok(RuntimeMetrics {
        references: reference_metrics(),
        managed_resources: live_managed_resource_count()?,
        async_streams: live_async_stream_count(),
        deferreds: live_deferred_count(),
        promise_observers: live_promise_observer_count(),
        threadsafe_pending_calls: pending_threadsafe_function_call_count(),
        stream_waiters: pending_async_stream_waiter_count(),
        cancel_tokens: crate::async_runtime::live_runtime_cancel_token_count(),
        async_runtime: async_runtime_metrics(),
        scheduler: shared().metrics(),
    })
}

/// Wait for scheduled work to settle and assert that owned resources returned
/// to a previously captured baseline.
///
/// Completed-job and isolated-panic totals are monotonic diagnostics and are
/// intentionally excluded from the leak comparison.
pub fn assert_no_runtime_leaks(
    baseline: RuntimeMetrics,
    timeout: Duration,
) -> Result<RuntimeMetrics> {
    let deadline = std::time::Instant::now() + timeout;
    let current = loop {
        let current = runtime_metrics()?;
        // A checkpoint can race with work that has already settled at the ANI
        // boundary but has not yet run its Rust-side terminal cleanup. Counts
        // are therefore allowed to fall below the checkpoint; only retained
        // ownership above the checkpoint is a leak.
        let stable = current.references.global <= baseline.references.global
            && current.references.weak <= baseline.references.weak
            && current.managed_resources <= baseline.managed_resources
            && current.async_streams <= baseline.async_streams
            && current.deferreds <= baseline.deferreds
            && current.promise_observers <= baseline.promise_observers
            && current.threadsafe_pending_calls <= baseline.threadsafe_pending_calls
            && current.stream_waiters <= baseline.stream_waiters
            && current.cancel_tokens <= baseline.cancel_tokens
            && current.async_runtime.live_tasks <= baseline.async_runtime.live_tasks
            && current.async_runtime.pending_settlements
                <= baseline.async_runtime.pending_settlements
            && current.scheduler.queued == 0
            && current.scheduler.active == 0
            && current.scheduler.timers == 0
            && current.scheduler.cancellables <= baseline.scheduler.cancellables
            && !current.scheduler.closing
            && !current.async_runtime.changing_state;
        if stable {
            return Ok(current);
        }
        if std::time::Instant::now() >= deadline {
            break current;
        }
        std::thread::yield_now();
        std::thread::sleep(Duration::from_millis(1));
    };

    Err(Error::new(
        Status::GenericFailure,
        format!("ani-rs runtime leak detected: baseline={baseline:?}, current={current:?}"),
    )
    .with_code(100006))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::stream_channel;

    #[test]
    fn metrics_observe_live_streams() {
        let baseline = live_async_stream_count();
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        assert!(live_async_stream_count() > baseline);
        drop(sender);
        drop(stream);
    }
}
