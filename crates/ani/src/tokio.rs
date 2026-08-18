//! Tokio integration for ANI Promise-based APIs.
//!
//! Combined `async-runtime` + `tokio_rt` routing matches napi-rs:
//! - Generated `#[ani(async)]` futures, [`spawn_future`], and
//!   [`block_on_future_result`] follow the selected [`AsyncRuntime`].
//! - The established Tokio compatibility helpers ([`spawn`], [`block_on`],
//!   [`spawn_blocking`], [`within_runtime_if_available`]) stay Tokio-backed
//!   whenever `tokio_rt` is enabled, so Cargo feature unification cannot
//!   silently change their signatures or routing.
//! - Selecting a custom backend never constructs the built-in Tokio runtime;
//!   the first compatibility-helper call constructs it lazily.
//!
//! The built-in [`TokioAsyncRuntime`] is only the default backend. It runs
//! `RuntimeTask` carriers on a dedicated `LocalSet` so ANI `Env` values can
//! be rebuilt on one thread. That backend is independent of the helper
//! runtime used by [`spawn`] / [`block_on`] / [`spawn_blocking`].

#[cfg(feature = "tokio_rt")]
use crate::async_runtime::AsyncRuntime;

#[cfg(feature = "tokio_rt")]
mod imp {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, LazyLock, Mutex, OnceLock, PoisonError, RwLock};

    use tokio::runtime::Runtime;

    use crate::async_runtime::{AsyncRuntime, AsyncRuntimeRejection, RuntimeTask};
    use crate::conversions::{PromiseRaw, PromiseValue};
    use crate::env::Env;
    use crate::error::{AniErrorPayload, Error, Result, Status};

    static LOCAL_WORKER: Mutex<Option<LocalWorker>> = Mutex::new(None);
    static RT_CONSTRUCTED: AtomicBool = AtomicBool::new(false);
    static RT: LazyLock<RwLock<Option<Runtime>>> = LazyLock::new(|| {
        RT_CONSTRUCTED.store(true, Ordering::SeqCst);
        RwLock::new(Some(create_runtime()))
    });
    static USER_DEFINED_RT: OnceLock<RwLock<Option<Runtime>>> = OnceLock::new();
    static IS_USER_DEFINED_RT: OnceLock<bool> = OnceLock::new();

    type LocalJob = Box<dyn FnOnce() + Send + 'static>;
    type BlockingWork = Box<dyn FnOnce() + Send + 'static>;

    struct LocalWorker {
        tx: ::tokio::sync::mpsc::UnboundedSender<LocalJob>,
        join: std::thread::JoinHandle<()>,
    }

    fn create_runtime() -> Runtime {
        if IS_USER_DEFINED_RT.get().copied().unwrap_or(false)
            && let Some(user_defined_rt) = USER_DEFINED_RT
                .get()
                .and_then(|rt| rt.write().ok().and_then(|mut rt| rt.take()))
        {
            return user_defined_rt;
        }

        ::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("ani-tokio")
            .build()
            .expect("Create tokio runtime failed")
    }

    fn build_blocking_runtime() -> Result<Runtime> {
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
        let mut worker = LOCAL_WORKER.lock().unwrap_or_else(PoisonError::into_inner);
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

    // SAFETY: `shutdown` closes the local carrier queue, drops every LocalSet
    // future on its owning thread, and joins that thread before returning. The
    // Tokio compatibility-helper runtime is owned by the helper slot and is
    // drained separately by `shutdown_async_runtime`.
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
            work: BlockingWork,
        ) -> std::result::Result<(), AsyncRuntimeRejection<BlockingWork>> {
            let holder = Arc::new(Mutex::new(Some(work)));
            let scheduled = Arc::clone(&holder);
            match crate::scheduler::shared().schedule(move || {
                if let Some(work) = scheduled.lock().ok().and_then(|mut work| work.take()) {
                    work();
                }
            }) {
                Ok(()) => Ok(()),
                Err(error) => {
                    let work = holder
                        .lock()
                        .ok()
                        .and_then(|mut work| work.take())
                        .expect("failed blocking submission retained work ownership");
                    Err(AsyncRuntimeRejection::new(work, error))
                }
            }
        }

        fn start(&self) -> Result<()> {
            submit_local_job(Box::new(|| {}))
        }

        fn shutdown(&self) -> Result<()> {
            shutdown_local_worker();
            Ok(())
        }
    }

    fn shutdown_local_worker() {
        let worker = LOCAL_WORKER
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(LocalWorker { tx, join }) = worker {
            drop(tx);
            if join.thread().id() != std::thread::current().id() {
                let _ = join.join();
            }
        }
    }

    /// Handle to the shared multi-thread Tokio runtime used by compatibility
    /// helpers.
    pub fn runtime() -> Result<::tokio::runtime::Handle> {
        RT.read()
            .ok()
            .and_then(|rt| rt.as_ref().map(|rt| rt.handle().clone()))
            .ok_or_else(|| {
                Error::new(
                    Status::GenericFailure,
                    "Access tokio runtime failed in runtime",
                )
            })
    }

    /// Create a custom Tokio runtime used by ani-rs compatibility helpers.
    ///
    /// You can control the Tokio runtime configuration yourself. Call this
    /// from `#[ani(init)]` or a library constructor, before the first helper
    /// call constructs the default runtime.
    ///
    /// ### Example
    /// ```no_run
    /// use tokio::runtime::Builder;
    /// use ani::create_custom_tokio_runtime;
    ///
    /// fn init() {
    ///     let rt = Builder::new_multi_thread()
    ///         .enable_all()
    ///         .thread_stack_size(32 * 1024 * 1024)
    ///         .build()
    ///         .unwrap();
    ///     create_custom_tokio_runtime(rt);
    /// }
    /// ```
    pub fn create_custom_tokio_runtime(rt: Runtime) {
        USER_DEFINED_RT.get_or_init(move || RwLock::new(Some(rt)));
        IS_USER_DEFINED_RT.get_or_init(|| true);
    }

    /// Spawns a future onto the Tokio compatibility runtime.
    ///
    /// Depending on where you use it, you should await or abort the future in
    /// your drop function to avoid undefined behavior.
    pub fn spawn<F>(fut: F) -> ::tokio::task::JoinHandle<F::Output>
    where
        F: 'static + Send + Future,
        F::Output: Send + 'static,
    {
        RT.read()
            .ok()
            .and_then(|rt| rt.as_ref().map(|rt| rt.spawn(fut)))
            .expect("Access tokio runtime failed in spawn")
    }

    /// Runs a future to completion on the Tokio compatibility runtime.
    ///
    /// This is blocking. Only use it when it is absolutely necessary; prefer
    /// async functions elsewhere.
    pub fn block_on<F: Future>(fut: F) -> F::Output {
        RT.read()
            .ok()
            .and_then(|rt| rt.as_ref().map(|rt| rt.block_on(fut)))
            .expect("Access tokio runtime failed in block_on")
    }

    /// `spawn_blocking` on the Tokio compatibility runtime.
    pub fn spawn_blocking<F, R>(func: F) -> ::tokio::task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        RT.read()
            .ok()
            .and_then(|rt| rt.as_ref().map(|rt| rt.spawn_blocking(func)))
            .expect("Access tokio runtime failed in spawn_blocking")
    }

    /// Enter the Tokio compatibility runtime context and then call `f`.
    ///
    /// In combined `tokio_rt` + `async-runtime` builds this established helper
    /// stays Tokio-backed, so Cargo feature unification cannot silently change
    /// its routing.
    pub fn within_runtime_if_available<F: FnOnce() -> T, T>(f: F) -> T {
        RT.read()
            .ok()
            .and_then(|rt| {
                rt.as_ref().map(|rt| {
                    let rt_guard = rt.enter();
                    let ret = f();
                    drop(rt_guard);
                    ret
                })
            })
            .expect("Access tokio runtime failed in within_runtime_if_available")
    }

    /// Refill the built-in Tokio helper slot after a combined-build drain.
    /// Gated on `RT_CONSTRUCTED` so this never forces a first construction.
    pub(crate) fn refill_drained_tokio_runtime() {
        if RT_CONSTRUCTED.load(Ordering::SeqCst)
            && let Ok(mut rt) = RT.write()
            && rt.is_none()
        {
            *rt = Some(create_runtime());
        }
    }

    /// Drain a lazily-created helper runtime without forcing construction.
    pub(crate) fn drain_tokio_helper_runtime() {
        if let Some(rt) = RT_CONSTRUCTED
            .load(Ordering::SeqCst)
            .then(|| RT.write().ok().and_then(|mut rt| rt.take()))
            .flatten()
        {
            rt.shutdown_background();
        }
        if let Some(user_rt) = USER_DEFINED_RT
            .get()
            .and_then(|rt| rt.write().ok().and_then(|mut rt| rt.take()))
        {
            user_rt.shutdown_background();
        }
    }

    /// Ensure the helper slot holds a live runtime. Used by
    /// [`crate::async_runtime::start_async_runtime`] on the built-in Tokio
    /// path, where constructing the helper runtime is expected.
    pub(crate) fn ensure_tokio_helper_runtime() {
        if let Ok(mut rt) = RT.write()
            && rt.is_none()
        {
            *rt = Some(create_runtime());
        }
    }

    /// Stops module-owned async workers and drains pending local tasks.
    #[doc(hidden)]
    pub fn shutdown_runtime() {
        shutdown_local_worker();
        drain_tokio_helper_runtime();
    }

    /// Spawn a future factory through the selected [`AsyncRuntime`].
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

    /// Spawn a future factory returning [`crate::error::Result`].
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

    /// Drive a current-thread future through the selected [`AsyncRuntime`].
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

    /// Spawn an already-built `Send` future through the selected runtime.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        spawn_future_factory(env, move || future)
    }

    /// Spawn an already-built `Send` result future through the selected runtime.
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
        crate::async_runtime::shutdown_async_runtime();
    }

    /// Enter the registered [`AsyncRuntime`] backend's context. When no backend
    /// is registered, or entering the context fails, the closure runs directly.
    pub fn within_runtime_if_available<F: FnOnce() -> T, T>(f: F) -> T {
        crate::async_runtime::within_runtime_if_available(f)
    }

    /// Spawn through the registered executor-independent runtime backend.
    pub fn spawn_future<'env, T, F>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = crate::error::Result<T>> + Send + 'static,
    {
        crate::async_runtime::spawn_future(env, future)
    }

    /// Spawn through the registered executor-independent runtime backend.
    pub fn spawn_future_result<'env, T, F, E>(
        env: &Env<'env>,
        future: F,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: Send + 'static + for<'vm> PromiseValue<'vm>,
        F: Future<Output = std::result::Result<T, E>> + Send + 'static,
        E: AniErrorPayload,
    {
        crate::async_runtime::spawn_future_result(env, future)
    }

    /// Spawn a future factory through the registered custom runtime.
    pub fn spawn_future_factory<'env, T, Build, F>(
        env: &Env<'env>,
        build: Build,
    ) -> Result<PromiseRaw<'env, T>>
    where
        T: for<'vm> PromiseValue<'vm>,
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = crate::error::Result<T>> + 'static,
    {
        crate::async_runtime::spawn_future_factory(env, build)
    }

    /// Spawn a result factory through the registered custom runtime.
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

    /// Block through the registered custom runtime backend.
    pub fn block_on_future_result<F, T, E>(future: F) -> Result<std::result::Result<T, E>>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        crate::async_runtime::block_on_future_result(future)
    }
}

/// Process-wide built-in Tokio backend used when no custom runtime is
/// registered. Constructing this value does not start Tokio worker threads.
#[cfg(feature = "tokio_rt")]
pub(crate) fn tokio_fallback_runtime() -> &'static dyn AsyncRuntime {
    static RUNTIME: TokioAsyncRuntime = TokioAsyncRuntime::new();
    &RUNTIME
}

#[cfg(feature = "tokio_rt")]
pub use imp::TokioAsyncRuntime;
#[cfg(feature = "tokio_rt")]
pub use imp::{block_on, create_custom_tokio_runtime, runtime, spawn, spawn_blocking};
pub use imp::{
    block_on_future_result, shutdown_runtime, spawn_future, spawn_future_factory,
    spawn_future_result, spawn_future_result_factory, within_runtime_if_available,
};
#[cfg(feature = "tokio_rt")]
pub(crate) use imp::{
    drain_tokio_helper_runtime, ensure_tokio_helper_runtime, refill_drained_tokio_runtime,
};

#[cfg(all(test, feature = "tokio_rt"))]
mod tests {
    use super::{block_on, spawn, spawn_blocking, within_runtime_if_available};
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn tokio_helpers_run_on_the_helper_runtime() {
        let (tx, rx) = mpsc::channel();
        spawn(async move {
            tx.send(7).expect("deliver helper spawn result");
        });
        assert_eq!(
            rx.recv_timeout(Duration::from_secs(5)).expect("spawn ran"),
            7
        );

        let value = block_on(async { 9 });
        assert_eq!(value, 9);

        let (tx, rx) = mpsc::channel();
        spawn_blocking(move || {
            tx.send(11).expect("deliver helper blocking result");
        });
        assert_eq!(
            rx.recv_timeout(Duration::from_secs(5))
                .expect("spawn_blocking ran"),
            11
        );

        assert!(within_runtime_if_available(|| {
            tokio::runtime::Handle::try_current().is_ok()
        }));
    }
}
