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

use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use crate::bindgen_runtime::ToAni as BindgenToAni;
use crate::env::Env;
use crate::error::{AniErrorPayload, Error, Result, Status};
use crate::sys;
use crate::types::{AniError, AniObject, AniRef, AniResolver, AniString, GlobalRef};
use crate::vm::AniVm;
use crate::{ani_call, ani_call_2ret};

use super::function::{Function, FunctionRef, ToAniArgs};
use super::{FromAni, ToAni as ConversionToAni, TypeInfo, Unboxable};

const PROMISE_STATE_PENDING: i32 = 0;
const PROMISE_STATE_LINKED: i32 = 1;
const PROMISE_STATE_RESOLVED: i32 = 2;
const PROMISE_STATE_REJECTED: i32 = 3;
const DEFAULT_PROMISE_POLL_INTERVAL: Duration = Duration::from_millis(2);

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

        let deferred = Deferred {
            resolver: unsafe { AniResolver::from_raw(resolver) },
            _marker: PhantomData,
        };

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

struct PromiseFutureState<T> {
    vm: AniVm,
    promise: Mutex<Option<GlobalRef>>,
    cancelled: AtomicBool,
    interval_ms: AtomicU64,
    result: Mutex<Option<Result<T>>>,
    waker: Mutex<Option<Waker>>,
}

impl<T> PromiseFutureState<T> {
    fn finish(&self, result: Result<T>) {
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
}

enum PromisePoll<T> {
    Pending,
    Ready(Result<T>),
}

/// A cancellable Rust [`Future`](std::future::Future) backed by an ArkTS
/// Promise.
///
/// ANI currently exposes Promise creation and settlement but no native
/// continuation-registration API. A single background observer therefore
/// checks the runtime Promise state on a short timer and wakes the Rust task
/// only when the Promise settles or is cancelled. It never blocks or repeatedly
/// reschedules the executor, retains the Promise with a global reference, and
/// automatically attaches the observer to the owning VM.
pub struct PromiseFuture<T> {
    state: Arc<PromiseFutureState<T>>,
    completed: bool,
}

impl<T: PromiseFutureValue> PromiseFuture<T> {
    /// Creates a future and promotes the Promise to a global ANI reference.
    pub fn new<'env>(env: &Env<'env>, promise: PromiseRaw<'env, T>) -> Result<Self> {
        let promise_ref = unsafe { AniRef::from_raw(promise.into_raw() as sys::ani_ref) };
        let state = Arc::new(PromiseFutureState {
            vm: env.get_vm()?,
            promise: Mutex::new(Some(env.create_global_ref(&promise_ref)?)),
            result: Mutex::new(None),
            waker: Mutex::new(None),
            interval_ms: AtomicU64::new(DEFAULT_PROMISE_POLL_INTERVAL.as_millis() as u64),
            cancelled: AtomicBool::new(false),
        });
        let observer = Arc::clone(&state);
        if let Err(error) = crate::scheduler::shared().schedule(move || observe_promise(observer)) {
            let _ = state.release_promise();
            return Err(error);
        }
        Ok(Self {
            state,
            completed: false,
        })
    }

    /// Changes the interval used while the Promise is pending.
    ///
    /// Values below one millisecond are clamped to one millisecond to avoid a
    /// busy loop.
    pub fn with_poll_interval(self, interval: Duration) -> Self {
        self.state.interval_ms.store(
            interval.max(Duration::from_millis(1)).as_millis() as u64,
            Ordering::Release,
        );
        self
    }

    /// Releases this future's Promise reference.
    ///
    /// Cancellation is idempotent and does not abort the ArkTS operation
    /// itself because ANI has no Promise cancellation primitive.
    pub fn cancel(&mut self) -> Result<bool> {
        if self.state.cancelled.swap(true, Ordering::AcqRel) {
            return Ok(false);
        }
        Ok(true)
    }

    /// Returns whether the Rust-side wait has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }
}

fn inspect_promise<T: PromiseFutureValue>(state: &PromiseFutureState<T>) -> Result<PromisePoll<T>> {
    let promise = state
        .promise
        .lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Promise reference lock poisoned"))?;
    let global = promise.as_ref().ok_or_else(|| {
        Error::new(
            Status::GenericFailure,
            "Promise observer no longer owns a Promise",
        )
    })?;
    state.vm.with_attached(|env| {
        let promise = env.local_object_from_global_ref(global)?;
        match env.get_field_by_name_int(&promise, "state")? {
            PROMISE_STATE_PENDING | PROMISE_STATE_LINKED => Ok(PromisePoll::Pending),
            PROMISE_STATE_RESOLVED => {
                let value = env.get_field_by_name_ref(&promise, "value")?;
                Ok(PromisePoll::Ready(T::from_promise_ref(env, value)))
            }
            PROMISE_STATE_REJECTED => {
                let value = env.get_field_by_name_ref(&promise, "value")?;
                Ok(PromisePoll::Ready(Err(promise_rejection_error(env, value))))
            }
            value => Ok(PromisePoll::Ready(Err(Error::new(
                Status::Error,
                format!("ArkTS Promise reported unknown state {value}"),
            )))),
        }
    })
}

fn observe_promise<T: PromiseFutureValue>(state: Arc<PromiseFutureState<T>>) {
    if state.cancelled.load(Ordering::Acquire) {
        finish_promise_observer(
            state,
            Err(Error::new(
                Status::Cancelled,
                "Promise future was cancelled",
            )),
        );
        return;
    }
    match inspect_promise(&state) {
        Ok(PromisePoll::Pending) => {
            let interval = Duration::from_millis(state.interval_ms.load(Ordering::Acquire));
            let next = Arc::clone(&state);
            if let Err(error) =
                crate::scheduler::shared().schedule_after(interval, move || observe_promise(next))
            {
                finish_promise_observer(state, Err(error));
            }
        }
        Ok(PromisePoll::Ready(result)) => finish_promise_observer(state, result),
        Err(error) => finish_promise_observer(state, Err(error)),
    }
}

fn finish_promise_observer<T>(state: Arc<PromiseFutureState<T>>, result: Result<T>) {
    let result = match state.release_promise() {
        Ok(()) => result,
        Err(error) => Err(error),
    };
    state.finish(result);
}

impl<T: PromiseFutureValue> std::future::Future for PromiseFuture<T> {
    type Output = Result<T>;

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

impl<T> Drop for PromiseFuture<T> {
    fn drop(&mut self) {
        self.state.cancelled.store(true, Ordering::Release);
    }
}

fn promise_rejection_error(env: &Env<'_>, value: AniRef<'_>) -> Error {
    let error_object = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
    let reason = env
        .get_property_by_name_ref(&error_object, "message")
        .and_then(|message| {
            let message = unsafe { AniString::from_raw(message.as_raw() as sys::ani_string) };
            env.get_string(&message)
        })
        .unwrap_or_else(|_| "ArkTS Promise rejected".to_string());
    let status = env
        .get_property_by_name_ref(&error_object, "name")
        .and_then(|status| {
            let status = unsafe { AniString::from_raw(status.as_raw() as sys::ani_string) };
            env.get_string(&status)
        })
        .unwrap_or_else(|_| "GenericFailure".to_string());
    let code = env.get_property_by_name_int(&error_object, "code").ok();
    let mut error = Error::new(Status::GenericFailure, reason).with_status_name(status);
    error.code = code;
    error
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
        }
    }

    /// Rebind the phantom payload type without changing the underlying resolver.
    #[inline]
    pub fn cast<U>(self) -> Deferred<U> {
        Deferred {
            resolver: self.resolver,
            _marker: PhantomData,
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
