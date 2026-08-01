//! Executor-independent asynchronous runtime domain.
//!
//! Generated bindings submit an opaque, `Send` [`RuntimeTask`] carrier.  The
//! selected backend opens that carrier on one of its execution threads and
//! polls the resulting thread-affine future there.  This extra factory step is
//! required by ANI: an [`crate::env::Env`] and local ANI references are not
//! `Send`, even when the Rust future using them is otherwise asynchronous.
//!
//! Applications may register a completely custom backend.  Tokio is merely
//! the default backend selected when the `tokio_rt` feature is enabled.

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use crate::conversions::{Deferred, PromiseRaw, PromiseValue};
use crate::env::Env;
use crate::error::{AniErrorPayload, DynAniError, Error, Result, Status};
use crate::scheduler::{RuntimeCancellable, RuntimeRegistration};

type LocalFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
type LocalFutureFactory = Box<dyn FnOnce() -> LocalFuture + Send + 'static>;
type RejectCallback = Box<dyn FnOnce(DynAniError) + Send + 'static>;

static LIVE_TASKS: AtomicUsize = AtomicUsize::new(0);
static PENDING_SETTLEMENTS: AtomicUsize = AtomicUsize::new(0);
static COMPLETED_TASKS: AtomicUsize = AtomicUsize::new(0);
static CANCELLED_TASKS: AtomicUsize = AtomicUsize::new(0);
static RUNTIME_GENERATION: AtomicU64 = AtomicU64::new(0);
static NEXT_CANCEL_TOKEN: AtomicU64 = AtomicU64::new(1);

fn cancel_bridge_registry() -> &'static Mutex<HashMap<u64, Weak<RuntimeTaskControlInner>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<u64, Weak<RuntimeTaskControlInner>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Why a runtime-owned task was cancelled before normal completion.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuntimeCancelReason {
    /// The module/runtime domain is shutting down.
    Shutdown,
    /// The backend dropped an accepted task without completing it.
    Dropped,
    /// The backend declined a task.
    BackendDeclined(String),
    /// A backend hook or Rust future panicked.
    Panic(String),
    /// An application-facing cancel handle requested cancellation.
    Explicit(String),
}

impl RuntimeCancelReason {
    fn message(&self) -> String {
        match self {
            Self::Shutdown => "asynchronous operation cancelled during runtime shutdown".into(),
            Self::Dropped => "asynchronous runtime dropped a pending operation".into(),
            Self::BackendDeclined(message) => {
                format!("asynchronous runtime declined the operation: {message}")
            }
            Self::Panic(message) => format!("panic in asynchronous operation: {message}"),
            Self::Explicit(message) => message.clone(),
        }
    }
}

type CancellationErrorFactory = dyn Fn(RuntimeCancelReason) -> DynAniError + Send + Sync + 'static;

fn cancellation_error_factory() -> &'static Arc<CancellationErrorFactory> {
    static FACTORY: OnceLock<Arc<CancellationErrorFactory>> = OnceLock::new();
    FACTORY.get_or_init(|| {
        Arc::new(|reason: RuntimeCancelReason| {
            Box::new(Error::new(Status::Cancelled, reason.message())) as DynAniError
        })
    })
}

/// Installs the process-wide factory used for framework-originated
/// cancellation errors.
///
/// Registration must happen before the first async task is created.  The
/// produced payload can materialize any application-defined ArkTS Error class.
pub fn register_cancellation_error_factory<F>(factory: F) -> Result<()>
where
    F: Fn(RuntimeCancelReason) -> DynAniError + Send + Sync + 'static,
{
    static CUSTOM_FACTORY: OnceLock<()> = OnceLock::new();
    if LIVE_TASKS.load(Ordering::Acquire) != 0 || CUSTOM_FACTORY.set(()).is_err() {
        return Err(Error::new(
            Status::AlreadyBound,
            "cancellation error factory is already frozen",
        ));
    }
    // The default accessor may already have initialized its OnceLock.  Keep a
    // separate override so merely reading metrics does not freeze selection.
    cancellation_factory_override()
        .set(Arc::new(factory))
        .map_err(|_| Error::new(Status::AlreadyBound, "cancellation error factory is set"))
}

fn cancellation_factory_override() -> &'static OnceLock<Arc<CancellationErrorFactory>> {
    static OVERRIDE: OnceLock<Arc<CancellationErrorFactory>> = OnceLock::new();
    &OVERRIDE
}

/// Materializes a framework cancellation through the registered application
/// factory. Promise, Task, TSFN and Stream cancellation all use this path.
pub fn runtime_cancellation_error(reason: RuntimeCancelReason) -> DynAniError {
    cancellation_factory_override()
        .get()
        .unwrap_or_else(|| cancellation_error_factory())(reason)
}

fn panic_message(panic: Box<dyn Any + Send>) -> String {
    if let Some(message) = panic.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        (*message).to_string()
    } else {
        "unknown panic payload".to_string()
    }
}

struct RuntimeTaskControlInner {
    terminal: AtomicBool,
    cancelled: AtomicBool,
    reject: Mutex<Option<RejectCallback>>,
    waker: Mutex<Option<Waker>>,
    registration: Mutex<Option<RuntimeRegistration>>,
    bridge_tokens: Mutex<Vec<u64>>,
}

impl RuntimeTaskControlInner {
    fn new(reject: RejectCallback) -> Arc<Self> {
        LIVE_TASKS.fetch_add(1, Ordering::AcqRel);
        Arc::new(Self {
            terminal: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
            reject: Mutex::new(Some(reject)),
            waker: Mutex::new(None),
            registration: Mutex::new(None),
            bridge_tokens: Mutex::new(Vec::new()),
        })
    }

    fn install_registration(&self, registration: RuntimeRegistration) {
        if self.terminal.load(Ordering::Acquire) {
            drop(registration);
        } else if let Ok(mut slot) = self.registration.lock() {
            *slot = Some(registration);
        }
    }

    fn complete(&self) -> bool {
        if self
            .terminal
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        if let Ok(mut reject) = self.reject.lock() {
            reject.take();
        }
        self.finish_terminal(false);
        true
    }

    fn cancel(&self, error: DynAniError) -> bool {
        if self
            .terminal
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        self.cancelled.store(true, Ordering::Release);
        let reject = self.reject.lock().ok().and_then(|mut reject| reject.take());
        if let Some(reject) = reject {
            reject(error);
        }
        if let Ok(mut waker) = self.waker.lock()
            && let Some(waker) = waker.take()
        {
            waker.wake();
        }
        self.finish_terminal(true);
        true
    }

    fn finish_terminal(&self, cancelled: bool) {
        if let Ok(mut registration) = self.registration.lock() {
            registration.take();
        }
        if let Ok(mut tokens) = self.bridge_tokens.lock()
            && let Ok(mut registry) = cancel_bridge_registry().lock()
        {
            for token in tokens.drain(..) {
                registry.remove(&token);
            }
        }
        LIVE_TASKS.fetch_sub(1, Ordering::AcqRel);
        if cancelled {
            CANCELLED_TASKS.fetch_add(1, Ordering::AcqRel);
        } else {
            COMPLETED_TASKS.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn register_waker(&self, waker: &Waker) {
        if let Ok(mut slot) = self.waker.lock()
            && slot
                .as_ref()
                .is_none_or(|current| !current.will_wake(waker))
        {
            *slot = Some(waker.clone());
        }
    }
}

impl RuntimeCancellable for RuntimeTaskControlInner {
    fn cancel_for_runtime_shutdown(&self) {
        self.cancel(runtime_cancellation_error(RuntimeCancelReason::Shutdown));
    }
}

/// Cloneable application/runtime handle for one submitted task.
#[derive(Clone)]
pub struct RuntimeTaskHandle {
    control: Arc<RuntimeTaskControlInner>,
}

impl std::fmt::Debug for RuntimeTaskHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeTaskHandle")
            .field("finished", &self.is_finished())
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

impl RuntimeTaskHandle {
    /// Cancels with an application-defined payload/materializer.
    pub fn cancel_with(&self, error: DynAniError) -> bool {
        self.control.cancel(error)
    }

    /// Cancels using the registered cancellation-error factory.
    pub fn cancel(&self, reason: RuntimeCancelReason) -> bool {
        self.cancel_with(runtime_cancellation_error(reason))
    }

    /// Whether this task resolved, rejected, or was cancelled.
    pub fn is_finished(&self) -> bool {
        self.control.terminal.load(Ordering::Acquire)
    }

    /// Whether cancellation won the exactly-once terminal transition.
    pub fn is_cancelled(&self) -> bool {
        self.control.cancelled.load(Ordering::Acquire)
    }

    /// Registers this task with the generated ETS cancellation bridge and
    /// returns its opaque token.
    pub fn bridge_token(&self) -> Result<i64> {
        if self.is_finished() {
            return Err(Error::new(
                Status::Closing,
                "runtime task is already finished",
            ));
        }
        let token = NEXT_CANCEL_TOKEN.fetch_add(1, Ordering::AcqRel);
        let signed = i64::try_from(token)
            .map_err(|_| Error::new(Status::OutOfRange, "cancel token space exhausted"))?;
        cancel_bridge_registry()
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "cancel registry lock poisoned"))?
            .insert(token, Arc::downgrade(&self.control));
        self.control
            .bridge_tokens
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "cancel token lock poisoned"))?
            .push(token);
        Ok(signed)
    }
}

/// Number of live task tokens exposed to the ETS cancellation bridge.
pub fn live_runtime_cancel_token_count() -> usize {
    cancel_bridge_registry()
        .lock()
        .map(|mut registry| {
            registry.retain(|_, control| control.strong_count() > 0);
            registry.len()
        })
        .unwrap_or(usize::MAX)
}

/// Native target called on the AbortSignal/CancelHandle owning ArkTS thread.
/// The exact `reason` object becomes the Promise rejection through its custom
/// materializer; the worker never reads a thread-affine AbortSignal.
#[doc(hidden)]
pub unsafe extern "C" fn cancel_runtime_task_from_ets(
    env: *mut crate::sys::ani_env,
    token: i64,
    reason: crate::sys::ani_ref,
) {
    if env.is_null() || token <= 0 {
        return;
    }
    let control = cancel_bridge_registry()
        .lock()
        .ok()
        .and_then(|mut registry| registry.remove(&(token as u64)))
        .and_then(|control| control.upgrade());
    let Some(control) = control else { return };
    let env = unsafe { Env::from_raw_unchecked(env) };
    let payload = if reason.is_null() {
        runtime_cancellation_error(RuntimeCancelReason::Explicit(
            "ArkTS requested cancellation".into(),
        ))
    } else {
        let reason = unsafe { crate::types::AniRef::from_raw(reason) };
        match crate::error::PreservedArktsError::new(&env, &reason) {
            Ok(reason) => Box::new(reason) as DynAniError,
            Err(error) => Box::new(error) as DynAniError,
        }
    };
    control.cancel(payload);
}

/// A `Send` carrier which creates its potentially `!Send` future only on the
/// backend's selected execution thread.
pub struct RuntimeTask {
    factory: Option<LocalFutureFactory>,
    control: Arc<RuntimeTaskControlInner>,
}

/// Opaque carrier for CPU/blocking work owned by the same runtime domain.
pub struct RuntimeBlockingTask {
    work: Option<Box<dyn FnOnce() + Send + 'static>>,
    control: Arc<RuntimeTaskControlInner>,
}

impl std::fmt::Debug for RuntimeBlockingTask {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeBlockingTask")
            .field("started", &self.work.is_none())
            .field("handle", &self.handle())
            .finish()
    }
}

impl RuntimeBlockingTask {
    /// Creates blocking work with exactly-once cancellation rejection.
    pub fn new<Work, Reject>(work: Work, reject: Reject) -> (Self, RuntimeTaskHandle)
    where
        Work: FnOnce() + Send + 'static,
        Reject: FnOnce(DynAniError) + Send + 'static,
    {
        let control = RuntimeTaskControlInner::new(Box::new(reject));
        let handle = RuntimeTaskHandle {
            control: Arc::clone(&control),
        };
        (
            Self {
                work: Some(Box::new(work)),
                control,
            },
            handle,
        )
    }

    /// Returns a cancellation/status handle.
    pub fn handle(&self) -> RuntimeTaskHandle {
        RuntimeTaskHandle {
            control: Arc::clone(&self.control),
        }
    }

    /// Executes the work on a backend-owned blocking thread.
    pub fn run(mut self) {
        if self.control.terminal.load(Ordering::Acquire) {
            self.work.take();
            return;
        }
        let work = self
            .work
            .take()
            .expect("RuntimeBlockingTask run more than once");
        match catch_unwind(AssertUnwindSafe(work)) {
            Ok(()) => {
                self.control.complete();
            }
            Err(panic) => {
                self.control
                    .cancel(runtime_cancellation_error(RuntimeCancelReason::Panic(
                        panic_message(panic),
                    )));
            }
        }
    }

    /// Rejects work declined by a backend.
    pub fn reject_with(mut self, error: DynAniError) {
        self.work.take();
        self.control.cancel(error);
    }
}

impl Drop for RuntimeBlockingTask {
    fn drop(&mut self) {
        if self.work.is_some() {
            self.control
                .cancel(runtime_cancellation_error(RuntimeCancelReason::Dropped));
        }
    }
}

impl std::fmt::Debug for RuntimeTask {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeTask")
            .field("materialized", &self.factory.is_none())
            .field("handle", &self.handle())
            .finish()
    }
}

impl RuntimeTask {
    /// Creates an opaque task and its cancellation handle.
    pub fn new<Build, F, Reject>(build: Build, reject: Reject) -> (Self, RuntimeTaskHandle)
    where
        Build: FnOnce() -> F + Send + 'static,
        F: Future<Output = ()> + 'static,
        Reject: FnOnce(DynAniError) + Send + 'static,
    {
        let control = RuntimeTaskControlInner::new(Box::new(reject));
        let handle = RuntimeTaskHandle {
            control: Arc::clone(&control),
        };
        (
            Self {
                factory: Some(Box::new(move || Box::pin(build()))),
                control,
            },
            handle,
        )
    }

    /// Returns a cancellation/status handle.
    pub fn handle(&self) -> RuntimeTaskHandle {
        RuntimeTaskHandle {
            control: Arc::clone(&self.control),
        }
    }

    /// Rejects a declined carrier with the backend-provided custom payload.
    pub fn reject_with(mut self, error: DynAniError) {
        self.factory.take();
        self.control.cancel(error);
    }

    /// Opens the carrier on the backend execution thread.
    ///
    /// The returned future is deliberately `!Send`.  A backend must poll and
    /// drop it on the same thread on which this method was called.
    pub fn into_local_future(mut self) -> RuntimeLocalTask {
        let factory = self
            .factory
            .take()
            .expect("RuntimeTask opened more than once");
        let inner = match catch_unwind(AssertUnwindSafe(factory)) {
            Ok(future) => Some(future),
            Err(panic) => {
                self.control
                    .cancel(runtime_cancellation_error(RuntimeCancelReason::Panic(
                        panic_message(panic),
                    )));
                None
            }
        };
        RuntimeLocalTask {
            inner,
            control: Arc::clone(&self.control),
        }
    }
}

impl Drop for RuntimeTask {
    fn drop(&mut self) {
        if self.factory.is_some() {
            self.control
                .cancel(runtime_cancellation_error(RuntimeCancelReason::Dropped));
        }
    }
}

/// The thread-affine future materialized from [`RuntimeTask`].
pub struct RuntimeLocalTask {
    inner: Option<LocalFuture>,
    control: Arc<RuntimeTaskControlInner>,
}

impl Future for RuntimeLocalTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.control.terminal.load(Ordering::Acquire) {
            self.inner.take();
            return Poll::Ready(());
        }
        self.control.register_waker(context.waker());
        let Some(inner) = self.inner.as_mut() else {
            return Poll::Ready(());
        };
        match catch_unwind(AssertUnwindSafe(|| inner.as_mut().poll(context))) {
            Ok(Poll::Pending) => Poll::Pending,
            Ok(Poll::Ready(())) => {
                self.inner.take();
                self.control.complete();
                Poll::Ready(())
            }
            Err(panic) => {
                self.inner.take();
                self.control
                    .cancel(runtime_cancellation_error(RuntimeCancelReason::Panic(
                        panic_message(panic),
                    )));
                Poll::Ready(())
            }
        }
    }
}

impl Drop for RuntimeLocalTask {
    fn drop(&mut self) {
        if self.inner.is_some() {
            self.control
                .cancel(runtime_cancellation_error(RuntimeCancelReason::Dropped));
        }
    }
}

/// Marker returned by [`AsyncRuntime::enter`].
pub trait AsyncRuntimeGuard {}

impl AsyncRuntimeGuard for () {}

/// Ownership-preserving rejection from an async runtime hook.
#[derive(Debug)]
pub struct AsyncRuntimeRejection<T> {
    /// Work returned untouched so ani-rs can reject/cancel it exactly once.
    pub work: T,
    /// Extensible error payload explaining why the backend declined the work.
    pub error: DynAniError,
}

impl<T> AsyncRuntimeRejection<T> {
    /// Creates a rejection that returns ownership of `work`.
    pub fn new(work: T, error: impl AniErrorPayload) -> Self {
        Self {
            work,
            error: Box::new(error),
        }
    }
}

/// Fully replaceable async execution backend.
///
/// # Safety
///
/// `shutdown` is a native-image safety boundary.  Before it returns, every
/// backend thread, task, closure, waker, and blocking job that could execute
/// ani-rs/addon code must have quiesced.  A backend which cannot satisfy that
/// contract must abort the process instead of returning.
pub unsafe trait AsyncRuntime: Send + Sync + 'static {
    /// Accepts an asynchronous task carrier.
    fn spawn(
        &self,
        task: RuntimeTask,
    ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>>;

    /// Drives one borrowed, current-thread future synchronously.
    fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()>;

    /// Accepts CPU/blocking work. Backends that do not support a blocking lane
    /// return the carrier untouched.
    fn spawn_blocking(
        &self,
        task: RuntimeBlockingTask,
    ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeBlockingTask>> {
        Err(AsyncRuntimeRejection::new(
            task,
            Error::new(
                Status::NotFound,
                "selected AsyncRuntime does not implement spawn_blocking",
            ),
        ))
    }

    /// Optionally enters backend context on the current thread.
    fn enter(&self) -> Result<Box<dyn AsyncRuntimeGuard>> {
        Ok(Box::new(()))
    }

    /// Idempotently starts a fresh backend generation.
    fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Quiesces the complete backend.  See the safety contract above.
    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LifecyclePhase {
    Idle,
    Starting,
    Started,
    Stopping,
}

struct RuntimeRegistryState {
    selected: Option<Arc<dyn AsyncRuntime>>,
    selection_frozen: bool,
    phase: LifecyclePhase,
}

struct RuntimeRegistry {
    state: Mutex<RuntimeRegistryState>,
    lifecycle: Mutex<()>,
    changed: Condvar,
}

impl RuntimeRegistry {
    fn new() -> Self {
        Self {
            state: Mutex::new(RuntimeRegistryState {
                selected: None,
                selection_frozen: false,
                phase: LifecyclePhase::Idle,
            }),
            lifecycle: Mutex::new(()),
            changed: Condvar::new(),
        }
    }

    fn select_default(state: &mut RuntimeRegistryState) -> Result<()> {
        if state.selected.is_none() {
            #[cfg(feature = "tokio_rt")]
            {
                state.selected = Some(Arc::new(crate::tokio::TokioAsyncRuntime::new()));
            }
            #[cfg(not(feature = "tokio_rt"))]
            {
                return Err(Error::new(
                    Status::NotFound,
                    "no AsyncRuntime is registered; register one or enable `tokio_rt`",
                ));
            }
        }
        state.selection_frozen = true;
        Ok(())
    }

    fn activate(&self) -> Result<Arc<dyn AsyncRuntime>> {
        let _lifecycle = self.lifecycle.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async runtime lifecycle lock poisoned",
            )
        })?;
        let runtime = {
            let mut state = self.state.lock().map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "async runtime registry lock poisoned",
                )
            })?;
            Self::select_default(&mut state)?;
            if state.phase == LifecyclePhase::Started {
                return Ok(Arc::clone(state.selected.as_ref().expect("selected above")));
            }
            if state.phase != LifecyclePhase::Idle {
                return Err(Error::new(
                    Status::Closing,
                    "async runtime is changing state",
                ));
            }
            state.phase = LifecyclePhase::Starting;
            Arc::clone(state.selected.as_ref().expect("selected above"))
        };
        let started = catch_unwind(AssertUnwindSafe(|| runtime.start())).map_err(|panic| {
            Error::new(
                Status::GenericFailure,
                format!("AsyncRuntime::start panicked: {}", panic_message(panic)),
            )
        })?;
        let mut state = self.state.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async runtime registry lock poisoned",
            )
        })?;
        match started {
            Ok(()) => {
                state.phase = LifecyclePhase::Started;
                RUNTIME_GENERATION.fetch_add(1, Ordering::AcqRel);
                self.changed.notify_all();
                Ok(runtime)
            }
            Err(error) => {
                state.phase = LifecyclePhase::Idle;
                self.changed.notify_all();
                Err(error)
            }
        }
    }

    fn shutdown(&self) -> Result<()> {
        let _lifecycle = self.lifecycle.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async runtime lifecycle lock poisoned",
            )
        })?;
        let runtime = {
            let mut state = self.state.lock().map_err(|_| {
                Error::new(
                    Status::GenericFailure,
                    "async runtime registry lock poisoned",
                )
            })?;
            if state.phase == LifecyclePhase::Idle {
                return Ok(());
            }
            if state.phase != LifecyclePhase::Started {
                return Err(Error::new(
                    Status::Closing,
                    "async runtime is changing state",
                ));
            }
            state.phase = LifecyclePhase::Stopping;
            Arc::clone(
                state
                    .selected
                    .as_ref()
                    .expect("started runtime is selected"),
            )
        };
        let result = match catch_unwind(AssertUnwindSafe(|| runtime.shutdown())) {
            Ok(result) => result,
            Err(panic) => {
                eprintln!(
                    "AsyncRuntime::shutdown panicked after selection: {}; aborting because native-image quiescence cannot be proven",
                    panic_message(panic)
                );
                std::process::abort();
            }
        };
        let mut state = self.state.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async runtime registry lock poisoned",
            )
        })?;
        // Even an error-returning backend is required by the unsafe trait
        // contract to be quiescent, so restart remains well-defined.
        state.phase = LifecyclePhase::Idle;
        self.changed.notify_all();
        result
    }

    fn phase(&self) -> LifecyclePhase {
        self.state
            .lock()
            .map(|state| state.phase)
            .unwrap_or(LifecyclePhase::Stopping)
    }
}

fn registry() -> &'static RuntimeRegistry {
    static REGISTRY: OnceLock<RuntimeRegistry> = OnceLock::new();
    REGISTRY.get_or_init(RuntimeRegistry::new)
}

/// Registers a complete application-provided runtime before first use.
pub fn try_register_async_runtime<R>(runtime: R) -> std::result::Result<(), R>
where
    R: AsyncRuntime,
{
    let Ok(mut state) = registry().state.lock() else {
        return Err(runtime);
    };
    if state.selection_frozen || state.selected.is_some() || state.phase != LifecyclePhase::Idle {
        return Err(runtime);
    }
    state.selected = Some(Arc::new(runtime));
    Ok(())
}

/// Registers a complete custom runtime or records an actionable error.
pub fn register_async_runtime<R>(runtime: R) -> Result<()>
where
    R: AsyncRuntime,
{
    match try_register_async_runtime(runtime) {
        Ok(()) => Ok(()),
        Err(runtime) => {
            match catch_unwind(AssertUnwindSafe(|| runtime.shutdown())) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!(
                        "unselected AsyncRuntime panicked while being retired; aborting because quiescence cannot be proven"
                    );
                    std::process::abort();
                }
            }
            Err(Error::new(
                Status::AlreadyBound,
                "AsyncRuntime must be registered exactly once before first use",
            ))
        }
    }
}

/// Starts the selected backend without submitting work.
pub fn activate_async_runtime() -> Result<()> {
    registry().activate().map(|_| ())
}

/// Submits one carrier to the selected runtime.
pub fn spawn_runtime_task(task: RuntimeTask) -> Result<RuntimeTaskHandle> {
    let handle = task.handle();
    let registration = crate::scheduler::shared().register_cancellable(&task.control)?;
    task.control.install_registration(registration);
    let runtime = match registry().activate() {
        Ok(runtime) => runtime,
        Err(error) => {
            task.reject_with(Box::new(error));
            return Ok(handle);
        }
    };
    match catch_unwind(AssertUnwindSafe(|| runtime.spawn(task))) {
        Ok(Ok(())) => Ok(handle),
        Ok(Err(rejection)) => {
            rejection.work.reject_with(rejection.error);
            Ok(handle)
        }
        Err(panic) => {
            // The task argument is dropped during unwinding, which performs the
            // exactly-once rejection.  Report the backend contract violation.
            Err(Error::new(
                Status::GenericFailure,
                format!("AsyncRuntime::spawn panicked: {}", panic_message(panic)),
            ))
        }
    }
}

/// Submits blocking work to the selected runtime backend.
pub fn spawn_runtime_blocking_task(task: RuntimeBlockingTask) -> Result<RuntimeTaskHandle> {
    let handle = task.handle();
    let registration = crate::scheduler::shared().register_cancellable(&task.control)?;
    task.control.install_registration(registration);
    let runtime = match registry().activate() {
        Ok(runtime) => runtime,
        Err(error) => {
            task.reject_with(Box::new(error));
            return Ok(handle);
        }
    };
    match catch_unwind(AssertUnwindSafe(|| runtime.spawn_blocking(task))) {
        Ok(Ok(())) => Ok(handle),
        Ok(Err(rejection)) => {
            rejection.work.reject_with(rejection.error);
            Ok(handle)
        }
        Err(panic) => Err(Error::new(
            Status::GenericFailure,
            format!(
                "AsyncRuntime::spawn_blocking panicked: {}",
                panic_message(panic)
            ),
        )),
    }
}

struct PromiseSettlement {
    deferred: Mutex<Option<Deferred<()>>>,
    vm: Arc<crate::vm::AniVm>,
}

impl PromiseSettlement {
    fn new(deferred: Deferred<()>, vm: crate::vm::AniVm) -> Arc<Self> {
        PENDING_SETTLEMENTS.fetch_add(1, Ordering::AcqRel);
        Arc::new(Self {
            deferred: Mutex::new(Some(deferred)),
            vm: Arc::new(vm),
        })
    }

    fn take(&self) -> Option<Deferred<()>> {
        let deferred = self.deferred.lock().ok().and_then(|mut value| value.take());
        if deferred.is_some() {
            PENDING_SETTLEMENTS.fetch_sub(1, Ordering::AcqRel);
        }
        deferred
    }

    fn reject(&self, error: DynAniError) {
        let Some(deferred) = self.take() else { return };
        let _ = self
            .vm
            .with_attached(|env| deferred.reject_with_payload(env, error));
    }
}

/// Executes an async factory on the selected runtime and returns its Promise.
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
    spawn_future_result_factory_with_handle(env, build).map(|(promise, _handle)| promise)
}

/// Executes an async factory and also returns a handle suitable for explicit
/// cancellation or [`RuntimeTaskHandle::bridge_token`].
pub fn spawn_future_result_factory_with_handle<'env, T, Build, F, E>(
    env: &Env<'env>,
    build: Build,
) -> Result<(PromiseRaw<'env, T>, RuntimeTaskHandle)>
where
    T: for<'vm> PromiseValue<'vm>,
    Build: FnOnce() -> F + Send + 'static,
    F: Future<Output = std::result::Result<T, E>> + 'static,
    E: AniErrorPayload,
{
    let (deferred, promise) = PromiseRaw::<T>::deferred(env)?;
    let settlement = PromiseSettlement::new(deferred.cast::<()>(), env.get_vm()?);
    let reject_settlement = Arc::clone(&settlement);
    let (task, _handle) = RuntimeTask::new(
        move || async move {
            let outcome = build().await;
            let Some(deferred) = settlement.take() else {
                return;
            };
            let _ = settlement.vm.with_attached(|env| match outcome {
                Ok(value) => deferred.resolve_value(env, value),
                Err(error) => deferred.reject_with_payload(env, error),
            });
        },
        move |error| reject_settlement.reject(error),
    );
    // Submission failures are converted into Promise rejection by the task
    // state machine; only infrastructure failures creating the Promise escape.
    let handle = spawn_runtime_task(task)?;
    Ok((promise, handle))
}

/// Convenience form for ani-rs' built-in [`crate::error::Result`].
pub fn spawn_future_factory<'env, T, Build, F>(
    env: &Env<'env>,
    build: Build,
) -> Result<PromiseRaw<'env, T>>
where
    T: for<'vm> PromiseValue<'vm>,
    Build: FnOnce() -> F + Send + 'static,
    F: Future<Output = Result<T>> + 'static,
{
    spawn_future_result_factory(env, build)
}

/// Convenience helper for an already-built `Send` future.
pub fn spawn_future_result<'env, T, F, E>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
where
    T: Send + 'static + for<'vm> PromiseValue<'vm>,
    F: Future<Output = std::result::Result<T, E>> + Send + 'static,
    E: AniErrorPayload,
{
    spawn_future_result_factory(env, move || future)
}

/// Convenience helper for an already-built ani-rs result future.
pub fn spawn_future<'env, T, F>(env: &Env<'env>, future: F) -> Result<PromiseRaw<'env, T>>
where
    T: Send + 'static + for<'vm> PromiseValue<'vm>,
    F: Future<Output = Result<T>> + Send + 'static,
{
    spawn_future_factory(env, move || future)
}

/// Drives a current-thread future through the selected backend.
pub fn block_on_future_result<F, T, E>(future: F) -> Result<std::result::Result<T, E>>
where
    F: Future<Output = std::result::Result<T, E>>,
{
    let mut outcome = None;
    let mut driver = Box::pin(async {
        outcome = Some(future.await);
    });
    let runtime = registry().activate()?;
    match catch_unwind(AssertUnwindSafe(|| runtime.block_on(driver.as_mut()))) {
        Ok(result) => result?,
        Err(panic) => {
            drop(driver);
            return Err(Error::new(
                Status::GenericFailure,
                format!("AsyncRuntime::block_on panicked: {}", panic_message(panic)),
            ));
        }
    }
    drop(driver);
    outcome.ok_or_else(|| {
        Error::new(
            Status::GenericFailure,
            "AsyncRuntime::block_on returned before the future completed",
        )
    })
}

/// Async-runtime counters included in leak and release gates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AsyncRuntimeMetrics {
    /// Current started backend generation.
    pub generation: u64,
    /// Tasks which have not reached an exactly-once terminal state.
    pub live_tasks: usize,
    /// Promise resolvers awaiting resolve/reject.
    pub pending_settlements: usize,
    /// Tasks completed normally across generations.
    pub completed: usize,
    /// Tasks cancelled/rejected across generations.
    pub cancelled: usize,
    /// Whether the backend is started.
    pub started: bool,
    /// Whether the backend is starting or stopping.
    pub changing_state: bool,
}

/// Captures executor-independent runtime counters.
pub fn async_runtime_metrics() -> AsyncRuntimeMetrics {
    let phase = registry().phase();
    AsyncRuntimeMetrics {
        generation: RUNTIME_GENERATION.load(Ordering::Acquire),
        live_tasks: LIVE_TASKS.load(Ordering::Acquire),
        pending_settlements: PENDING_SETTLEMENTS.load(Ordering::Acquire),
        completed: COMPLETED_TASKS.load(Ordering::Acquire),
        cancelled: CANCELLED_TASKS.load(Ordering::Acquire),
        started: phase == LifecyclePhase::Started,
        changing_state: matches!(phase, LifecyclePhase::Starting | LifecyclePhase::Stopping),
    }
}

fn configured_shutdown_deadline() -> Option<Duration> {
    let milliseconds = std::env::var("ANI_RUNTIME_SHUTDOWN_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(30_000);
    (milliseconds != 0).then(|| Duration::from_millis(milliseconds))
}

struct WatchdogState {
    finished: Mutex<bool>,
    changed: Condvar,
}

struct ShutdownWatchdog(Option<Arc<WatchdogState>>);

impl ShutdownWatchdog {
    fn start(deadline: Option<Duration>) -> Self {
        let Some(deadline) = deadline else {
            return Self(None);
        };
        let state = Arc::new(WatchdogState {
            finished: Mutex::new(false),
            changed: Condvar::new(),
        });
        let watcher = Arc::clone(&state);
        std::thread::Builder::new()
            .name("ani-runtime-shutdown-watchdog".into())
            .spawn(move || {
                let Ok(finished) = watcher.finished.lock() else {
                    std::process::abort();
                };
                let Ok((finished, wait)) = watcher.changed.wait_timeout(finished, deadline) else {
                    std::process::abort();
                };
                if wait.timed_out() && !*finished {
                    eprintln!(
                        "ANI runtime failed to quiesce before {:?}; aborting to prevent native image unload while Rust code is live",
                        deadline
                    );
                    std::process::abort();
                }
            })
            .expect("failed to create ANI runtime shutdown watchdog");
        Self(Some(state))
    }
}

impl Drop for ShutdownWatchdog {
    fn drop(&mut self) {
        if let Some(state) = self.0.take()
            && let Ok(mut finished) = state.finished.lock()
        {
            *finished = true;
            state.changed.notify_all();
        }
    }
}

/// Cancels every operation, joins all runtime-owned execution contexts, and
/// leaves the domain restartable.
///
/// A watchdog aborts if a non-cooperative blocking task exceeds
/// `ANI_RUNTIME_SHUTDOWN_TIMEOUT_MS` (30 seconds by default, `0` disables the
/// deadline).  Returning while such a task can still execute addon code would
/// make unloading the native image unsound.
pub fn shutdown_runtime_domain() -> Result<()> {
    let _watchdog = ShutdownWatchdog::start(configured_shutdown_deadline());
    let scheduler_result = crate::scheduler::shared().shutdown();
    let async_result = registry().shutdown();
    scheduler_result.and(async_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::task::{RawWaker, RawWakerVTable};

    fn noop_waker() -> Waker {
        unsafe fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        unsafe fn wake(_: *const ()) {}
        unsafe fn wake_by_ref(_: *const ()) {}
        unsafe fn drop(_: *const ()) {}
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    #[test]
    fn dropping_carrier_rejects_exactly_once() {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let (task, handle) = RuntimeTask::new(
            || async {},
            move |_| {
                observed.fetch_add(1, Ordering::AcqRel);
            },
        );
        drop(task);
        handle.cancel(RuntimeCancelReason::Shutdown);
        assert_eq!(calls.load(Ordering::Acquire), 1);
        assert!(handle.is_cancelled());
    }

    #[test]
    fn local_completion_disarms_rejection() {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let (task, handle) = RuntimeTask::new(
            || async {},
            move |_| {
                observed.fetch_add(1, Ordering::AcqRel);
            },
        );
        let mut local = Box::pin(task.into_local_future());
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        assert!(matches!(local.as_mut().poll(&mut context), Poll::Ready(())));
        drop(local);
        assert_eq!(calls.load(Ordering::Acquire), 0);
        assert!(handle.is_finished());
        assert!(!handle.is_cancelled());
    }

    #[test]
    fn explicit_custom_payload_wins_drop_race() {
        #[derive(Debug)]
        struct DomainError;
        impl std::fmt::Display for DomainError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("domain cancellation")
            }
        }
        impl AniErrorPayload for DomainError {
            fn ani_status(&self) -> &str {
                "DomainCancelled"
            }
            fn ani_message(&self) -> &str {
                "domain cancellation"
            }
        }
        let status = Arc::new(Mutex::new(None));
        let observed = Arc::clone(&status);
        let (task, handle) = RuntimeTask::new(std::future::pending::<()>, move |error| {
            *observed.lock().unwrap() = Some(error.ani_status().to_string())
        });
        assert!(handle.cancel_with(Box::new(DomainError)));
        drop(task);
        assert_eq!(status.lock().unwrap().as_deref(), Some("DomainCancelled"));
    }

    #[test]
    fn loom_terminal_transition_is_exactly_once() {
        loom::model(|| {
            let terminal = loom::sync::Arc::new(loom::sync::atomic::AtomicBool::new(false));
            let calls = loom::sync::Arc::new(loom::sync::atomic::AtomicUsize::new(0));
            let mut threads = Vec::new();
            for _ in 0..3 {
                let terminal = terminal.clone();
                let calls = calls.clone();
                threads.push(loom::thread::spawn(move || {
                    if terminal
                        .compare_exchange(
                            false,
                            true,
                            loom::sync::atomic::Ordering::AcqRel,
                            loom::sync::atomic::Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        calls.fetch_add(1, loom::sync::atomic::Ordering::AcqRel);
                    }
                }));
            }
            for thread in threads {
                thread.join().unwrap();
            }
            assert_eq!(calls.load(loom::sync::atomic::Ordering::Acquire), 1);
        });
    }
}
