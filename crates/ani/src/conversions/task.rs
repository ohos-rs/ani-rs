//! Cancellable background tasks exposed as ArkTS Promises.

use std::any::Any;
use std::marker::PhantomData;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::env::Env;
use crate::error::{Error, Result, Status};
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

/// Work that can run away from the ANI thread and resolve on an attached VM thread.
pub trait Task: Send + Sized + 'static {
    /// Worker-thread result passed to [`resolve`](Self::resolve).
    type Output: Send + 'static;
    /// ArkTS value used to resolve the resulting Promise.
    type JsValue: for<'env> PromiseValue<'env>;

    /// Performs worker-thread work. Implementations should periodically call
    /// [`CancellationToken::check`] for cooperative cancellation.
    fn compute(&mut self, cancellation: &CancellationToken) -> Result<Self::Output>;

    /// Converts worker output while attached to the owning ANI VM.
    fn resolve<'env>(self, env: &Env<'env>, output: Self::Output) -> Result<Self::JsValue>;
}

/// A napi-rs-style background task that converts directly to `Promise<Object>`.
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
        let vm = env.get_vm()?;
        let (deferred, promise) = PromiseRaw::<T::JsValue>::deferred(env)?;
        let cancellation = self.cancellation;
        let mut task = self.task;

        std::thread::Builder::new()
            .name("ani-async-task".to_string())
            .spawn(move || {
                let computed = catch_unwind(AssertUnwindSafe(|| {
                    cancellation.check()?;
                    let output = task.compute(&cancellation)?;
                    cancellation.check()?;
                    Ok(output)
                }));

                let _ = vm.with_attached(|env| match computed {
                    Ok(Ok(output)) => match task.resolve(env, output) {
                        Ok(value) => deferred.resolve_value(env, value),
                        Err(error) => deferred.reject_with_error(env, error),
                    },
                    Ok(Err(error)) => deferred.reject_with_error(env, error),
                    Err(payload) => deferred.reject(
                        env,
                        format!("panic in AsyncTask: {}", panic_message(payload)),
                    ),
                });
            })
            .map_err(|error| {
                Error::new(
                    Status::GenericFailure,
                    format!("failed to spawn AsyncTask worker: {error}"),
                )
            })?;

        Ok(promise)
    }
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else {
        "unknown panic payload".to_string()
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
