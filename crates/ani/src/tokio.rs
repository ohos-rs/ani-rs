//! Tokio integration for ANI Promise-based APIs.
//!
//! This module provides a minimal bridge from Rust futures to ArkTS `Promise<T>`.

#[cfg(feature = "tokio_rt")]
mod imp {
    use std::future::Future;
    use std::sync::OnceLock;

    use crate::conversions::{Deferred, PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{Error, Result, Status};

    static TOKIO_RUNTIME: OnceLock<::tokio::runtime::Runtime> = OnceLock::new();

    fn build_runtime() -> Result<::tokio::runtime::Runtime> {
        ::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("ani-tokio")
            .build()
            .map_err(|err| {
                Error::new(
                    Status::GenericFailure,
                    format!("failed to create tokio runtime: {err}"),
                )
            })
    }

    /// Get the shared tokio runtime used by ANI Promise helpers.
    pub fn runtime() -> Result<&'static ::tokio::runtime::Runtime> {
        if let Some(runtime) = TOKIO_RUNTIME.get() {
            return Ok(runtime);
        }

        let runtime = build_runtime()?;
        let _ = TOKIO_RUNTIME.set(runtime);
        Ok(TOKIO_RUNTIME
            .get()
            .expect("tokio runtime must be initialized after set attempt"))
    }

    /// Spawn a Rust future and expose its completion as an ArkTS `Promise<T>`.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        let vm = env.get_vm()?;
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;

        // Run the user future in one task, then await the join handle in another task so we can
        // turn panics into Promise rejections (mirrors napi-rs semantics).
        let join_handle = runtime()?.spawn(future);
        runtime()?.spawn(async move {
            match join_handle.await {
                Ok(outcome) => {
                    let _ = finish_promise(vm, deferred, outcome);
                }
                Err(join_err) => {
                    let _ = reject_join_error(vm, deferred, join_err);
                }
            }
        });

        Ok(promise)
    }

    /// Spawn a Rust future and expose its completion as an ArkTS `Promise<T>`.
    ///
    /// Unlike [`spawn_future`], the error type can be any `Display` value, which is
    /// converted into a rejection message via `to_string()`.
    pub fn spawn_future_result<'env, T, F, E>(
        env: &Env<'env>,
        future: F,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = std::result::Result<T, E>> + Send + 'static,
        E: std::fmt::Display + Send + 'static,
    {
        let vm = env.get_vm()?;
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;

        let join_handle = runtime()?.spawn(future);
        runtime()?.spawn(async move {
            match join_handle.await {
                Ok(outcome) => {
                    let _ = finish_promise_display(vm, deferred, outcome);
                }
                Err(join_err) => {
                    let _ = reject_join_error(vm, deferred, join_err);
                }
            }
        });

        Ok(promise)
    }

    fn finish_promise<T>(
        vm: crate::vm::AniVm,
        deferred: Deferred,
        outcome: crate::error::Result<T>,
    ) -> Result<()>
    where
        T: for<'vm> PromiseValue<'vm>,
    {
        let guard = vm.attach_current_thread_scoped()?;
        let env = guard.env();
        match outcome {
            Ok(value) => deferred.resolve_value(env, value),
            Err(error) => deferred.reject(env, error.reason),
        }
    }

    fn finish_promise_display<T, E>(
        vm: crate::vm::AniVm,
        deferred: Deferred,
        outcome: std::result::Result<T, E>,
    ) -> Result<()>
    where
        T: for<'vm> PromiseValue<'vm>,
        E: std::fmt::Display,
    {
        let guard = vm.attach_current_thread_scoped()?;
        let env = guard.env();
        match outcome {
            Ok(value) => deferred.resolve_value(env, value),
            Err(error) => deferred.reject(env, error.to_string()),
        }
    }

    fn reject_join_error(
        vm: crate::vm::AniVm,
        deferred: Deferred,
        join_err: ::tokio::task::JoinError,
    ) -> Result<()> {
        let message = if join_err.is_cancelled() {
            "async task cancelled".to_string()
        } else if join_err.is_panic() {
            match join_err.try_into_panic() {
                Ok(reason) => {
                    if let Some(string) = reason.downcast_ref::<String>() {
                        string.clone()
                    } else if let Some(string) = reason.downcast_ref::<&str>() {
                        (*string).to_string()
                    } else {
                        "panic in async function".to_string()
                    }
                }
                Err(_) => "panic in async function".to_string(),
            }
        } else {
            "async task failed".to_string()
        };

        let guard = vm.attach_current_thread_scoped()?;
        let env = guard.env();
        deferred.reject(env, message)
    }
}

#[cfg(not(feature = "tokio_rt"))]
mod imp {
    use std::future::Future;

    use crate::conversions::{PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::Result;

    const MISSING_TOKIO_RT: &str =
        "async bindings require enabling `ani` feature `tokio_rt` (ani = { ..., features = [\"tokio_rt\"] })";

    /// Stub implementation that rejects immediately when `tokio_rt` is not enabled.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, _future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }

    /// Stub implementation that rejects immediately when `tokio_rt` is not enabled.
    pub fn spawn_future_result<'env, T, F, E>(
        env: &Env<'env>,
        _future: F,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = std::result::Result<T, E>> + Send + 'static,
        E: std::fmt::Display + Send + 'static,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }
}

#[cfg(feature = "tokio_rt")]
pub use imp::runtime;
pub use imp::{spawn_future, spawn_future_result};

