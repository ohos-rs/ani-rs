//! Executor-independent asynchronous runtime domain.
//!
//! The `async-runtime` feature (always compiled into `ani`, and selected by
//! `tokio_rt` / `async`) exposes an SPI without imposing a module-load
//! requirement. Implement [`AsyncRuntime`] to back ani-rs with your own
//! scheduler and register exactly one instance from `#[ani(init)]` or a
//! library constructor. A build with no registration can still load and
//! expose synchronous APIs; runtime-backed operations reject with a
//! missing-backend error.
//!
//! Generated bindings submit an opaque, `Send` [`RuntimeTask`] carrier. The
//! selected backend opens that carrier on one of its execution threads and
//! polls the resulting thread-affine future there. This extra factory step is
//! required by ANI: an [`crate::env::Env`] and local ANI references are not
//! `Send`, even when the Rust future using them is otherwise asynchronous.
//!
//! If no custom backend has been registered, the registration window closes
//! when `ANI_Constructor` begins activation, or earlier when a runtime-backed
//! operation commits a backend choice. In a combined `async-runtime` +
//! `tokio_rt` build that choice defaults generated `#[ani(async)]` futures to
//! the built-in Tokio backend. The established free `spawn`, `spawn_blocking`,
//! `block_on`, and `within_runtime_if_available` names remain Tokio
//! compatibility APIs whenever `tokio_rt` is enabled. Selecting and starting a
//! custom backend does not construct Tokio; the first Tokio compatibility
//! helper call constructs it lazily. In a pure `async-runtime` build there is
//! no Tokio at all, and a missing-backend error before any environment is
//! activated leaves the selection undecided and does not prevent later
//! registration.
//!
//! # Safety
//!
//! ArkTS may unload an addon's native image immediately after
//! `ANI_Destructor` returns. Implementations must ensure that, after
//! [`AsyncRuntime::shutdown`] returns, no backend-owned thread, task,
//! closure, destructor, cancellation callback, or future ANI callback can
//! execute code or access data from that image.

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, PoisonError, Weak};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use crate::conversions::{Deferred, PromiseRaw, PromiseValue};
use crate::env::Env;
use crate::error::{AniErrorPayload, DynAniError, Error, Result, Status};
use crate::scheduler::{RuntimeCancellable, RuntimeRegistration};

type LocalFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
type LocalFutureFactory = Box<dyn FnOnce() -> LocalFuture + Send + 'static>;
type RejectCallback = Box<dyn FnOnce(DynAniError) + Send + 'static>;
type BlockingWork = Box<dyn FnOnce() + Send + 'static>;

const DUPLICATE_RUNTIME_ERROR: &str =
    "register_async_runtime was called more than once for the same addon image";
const LATE_RUNTIME_REGISTRATION_ERROR: &str = "register_async_runtime must be called before the first ANI environment begins activation or an earlier runtime-backed operation commits a backend choice";
const MISSING_RUNTIME_BACKEND_ERROR: &str = "no AsyncRuntime backend is registered; call `register_async_runtime` from `#[ani(init)]` before invoking runtime-backed operations";

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
///
/// The type-erased guard is deliberately not `Send`, so the entered runtime
/// context cannot migrate to another thread. The unit type `()` implements it
/// as the no-op guard used by the default [`AsyncRuntime::enter`].
pub trait AsyncRuntimeGuard {}

impl AsyncRuntimeGuard for () {}

/// Carrier for work an [`AsyncRuntime`] backend declined to accept.
///
/// The backend must hand the work back **untouched** together with a
/// diagnostic payload; ani-rs then drops the recovered work through its
/// cancellation path instead of leaving a Promise pending forever.
#[derive(Debug)]
pub struct AsyncRuntimeRejection<T> {
    work: T,
    error: DynAniError,
}

impl<T> AsyncRuntimeRejection<T> {
    /// Create a rejection from the declined work and a diagnostic error.
    pub fn new(work: T, error: impl AniErrorPayload) -> Self {
        Self {
            work,
            error: Box::new(error),
        }
    }

    /// The diagnostic error describing why the work was declined.
    pub fn error(&self) -> &dyn AniErrorPayload {
        self.error.as_ref()
    }

    /// Recover the declined work and the diagnostic error.
    pub fn into_parts(self) -> (T, DynAniError) {
        (self.work, self.error)
    }
}

/// Dispose of a value recovered from a backend hook without letting a
/// panicking `Drop` escape containment.
fn drop_contained<T>(value: T) {
    if let Err(second_payload) = catch_unwind(AssertUnwindSafe(move || drop(value))) {
        std::mem::forget(second_payload);
    }
}

/// Wraps a backend-provided [`AsyncRuntimeGuard`] so its `Drop` is disposed
/// through [`drop_contained`].
struct ContainedGuard<'a>(Option<Box<dyn AsyncRuntimeGuard + 'a>>);

impl Drop for ContainedGuard<'_> {
    fn drop(&mut self) {
        if let Some(guard) = self.0.take() {
            drop_contained(guard);
        }
    }
}

/// Fully replaceable async execution backend.
///
/// The implementation is stored once per linked addon image and shared across
/// its threads, hence the `Send + Sync + 'static` bound. The backend's
/// [`Drop`] is not guaranteed to run; [`shutdown`](AsyncRuntime::shutdown) is
/// the sole resource-release and quiescence hook. Keep a newly constructed
/// backend dormant, create active resources in [`start`](AsyncRuntime::start),
/// and release them in `shutdown`.
///
/// # Safety
///
/// `shutdown` is a native-image safety boundary. Before it returns, every
/// backend thread, task, closure, waker, and blocking job that could execute
/// ani-rs/addon code must have quiesced. A backend which cannot satisfy that
/// contract must abort the process instead of returning. This requirement
/// applies to both `Ok` and `Err` returns.
pub unsafe trait AsyncRuntime: Send + Sync + 'static {
    /// Submit a task to run to completion in the background.
    ///
    /// Return `Ok(())` only after taking ownership of the task. Return
    /// `Err(AsyncRuntimeRejection::new(task, error))` when the runtime is
    /// stopped, saturated, or otherwise unable to accept it. Dropping an
    /// accepted task invokes its cancellation callback. Never forget an
    /// accepted task: retain it until completion or drop it on cancellation.
    ///
    /// Submissions can arrive before the first [`start`](AsyncRuntime::start)
    /// of an environment cycle completes, or after a `start` that failed. A
    /// dormant or not-ready backend must decline such work or accept it and
    /// defer execution.
    fn spawn(
        &self,
        task: RuntimeTask,
    ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>>;

    /// Block the current thread, fully driving the pinned future to completion
    /// before returning.
    ///
    /// The borrowed future must not be retained, moved to another thread, or
    /// accessed after this method returns.
    fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()>;

    /// Enter the runtime context and return a guard that establishes it for
    /// the calling thread.
    ///
    /// In pure `async-runtime` builds [`within_runtime_if_available`]
    /// delegates here; combined `tokio_rt` builds retain its established Tokio
    /// routing. The default implementation returns a no-op guard.
    fn enter(&self) -> Result<Box<dyn AsyncRuntimeGuard + '_>> {
        Ok(Box::new(()))
    }

    /// Start (or restart) the runtime.
    ///
    /// Called when the first live ANI environment starts, or earlier when a
    /// module-init hook triggers the first runtime-backed dispatch of the
    /// cycle. Implement it idempotently. If this returns an error, or panics
    /// on an unwind-enabled build, ani-rs calls
    /// [`shutdown`](AsyncRuntime::shutdown) to roll back the partial start.
    fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Shut the runtime down.
    ///
    /// Stop accepting work before returning and drop queued [`RuntimeTask`]
    /// values and queued [`spawn_blocking`](AsyncRuntime::spawn_blocking)
    /// closures so their promises are cancelled. The hook must be idempotent
    /// and tolerate being called before `start`, after a partial failed
    /// `start`, and repeatedly without an intervening `start`.
    fn shutdown(&self) -> Result<()>;

    /// Optional hook: run `work` on the backend's blocking-capable lane.
    ///
    /// Return `Ok(())` once the work is accepted. Return
    /// `Err(AsyncRuntimeRejection::new(work, error))` to decline; ani-rs
    /// surfaces that diagnostic and does not create an unbounded fallback
    /// thread. The default implementation declines.
    fn spawn_blocking(
        &self,
        work: BlockingWork,
    ) -> std::result::Result<(), AsyncRuntimeRejection<BlockingWork>> {
        Err(AsyncRuntimeRejection::new(
            work,
            Error::new(
                Status::GenericFailure,
                "The AsyncRuntime backend does not support blocking work",
            ),
        ))
    }
}

/// Lifecycle phase of the selected backend for the current zero-to-live
/// environment cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LifecyclePhase {
    /// No start-with-rollback sequence ran for this cycle, the last one was
    /// rolled back, or a teardown completed.
    Idle,
    /// A dispatch or activation claimed the start; its start-with-rollback
    /// sequence has not completed yet. Further dispatches proceed without
    /// waiting.
    Starting,
    /// The start-with-rollback sequence completed successfully for this cycle.
    Started,
}

struct RegistryState {
    selection_frozen: bool,
    phase: LifecyclePhase,
}

/// Process-global (per addon image) registry holding the custom
/// [`AsyncRuntime`] selection.
struct AsyncRuntimeRegistry {
    backend: OnceLock<Box<dyn AsyncRuntime>>,
    state: Mutex<RegistryState>,
    lifecycle: Mutex<()>,
    deferred_registration_error: Mutex<Option<&'static str>>,
}

impl AsyncRuntimeRegistry {
    #[cfg(test)]
    fn new() -> Self {
        Self {
            backend: OnceLock::new(),
            state: Mutex::new(RegistryState {
                selection_frozen: false,
                phase: LifecyclePhase::Idle,
            }),
            lifecycle: Mutex::new(()),
            deferred_registration_error: Mutex::new(None),
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, RegistryState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn lock_lifecycle(&self) -> MutexGuard<'_, ()> {
        self.lifecycle
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn try_register(
        &self,
        runtime: Box<dyn AsyncRuntime>,
    ) -> std::result::Result<(), (&'static str, Box<dyn AsyncRuntime>)> {
        let _state = self.lock_state();
        if self.backend.get().is_some() {
            return Err((DUPLICATE_RUNTIME_ERROR, runtime));
        }
        if _state.selection_frozen {
            return Err((LATE_RUNTIME_REGISTRATION_ERROR, runtime));
        }
        match self.backend.set(runtime) {
            Ok(()) => Ok(()),
            Err(rejected) => Err((DUPLICATE_RUNTIME_ERROR, rejected)),
        }
    }

    fn record_registration_error(&self, reason: &'static str) {
        let mut slot = self
            .deferred_registration_error
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if slot.is_none() {
            *slot = Some(reason);
        }
    }

    fn deferred_registration_error(&self) -> Option<&'static str> {
        *self
            .deferred_registration_error
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Commit the backend selection for a runtime-backed operation and return
    /// the custom backend if one is selected. `fallback_commits` is `true`
    /// when a built-in Tokio fallback exists: taking the fallback also
    /// commits a choice and closes the registration window. In a pure
    /// `async-runtime` build a missing backend leaves the selection
    /// undecided.
    fn commit_selection(&self, fallback_commits: bool) -> Option<&dyn AsyncRuntime> {
        let mut state = self.lock_state();
        let backend = self.backend.get().map(|backend| backend.as_ref());
        if backend.is_some() || fallback_commits {
            state.selection_frozen = true;
        }
        backend
    }

    fn dispatch_backend(&self) -> Option<&dyn AsyncRuntime> {
        if let Some(backend) = self.commit_selection(cfg!(feature = "tokio_rt")) {
            return Some(backend);
        }
        #[cfg(feature = "tokio_rt")]
        {
            Some(crate::tokio::tokio_fallback_runtime())
        }
        #[cfg(not(feature = "tokio_rt"))]
        None
    }

    fn ensure_started(&self, backend: &dyn AsyncRuntime) {
        {
            let mut state = self.lock_state();
            match state.phase {
                LifecyclePhase::Starting | LifecyclePhase::Started => return,
                LifecyclePhase::Idle => state.phase = LifecyclePhase::Starting,
            }
        }
        self.run_claimed_start(backend);
    }

    fn within_runtime<F: FnOnce() -> T, T>(&self, f: F) -> T {
        if let Some(backend) = self.commit_selection(false) {
            self.ensure_started(backend);
            let _guard = match catch_unwind(AssertUnwindSafe(|| backend.enter())) {
                Ok(Ok(guard)) => Some(ContainedGuard(Some(guard))),
                Ok(Err(error)) => {
                    drop_contained(error);
                    None
                }
                Err(payload) => {
                    drop_contained(payload);
                    None
                }
            };
            return f();
        }
        f()
    }

    fn run_claimed_start(&self, backend: &dyn AsyncRuntime) {
        let _lifecycle = self.lock_lifecycle();
        if self.lock_state().phase != LifecyclePhase::Starting {
            return;
        }
        let phase = if start_backend_with_rollback(backend) {
            #[cfg(feature = "tokio_rt")]
            crate::tokio::refill_drained_tokio_runtime();
            RUNTIME_GENERATION.fetch_add(1, Ordering::AcqRel);
            LifecyclePhase::Started
        } else {
            LifecyclePhase::Idle
        };
        self.lock_state().phase = phase;
    }

    /// First-env-activation hook. Returns `true` when a custom backend owns
    /// the runtime lifecycle.
    fn activate(&self) -> bool {
        let backend = {
            let mut state = self.lock_state();
            state.selection_frozen = true;
            let Some(backend) = self.backend.get() else {
                return false;
            };
            match state.phase {
                LifecyclePhase::Starting | LifecyclePhase::Started => return true,
                LifecyclePhase::Idle => state.phase = LifecyclePhase::Starting,
            }
            backend
        };
        self.run_claimed_start(backend.as_ref());
        true
    }

    /// Last-env-teardown hook. Returns `true` when a custom backend owned the
    /// lifecycle.
    fn deactivate(&self) -> bool {
        let Some(backend) = self.backend.get() else {
            return false;
        };
        let _lifecycle = self.lock_lifecycle();
        match catch_unwind(AssertUnwindSafe(|| backend.shutdown())) {
            Ok(shutdown_result) => drop_contained(shutdown_result),
            Err(payload) => {
                std::mem::forget(payload);
                std::process::abort();
            }
        }
        self.lock_state().phase = LifecyclePhase::Idle;
        true
    }

    fn shutdown_fallback(&self) {
        #[cfg(feature = "tokio_rt")]
        {
            let _lifecycle = self.lock_lifecycle();
            match catch_unwind(AssertUnwindSafe(|| {
                crate::tokio::tokio_fallback_runtime().shutdown()
            })) {
                Ok(shutdown_result) => drop_contained(shutdown_result),
                Err(payload) => {
                    std::mem::forget(payload);
                    std::process::abort();
                }
            }
            self.lock_state().phase = LifecyclePhase::Idle;
        }
    }

    fn phase(&self) -> LifecyclePhase {
        self.lock_state().phase
    }
}

fn start_backend_with_rollback(backend: &dyn AsyncRuntime) -> bool {
    match catch_unwind(AssertUnwindSafe(|| backend.start())) {
        Ok(Ok(())) => return true,
        Ok(Err(start_error)) => drop_contained(start_error),
        Err(payload) => drop_contained(payload),
    }
    match catch_unwind(AssertUnwindSafe(|| backend.shutdown())) {
        Ok(shutdown_result) => drop_contained(shutdown_result),
        Err(payload) => {
            std::mem::forget(payload);
            std::process::abort();
        }
    }
    false
}

fn registry() -> &'static AsyncRuntimeRegistry {
    static REGISTRY: AsyncRuntimeRegistry = AsyncRuntimeRegistry {
        backend: OnceLock::new(),
        state: Mutex::new(RegistryState {
            selection_frozen: false,
            phase: LifecyclePhase::Idle,
        }),
        lifecycle: Mutex::new(()),
        deferred_registration_error: Mutex::new(None),
    };
    &REGISTRY
}

fn retire_rejected_async_runtime(runtime: Box<dyn AsyncRuntime>) {
    match catch_unwind(AssertUnwindSafe(|| runtime.shutdown())) {
        Ok(shutdown_result) => drop_contained(shutdown_result),
        Err(payload) => {
            std::mem::forget(payload);
            std::mem::forget(runtime);
            std::process::abort();
        }
    }
    if let Err(payload) = catch_unwind(AssertUnwindSafe(move || drop(runtime))) {
        std::mem::forget(payload);
        std::process::abort();
    }
}

/// Register the custom [`AsyncRuntime`] backend for this linked addon image.
///
/// Call this once from `#[ani(init)]` or a library constructor. Registration
/// only publishes a dormant backend; ani-rs calls [`AsyncRuntime::start`]
/// before the backend's first dispatch — normally during `ANI_Constructor`,
/// or earlier when a module-init hook invokes a runtime-backed API.
///
/// Registration is first-writer-wins. This infallible wrapper never panics:
/// a duplicate or late registration records the error, and every later
/// runtime-backed operation surfaces it by rejecting its Promise. The
/// fallible [`try_register_async_runtime`] form returns the error directly.
pub fn register_async_runtime<R: AsyncRuntime>(runtime: R) {
    if let Err((reason, rejected)) = registry().try_register(Box::new(runtime)) {
        retire_rejected_async_runtime(rejected);
        registry().record_registration_error(reason);
    }
}

/// Try to register a custom async runtime without deferring errors.
///
/// Library constructors should normally use [`register_async_runtime`].
/// Registration after ani-rs begins activating an environment, or after an
/// earlier runtime-backed operation commits a backend choice, returns an
/// error and safely retires the rejected backend.
pub fn try_register_async_runtime<R: AsyncRuntime>(runtime: R) -> Result<()> {
    match registry().try_register(Box::new(runtime)) {
        Ok(()) => Ok(()),
        Err((reason, rejected)) => {
            retire_rejected_async_runtime(rejected);
            Err(Error::new(Status::GenericFailure, reason))
        }
    }
}

/// Start the async runtime.
///
/// When a custom [`AsyncRuntime`] backend has been registered, this closes
/// the registration window and calls the backend's [`AsyncRuntime::start`]
/// hook. If that hook returns an error or panics,
/// [`AsyncRuntime::shutdown`] is called to roll back the partial start.
/// Selecting a custom backend never constructs the built-in Tokio runtime.
///
/// Otherwise (the `tokio_rt` path) the built-in Tokio backend and the Tokio
/// compatibility-helper runtime are started so they survive environment
/// recreation after an earlier shutdown.
pub fn start_async_runtime() {
    if registry().activate() {
        #[cfg(feature = "tokio_rt")]
        crate::tokio::refill_drained_tokio_runtime();
        #[cfg(feature = "tokio_rt")]
        return;
    }
    #[cfg(feature = "tokio_rt")]
    {
        registry().ensure_started(crate::tokio::tokio_fallback_runtime());
        crate::tokio::ensure_tokio_helper_runtime();
    }
}

/// Starts the selected backend without submitting work.
///
/// Prefer [`start_async_runtime`], which matches the napi-rs name and is
/// infallible. This alias remains for existing callers.
pub fn activate_async_runtime() -> Result<()> {
    start_async_runtime();
    Ok(())
}

/// Shutdown the async runtime.
///
/// When a custom backend has been registered, this calls its
/// [`AsyncRuntime::shutdown`] hook. In combined `async-runtime` + `tokio_rt`
/// builds a built-in Tokio runtime that a compatibility helper constructed
/// lazily is also drained. The next [`start_async_runtime`] or the next
/// runtime-backed dispatch refills the drained pair.
pub fn shutdown_async_runtime() {
    if registry().deactivate() {
        #[cfg(feature = "tokio_rt")]
        crate::tokio::drain_tokio_helper_runtime();
        #[cfg(feature = "tokio_rt")]
        return;
    }
    registry().shutdown_fallback();
    #[cfg(feature = "tokio_rt")]
    crate::tokio::drain_tokio_helper_runtime();
}

/// Enter the registered backend's context around `f`.
///
/// Combined `tokio_rt` builds re-export a Tokio-backed helper of the same
/// name from [`crate::tokio`]. This function is the pure `async-runtime`
/// path.
pub fn within_runtime_if_available<F: FnOnce() -> T, T>(f: F) -> T {
    registry().within_runtime(f)
}

fn reject_task_with(task: RuntimeTask, error: impl AniErrorPayload) {
    task.reject_with(Box::new(error));
}

/// Submits one carrier to the selected runtime.
pub fn spawn_runtime_task(task: RuntimeTask) -> Result<RuntimeTaskHandle> {
    let handle = task.handle();
    let registration = crate::scheduler::shared().register_cancellable(&task.control)?;
    task.control.install_registration(registration);

    if let Some(reason) = registry().deferred_registration_error() {
        reject_task_with(task, Error::new(Status::GenericFailure, reason));
        return Ok(handle);
    }

    #[cfg(not(feature = "tokio_rt"))]
    if registry().commit_selection(false).is_none() {
        reject_task_with(
            task,
            Error::new(Status::GenericFailure, MISSING_RUNTIME_BACKEND_ERROR),
        );
        return Ok(handle);
    }

    let Some(backend) = registry().dispatch_backend() else {
        reject_task_with(
            task,
            Error::new(Status::GenericFailure, MISSING_RUNTIME_BACKEND_ERROR),
        );
        return Ok(handle);
    };
    registry().ensure_started(backend);
    match catch_unwind(AssertUnwindSafe(|| backend.spawn(task))) {
        Ok(Ok(())) => Ok(handle),
        Ok(Err(rejection)) => {
            let (task, error) = rejection.into_parts();
            task.reject_with(error);
            Ok(handle)
        }
        Err(payload) => {
            drop_contained(payload);
            Ok(handle)
        }
    }
}

/// Submits blocking work to the selected runtime backend.
pub fn spawn_runtime_blocking_task(task: RuntimeBlockingTask) -> Result<RuntimeTaskHandle> {
    let handle = task.handle();
    let registration = crate::scheduler::shared().register_cancellable(&task.control)?;
    task.control.install_registration(registration);

    if let Some(reason) = registry().deferred_registration_error() {
        task.reject_with(Box::new(Error::new(Status::GenericFailure, reason)));
        return Ok(handle);
    }

    #[cfg(not(feature = "tokio_rt"))]
    if registry().commit_selection(false).is_none() {
        task.reject_with(Box::new(Error::new(
            Status::GenericFailure,
            MISSING_RUNTIME_BACKEND_ERROR,
        )));
        return Ok(handle);
    }

    let Some(backend) = registry().dispatch_backend() else {
        task.reject_with(Box::new(Error::new(
            Status::GenericFailure,
            MISSING_RUNTIME_BACKEND_ERROR,
        )));
        return Ok(handle);
    };
    registry().ensure_started(backend);

    let holder = Arc::new(Mutex::new(Some(task)));
    let scheduled = Arc::clone(&holder);
    let work: BlockingWork = Box::new(move || {
        if let Some(task) = scheduled.lock().ok().and_then(|mut task| task.take()) {
            task.run();
        }
    });
    match catch_unwind(AssertUnwindSafe(|| backend.spawn_blocking(work))) {
        Ok(Ok(())) => Ok(handle),
        Ok(Err(rejection)) => {
            let (work, error) = rejection.into_parts();
            drop(work);
            if let Some(task) = holder.lock().ok().and_then(|mut task| task.take()) {
                task.reject_with(error);
            }
            Ok(handle)
        }
        Err(payload) => {
            drop_contained(payload);
            Ok(handle)
        }
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
    if let Some(reason) = registry().deferred_registration_error() {
        return Err(Error::new(Status::GenericFailure, reason));
    }

    #[cfg(not(feature = "tokio_rt"))]
    if registry().commit_selection(false).is_none() {
        return Err(Error::new(
            Status::GenericFailure,
            MISSING_RUNTIME_BACKEND_ERROR,
        ));
    }

    let backend = registry()
        .dispatch_backend()
        .ok_or_else(|| Error::new(Status::GenericFailure, MISSING_RUNTIME_BACKEND_ERROR))?;
    registry().ensure_started(backend);

    let mut outcome = None;
    let mut driver = Box::pin(async {
        outcome = Some(future.await);
    });
    match catch_unwind(AssertUnwindSafe(|| backend.block_on(driver.as_mut()))) {
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
        changing_state: phase == LifecyclePhase::Starting,
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
    shutdown_async_runtime();
    scheduler_result
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

    const BACKEND_STOPPED_ERROR: &str = "mock backend is stopped";
    const HANG_PROTECTION: Duration = Duration::from_secs(30);

    #[derive(Default)]
    struct BackendProbe {
        start_calls: AtomicUsize,
        shutdown_calls: AtomicUsize,
        spawn_calls: AtomicUsize,
        spawns_before_start: AtomicUsize,
        running: AtomicBool,
    }

    struct MockRuntime {
        probe: Arc<BackendProbe>,
        fail_start: bool,
    }

    impl MockRuntime {
        fn new(probe: &Arc<BackendProbe>) -> Self {
            Self {
                probe: Arc::clone(probe),
                fail_start: false,
            }
        }

        fn failing_start(probe: &Arc<BackendProbe>) -> Self {
            Self {
                probe: Arc::clone(probe),
                fail_start: true,
            }
        }
    }

    fn poll_task_now(task: RuntimeTask) {
        let mut local = Box::pin(task.into_local_future());
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let _ = local.as_mut().poll(&mut context);
    }

    unsafe impl AsyncRuntime for MockRuntime {
        fn spawn(
            &self,
            task: RuntimeTask,
        ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
            if self.probe.start_calls.load(Ordering::SeqCst) == 0 {
                self.probe
                    .spawns_before_start
                    .fetch_add(1, Ordering::SeqCst);
            }
            self.probe.spawn_calls.fetch_add(1, Ordering::SeqCst);
            if !self.probe.running.load(Ordering::SeqCst) {
                return Err(AsyncRuntimeRejection::new(
                    task,
                    Error::new(Status::GenericFailure, BACKEND_STOPPED_ERROR),
                ));
            }
            poll_task_now(task);
            Ok(())
        }

        fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
            let waker = noop_waker();
            let mut context = Context::from_waker(&waker);
            let _ = future.poll(&mut context);
            Ok(())
        }

        fn start(&self) -> Result<()> {
            self.probe.start_calls.fetch_add(1, Ordering::SeqCst);
            if self.fail_start {
                return Err(Error::new(Status::GenericFailure, "start failed"));
            }
            self.probe.running.store(true, Ordering::SeqCst);
            Ok(())
        }

        fn shutdown(&self) -> Result<()> {
            self.probe.running.store(false, Ordering::SeqCst);
            self.probe.shutdown_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn dummy_task() -> RuntimeTask {
        RuntimeTask::new(|| async {}, |_| {}).0
    }

    #[test]
    fn duplicate_registration_defers_error_and_retires_backend() {
        let registry = AsyncRuntimeRegistry::new();
        let first_probe = Arc::new(BackendProbe::default());
        let second_probe = Arc::new(BackendProbe::default());

        assert!(
            registry
                .try_register(Box::new(MockRuntime::new(&first_probe)))
                .is_ok()
        );

        let (reason, rejected) = registry
            .try_register(Box::new(MockRuntime::new(&second_probe)))
            .expect_err("second registration must be rejected");
        assert_eq!(reason, DUPLICATE_RUNTIME_ERROR);

        retire_rejected_async_runtime(rejected);
        assert_eq!(second_probe.shutdown_calls.load(Ordering::SeqCst), 1);
        assert!(registry.commit_selection(false).is_some());
        assert_eq!(first_probe.shutdown_calls.load(Ordering::SeqCst), 0);

        registry.record_registration_error(reason);
        assert_eq!(
            registry.deferred_registration_error(),
            Some(DUPLICATE_RUNTIME_ERROR)
        );
    }

    #[test]
    fn late_registration_after_env_activation_rejected() {
        let registry = AsyncRuntimeRegistry::new();
        assert!(!registry.activate());

        let probe = Arc::new(BackendProbe::default());
        let (reason, rejected) = registry
            .try_register(Box::new(MockRuntime::new(&probe)))
            .expect_err("registration after env activation must be rejected");
        retire_rejected_async_runtime(rejected);
        assert_eq!(reason, LATE_RUNTIME_REGISTRATION_ERROR);
    }

    #[test]
    fn tokio_fallback_selection_commits_and_closes_registration() {
        let registry = AsyncRuntimeRegistry::new();
        assert!(registry.commit_selection(true).is_none());

        let probe = Arc::new(BackendProbe::default());
        let (reason, rejected) = registry
            .try_register(Box::new(MockRuntime::new(&probe)))
            .expect_err("registration after a committed fallback choice must be rejected");
        retire_rejected_async_runtime(rejected);
        assert_eq!(reason, LATE_RUNTIME_REGISTRATION_ERROR);
    }

    #[test]
    fn missing_backend_does_not_freeze_selection() {
        let registry = AsyncRuntimeRegistry::new();
        assert!(registry.commit_selection(false).is_none());

        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(MockRuntime::new(&probe)))
                .is_ok()
        );
        assert!(registry.commit_selection(false).is_some());
    }

    #[test]
    fn spawn_blocking_default_declines_with_work_returned() {
        struct SpawnOnlyRuntime;
        unsafe impl AsyncRuntime for SpawnOnlyRuntime {
            fn spawn(
                &self,
                task: RuntimeTask,
            ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
                poll_task_now(task);
                Ok(())
            }
            fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
                let waker = noop_waker();
                let mut context = Context::from_waker(&waker);
                let _ = future.poll(&mut context);
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_in_work = Arc::clone(&ran);
        let rejection = SpawnOnlyRuntime
            .spawn_blocking(Box::new(move || {
                ran_in_work.fetch_add(1, Ordering::SeqCst);
            }))
            .expect_err("the default spawn_blocking implementation must decline");
        assert_eq!(rejection.error().ani_status(), "GenericFailure");

        let (work, _error) = rejection.into_parts();
        assert_eq!(ran.load(Ordering::SeqCst), 0);
        work();
        assert_eq!(ran.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn pre_activation_dispatch_starts_backend_before_first_spawn() {
        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(MockRuntime::new(&probe)))
                .is_ok()
        );

        let backend = registry
            .commit_selection(cfg!(feature = "tokio_rt"))
            .expect("custom backend must be selected");
        registry.ensure_started(backend);
        assert!(backend.spawn(dummy_task()).is_ok());

        assert_eq!(probe.spawn_calls.load(Ordering::SeqCst), 1);
        assert_eq!(probe.spawns_before_start.load(Ordering::SeqCst), 0);
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);

        assert!(registry.activate());
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);

        assert!(registry.deactivate());
        assert!(registry.activate());
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn deactivate_waits_out_inflight_start() {
        use std::sync::mpsc;

        struct GatedStartRuntime {
            events: Arc<Mutex<Vec<&'static str>>>,
            start_entered: mpsc::Sender<()>,
            release_start: Mutex<mpsc::Receiver<()>>,
        }

        unsafe impl AsyncRuntime for GatedStartRuntime {
            fn spawn(
                &self,
                task: RuntimeTask,
            ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
                poll_task_now(task);
                Ok(())
            }

            fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
                let waker = noop_waker();
                let mut context = Context::from_waker(&waker);
                let _ = future.poll(&mut context);
                Ok(())
            }

            fn start(&self) -> Result<()> {
                self.events.lock().unwrap().push("start:enter");
                self.start_entered
                    .send(())
                    .expect("test driver dropped the start-entered channel");
                self.release_start
                    .lock()
                    .unwrap()
                    .recv_timeout(HANG_PROTECTION)
                    .expect("test driver never released the gated start");
                self.events.lock().unwrap().push("start:exit");
                Ok(())
            }

            fn shutdown(&self) -> Result<()> {
                self.events.lock().unwrap().push("shutdown");
                Ok(())
            }
        }

        let events: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let (start_entered_tx, start_entered_rx) = mpsc::channel();
        let (release_start_tx, release_start_rx) = mpsc::channel();
        let (deactivate_called_tx, deactivate_called_rx) = mpsc::channel();

        let registry = AsyncRuntimeRegistry::new();
        assert!(
            registry
                .try_register(Box::new(GatedStartRuntime {
                    events: Arc::clone(&events),
                    start_entered: start_entered_tx,
                    release_start: Mutex::new(release_start_rx),
                }))
                .is_ok()
        );

        std::thread::scope(|scope| {
            let starter = scope.spawn(|| assert!(registry.activate()));
            start_entered_rx
                .recv_timeout(HANG_PROTECTION)
                .expect("start was never entered");

            let stopper = scope.spawn(|| {
                deactivate_called_tx
                    .send(())
                    .expect("test driver dropped the deactivate-called channel");
                assert!(registry.deactivate());
                events.lock().unwrap().push("deactivate:returned");
            });
            deactivate_called_rx
                .recv_timeout(HANG_PROTECTION)
                .expect("deactivate was never called");
            release_start_tx
                .send(())
                .expect("the gated start is no longer waiting for its release");

            starter.join().expect("starter thread panicked");
            stopper.join().expect("stopper thread panicked");
        });

        assert_eq!(
            *events.lock().unwrap(),
            [
                "start:enter",
                "start:exit",
                "shutdown",
                "deactivate:returned"
            ]
        );
    }

    #[test]
    fn completed_teardown_revokes_pending_start_claim() {
        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(MockRuntime::new(&probe)))
                .is_ok()
        );

        registry.lock_state().phase = LifecyclePhase::Starting;
        assert!(registry.deactivate());
        assert_eq!(probe.shutdown_calls.load(Ordering::SeqCst), 1);

        let backend = registry
            .commit_selection(cfg!(feature = "tokio_rt"))
            .expect("custom backend must stay selected");
        registry.run_claimed_start(backend);
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 0);

        let rejection = backend
            .spawn(dummy_task())
            .expect_err("a stopped conforming backend must reject the spawn");
        let (task, error) = rejection.into_parts();
        task.reject_with(error);

        registry.ensure_started(backend);
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn start_error_triggers_shutdown_rollback() {
        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(MockRuntime::failing_start(&probe)))
                .is_ok()
        );

        assert!(registry.activate());
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);
        assert_eq!(probe.shutdown_calls.load(Ordering::SeqCst), 1);

        assert!(registry.deactivate());
        assert_eq!(probe.shutdown_calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn panicking_payload_drop_from_start_is_contained() {
        struct PanickingDropPayload {
            drops: Arc<AtomicUsize>,
        }

        impl Drop for PanickingDropPayload {
            fn drop(&mut self) {
                self.drops.fetch_add(1, Ordering::SeqCst);
                panic!("panic payload drop");
            }
        }

        struct PanicOnFirstStartRuntime {
            probe: Arc<BackendProbe>,
            payload_drops: Arc<AtomicUsize>,
        }

        unsafe impl AsyncRuntime for PanicOnFirstStartRuntime {
            fn spawn(
                &self,
                task: RuntimeTask,
            ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
                poll_task_now(task);
                Ok(())
            }

            fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
                let waker = noop_waker();
                let mut context = Context::from_waker(&waker);
                let _ = future.poll(&mut context);
                Ok(())
            }

            fn start(&self) -> Result<()> {
                if self.probe.start_calls.fetch_add(1, Ordering::SeqCst) == 0 {
                    std::panic::panic_any(PanickingDropPayload {
                        drops: Arc::clone(&self.payload_drops),
                    });
                }
                self.probe.running.store(true, Ordering::SeqCst);
                Ok(())
            }

            fn shutdown(&self) -> Result<()> {
                self.probe.running.store(false, Ordering::SeqCst);
                self.probe.shutdown_calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }

        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        let payload_drops = Arc::new(AtomicUsize::new(0));
        assert!(
            registry
                .try_register(Box::new(PanicOnFirstStartRuntime {
                    probe: Arc::clone(&probe),
                    payload_drops: Arc::clone(&payload_drops),
                }))
                .is_ok()
        );

        let activated = catch_unwind(AssertUnwindSafe(|| registry.activate()))
            .expect("a panicking payload Drop must not unwind out of the activate path");
        assert!(activated);
        assert_eq!(payload_drops.load(Ordering::SeqCst), 1);
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);
        assert_eq!(probe.shutdown_calls.load(Ordering::SeqCst), 1);
        assert_eq!(registry.lock_state().phase, LifecyclePhase::Idle);

        assert!(registry.activate());
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 2);
        assert!(probe.running.load(Ordering::SeqCst));
    }

    #[cfg(not(feature = "tokio_rt"))]
    #[test]
    fn within_runtime_starts_dormant_backend_before_enter() {
        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(MockRuntime::new(&probe)))
                .is_ok()
        );
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 0);

        let ran = registry.within_runtime(|| true);

        assert!(ran, "the wrapped closure must run");
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);
    }

    #[cfg(not(feature = "tokio_rt"))]
    #[test]
    fn within_runtime_contains_panicking_enter_hook() {
        struct PanicOnEnterRuntime {
            probe: Arc<BackendProbe>,
        }

        unsafe impl AsyncRuntime for PanicOnEnterRuntime {
            fn spawn(
                &self,
                task: RuntimeTask,
            ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
                poll_task_now(task);
                Ok(())
            }

            fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
                let waker = noop_waker();
                let mut context = Context::from_waker(&waker);
                let _ = future.poll(&mut context);
                Ok(())
            }

            fn start(&self) -> Result<()> {
                self.probe.start_calls.fetch_add(1, Ordering::SeqCst);
                self.probe.running.store(true, Ordering::SeqCst);
                Ok(())
            }

            fn enter(&self) -> Result<Box<dyn AsyncRuntimeGuard + '_>> {
                panic!("enter hook panic");
            }

            fn shutdown(&self) -> Result<()> {
                self.probe.running.store(false, Ordering::SeqCst);
                self.probe.shutdown_calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }

        let registry = AsyncRuntimeRegistry::new();
        let probe = Arc::new(BackendProbe::default());
        assert!(
            registry
                .try_register(Box::new(PanicOnEnterRuntime {
                    probe: Arc::clone(&probe)
                }))
                .is_ok()
        );

        let ran = registry.within_runtime(|| true);
        assert!(ran);
        assert_eq!(probe.start_calls.load(Ordering::SeqCst), 1);
    }

    /// A Tokio-free backend: one worker thread owns `!Send` local futures.
    /// This is the shape an FFRT / thread-pool implementer should follow.
    struct WorkerQueueRuntime {
        jobs: Mutex<Option<std::sync::mpsc::Sender<RuntimeTask>>>,
        join: Mutex<Option<std::thread::JoinHandle<()>>>,
        running: AtomicBool,
    }

    impl WorkerQueueRuntime {
        fn new() -> Self {
            let (tx, rx) = std::sync::mpsc::channel::<RuntimeTask>();
            let join = std::thread::Builder::new()
                .name("ani-test-worker".into())
                .spawn(move || {
                    while let Ok(task) = rx.recv() {
                        drive_local_task(task);
                    }
                })
                .expect("spawn worker");
            Self {
                jobs: Mutex::new(Some(tx)),
                join: Mutex::new(Some(join)),
                running: AtomicBool::new(false),
            }
        }
    }

    fn drive_local_task(task: RuntimeTask) {
        let mut local = std::pin::pin!(task.into_local_future());
        let parked = Arc::new(std::thread::current());
        struct ThreadWake(Arc<std::thread::Thread>);
        impl std::task::Wake for ThreadWake {
            fn wake(self: Arc<Self>) {
                self.0.unpark();
            }
        }
        let waker = Waker::from(Arc::new(ThreadWake(parked)));
        let mut context = Context::from_waker(&waker);
        while local.as_mut().poll(&mut context).is_pending() {
            std::thread::park();
        }
    }

    unsafe impl AsyncRuntime for WorkerQueueRuntime {
        fn spawn(
            &self,
            task: RuntimeTask,
        ) -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>> {
            if !self.running.load(Ordering::SeqCst) {
                return Err(AsyncRuntimeRejection::new(
                    task,
                    Error::new(Status::GenericFailure, BACKEND_STOPPED_ERROR),
                ));
            }
            let jobs = self.jobs.lock().unwrap();
            match jobs.as_ref() {
                Some(tx) => tx.send(task).map_err(|std::sync::mpsc::SendError(task)| {
                    AsyncRuntimeRejection::new(
                        task,
                        Error::new(Status::GenericFailure, "worker queue closed"),
                    )
                }),
                None => Err(AsyncRuntimeRejection::new(
                    task,
                    Error::new(Status::GenericFailure, "worker queue closed"),
                )),
            }
        }

        fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> {
            let parked = Arc::new(std::thread::current());
            struct ThreadWake(Arc<std::thread::Thread>);
            impl std::task::Wake for ThreadWake {
                fn wake(self: Arc<Self>) {
                    self.0.unpark();
                }
            }
            let waker = Waker::from(Arc::new(ThreadWake(parked)));
            let mut context = Context::from_waker(&waker);
            let mut future = future;
            while future.as_mut().poll(&mut context).is_pending() {
                std::thread::park();
            }
            Ok(())
        }

        fn spawn_blocking(
            &self,
            work: BlockingWork,
        ) -> std::result::Result<(), AsyncRuntimeRejection<BlockingWork>> {
            if !self.running.load(Ordering::SeqCst) {
                return Err(AsyncRuntimeRejection::new(
                    work,
                    Error::new(Status::GenericFailure, BACKEND_STOPPED_ERROR),
                ));
            }
            std::thread::spawn(work);
            Ok(())
        }

        fn start(&self) -> Result<()> {
            self.running.store(true, Ordering::SeqCst);
            Ok(())
        }

        fn shutdown(&self) -> Result<()> {
            self.running.store(false, Ordering::SeqCst);
            self.jobs.lock().unwrap().take();
            if let Some(join) = self.join.lock().unwrap().take() {
                let _ = join.join();
            }
            Ok(())
        }
    }

    #[test]
    fn custom_non_tokio_runtime_runs_spawn_and_block_on() {
        let runtime = WorkerQueueRuntime::new();
        runtime.start().unwrap();

        let done = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&done);
        let (task, handle) = RuntimeTask::new(
            move || async move {
                observed.fetch_add(1, Ordering::SeqCst);
            },
            |_| {},
        );
        runtime.spawn(task).expect("worker accepts the task");
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while !handle.is_finished() {
            assert!(
                std::time::Instant::now() < deadline,
                "custom runtime did not finish the task"
            );
            std::thread::yield_now();
        }
        assert_eq!(done.load(Ordering::SeqCst), 1);

        let blocked = Arc::new(AtomicUsize::new(0));
        let blocked_in_future = Arc::clone(&blocked);
        let mut driver = std::pin::pin!(async move {
            blocked_in_future.fetch_add(1, Ordering::SeqCst);
        });
        runtime.block_on(driver.as_mut()).unwrap();
        assert_eq!(blocked.load(Ordering::SeqCst), 1);

        let blocking_done = Arc::new(AtomicUsize::new(0));
        let blocking_in_work = Arc::clone(&blocking_done);
        assert!(
            runtime
                .spawn_blocking(Box::new(move || {
                    blocking_in_work.fetch_add(1, Ordering::SeqCst);
                }))
                .is_ok()
        );
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while blocking_done.load(Ordering::SeqCst) == 0 {
            assert!(std::time::Instant::now() < deadline);
            std::thread::yield_now();
        }

        runtime.shutdown().unwrap();
        let (task, handle) = RuntimeTask::new(|| async {}, |_| {});
        let rejection = runtime
            .spawn(task)
            .expect_err("stopped custom runtime must decline");
        let (task, error) = rejection.into_parts();
        task.reject_with(error);
        assert!(handle.is_cancelled());
    }
}
