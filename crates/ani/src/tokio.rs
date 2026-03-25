//! Tokio integration for ANI Promise-based APIs.
//!
//! This module provides two async execution modes:
//! - Promise helpers backed by a dedicated single-thread `LocalSet`, allowing
//!   async bindings to rebuild thread-affine ANI values on the runtime thread.
//! - Synchronous `block_on` helpers backed by a current-thread runtime for
//!   bindings such as constructors/getters/setters that must preserve a
//!   synchronous ArkTS shape.
//!
//! Enable `ani` feature `async` for the ergonomic napi-rs-style alias, or
//! enable the lower-level `tokio_rt` feature directly. Additional `tokio_*`
//! passthrough features mirror napi-rs naming.

#[cfg(feature = "tokio_rt")]
mod imp {
    use std::any::Any;
    use std::future::Future;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::pin::Pin;
    use std::sync::OnceLock;
    use std::task::{Context, Poll};

    use crate::conversions::{Deferred, PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{Error, Result, Status};

    static TOKIO_RUNTIME: OnceLock<::tokio::runtime::Runtime> = OnceLock::new();
    static LOCAL_WORKER: OnceLock<LocalWorker> = OnceLock::new();

    type LocalJob = Box<dyn FnOnce() + Send + 'static>;

    struct LocalWorker {
        tx: ::tokio::sync::mpsc::UnboundedSender<LocalJob>,
    }

    struct CatchUnwindFuture<F> {
        inner: F,
    }

    impl<F> CatchUnwindFuture<F> {
        fn new(inner: F) -> Self {
            Self { inner }
        }
    }

    impl<F> Future for CatchUnwindFuture<F>
    where
        F: Future,
    {
        type Output = std::result::Result<F::Output, Box<dyn Any + Send>>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let inner = unsafe { self.map_unchecked_mut(|this| &mut this.inner) };
            match catch_unwind(AssertUnwindSafe(|| inner.poll(cx))) {
                Ok(Poll::Ready(output)) => Poll::Ready(Ok(output)),
                Ok(Poll::Pending) => Poll::Pending,
                Err(panic) => Poll::Ready(Err(panic)),
            }
        }
    }

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

    fn build_blocking_runtime() -> Result<::tokio::runtime::Runtime> {
        ::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| {
                Error::new(
                    Status::GenericFailure,
                    format!("failed to create current-thread tokio runtime: {err}"),
                )
            })
    }

    fn build_local_worker() -> Result<LocalWorker> {
        let (tx, mut rx) = ::tokio::sync::mpsc::unbounded_channel::<LocalJob>();
        std::thread::Builder::new()
            .name("ani-tokio-local".to_string())
            .spawn(move || {
                let runtime = match ::tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .thread_name("ani-tokio-local")
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(err) => {
                        eprintln!("failed to create ani local tokio runtime: {err}");
                        return;
                    }
                };
                let local = ::tokio::task::LocalSet::new();
                local.block_on(&runtime, async move {
                    while let Some(job) = rx.recv().await {
                        job();
                    }
                });
            })
            .map_err(|err| {
                Error::new(
                    Status::GenericFailure,
                    format!("failed to spawn ani local tokio worker: {err}"),
                )
            })?;
        Ok(LocalWorker { tx })
    }

    fn local_worker() -> Result<&'static LocalWorker> {
        if let Some(worker) = LOCAL_WORKER.get() {
            return Ok(worker);
        }

        let worker = build_local_worker()?;
        let _ = LOCAL_WORKER.set(worker);
        Ok(LOCAL_WORKER
            .get()
            .expect("ani local worker must be initialized after set attempt"))
    }

    fn submit_local_job(job: LocalJob) -> Result<()> {
        local_worker()?
            .tx
            .send(job)
            .map_err(|_| Error::new(Status::GenericFailure, "ani local tokio worker stopped"))
    }

    fn panic_message(panic: Box<dyn Any + Send>) -> String {
        if let Some(string) = panic.downcast_ref::<String>() {
            string.clone()
        } else if let Some(string) = panic.downcast_ref::<&str>() {
            (*string).to_string()
        } else {
            "panic in async function".to_string()
        }
    }

    fn reject_panic_payload(
        vm: crate::vm::AniVm,
        deferred: Deferred,
        panic: Box<dyn Any + Send>,
    ) -> Result<()> {
        let guard = vm.attach_current_thread_scoped()?;
        let env = guard.env();
        deferred.reject(env, panic_message(panic))
    }

    /// Get the shared multi-thread Tokio runtime used by manual ANI helpers.
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

    /// Spawn a future factory on the dedicated local runtime thread and expose
    /// completion as an ArkTS `Promise<T>`.
    pub fn spawn_future_result_factory<'env, T, Build, F, E>(
        env: &Env<'env>,
        build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = std::result::Result<T, E>> + 'static,
        E: std::fmt::Display,
    {
        let vm = env.get_vm()?;
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;

        submit_local_job(Box::new(move || {
            match catch_unwind(AssertUnwindSafe(build)) {
                Ok(future) => {
                    ::tokio::task::spawn_local(async move {
                        match CatchUnwindFuture::new(future).await {
                            Ok(outcome) => {
                                let _ = finish_promise_display(vm, deferred, outcome);
                            }
                            Err(panic) => {
                                let _ = reject_panic_payload(vm, deferred, panic);
                            }
                        }
                    });
                }
                Err(panic) => {
                    let _ = reject_panic_payload(vm, deferred, panic);
                }
            }
        }))?;

        Ok(promise)
    }

    /// Spawn a future factory returning [`crate::error::Result`] on the local
    /// runtime thread and expose completion as an ArkTS `Promise<T>`.
    pub fn spawn_future_factory<'env, T, Build, F>(
        env: &Env<'env>,
        build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = crate::error::Result<T>> + 'static,
    {
        spawn_future_result_factory(env, build)
    }

    /// Run a future to completion on a temporary current-thread runtime.
    ///
    /// This keeps execution on the caller thread, which is required for
    /// async constructors/getters/setters that preserve synchronous ArkTS
    /// signatures while still allowing Rust async bodies.
    pub fn block_on_future_result<F, T, E>(future: F) -> Result<std::result::Result<T, E>>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        let runtime = build_blocking_runtime()?;
        match runtime.block_on(CatchUnwindFuture::new(future)) {
            Ok(outcome) => Ok(outcome),
            Err(panic) => Err(Error::new(Status::GenericFailure, panic_message(panic))),
        }
    }

    /// Backwards-compatible helper for callers that already built a `Send`
    /// future on the current thread.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        spawn_future_factory(env, move || future)
    }

    /// Backwards-compatible helper for callers that already built a `Send`
    /// future on the current thread.
    pub fn spawn_future_result<'env, T, F, E>(
        env: &Env<'env>,
        future: F,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = std::result::Result<T, E>> + Send + 'static,
        E: std::fmt::Display + Send + 'static,
    {
        spawn_future_result_factory(env, move || future)
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
}

#[cfg(not(feature = "tokio_rt"))]
mod imp {
    use std::future::Future;

    use crate::conversions::{PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{Error, Result, Status};

    const MISSING_TOKIO_RT: &str =
        "async bindings require enabling `ani` feature `async` (or `tokio_rt`)";

    pub fn spawn_future<'env, T, F>(env: &Env<'env>, _future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }

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

    pub fn spawn_future_factory<'env, T, Build, F>(
        env: &Env<'env>,
        _build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = crate::error::Result<T>> + 'static,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }

    pub fn spawn_future_result_factory<'env, T, Build, F, E>(
        env: &Env<'env>,
        _build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = std::result::Result<T, E>> + 'static,
        E: std::fmt::Display,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }

    pub fn block_on_future_result<F, T, E>(_future: F) -> Result<std::result::Result<T, E>>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        Err(Error::new(Status::GenericFailure, MISSING_TOKIO_RT))
    }
}

#[cfg(feature = "tokio_rt")]
pub use imp::runtime;
pub use imp::{
    block_on_future_result, spawn_future, spawn_future_factory, spawn_future_result,
    spawn_future_result_factory,
};
