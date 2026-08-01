//! Promise support for ANI
//!
//! This module provides Promise types with an extensible structured rejection contract:
//!
//! - [`PromiseRaw`] - Raw Promise value with lifetime, for synchronous contexts
//! - [`Deferred`] - Deferred resolver for async Promise resolution
//! - [`PromiseFuture`] - Rust `Future` view of an ArkTS Promise
//!
//! # Examples
//!
//! ## Creating and immediately resolving a Promise
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! fn create_resolved_promise(env: &Env) -> Result<PromiseRaw> {
//!     let value = env.create_string("done")?;
//!     PromiseRaw::resolve(env, &value.into())
//! }
//! ```
//!
//! ## Creating a Promise with deferred resolution
//!
//! ```rust,ignore
//! use ani::prelude::*;
//! use std::thread;
//!
//! fn create_async_promise(env: &Env) -> Result<PromiseRaw> {
//!     let (deferred, promise) = PromiseRaw::deferred(env)?;
//!
//!     // Store deferred for later resolution (e.g., in another thread)
//!     // Note: For cross-thread use, you need to handle env lifetime carefully
//!     
//!     // Later, resolve or reject:
//!     let result = env.create_string("result")?;
//!     deferred.resolve(env, &result.into())?;
//!
//!     Ok(promise)
//! }
//! ```

use std::collections::HashMap;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock, Weak};
use std::task::{Context, Poll, Waker};

use crate::bindgen_runtime::ToAni as BindgenToAni;
use crate::env::Env;
use crate::error::{
    AniErrorDecodeLimits, AniErrorPayload, AniErrorValue, DynAniError, Error, Result, Status,
};
use crate::scheduler::{RuntimeCancellable, RuntimeRegistration};
use crate::sys;
use crate::types::{
    AniError, AniObject, AniRef, AniResolver, AniString, GlobalRef, ani_value_long, ani_value_ref,
};
use crate::vm::AniVm;
use crate::{ani_call, ani_call_2ret};

use super::function::{Function, FunctionRef, ToAniArgs};
use super::{FromAni, RefContainer, ToAni as ConversionToAni, TypeInfo, Unboxable};

const PROMISE_BRIDGE_OBSERVE: &str = "__ani_rs_observe_promise";
const PROMISE_BRIDGE_RESOLVE: &str = "__ani_rs_promise_resolve\0";
const PROMISE_BRIDGE_REJECT: &str = "__ani_rs_promise_reject\0";
const RUNTIME_TASK_CANCEL: &str = "__ani_rs_cancel_runtime_task\0";
const PROMISE_BRIDGE_SETTLE_SIGNATURE: &str = "lC{std.core.Object}:\0";
const PROMISE_BRIDGE_OBSERVE_SIGNATURE: &str = "C{std.core.Promise}l:";

trait PromiseBridgeObserver: Send + Sync {
    fn settle(&self, env: &Env<'_>, value: AniRef<'_>, rejected: bool);
}

type PromiseObserverRegistry = HashMap<u64, Weak<dyn PromiseBridgeObserver>>;

static PROMISE_BRIDGE_MODULES: OnceLock<RwLock<Vec<&'static str>>> = OnceLock::new();
static PROMISE_OBSERVERS: OnceLock<Mutex<PromiseObserverRegistry>> = OnceLock::new();
static NEXT_PROMISE_OBSERVER: AtomicU64 = AtomicU64::new(1);
static LIVE_DEFERREDS: AtomicUsize = AtomicUsize::new(0);

fn promise_bridge_modules() -> &'static RwLock<Vec<&'static str>> {
    PROMISE_BRIDGE_MODULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn promise_observers() -> &'static Mutex<PromiseObserverRegistry> {
    PROMISE_OBSERVERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Record a generated ETS module that contains the public Promise
/// continuation bridge. The derive-generated module constructor calls this
/// before native bindings are installed.
#[doc(hidden)]
pub fn register_promise_bridge_module(module: &'static str) {
    if let Ok(mut modules) = promise_bridge_modules().write()
        && !modules.contains(&module)
    {
        modules.push(module);
    }
}

/// Queue the native settlement callbacks for all generated ETS bridges.
#[doc(hidden)]
pub fn queue_registered_promise_bridges() -> sys::ani_status {
    let Ok(modules) = promise_bridge_modules().read() else {
        return sys::ani_status_ANI_ERROR;
    };
    for module in modules.iter().copied() {
        for (name, callback) in [
            (
                PROMISE_BRIDGE_RESOLVE,
                promise_bridge_resolve as *const () as *const c_void,
            ),
            (
                PROMISE_BRIDGE_REJECT,
                promise_bridge_reject as *const () as *const c_void,
            ),
        ] {
            let status = crate::module_register::queue_module_binding(
                module,
                name,
                PROMISE_BRIDGE_SETTLE_SIGNATURE,
                callback,
            );
            if status != sys::ani_status_ANI_OK {
                return status;
            }
        }
        let status = crate::module_register::queue_module_binding(
            module,
            RUNTIME_TASK_CANCEL,
            PROMISE_BRIDGE_SETTLE_SIGNATURE,
            crate::async_runtime::cancel_runtime_task_from_ets as *const () as *const c_void,
        );
        if status != sys::ani_status_ANI_OK {
            return status;
        }
    }
    sys::ani_status_ANI_OK
}

fn register_promise_observer(observer: &Arc<dyn PromiseBridgeObserver>) -> Result<u64> {
    let token = NEXT_PROMISE_OBSERVER.fetch_add(1, Ordering::Relaxed);
    promise_observers()
        .lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Promise observer registry poisoned"))?
        .insert(token, Arc::downgrade(observer));
    Ok(token)
}

fn unregister_promise_observer(token: u64) {
    if token != 0
        && let Ok(mut observers) = promise_observers().lock()
    {
        observers.remove(&token);
    }
}

/// Number of live generated-bridge observers retained by Rust.
pub fn live_promise_observer_count() -> usize {
    promise_observers()
        .lock()
        .map(|mut observers| {
            observers.retain(|_, observer| observer.strong_count() > 0);
            observers.len()
        })
        .unwrap_or(usize::MAX)
}

/// Number of typed Promise resolvers currently owned by Rust.
pub fn live_deferred_count() -> usize {
    LIVE_DEFERREDS.load(Ordering::Acquire)
}

fn dispatch_promise_settlement(
    env: *mut sys::ani_env,
    token: i64,
    value: sys::ani_ref,
    rejected: bool,
) {
    if env.is_null() || token <= 0 {
        return;
    }
    let observer = promise_observers()
        .lock()
        .ok()
        .and_then(|mut observers| observers.remove(&(token as u64)))
        .and_then(|observer| observer.upgrade());
    let Some(observer) = observer else {
        return;
    };
    let env = unsafe { Env::from_raw_unchecked(env) };
    observer.settle(&env, unsafe { AniRef::from_raw(value) }, rejected);
}

/// Native target called by the generated ETS `then` continuation.
#[doc(hidden)]
pub unsafe extern "C" fn promise_bridge_resolve(
    env: *mut sys::ani_env,
    token: i64,
    value: sys::ani_ref,
) {
    let _ = std::panic::catch_unwind(|| {
        dispatch_promise_settlement(env, token, value, false);
    });
}

/// Native target called by the generated ETS rejection continuation.
#[doc(hidden)]
pub unsafe extern "C" fn promise_bridge_reject(
    env: *mut sys::ani_env,
    token: i64,
    value: sys::ani_ref,
) {
    let _ = std::panic::catch_unwind(|| {
        dispatch_promise_settlement(env, token, value, true);
    });
}

/// Raw Promise value in ANI
///
/// `PromiseRaw<'env>` represents a raw Promise value. It contains a lifetime
/// so it can only be used in synchronous contexts.
///
/// Rejections may carry any [`AniErrorPayload`] without string erasure.
///
/// # Examples
///
/// ```rust,ignore
/// use ani::prelude::*;
///
/// // Create a resolved Promise
/// let value = env.create_string("done")?;
/// let promise = PromiseRaw::resolve(&env, &value.into())?;
///
/// // Create a rejected Promise
/// let promise = PromiseRaw::reject(&env, "error message")?;
/// ```
#[repr(transparent)]
pub struct PromiseRaw<'env, T = ()> {
    inner: sys::ani_object,
    _marker: PhantomData<(&'env (), T)>,
}

impl<T> TypeInfo for PromiseRaw<'_, T> {
    fn type_signature() -> &'static str {
        "Lstd/core/Promise;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T> PromiseRaw<'env, T> {
    /// Create a new PromiseRaw from raw ani_object
    ///
    /// # Safety
    ///
    /// The caller must ensure the raw value is a valid Promise object.
    #[inline]
    pub unsafe fn from_raw(raw: sys::ani_object) -> Self {
        Self {
            inner: raw,
            _marker: PhantomData,
        }
    }

    /// Get the raw ani_object
    #[inline]
    pub fn as_raw(&self) -> sys::ani_object {
        self.inner
    }

    /// Convert to raw ani_object, consuming self
    #[inline]
    pub fn into_raw(self) -> sys::ani_object {
        self.inner
    }

    /// Convert to AniObject
    #[inline]
    pub fn into_object(self) -> AniObject<'env> {
        unsafe { AniObject::from_raw(self.inner) }
    }

    /// Cast the Promise phantom payload type without changing the underlying value.
    #[inline]
    pub fn cast<U>(self) -> PromiseRaw<'env, U> {
        PromiseRaw {
            inner: self.inner,
            _marker: PhantomData,
        }
    }

    /// Rebind the phantom environment lifetime without changing the underlying value.
    #[inline]
    pub fn with_lifetime<'other>(self) -> PromiseRaw<'other, T> {
        PromiseRaw {
            inner: self.inner,
            _marker: PhantomData,
        }
    }

    /// Convenience helper for exported functions that only need to hand the promise
    /// back to ArkTS and do not retain any Rust-side borrow relationship.
    #[inline]
    pub fn into_static(self) -> PromiseRaw<'static, T> {
        self.with_lifetime()
    }

    /// Promotes this ArkTS Promise to a cancellable Rust [`Future`].
    ///
    /// The Promise is retained with a global ANI reference. Dropping or
    /// explicitly cancelling the future releases that reference; it does not
    /// attempt to abort the underlying ArkTS operation.
    pub fn into_future(self, env: &Env<'env>) -> Result<PromiseFuture<T>>
    where
        T: PromiseFutureValue,
    {
        PromiseFuture::new(env, self)
    }

    /// Promotes this Promise using an application-defined rejection decoder.
    pub fn into_future_with_decoder<E>(
        self,
        env: &Env<'env>,
        decoder: Arc<dyn RejectionDecoder<E>>,
    ) -> Result<PromiseFuture<T, E>>
    where
        T: PromiseFutureValue,
        E: Send + 'static,
    {
        PromiseFuture::with_decoder(env, self, decoder)
    }

    /// Create a new Promise and immediately resolve it with the given value
    ///
    /// This is a convenience method for creating a resolved Promise.
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The value to resolve the Promise with (as AniRef)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let result = env.create_string("done")?;
    /// let promise = PromiseRaw::resolve(&env, &result.into())?;
    /// ```
    pub fn resolve(env: &Env<'env>, value: &AniRef<'_>) -> Result<Self> {
        let (resolver, promise) = ani_call_2ret!(
            env,
            Promise_New,
            sys::ani_resolver,
            sys::ani_object,
            ptr::null_mut(),
            ptr::null_mut()
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to create promise"))?;

        ani_call!(env, PromiseResolver_Resolve, resolver, value.as_raw())
            .map_err(|_| Error::new(Status::GenericFailure, "Failed to resolve promise"))?;

        Ok(Self {
            inner: promise,
            _marker: PhantomData,
        })
    }

    /// Create a new Promise and immediately resolve it with an int value
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The int value to resolve with (will be boxed to Int)
    pub fn resolve_int(env: &Env<'env>, value: i32) -> Result<Self> {
        let boxed = create_boxed_int(env, value)?;
        Self::resolve(env, &boxed.into())
    }

    /// Create a new Promise and immediately resolve it with a string value
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The string value to resolve with
    pub fn resolve_string(env: &Env<'env>, value: &str) -> Result<Self> {
        let ani_str = env.create_string(value)?;
        Self::resolve(env, &ani_str.into())
    }

    /// Create a new Promise and immediately resolve it with any supported Rust value.
    pub fn resolve_value<V>(env: &Env<'env>, value: V) -> Result<Self>
    where
        V: PromiseValue<'env>,
    {
        let (deferred, promise) = Self::deferred(env)?;
        deferred.resolve_value(env, value)?;
        Ok(promise)
    }

    /// Create a new Promise and immediately reject it with the given error message
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `error` - The error message
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let promise = PromiseRaw::reject(&env, "Something went wrong")?;
    /// ```
    pub fn reject(env: &Env<'env>, error: impl AsRef<str>) -> Result<Self> {
        let (resolver, promise) = ani_call_2ret!(
            env,
            Promise_New,
            sys::ani_resolver,
            sys::ani_object,
            ptr::null_mut(),
            ptr::null_mut()
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to create promise"))?;

        let error_obj = create_promise_error(env, error.as_ref())?;
        ani_call!(env, PromiseResolver_Reject, resolver, error_obj.as_raw())
            .map_err(|_| Error::new(Status::GenericFailure, "Failed to reject promise"))?;

        Ok(Self {
            inner: promise,
            _marker: PhantomData,
        })
    }

    /// Create a new Promise and immediately reject it with a typed [`Error`].
    pub fn reject_with_error<S>(env: &Env<'env>, error: Error<S>) -> Result<Self>
    where
        S: AsRef<str> + std::fmt::Debug + Send + Sync + 'static,
    {
        Self::reject_with_payload(env, error)
    }

    /// Create a Promise rejected with an application-defined error payload.
    pub fn reject_with_payload<E: AniErrorPayload>(env: &Env<'env>, error: E) -> Result<Self> {
        let (deferred, promise) = Self::deferred(env)?;
        deferred.reject_with_payload(env, error)?;
        Ok(promise)
    }

    /// Create a new Promise with a deferred resolver
    ///
    /// Returns a tuple of (Deferred, PromiseRaw). The Deferred can be used
    /// to resolve or reject the Promise later.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let (deferred, promise) = PromiseRaw::deferred(&env)?;
    ///
    /// // Later...
    /// let result = env.create_string("done")?;
    /// deferred.resolve(&env, &result.into())?;
    /// ```
    pub fn deferred(env: &Env<'env>) -> Result<(Deferred<T>, Self)> {
        let (resolver, promise) = ani_call_2ret!(
            env,
            Promise_New,
            sys::ani_resolver,
            sys::ani_object,
            ptr::null_mut(),
            ptr::null_mut()
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to create promise"))?;

        let deferred = Deferred::from_resolver(unsafe { AniResolver::from_raw(resolver) });

        let promise_raw = Self {
            inner: promise,
            _marker: PhantomData,
        };

        Ok((deferred, promise_raw))
    }
}

/// Converts the reference stored in a settled ArkTS Promise into owned Rust
/// data.
///
/// Implement this trait for application object types that need to be awaited
/// from Rust. The conversion runs while the polling thread is attached to the
/// VM, so returned values must not retain local ANI references.
pub trait PromiseFutureValue: Send + Sized + 'static {
    /// Converts a resolved Promise reference into an owned Rust value.
    fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self>;
}

macro_rules! impl_primitive_promise_future_value {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl PromiseFutureValue for $ty {
                fn from_promise_ref<'env>(
                    env: &Env<'env>,
                    value: AniRef<'env>,
                ) -> Result<Self> {
                    let object =
                        unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
                    <Self as Unboxable<'env>>::unbox(env, &object)
                }
            }
        )+
    };
}

impl_primitive_promise_future_value!(bool, i8, i16, u16, i32, i64, f32, f64);

impl PromiseFutureValue for char {
    fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self> {
        let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        unsafe { Self::from_ani(env, value) }
    }
}

macro_rules! impl_checked_primitive_promise_future_value {
    ($ty:ty, $ani_ty:ty) => {
        impl PromiseFutureValue for $ty {
            fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self> {
                let value = <$ani_ty>::from_promise_ref(env, value)?;
                <$ty>::try_from(value).map_err(|_| {
                    Error::new(
                        Status::OutOfRange,
                        format!("Promise value {value} does not fit in {}", stringify!($ty)),
                    )
                })
            }
        }
    };
}

impl_checked_primitive_promise_future_value!(u8, i16);
impl_checked_primitive_promise_future_value!(u32, i64);
impl_checked_primitive_promise_future_value!(usize, i64);
impl_checked_primitive_promise_future_value!(isize, i64);

impl PromiseFutureValue for String {
    fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self> {
        let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        env.get_string(&value)
    }
}

impl PromiseFutureValue for super::BigInt {
    fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self> {
        unsafe { Self::from_ani(env, value.as_raw() as sys::ani_object) }
    }
}

macro_rules! impl_bigint_promise_future_value {
    ($ty:ty, $method:ident) => {
        impl PromiseFutureValue for $ty {
            fn from_promise_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<Self> {
                super::BigInt::from_promise_ref(env, value)?.$method()
            }
        }
    };
}

impl_bigint_promise_future_value!(u64, to_u64);
impl_bigint_promise_future_value!(i128, to_i128);
impl_bigint_promise_future_value!(u128, to_u128);

impl PromiseFutureValue for () {
    fn from_promise_ref<'env>(_env: &Env<'env>, _value: AniRef<'env>) -> Result<Self> {
        Ok(())
    }
}

/// Default open-ended rejection returned when Rust awaits an ArkTS Promise.
///
/// The payload is not normalized to ani-rs' built-in [`Error`]. An ArkTS
/// rejection retains its exact object, while runtime cancellation retains the
/// application payload created by the registered cancellation factory.
#[derive(Debug)]
pub struct ArktsRejection {
    payload: DynAniError,
}

impl ArktsRejection {
    /// Wrap any structured rejection payload without erasing its materializer.
    pub fn new(payload: DynAniError) -> Self {
        Self { payload }
    }

    /// Borrow the original open-ended payload.
    pub fn payload(&self) -> &dyn AniErrorPayload {
        self.payload.as_ref()
    }

    /// Consume this rejection and recover the original payload.
    pub fn into_payload(self) -> DynAniError {
        self.payload
    }
}

impl std::fmt::Display for ArktsRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload.fmt(formatter)
    }
}

impl AniErrorPayload for ArktsRejection {
    fn ani_status(&self) -> &str {
        self.payload.ani_status()
    }

    fn ani_code(&self) -> i32 {
        self.payload.ani_code()
    }

    fn ani_message(&self) -> &str {
        self.payload.ani_message()
    }

    fn ani_cause(&self) -> Option<&dyn AniErrorPayload> {
        self.payload.ani_cause()
    }

    fn visit_ani_metadata(&self, visitor: &mut dyn FnMut(&str, &str)) {
        self.payload.visit_ani_metadata(visitor);
    }

    fn visit_ani_properties(&self, visitor: &mut dyn FnMut(&str, &AniErrorValue)) {
        self.payload.visit_ani_properties(visitor);
    }

    fn ani_stack(&self) -> Option<&str> {
        self.payload.ani_stack()
    }

    fn materialize_ani_error<'env>(&self, env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        self.payload.materialize_ani_error(env)
    }
}

/// Object-safe conversion from an ArkTS rejection into any application error.
pub trait RejectionDecoder<E>: Send + Sync + 'static {
    /// Decode the exact rejected value while attached to its owning VM.
    fn decode(&self, env: &Env<'_>, rejection: AniRef<'_>) -> E;

    /// Convert ani-rs infrastructure failures and Rust-side cancellation.
    fn runtime_error(&self, error: DynAniError) -> E;
}

/// Bounded default decoder preserving name, message, code, stack, typed
/// metadata, cause relationships, and the exact raw rejection.
#[derive(Clone, Copy, Debug, Default)]
pub struct ArktsRejectionDecoder {
    limits: AniErrorDecodeLimits,
}

impl ArktsRejectionDecoder {
    /// Creates a decoder with explicit untrusted-graph limits.
    pub const fn new(limits: AniErrorDecodeLimits) -> Self {
        Self { limits }
    }

    /// Returns the configured graph limits.
    pub const fn limits(&self) -> AniErrorDecodeLimits {
        self.limits
    }
}

impl RejectionDecoder<ArktsRejection> for ArktsRejectionDecoder {
    fn decode(&self, env: &Env<'_>, rejection: AniRef<'_>) -> ArktsRejection {
        ArktsRejection::new(Box::new(promise_rejection_error_with_limits(
            env,
            rejection,
            self.limits,
        )))
    }

    fn runtime_error(&self, error: DynAniError) -> ArktsRejection {
        ArktsRejection::new(error)
    }
}

struct PromiseFutureState<T, E> {
    vm: AniVm,
    promise: Mutex<Option<GlobalRef>>,
    bridge_token: AtomicU64,
    cancelled: AtomicBool,
    finished: AtomicBool,
    result: Mutex<Option<std::result::Result<T, E>>>,
    waker: Mutex<Option<Waker>>,
    registration: Mutex<Option<RuntimeRegistration>>,
    decoder: Arc<dyn RejectionDecoder<E>>,
}

impl<T, E: Send + 'static> PromiseFutureState<T, E> {
    fn finish(&self, result: std::result::Result<T, E>) {
        if let Ok(mut slot) = self.result.lock() {
            *slot = Some(result);
        }
        if let Ok(mut waker) = self.waker.lock()
            && let Some(waker) = waker.take()
        {
            waker.wake();
        }
    }

    fn release_promise(&self) -> Result<()> {
        let global = self
            .promise
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "Promise reference lock poisoned"))?
            .take();
        match global {
            Some(global) => self.vm.with_attached(|env| env.delete_global_ref(global)),
            None => Ok(()),
        }
    }

    fn cancel_wait(&self, reason: crate::async_runtime::RuntimeCancelReason) -> bool {
        if self.cancelled.swap(true, Ordering::AcqRel) {
            return false;
        }
        unregister_promise_observer(self.bridge_token.swap(0, Ordering::AcqRel));
        if !self.finished.swap(true, Ordering::AcqRel) {
            let result = match self.release_promise() {
                Ok(()) => Err(self
                    .decoder
                    .runtime_error(crate::async_runtime::runtime_cancellation_error(reason))),
                Err(error) => Err(self.decoder.runtime_error(Box::new(error))),
            };
            self.finish(result);
            if let Ok(mut registration) = self.registration.lock() {
                registration.take();
            }
        }
        true
    }
}

impl<T: Send + 'static, E: Send + 'static> RuntimeCancellable for PromiseFutureState<T, E> {
    fn cancel_for_runtime_shutdown(&self) {
        self.cancel_wait(crate::async_runtime::RuntimeCancelReason::Shutdown);
    }
}

impl<T: PromiseFutureValue, E: Send + 'static> PromiseBridgeObserver for PromiseFutureState<T, E> {
    fn settle(&self, env: &Env<'_>, value: AniRef<'_>, rejected: bool) {
        self.bridge_token.store(0, Ordering::Release);
        if self.finished.swap(true, Ordering::AcqRel) {
            return;
        }
        let result = if rejected {
            Err(self.decoder.decode(env, value))
        } else {
            T::from_promise_ref(env, value)
                .map_err(|error| self.decoder.runtime_error(Box::new(error)))
        };
        let result = match self.release_promise() {
            Ok(()) => result,
            Err(error) => Err(self.decoder.runtime_error(Box::new(error))),
        };
        self.finish(result);
        if let Ok(mut registration) = self.registration.lock() {
            registration.take();
        }
    }
}

/// A cancellable Rust [`Future`](std::future::Future) backed by an ArkTS
/// Promise.
///
/// ANI currently exposes Promise creation and settlement but no native
/// continuation-registration API. ani-rs therefore asks its generated ETS
/// bridge to attach public `then` continuations. Those continuations settle a
/// tokenized Rust waiter directly; no runtime-private Promise fields are read
/// and no scheduler worker or timer is consumed while the Promise is pending.
pub struct PromiseFuture<T, E: Send + 'static = ArktsRejection> {
    state: Arc<PromiseFutureState<T, E>>,
    completed: bool,
}

impl<T: PromiseFutureValue> PromiseFuture<T, ArktsRejection> {
    /// Creates a future and promotes the Promise to a global ANI reference.
    pub fn new<'env>(env: &Env<'env>, promise: PromiseRaw<'env, T>) -> Result<Self> {
        Self::with_decoder(env, promise, Arc::new(ArktsRejectionDecoder::default()))
    }
}

impl<T: PromiseFutureValue, E: Send + 'static> PromiseFuture<T, E> {
    /// Creates a future with an application-provided, object-safe rejection
    /// decoder. The decoder controls both ArkTS rejections and runtime-originated
    /// cancellation errors.
    pub fn with_decoder<'env>(
        env: &Env<'env>,
        promise: PromiseRaw<'env, T>,
        decoder: Arc<dyn RejectionDecoder<E>>,
    ) -> Result<Self> {
        let promise_ref = unsafe { AniRef::from_raw(promise.into_raw() as sys::ani_ref) };
        let state = Arc::new(PromiseFutureState {
            vm: env.get_vm()?,
            promise: Mutex::new(Some(env.create_global_ref(&promise_ref)?)),
            bridge_token: AtomicU64::new(0),
            result: Mutex::new(None),
            waker: Mutex::new(None),
            cancelled: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            registration: Mutex::new(None),
            decoder,
        });
        let registration = match crate::scheduler::shared().register_cancellable(&state) {
            Ok(registration) => registration,
            Err(error) => {
                let _ = state.release_promise();
                return Err(error);
            }
        };
        *state.registration.lock().map_err(|_| {
            Error::new(Status::GenericFailure, "Promise registration lock poisoned")
        })? = Some(registration);

        let observer: Arc<dyn PromiseBridgeObserver> = state.clone();
        let token = register_promise_observer(&observer)?;
        state.bridge_token.store(token, Ordering::Release);
        if let Err(error) = attach_promise_bridge(env, &promise_ref, token) {
            unregister_promise_observer(state.bridge_token.swap(0, Ordering::AcqRel));
            let _ = state.release_promise();
            return Err(error);
        }
        Ok(Self {
            state,
            completed: false,
        })
    }

    /// Releases this future's Promise reference.
    ///
    /// Cancellation is idempotent and does not abort the ArkTS operation
    /// itself because ANI has no Promise cancellation primitive.
    pub fn cancel(&mut self) -> Result<bool> {
        Ok(self
            .state
            .cancel_wait(crate::async_runtime::RuntimeCancelReason::Explicit(
                "Promise future was cancelled".into(),
            )))
    }

    /// Returns whether the Rust-side wait has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }
}

fn attach_promise_bridge(env: &Env<'_>, promise: &AniRef<'_>, token: u64) -> Result<()> {
    let modules = promise_bridge_modules()
        .read()
        .map_err(|_| Error::new(Status::GenericFailure, "Promise bridge registry poisoned"))?;
    let mut last_error = None;
    for descriptor in modules.iter().rev() {
        let result = (|| {
            let module = env.find_module(descriptor)?;
            let function = env.find_module_function(
                &module,
                PROMISE_BRIDGE_OBSERVE,
                PROMISE_BRIDGE_OBSERVE_SIGNATURE,
            )?;
            env.call_function_void(
                &function,
                &[
                    ani_value_ref(promise.as_raw()),
                    ani_value_long(token as i64),
                ],
            )
        })();
        match result {
            Ok(()) => return Ok(()),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or_else(|| {
        Error::new(
            Status::NotFound,
            "generated ETS Promise continuation bridge is not registered",
        )
    }))
}

impl<T: PromiseFutureValue, E: Send + 'static> std::future::Future for PromiseFuture<T, E> {
    type Output = std::result::Result<T, E>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.completed, "PromiseFuture polled after completion");
        let ready = if let Ok(mut result) = self.state.result.lock() {
            let ready = result.take();
            // Publish the waker while holding the result lock so completion
            // cannot occur between the empty check and waker registration.
            if ready.is_none()
                && let Ok(mut waker) = self.state.waker.lock()
            {
                *waker = Some(context.waker().clone());
            }
            ready
        } else {
            None
        };
        if let Some(result) = ready {
            self.completed = true;
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

impl<T, E: Send + 'static> Drop for PromiseFuture<T, E> {
    fn drop(&mut self) {
        self.state
            .cancel_wait(crate::async_runtime::RuntimeCancelReason::Dropped);
    }
}

struct RejectionGraphState {
    limits: AniErrorDecodeLimits,
    nodes: usize,
    visited: Vec<sys::ani_ref>,
}

impl RejectionGraphState {
    fn identity(&self, env: &Env<'_>, value: &AniRef<'_>) -> Option<usize> {
        self.visited.iter().position(|previous| {
            let previous = unsafe { AniRef::from_raw(*previous) };
            env.reference_strict_equals(&previous, value)
                .unwrap_or(false)
        })
    }
}

fn promise_rejection_error_with_limits(
    env: &Env<'_>,
    value: AniRef<'_>,
    limits: AniErrorDecodeLimits,
) -> Error {
    decode_promise_rejection(
        env,
        value,
        0,
        &mut RejectionGraphState {
            limits,
            nodes: 0,
            visited: Vec::new(),
        },
    )
}

fn decode_promise_rejection(
    env: &Env<'_>,
    value: AniRef<'_>,
    depth: usize,
    graph: &mut RejectionGraphState,
) -> Error {
    if depth > graph.limits.max_depth || graph.nodes >= graph.limits.max_nodes {
        let mut error = Error::new(
            Status::OutOfRange,
            "ArkTS rejection cause graph exceeds configured decode limits",
        )
        .with_status_name("ArktsRejectionLimit");
        error.preserve_rejection(RefContainer::new(env, &value).ok());
        return error;
    }
    graph.nodes += 1;
    graph.visited.push(value.as_raw());
    let error_object = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
    let string_property = |name: &str| {
        env.get_property_by_name_ref(&error_object, name)
            .and_then(|value| {
                if env.is_nullish(&value)? {
                    return Err(Error::new(
                        Status::NotFound,
                        "error string property is absent",
                    ));
                }
                let object = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
                let string_class = env.find_class("std.core.String")?;
                if !env.object_instance_of(&object, &string_class)? {
                    return Err(Error::new(
                        Status::InvalidType,
                        "error property is not a string",
                    ));
                }
                let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
                env.get_string(&value)
            })
            .ok()
    };
    let reason = string_property("message").unwrap_or_else(|| {
        let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        env.get_string(&value)
            .unwrap_or_else(|_| "ArkTS Promise rejected".to_string())
    });
    let status = string_property("name").unwrap_or_else(|| "GenericFailure".to_string());
    let code = env.get_property_by_name_int(&error_object, "code").ok();
    let mut error = Error::new(Status::GenericFailure, reason).with_status_name(status);
    error.code = code;
    error.set_stack(string_property("stack"));
    error.preserve_rejection(RefContainer::new(env, &value).ok());

    if let Ok(context) = env.get_property_by_name_ref(&error_object, "cause")
        && !env.is_nullish(&context).unwrap_or(true)
    {
        decode_error_context(env, &context, &mut error, depth, graph);
    }
    error
}

fn decode_error_context(
    env: &Env<'_>,
    context: &AniRef<'_>,
    error: &mut Error,
    depth: usize,
    graph: &mut RejectionGraphState,
) {
    if env.is_nullish(context).unwrap_or(true) {
        return;
    }
    let context_object = unsafe { AniObject::from_raw(context.as_raw() as sys::ani_object) };
    let Ok(values) = (unsafe {
        std::collections::HashMap::<String, AniRef<'_>>::from_ani(env, context_object.as_raw())
    }) else {
        if let Some(index) = graph.identity(env, context) {
            error.insert_property("$causeRef".into(), AniErrorValue::Integer(index as i64));
        } else {
            error.cause = Some(Box::new(decode_promise_rejection(
                env,
                unsafe { AniRef::from_raw(context.as_raw()) },
                depth + 1,
                graph,
            )));
        }
        return;
    };

    if let Some(status) = values.get("status") {
        let status = unsafe { AniString::from_raw(status.as_raw() as sys::ani_string) };
        if let Ok(status) = env.get_string(&status) {
            error.status_name = Some(status);
        }
    }
    if let Some(metadata) = values.get("metadata") {
        let metadata = unsafe { AniObject::from_raw(metadata.as_raw() as sys::ani_object) };
        if let Ok(properties) = unsafe {
            std::collections::HashMap::<String, AniRef<'_>>::from_ani(env, metadata.as_raw())
        } {
            for (key, value) in properties {
                if let Ok(value) =
                    AniErrorValue::from_ani_ref_with_limits(env, &value, graph.limits)
                {
                    if let AniErrorValue::String(text) = &value {
                        error.metadata.insert(key.clone(), text.clone());
                    }
                    error.insert_property(key, value);
                }
            }
        }
    }
    if let Some(cause) = values.get("cause") {
        if let Some(index) = graph.identity(env, cause) {
            error.insert_property("$causeRef".into(), AniErrorValue::Integer(index as i64));
        } else {
            error.cause = Some(Box::new(decode_promise_rejection(
                env,
                unsafe { AniRef::from_raw(cause.as_raw()) },
                depth + 1,
                graph,
            )));
        }
    }
}

/// Deferred resolver for Promise
///
/// `Deferred` holds the resolver part of a Promise and can be used to
/// resolve or reject the Promise at a later time.
///
/// # Thread Safety
///
/// The `Deferred` itself is `Send`, but resolving/rejecting requires access
/// to the ANI environment, which has thread restrictions. For cross-thread
/// use, you need to use `AttachCurrentThread` to get a valid env.
///
/// # Examples
///
/// ```rust,ignore
/// use ani::prelude::*;
///
/// fn async_operation(env: &Env) -> Result<PromiseRaw> {
///     let (deferred, promise) = PromiseRaw::deferred(env)?;
///
///     // Store deferred somewhere for later use
///     // When ready, resolve:
///     let result = env.create_string("done")?;
///     deferred.resolve(env, &result.into())?;
///
///     Ok(promise)
/// }
/// ```
pub struct Deferred<T = ()> {
    resolver: AniResolver,
    _marker: PhantomData<fn() -> T>,
    _metric: Arc<DeferredMetric>,
}

struct DeferredMetric;

impl DeferredMetric {
    fn new() -> Arc<Self> {
        LIVE_DEFERREDS.fetch_add(1, Ordering::AcqRel);
        Arc::new(Self)
    }
}

impl Drop for DeferredMetric {
    fn drop(&mut self) {
        LIVE_DEFERREDS.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> TypeInfo for Deferred<T> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_resolver"
    }
}

/// Value that can resolve an ANI Promise.
pub trait PromiseValue<'env>: Sized {
    /// Convert the Rust value into a reference value that ANI Promise APIs accept.
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>>;
}

// Deferred can be sent across threads
// But resolving/rejecting requires a valid env for the current thread
unsafe impl<T> Send for Deferred<T> {}
unsafe impl<T> Sync for Deferred<T> {}

impl AniResolver {
    /// Resolve the associated Promise with a reference value.
    pub fn resolve_ref(&self, env: &Env<'_>, value: &AniRef<'_>) -> Result<()> {
        env.promise_resolve(self, value)
    }

    /// Resolve the associated Promise with any supported Rust value.
    pub fn resolve_value<'env, T>(&self, env: &Env<'env>, value: T) -> Result<()>
    where
        T: PromiseValue<'env>,
    {
        let value_ref = value.into_promise_ref(env)?;
        self.resolve_ref(env, &value_ref)
    }

    /// Reject the associated Promise with an ANI error object.
    pub fn reject_error(&self, env: &Env<'_>, error: &AniError<'_>) -> Result<()> {
        env.promise_reject(self, error)
    }

    /// Reject the associated Promise with a string message.
    pub fn reject_message(&self, env: &Env<'_>, error: impl AsRef<str>) -> Result<()> {
        env.promise_reject_with_message(self, error.as_ref())
    }

    /// Reject the associated Promise with a typed [`Error`].
    pub fn reject_with_error<S>(&self, env: &Env<'_>, error: Error<S>) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug + Send + Sync + 'static,
    {
        self.reject_with_payload(env, &error)
    }

    /// Reject with any application-defined structured error payload.
    pub fn reject_with_payload(&self, env: &Env<'_>, error: &dyn AniErrorPayload) -> Result<()> {
        let error = crate::error::payload_to_ani_error(env, error)?;
        self.reject_error(env, &error)
    }

    /// Wrap this raw resolver in a typed [`Deferred<T>`] facade.
    #[inline]
    pub fn into_deferred<T>(self) -> Deferred<T> {
        Deferred::from_resolver(self)
    }
}

macro_rules! impl_boxed_promise_value {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'env> PromiseValue<'env> for $ty {
                fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
                    let boxed = <Self as crate::conversions::Boxable<'env>>::box_value(self, env)?;
                    Ok(boxed.into())
                }
            }
        )+
    };
}

impl_boxed_promise_value!(bool, i8, i16, u16, i32, i64, f32, f64);

impl<'env> PromiseValue<'env> for char {
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = <Self as ConversionToAni<'env>>::to_ani(self, env)?;
        Ok(value.into())
    }
}

macro_rules! impl_checked_boxed_promise_value {
    ($ty:ty, $ani_ty:ty) => {
        impl<'env> PromiseValue<'env> for $ty {
            fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
                let value = <$ani_ty>::try_from(self).map_err(|_| {
                    Error::new(
                        Status::OutOfRange,
                        concat!("Rust ", stringify!($ty), " does not fit in ArkTS primitive"),
                    )
                })?;
                let boxed = <$ani_ty as crate::conversions::Boxable<'env>>::box_value(value, env)?;
                Ok(boxed.into())
            }
        }
    };
}

impl_checked_boxed_promise_value!(u8, i16);
impl_checked_boxed_promise_value!(u32, i64);
impl_checked_boxed_promise_value!(usize, i64);
impl_checked_boxed_promise_value!(isize, i64);

impl<'env> PromiseValue<'env> for String {
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = env.create_string(&self)?;
        Ok(value.into())
    }
}

impl<'env> PromiseValue<'env> for AniString<'env> {
    fn into_promise_ref(self, _env: &Env<'env>) -> Result<AniRef<'env>> {
        Ok(self.into())
    }
}

impl<'env> PromiseValue<'env> for AniRef<'env> {
    fn into_promise_ref(self, _env: &Env<'env>) -> Result<AniRef<'env>> {
        Ok(self)
    }
}

impl<'env> PromiseValue<'env> for super::BigInt {
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = <Self as ConversionToAni<'env>>::to_ani(self, env)?;
        Ok(unsafe { AniRef::from_raw(value as sys::ani_ref) })
    }
}

macro_rules! impl_bigint_promise_value {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'env> PromiseValue<'env> for $ty {
                fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
                    super::BigInt::from(self).into_promise_ref(env)
                }
            }
        )+
    };
}

impl_bigint_promise_value!(u64, i128, u128);

impl<'env, Args, Return> PromiseValue<'env> for Function<'env, Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a> + TypeInfo,
{
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = <Self as ConversionToAni<'env>>::to_ani(self, env)?;
        Ok(unsafe { AniRef::from_raw(value as sys::ani_ref) })
    }
}

impl<'env, Args, Return> PromiseValue<'env> for FunctionRef<Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a> + TypeInfo,
{
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = <Self as ConversionToAni<'env>>::to_ani(self, env)?;
        Ok(unsafe { AniRef::from_raw(value as sys::ani_ref) })
    }
}

impl<'env> PromiseValue<'env> for () {
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        // `Promise<void>` resolves to `undefined` in ArkTS.
        let raw = env.get_undefined_object()?;
        Ok(unsafe { AniRef::from_raw(raw as sys::ani_ref) })
    }
}

impl<'env, T> PromiseValue<'env> for T
where
    T: BindgenToAni<'env, Output = sys::ani_object>,
{
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        Ok(unsafe { AniRef::from_raw(self.to_ani(env)? as sys::ani_ref) })
    }
}

impl<T> Deferred<T> {
    /// Create a new typed deferred/promise pair.
    #[inline]
    pub fn new<'env>(env: &Env<'env>) -> Result<(Self, PromiseRaw<'env, T>)> {
        PromiseRaw::deferred(env)
    }

    /// Build a typed deferred facade from an existing raw resolver.
    #[inline]
    pub fn from_resolver(resolver: AniResolver) -> Self {
        Self {
            resolver,
            _marker: PhantomData,
            _metric: DeferredMetric::new(),
        }
    }

    /// Rebind the phantom payload type without changing the underlying resolver.
    #[inline]
    pub fn cast<U>(self) -> Deferred<U> {
        Deferred {
            resolver: self.resolver,
            _marker: PhantomData,
            _metric: self._metric,
        }
    }

    /// Borrow the underlying raw ANI resolver wrapper.
    #[inline]
    pub fn as_resolver(&self) -> &AniResolver {
        &self.resolver
    }

    /// Consume this typed facade and return the underlying resolver.
    #[inline]
    pub fn into_resolver(self) -> AniResolver {
        self.resolver
    }

    /// Resolve the Promise with a value
    ///
    /// After calling this method, the Deferred is consumed and the resolver
    /// is freed.
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The value to resolve with (as AniRef)
    pub fn resolve(self, env: &Env<'_>, value: &AniRef<'_>) -> Result<()> {
        self.resolver.resolve_ref(env, value)
    }

    /// Resolve the Promise with an int value
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The int value to resolve with (will be boxed)
    pub fn resolve_int(self, env: &Env<'_>, value: i32) -> Result<()> {
        let boxed = create_boxed_int(env, value)?;
        self.resolve(env, &boxed.into())
    }

    /// Resolve the Promise with a string value
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `value` - The string value to resolve with
    pub fn resolve_string(self, env: &Env<'_>, value: &str) -> Result<()> {
        let ani_str = env.create_string(value)?;
        self.resolve(env, &ani_str.into())
    }

    /// Resolve the Promise with any supported Rust value.
    pub fn resolve_value<'env, V>(self, env: &Env<'env>, value: V) -> Result<()>
    where
        V: PromiseValue<'env>,
    {
        let value_ref = value.into_promise_ref(env)?;
        self.resolve(env, &value_ref)
    }

    /// Reject the Promise with an error message
    ///
    /// After calling this method, the Deferred is consumed and the resolver
    /// is freed.
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `error` - The error message
    pub fn reject(self, env: &Env<'_>, error: impl AsRef<str>) -> Result<()> {
        self.resolver.reject_message(env, error)
    }

    /// Reject the Promise with an Error
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `error` - The Error to reject with
    pub fn reject_with_error<S>(self, env: &Env<'_>, error: Error<S>) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug + Send + Sync + 'static,
    {
        self.resolver.reject_with_error(env, error)
    }

    /// Reject the Promise with a custom structured error payload.
    pub fn reject_with_payload<E: AniErrorPayload>(self, env: &Env<'_>, error: E) -> Result<()> {
        self.resolver.reject_with_payload(env, &error)
    }

    /// Get the raw resolver
    ///
    /// This is useful when you need to pass the resolver to FFI functions.
    #[inline]
    pub fn as_raw(&self) -> sys::ani_resolver {
        self.resolver.as_raw()
    }

    /// Consume and get the raw resolver
    ///
    /// This is useful when you need to pass the resolver to FFI functions
    /// and manage its lifetime manually.
    #[inline]
    pub fn into_raw(self) -> sys::ani_resolver {
        self.resolver.as_raw()
    }
}

// ============================================================================
// Type Conversions
// ============================================================================

impl<'env, T> ConversionToAni<'env> for PromiseRaw<'env, T> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env, T> FromAni<'env> for PromiseRaw<'env, T> {
    type Input = sys::ani_object;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Null pointer: promise"));
        }
        Ok(unsafe { Self::from_raw(value) })
    }
}

impl<'env, T> ConversionToAni<'env> for Deferred<T> {
    type Output = sys::ani_resolver;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env, T> FromAni<'env> for Deferred<T> {
    type Input = sys::ani_resolver;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Null pointer: resolver"));
        }
        Ok(Self::from_resolver(unsafe { AniResolver::from_raw(value) }))
    }
}

impl<'env, T> From<PromiseRaw<'env, T>> for AniObject<'env> {
    fn from(promise: PromiseRaw<'env, T>) -> Self {
        unsafe { AniObject::from_raw(promise.inner) }
    }
}

impl<'env, T> From<PromiseRaw<'env, T>> for sys::ani_object {
    fn from(promise: PromiseRaw<'env, T>) -> Self {
        promise.inner
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a boxed Int value
fn create_boxed_int<'a>(env: &Env<'a>, value: i32) -> Result<AniObject<'a>> {
    use crate::types::ani_value_int;

    // Find std/core/Int class
    let int_class = env.find_class("std.core.Int")?;

    // Find constructor <ctor>(I:V)
    let ctor = env.find_constructor(&int_class, "i:")?;

    // Create boxed object
    let args = [ani_value_int(value)];
    env.new_object(&int_class, &ctor, &args)
}

/// Create a boxed Long value
#[allow(dead_code)]
fn create_boxed_long<'a>(env: &Env<'a>, value: i64) -> Result<AniObject<'a>> {
    use crate::types::ani_value_long;

    // Find std/core/Long class
    let long_class = env.find_class("std.core.Long")?;

    // Find constructor <ctor>(J:V)
    let ctor = env.find_constructor(&long_class, "l:")?;

    // Create boxed object
    let args = [ani_value_long(value)];
    env.new_object(&long_class, &ctor, &args)
}

/// Create a boxed Double value
#[allow(dead_code)]
fn create_boxed_double<'a>(env: &Env<'a>, value: f64) -> Result<AniObject<'a>> {
    use crate::types::ani_value_double;

    // Find std/core/Double class
    let double_class = env.find_class("std.core.Double")?;

    // Find constructor <ctor>(D:V)
    let ctor = env.find_constructor(&double_class, "d:")?;

    // Create boxed object
    let args = [ani_value_double(value)];
    env.new_object(&double_class, &ctor, &args)
}

/// Create a boxed Boolean value
#[allow(dead_code)]
pub(crate) fn create_promise_error<'a>(
    env: &Env<'a>,
    message: &str,
) -> Result<crate::types::AniError<'a>> {
    if let Ok(err_cls) = env.find_class("std.core.Error")
        && let Ok(err_ctor) =
            env.find_constructor(&err_cls, "C{std.core.String}C{std.core.ErrorOptions}:")
    {
        let text = env.create_string(message)?;
        let undefined = env.get_undefined_object()?;
        let args = [
            crate::types::ani_value_ref(text.as_raw() as sys::ani_ref),
            crate::types::ani_value_ref(undefined as sys::ani_ref),
        ];
        let err_obj = env.new_object(&err_cls, &err_ctor, &args)?;
        return Ok(unsafe {
            crate::types::AniError::from_raw(err_obj.into_raw() as sys::ani_error)
        });
    }

    // Compatibility fallback for older runtimes.
    let err_cls = env
        .find_class("escompat.Error")
        .or_else(|_| env.find_class("@ohos.base.BusinessError"))?;
    let err_ctor = env.find_constructor(&err_cls, ":")?;
    let err_obj = env.new_object(&err_cls, &err_ctor, &[])?;

    let name = env.create_string("Error")?;
    let text = env.create_string(message)?;
    let name_ref = unsafe { AniRef::from_raw(name.into_raw() as sys::ani_ref) };
    let text_ref = unsafe { AniRef::from_raw(text.into_raw() as sys::ani_ref) };
    let _ = env.set_property_by_name_ref(&err_obj, "name", &name_ref);
    let _ = env.set_property_by_name_ref(&err_obj, "message", &text_ref);

    Ok(unsafe { crate::types::AniError::from_raw(err_obj.into_raw() as sys::ani_error) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deferred_type_is_send_sync_for_any_payload() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Deferred<()>>();
        assert_send_sync::<Deferred<String>>();
        assert_send_sync::<Deferred<FunctionRef<(String,), String>>>();
    }

    #[test]
    fn deferred_cast_and_resolver_bridge_compile() {
        fn compile<'env>(env: &Env<'env>, resolver: AniResolver, value: AniRef<'env>) {
            let deferred = Deferred::<String>::from_resolver(resolver);
            let _ = deferred.as_resolver().resolve_ref(env, &value);
            let _ = deferred.as_resolver().resolve_value(env, "ok".to_string());
            let _ = deferred
                .as_resolver()
                .reject_with_error(env, Error::new(Status::InvalidArgs, "bad"));
            let _ = env.promise_resolved("done".to_string());
            let _ = env.promise_rejected::<String>("boom");
            let _ = env
                .promise_rejected_with_error::<String, _>(Error::new(Status::InvalidArgs, "boom"));

            let deferred = deferred.cast::<bool>();
            let resolver = deferred.into_resolver();
            let _: Deferred<bool> = resolver.into_deferred();
            let _ = Deferred::<String>::new(env);
            let _ = unsafe {
                PromiseRaw::<String>::from_ani(
                    env,
                    PromiseRaw::<String>::resolve_string(env, "ok")
                        .expect("promise should resolve")
                        .into_raw(),
                )
            };
            let _ = unsafe { Deferred::<String>::from_ani(env, resolver.as_raw()) };
        }

        let _ = compile;
    }
}
