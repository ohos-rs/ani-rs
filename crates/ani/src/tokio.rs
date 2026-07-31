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
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    use crate::conversions::{Deferred, PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{Error, Result, Status};

    static TOKIO_RUNTIME: Mutex<Option<Arc<::tokio::runtime::Runtime>>> = Mutex::new(None);
    static LOCAL_WORKER: Mutex<Option<LocalWorker>> = Mutex::new(None);

    type LocalJob = Box<dyn FnOnce() + Send + 'static>;

    struct LocalWorker {
        tx: ::tokio::sync::mpsc::UnboundedSender<LocalJob>,
        join: std::thread::JoinHandle<()>,
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
        let join = std::thread::Builder::new()
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
        Ok(LocalWorker { tx, join })
    }

    fn submit_local_job(job: LocalJob) -> Result<()> {
        let mut worker = LOCAL_WORKER.lock().map_err(|_| {
            Error::new(Status::GenericFailure, "ani local worker lock was poisoned")
        })?;
        if worker.is_none() {
            *worker = Some(build_local_worker()?);
        }
        worker
            .as_ref()
            .expect("ani local worker initialized above")
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
        deferred: Deferred<()>,
        panic: Box<dyn Any + Send>,
    ) -> Result<()> {
        let guard = vm.attach_current_thread_scoped()?;
        let env = guard.env();
        deferred.reject(env, panic_message(panic))
    }

    /// Get the shared multi-thread Tokio runtime used by manual ANI helpers.
    pub fn runtime() -> Result<Arc<::tokio::runtime::Runtime>> {
        let mut runtime = TOKIO_RUNTIME
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "tokio runtime lock was poisoned"))?;
        if runtime.is_none() {
            *runtime = Some(Arc::new(build_runtime()?));
        }
        Ok(Arc::clone(
            runtime.as_ref().expect("tokio runtime initialized above"),
        ))
    }

    /// Stops module-owned async workers and cancels pending local tasks.
    #[doc(hidden)]
    pub fn shutdown_runtime() {
        let worker = LOCAL_WORKER
            .lock()
            .ok()
            .and_then(|mut worker| worker.take());
        if let Some(LocalWorker { tx, join }) = worker {
            drop(tx);
            if join.thread().id() != std::thread::current().id() {
                let _ = join.join();
            }
        }

        let runtime = TOKIO_RUNTIME
            .lock()
            .ok()
            .and_then(|mut runtime| runtime.take());
        if let Some(runtime) = runtime
            && let Ok(runtime) = Arc::try_unwrap(runtime)
        {
            runtime.shutdown_timeout(std::time::Duration::from_secs(1));
        }
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
        let deferred = deferred.cast::<()>();

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
        deferred: Deferred<()>,
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

    /// No-op when Tokio integration is not compiled in.
    #[doc(hidden)]
    pub fn shutdown_runtime() {}

    /// Return a rejected Promise explaining that Tokio integration is disabled.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, _future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
        deferred.reject(env, MISSING_TOKIO_RT)?;
        Ok(promise)
    }

    /// Return a rejected Promise explaining that Tokio integration is disabled.
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

    /// Return a rejected Promise instead of starting a disabled future factory.
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

    /// Return a rejected Promise instead of starting a disabled result factory.
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

    /// Return an error because blocking async execution requires Tokio support.
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
    block_on_future_result, shutdown_runtime, spawn_future, spawn_future_factory,
    spawn_future_result, spawn_future_result_factory,
};
