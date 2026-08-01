//! Cancellable background tasks exposed as ArkTS Promises.

use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::env::Env;
use crate::error::{AniErrorPayload, Error, Result, Status};
use crate::scheduler::RuntimeCancellable;
use crate::sys;

use super::{PromiseRaw, PromiseValue, ToAni, TypeInfo};

/// A cloneable cooperative-cancellation signal.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a token in the active state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation. Calling this more than once is harmless.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Returns a typed cancellation error when cancellation was requested.
    pub fn check(&self) -> Result<()> {
        if self.is_cancelled() {
            Err(Error::new(Status::Cancelled, "asynchronous task cancelled"))
        } else {
            Ok(())
        }
    }
}

impl RuntimeCancellable for CancellationToken {
    fn cancel_for_runtime_shutdown(&self) {
        self.cancel();
    }
}

/// Work that can run away from the ANI thread and resolve on an attached VM thread.
pub trait Task: Send + Sized + 'static {
    /// Worker-thread result passed to [`resolve`](Self::resolve).
    type Output: Send + 'static;
    /// ArkTS value used to resolve the resulting Promise.
    type JsValue: for<'env> PromiseValue<'env>;
    /// Structured application error retained across the scheduler boundary.
    type Error: AniErrorPayload;

    /// Performs worker-thread work. Implementations should periodically call
    /// [`CancellationToken::check`] for cooperative cancellation.
    fn compute(
        &mut self,
        cancellation: &CancellationToken,
    ) -> std::result::Result<Self::Output, Self::Error>;

    /// Converts worker output while attached to the owning ANI VM.
    fn resolve<'env>(
        self,
        env: &Env<'env>,
        output: Self::Output,
    ) -> std::result::Result<Self::JsValue, Self::Error>;
}

/// A scheduler-backed background task that converts directly to `Promise<Object>`.
pub struct AsyncTask<T: Task, V = <T as Task>::JsValue> {
    task: T,
    cancellation: CancellationToken,
    value: PhantomData<fn() -> V>,
}

impl<T, V> AsyncTask<T, V>
where
    T: Task<JsValue = V>,
    V: for<'env> PromiseValue<'env> + 'static,
{
    /// Creates a task with a fresh cancellation token.
    pub fn new(task: T) -> Self {
        Self {
            task,
            cancellation: CancellationToken::new(),
            value: PhantomData,
        }
    }

    /// Creates a task controlled by an existing cancellation token.
    pub fn with_token(task: T, cancellation: CancellationToken) -> Self {
        Self {
            task,
            cancellation,
            value: PhantomData,
        }
    }

    /// Returns a clone of the token that can cancel this task.
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Starts the task and returns its typed Promise.
    pub fn run<'env>(self, env: &Env<'env>) -> Result<PromiseRaw<'env, T::JsValue>> {
        let vm = Arc::new(env.get_vm()?);
        let (deferred, promise) = PromiseRaw::<T::JsValue>::deferred(env)?;
        let cancellation = Arc::new(self.cancellation);
        let settlement = Arc::new(Mutex::new(Some(deferred.cast::<()>())));
        let work_settlement = Arc::clone(&settlement);
        let reject_settlement = Arc::clone(&settlement);
        let work_vm = Arc::clone(&vm);
        let reject_vm = Arc::clone(&vm);
        let work_cancellation = Arc::clone(&cancellation);
        let reject_cancellation = Arc::clone(&cancellation);
        let mut task = self.task;

        let (runtime_task, _handle) = crate::async_runtime::RuntimeBlockingTask::new(
            move || {
                let computed = task.compute(&work_cancellation);
                let deferred = work_settlement
                    .lock()
                    .ok()
                    .and_then(|mut deferred| deferred.take());
                let Some(deferred) = deferred else { return };
                let _ = work_vm.with_attached(|env| match computed {
                    Ok(output) => match task.resolve(env, output) {
                        Ok(value) => deferred.resolve_value(env, value),
                        Err(error) => deferred.reject_with_payload(env, error),
                    },
                    Err(error) => deferred.reject_with_payload(env, error),
                });
            },
            move |error| {
                reject_cancellation.cancel();
                let deferred = reject_settlement
                    .lock()
                    .ok()
                    .and_then(|mut deferred| deferred.take());
                if let Some(deferred) = deferred {
                    let _ = reject_vm.with_attached(|env| deferred.reject_with_payload(env, error));
                }
            },
        );
        crate::async_runtime::spawn_runtime_blocking_task(runtime_task)?;

        Ok(promise)
    }
}

impl<T, V> TypeInfo for AsyncTask<T, V>
where
    T: Task<JsValue = V>,
    V: for<'env> PromiseValue<'env> + 'static,
{
    fn type_signature() -> &'static str {
        "Lstd/core/Promise;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T, V> ToAni<'env> for AsyncTask<T, V>
where
    T: Task<JsValue = V>,
    V: for<'vm> PromiseValue<'vm> + 'static,
{
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.run(env).map(PromiseRaw::into_raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_is_cloneable_and_idempotent() {
        let token = CancellationToken::new();
        let clone = token.clone();
        assert!(!token.is_cancelled());
        clone.cancel();
        token.cancel();
        assert!(token.is_cancelled());
        assert_eq!(token.check().unwrap_err().status, Status::Cancelled);
    }
}
