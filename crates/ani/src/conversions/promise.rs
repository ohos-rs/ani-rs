//! Promise support for ANI
//!
//! This module provides Promise types similar to napi-rs design:
//!
//! - [`PromiseRaw`] - Raw Promise value with lifetime, for synchronous contexts
//! - [`Deferred`] - Deferred resolver for async Promise resolution
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

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniObject, AniRef, AniResolver};
use crate::{ani_call, ani_call_2ret};

/// Raw Promise value in ANI
///
/// `PromiseRaw<'env>` represents a raw Promise value. It contains a lifetime
/// so it can only be used in synchronous contexts.
///
/// This is similar to napi-rs's `PromiseRaw` type.
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
pub struct PromiseRaw<'env> {
    inner: sys::ani_object,
    _marker: PhantomData<&'env ()>,
}

impl<'env> PromiseRaw<'env> {
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

        let error_str = env.create_string(error.as_ref())?;
        ani_call!(
            env,
            PromiseResolver_Reject,
            resolver,
            error_str.as_raw() as sys::ani_error
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to reject promise"))?;

        Ok(Self {
            inner: promise,
            _marker: PhantomData,
        })
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
    pub fn deferred(env: &Env<'env>) -> Result<(Deferred, Self)> {
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
        };

        let promise_raw = Self {
            inner: promise,
            _marker: PhantomData,
        };

        Ok((deferred, promise_raw))
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
pub struct Deferred {
    resolver: AniResolver,
}

// Deferred can be sent across threads
// But resolving/rejecting requires a valid env for the current thread
unsafe impl Send for Deferred {}
unsafe impl Sync for Deferred {}

impl Deferred {
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
        ani_call!(
            env,
            PromiseResolver_Resolve,
            self.resolver.as_raw(),
            value.as_raw()
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to resolve promise"))
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
        let error_str = env.create_string(error.as_ref())?;
        ani_call!(
            env,
            PromiseResolver_Reject,
            self.resolver.as_raw(),
            error_str.as_raw() as sys::ani_error
        )
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to reject promise"))
    }

    /// Reject the Promise with an Error
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `error` - The Error to reject with
    pub fn reject_with_error<S: AsRef<str> + std::fmt::Debug>(
        self,
        env: &Env<'_>,
        error: Error<S>,
    ) -> Result<()> {
        self.reject(env, error.to_string())
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

impl<'env> From<PromiseRaw<'env>> for AniObject<'env> {
    fn from(promise: PromiseRaw<'env>) -> Self {
        unsafe { AniObject::from_raw(promise.inner) }
    }
}

impl<'env> From<PromiseRaw<'env>> for sys::ani_object {
    fn from(promise: PromiseRaw<'env>) -> Self {
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
fn create_boxed_boolean<'a>(env: &Env<'a>, value: bool) -> Result<AniObject<'a>> {
    use crate::types::ani_value_boolean;

    // Find std/core/Boolean class
    let boolean_class = env.find_class("std.core.Boolean")?;

    // Find constructor <ctor>(Z:V)
    let ctor = env.find_constructor(&boolean_class, "z:")?;

    // Create boxed object
    let args = [ani_value_boolean(value)];
    env.new_object(&boolean_class, &ctor, &args)
}

#[cfg(test)]
mod tests {
    // Tests would go here
}
