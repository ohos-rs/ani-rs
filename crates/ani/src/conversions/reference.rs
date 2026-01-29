//! Reference Types for ANI
//!
//! This module provides reference types for managing ANI object lifetimes:
//!
//! - [`Ref<T>`] - A typed global reference that can be stored and used across native calls
//!
//! # Reference Model in ANI
//!
//! ANI follows a reference model similar to JNI:
//!
//! ## Local References
//!
//! - Created implicitly when ANI returns objects to native code
//! - Automatically released when the native method returns
//! - Represented by `AniRef<'env>`, `AniObject<'env>`, etc. with lifetime bound to `Env`
//! - Cannot be stored beyond the current native call
//!
//! ## Global References
//!
//! - Created explicitly using `GlobalReference_Create`
//! - Must be manually deleted using `GlobalReference_Delete`
//! - Can outlive the native method call
//! - Can be safely transferred between threads (Send + Sync)
//! - Represented by `GlobalRef` (untyped) or `Ref<T>` (typed)
//!
//! ## Weak References
//!
//! - Created from local references using `WeakReference_Create`
//! - May be garbage collected at any time
//! - Need to check validity before use with `WeakReference_GetReference`
//! - Represented by `WeakRef`
//!
//! # When to Use Each Type
//!
//! | Type | Use Case |
//! |------|----------|
//! | `AniRef<'env>` | Short-lived references within a single native call |
//! | `Ref<T>` | Storing typed references for later use |
//! | `GlobalRef` | Low-level untyped global reference |
//! | `WeakRef` | Cache that can be invalidated by GC |
//!
//! # Example
//!
//! ```rust,ignore
//! use ani::prelude::*;
//! use ani_derive::ani;
//!
//! // Store a reference for later use
//! static STORED_OBJECT: Mutex<Option<Ref<AniObject<'static>>>> = Mutex::new(None);
//!
//! #[ani]
//! pub fn store_object(_env: &Env, obj: Ref<AniObject<'static>>) -> Result<()> {
//!     let mut guard = STORED_OBJECT.lock().unwrap();
//!     *guard = Some(obj);
//!     Ok(())
//! }
//!
//! #[ani]
//! pub fn use_stored_object(env: &Env) -> Result<String> {
//!     let guard = STORED_OBJECT.lock().unwrap();
//!     if let Some(ref obj_ref) = *guard {
//!         // Borrow back as a local reference
//!         let obj = obj_ref.borrow(env);
//!         // Use the object...
//!         Ok("Object exists".to_string())
//!     } else {
//!         Err(Error::new(Status::GenericFailure, "No object stored"))
//!     }
//! }
//! ```

use std::marker::PhantomData;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniObject, AniRef, GlobalRef};

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// Ref<T> - Typed Global Reference
// ============================================================================

/// A typed global reference to an ANI object
///
/// `Ref<T>` wraps a global reference with type information, allowing you to:
/// - Store references beyond the current native call
/// - Safely use references across threads
/// - Maintain type safety when borrowing back
///
/// # Type Parameter
///
/// - `T` - The underlying ANI type (e.g., `AniObject<'static>`, `AniString<'static>`)
///
/// # Lifetime
///
/// `Ref<T>` does not have a lifetime parameter because global references
/// are not bound to any particular scope. The inner type `T` typically uses
/// `'static` as a placeholder lifetime.
///
/// # Thread Safety
///
/// `Ref<T>` is `Send + Sync`, meaning it can be safely shared between threads.
/// However, operations on the borrowed reference require an `Env`.
///
/// # Example
///
/// ```rust,ignore
/// use ani::prelude::*;
///
/// // Receive a global reference from ArkTS
/// fn store_callback(obj: Ref<AniObject<'static>>) {
///     // Store for later use...
/// }
///
/// // Use the stored reference
/// fn use_callback(env: &Env, obj_ref: &Ref<AniObject<'static>>) {
///     let obj = obj_ref.borrow(env);
///     // Use obj...
/// }
/// ```
pub struct Ref<T> {
    inner: GlobalRef,
    _marker: PhantomData<T>,
}

// Safety: GlobalRef is Send + Sync, and PhantomData<T> doesn't affect thread safety
unsafe impl<T> Send for Ref<T> {}
unsafe impl<T> Sync for Ref<T> {}

impl<T> Ref<T> {
    /// Create a new `Ref<T>` from a `GlobalRef`
    ///
    /// # Safety
    ///
    /// Caller must ensure the `GlobalRef` actually points to an object
    /// of type `T`.
    #[inline]
    pub unsafe fn from_global_ref(inner: GlobalRef) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Get the underlying raw pointer
    #[inline]
    pub fn as_raw(&self) -> sys::ani_ref {
        self.inner.as_raw()
    }

    /// Consume self and return the underlying `GlobalRef`
    #[inline]
    pub fn into_global_ref(self) -> GlobalRef {
        self.inner
    }

    /// Get a reference to the underlying `GlobalRef`
    #[inline]
    pub fn as_global_ref(&self) -> &GlobalRef {
        &self.inner
    }
}

// ============================================================================
// Ref<AniObject> - Object Reference
// ============================================================================

impl Ref<AniObject<'static>> {
    /// Borrow the reference as a local `AniObject`
    ///
    /// The returned object is valid for the lifetime of the borrow.
    /// This doesn't create a new reference - it just provides typed access
    /// to the underlying global reference.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn use_object(env: &Env, obj_ref: &Ref<AniObject<'static>>) {
    ///     let obj = obj_ref.borrow(env);
    ///     // obj is now usable as AniObject
    /// }
    /// ```
    #[inline]
    pub fn borrow<'env>(&self, _env: &Env<'env>) -> AniObject<'env> {
        unsafe { AniObject::from_raw(self.inner.as_raw() as sys::ani_object) }
    }

    /// Borrow the reference as a local `AniRef`
    #[inline]
    pub fn borrow_as_ref<'env>(&self, _env: &Env<'env>) -> AniRef<'env> {
        unsafe { AniRef::from_raw(self.inner.as_raw()) }
    }
}

// ============================================================================
// TypeInfo Implementation
// ============================================================================

impl<T> TypeInfo for Ref<T>
where
    T: TypeInfo,
{
    fn type_signature() -> &'static str {
        T::type_signature()
    }

    fn ani_c_type() -> &'static str {
        "ani_ref"
    }
}

// ============================================================================
// FromAni Implementation
// ============================================================================

impl<'env> FromAni<'env> for Ref<AniObject<'static>> {
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Object value is null"));
        }

        // Create a global reference from the local reference
        let ani_ref = unsafe { AniRef::from_raw(value as sys::ani_ref) };
        let global_ref = env.create_global_ref(&ani_ref)?;

        Ok(Ref {
            inner: global_ref,
            _marker: PhantomData,
        })
    }
}

// ============================================================================
// ToAni Implementation
// ============================================================================

impl<'env> ToAni<'env> for Ref<AniObject<'static>> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.inner.as_raw() as sys::ani_object)
    }
}

impl<'env> ToAni<'env> for &Ref<AniObject<'static>> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.inner.as_raw() as sys::ani_object)
    }
}

// ============================================================================
// Clone Implementation (creates a new global reference)
// ============================================================================

// Note: We intentionally do NOT implement Clone for Ref<T> because
// cloning a global reference requires an Env to create a new one.
// Users should use explicit methods if they need to clone.

impl<T> Ref<T> {
    /// Clone this reference by creating a new global reference
    ///
    /// This requires an `Env` because it calls `GlobalReference_Create`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let cloned = obj_ref.clone_ref(env)?;
    /// ```
    pub fn clone_ref(&self, env: &Env<'_>) -> Result<Self> {
        let ani_ref = unsafe { AniRef::from_raw(self.inner.as_raw()) };
        let new_global = env.create_global_ref(&ani_ref)?;
        Ok(Ref {
            inner: new_global,
            _marker: PhantomData,
        })
    }
}

// ============================================================================
// Debug Implementation
// ============================================================================

impl<T> std::fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ref")
            .field("raw", &self.inner.as_raw())
            .finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Ref<AniObject<'static>>>();
    }
}
