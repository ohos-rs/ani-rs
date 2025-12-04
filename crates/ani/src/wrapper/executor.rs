use std::sync::Arc;

use crate::{errors::*, anienv::ANIEnv, AniVM};

/// The capacity of local frames, allocated for attached threads by default.
pub const DEFAULT_LOCAL_FRAME_CAPACITY: i32 = 32;

/// Thread attachment manager. It allows to execute closures in attached threads with automatic
/// local references management.
#[derive(Clone)]
pub struct Executor {
    vm: Arc<AniVM>,
}

impl Executor {
    /// Creates new Executor with specified `AniVM`.
    pub fn new(vm: Arc<AniVM>) -> Self {
        Self { vm }
    }

    /// Executes a provided closure, making sure that the current thread is attached to the VM.
    /// If the current thread is not attached, it attaches it, executes the closure, and detaches
    /// after completion.
    pub fn with_attached<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut ANIEnv) -> Result<R>,
    {
        let mut guard = self.vm.attach_current_thread()?;
        f(&mut guard)
    }

    /// Executes a provided closure with a local frame, making sure that the current thread
    /// is attached to the VM.
    pub fn with_attached_capacity<F, R>(&self, capacity: i32, f: F) -> Result<R>
    where
        F: FnOnce(&mut ANIEnv) -> Result<R>,
    {
        let mut guard = self.vm.attach_current_thread()?;
        guard.push_local_frame(capacity as usize)?;
        let result = f(&mut guard);
        guard.pop_local_frame()?;
        result
    }
}
