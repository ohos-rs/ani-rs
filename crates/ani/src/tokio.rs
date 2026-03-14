//! Tokio integration for ANI Promise-based APIs.
//!
//! This module provides a minimal bridge from Rust futures to ArkTS `Promise<T>`.

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

    runtime()?.spawn(async move {
        let outcome = future.await;
        let _ = finish_promise(vm, deferred, outcome);
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
