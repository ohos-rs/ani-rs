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
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};

    use crate::conversions::{PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{AniErrorPayload, Error, Result, Status};

    use crate::async_runtime::{
        AsyncRuntime, AsyncRuntimeRejection, RuntimeBlockingTask, RuntimeTask,
    };

    static TOKIO_RUNTIME: Mutex<Option<Arc<::tokio::runtime::Runtime>>> = Mutex::new(None);
    static LOCAL_WORKER: Mutex<Option<LocalWorker>> = Mutex::new(None);

    type LocalJob = Box<dyn FnOnce() + Send + 'static>;

    struct LocalWorker {
        tx: ::tokio::sync::mpsc::UnboundedSender<LocalJob>,
        join: std::thread::JoinHandle<()>,
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

    /// Built-in Tokio implementation of ani-rs' executor-independent runtime
    /// contract.
    ///
    /// The type is public so applications can wrap or compose it, but it is
    /// selected automatically only when no custom runtime was registered.
    #[derive(Debug, Default)]
    pub struct TokioAsyncRuntime;

    impl TokioAsyncRuntime {
        /// Creates an unstarted backend. [`AsyncRuntime::start`] is lazy and
        /// restartable.
        pub const fn new() -> Self {
            Self
        }
    }

    // SAFETY: shutdown_runtime closes the local carrier queue, drops every
    // LocalSet future on its owning thread, joins that thread, and then drains
    // the module-owned multi-thread Tokio runtime before returning.
    unsafe impl AsyncRuntime for TokioAsyncRuntime {
        fn spawn(
            &self,
            task: RuntimeTask,
        ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
            let holder = Arc::new(Mutex::new(Some(task)));
            let submitted = Arc::clone(&holder);
            match submit_local_job(Box::new(move || {
                let task = submitted.lock().ok().and_then(|mut task| task.take());
                if let Some(task) = task {
                    ::tokio::task::spawn_local(task.into_local_future());
                }
            })) {
                Ok(()) => Ok(()),
                Err(error) => {
                    let task = holder
                        .lock()
                        .ok()
                        .and_then(|mut task| task.take())
                        .expect("failed Tokio submission retained RuntimeTask ownership");
                    Err(AsyncRuntimeRejection::new(task, error))
                }
            }
        }

        fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
            build_blocking_runtime()?.block_on(future);
            Ok(())
        }

        fn spawn_blocking(
            &self,
            task: RuntimeBlockingTask,
        ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeBlockingTask>> {
            let holder = Arc::new(Mutex::new(Some(task)));
            let scheduled = Arc::clone(&holder);
            match crate::scheduler::shared().schedule(move || {
                if let Some(task) = scheduled.lock().ok().and_then(|mut task| task.take()) {
                    task.run();
                }
            }) {
                Ok(()) => Ok(()),
                Err(error) => {
                    let task = holder
                        .lock()
                        .ok()
                        .and_then(|mut task| task.take())
                        .expect("failed blocking submission retained RuntimeBlockingTask");
                    Err(AsyncRuntimeRejection::new(task, error))
                }
            }
        }

        fn start(&self) -> Result<()> {
            submit_local_job(Box::new(|| {}))
        }

        fn shutdown(&self) -> Result<()> {
            shutdown_runtime();
            Ok(())
        }
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
        E: AniErrorPayload,
    {
        crate::async_runtime::spawn_future_result_factory(env, build)
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
        crate::async_runtime::block_on_future_result(future)
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
        E: AniErrorPayload,
    {
        spawn_future_result_factory(env, move || future)
    }
}

#[cfg(not(feature = "tokio_rt"))]
mod imp {
    use std::future::Future;

    use crate::conversions::{PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{AniErrorPayload, Result};

    /// Shuts down the executor-independent runtime domain.
    #[doc(hidden)]
    pub fn shutdown_runtime() {
        let _ = crate::async_runtime::shutdown_runtime_domain();
    }

    /// Spawn through the registered executor-independent runtime backend.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, _future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        crate::async_runtime::spawn_future(env, _future)
    }

    /// Spawn through the registered executor-independent runtime backend.
    pub fn spawn_future_result<'env, T, F, E>(
        env: &Env<'env>,
        _future: F,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = std::result::Result<T, E>> + Send + 'static,
        E: AniErrorPayload,
    {
        crate::async_runtime::spawn_future_result(env, _future)
    }

    /// Spawn a future factory through the registered custom runtime.
    pub fn spawn_future_factory<'env, T, Build, F>(
        env: &Env<'env>,
        _build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = crate::error::Result<T>> + 'static,
    {
        crate::async_runtime::spawn_future_factory(env, _build)
    }

    /// Spawn a result factory through the registered custom runtime.
    pub fn spawn_future_result_factory<'env, T, Build, F, E>(
        env: &Env<'env>,
        _build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = std::result::Result<T, E>> + 'static,
        E: AniErrorPayload,
    {
        crate::async_runtime::spawn_future_result_factory(env, _build)
    }

    /// Block through the registered custom runtime backend.
    pub fn block_on_future_result<F, T, E>(_future: F) -> Result<std::result::Result<T, E>>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        crate::async_runtime::block_on_future_result(_future)
    }
}

#[cfg(feature = "tokio_rt")]
pub use imp::TokioAsyncRuntime;
#[cfg(feature = "tokio_rt")]
pub use imp::runtime;
pub use imp::{
    block_on_future_result, shutdown_runtime, spawn_future, spawn_future_factory,
    spawn_future_result, spawn_future_result_factory,
};
