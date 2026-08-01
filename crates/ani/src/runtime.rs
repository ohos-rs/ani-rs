//! Runtime diagnostics and leak-gate helpers.

use std::time::Duration;

use crate::conversions::{live_async_stream_count, live_managed_resource_count};
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
    /// Shared scheduler state.
    pub scheduler: SchedulerMetrics,
}

/// Capture current ownership and scheduler counters.
pub fn runtime_metrics() -> Result<RuntimeMetrics> {
    Ok(RuntimeMetrics {
        references: reference_metrics(),
        managed_resources: live_managed_resource_count()?,
        async_streams: live_async_stream_count(),
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
    if !shared().wait_idle(timeout) {
        let metrics = shared().metrics();
        return Err(Error::new(
            Status::GenericFailure,
            format!(
                "runtime did not become idle: queued={}, active={}, timers={}",
                metrics.queued, metrics.active, metrics.timers
            ),
        )
        .with_code(100005));
    }

    let current = runtime_metrics()?;
    let stable = current.references == baseline.references
        && current.managed_resources == baseline.managed_resources
        && current.async_streams == baseline.async_streams
        && current.scheduler.queued == 0
        && current.scheduler.active == 0
        && current.scheduler.timers == 0
        && current.scheduler.cancellables == baseline.scheduler.cancellables
        && !current.scheduler.closing;
    if stable {
        return Ok(current);
    }

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
