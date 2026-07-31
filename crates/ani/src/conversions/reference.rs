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
use crate::types::*;
use crate::vm::AniVm;

use super::AnyValue;
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
    vm: Option<AniVm>,
    inner: Option<GlobalRef>,
    _marker: PhantomData<T>,
}

// Safety: GlobalRef is Send + Sync, and PhantomData<T> doesn't affect thread safety
unsafe impl<T> Send for Ref<T> {}
unsafe impl<T> Sync for Ref<T> {}

impl<T> Ref<T> {
    #[inline]
    fn managed(vm: AniVm, inner: GlobalRef) -> Self {
        Self {
            vm: Some(vm),
            inner: Some(inner),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn inner_ref(&self) -> &GlobalRef {
        self.inner
            .as_ref()
            .expect("Ref<T> should contain a global ref until consumed")
    }

    /// Create a new `Ref<T>` from a `GlobalRef`
    ///
    /// # Safety
    ///
    /// Caller must ensure the `GlobalRef` actually points to an object
    /// of type `T`.
    #[inline]
    pub unsafe fn from_global_ref(inner: GlobalRef) -> Self {
        Self {
            vm: None,
            inner: Some(inner),
            _marker: PhantomData,
        }
    }

    /// Create a managed `Ref<T>` from a `GlobalRef` and owning [`AniVm`].
    ///
    /// Values created via [`FromAni`] already use this path automatically.
    ///
    /// # Safety
    ///
    /// Caller must ensure the `GlobalRef` actually points to an object of
    /// type `T`, and that `vm` is the owning VM for that reference.
    #[inline]
    pub unsafe fn from_global_ref_managed(vm: AniVm, inner: GlobalRef) -> Self {
        Self::managed(vm, inner)
    }

    /// Get the underlying raw pointer
    #[inline]
    pub fn as_raw(&self) -> sys::ani_ref {
        self.inner_ref().as_raw()
    }

    /// Consume self and return the underlying `GlobalRef`
    #[inline]
    pub fn into_global_ref(mut self) -> GlobalRef {
        self.inner
            .take()
            .expect("Ref<T> should contain a global ref before into_global_ref")
    }

    /// Get a reference to the underlying `GlobalRef`
    #[inline]
    pub fn as_global_ref(&self) -> &GlobalRef {
        self.inner_ref()
    }

    /// Delete this global reference explicitly.
    #[inline]
    pub fn delete(mut self, env: &Env<'_>) -> Result<()> {
        let global = self
            .inner
            .take()
            .expect("Ref<T> should contain a global ref before delete");
        env.delete_global_ref(global)
    }
}

impl GlobalRef {
    /// Materialize this global handle as a local [`AniRef`] on the current
    /// thread.
    #[inline]
    pub fn to_local<'env>(&self, env: &Env<'env>) -> Result<AniRef<'env>> {
        env.local_ref_from_global_ref(self)
    }

    /// Materialize this global handle as a local [`AniObject`] on the current
    /// thread.
    #[inline]
    pub fn to_object<'env>(&self, env: &Env<'env>) -> Result<AniObject<'env>> {
        env.local_object_from_global_ref(self)
    }

    /// Materialize this global handle as a local [`AniClass`] on the current
    /// thread.
    #[inline]
    pub fn to_class<'env>(&self, env: &Env<'env>) -> Result<AniClass<'env>> {
        env.local_class_from_global_ref(self)
    }

    /// Clone this handle by creating a second global reference to the same
    /// object.
    pub fn clone_ref(&self, env: &Env<'_>) -> Result<Self> {
        let local = self.to_local(env)?;
        let cloned = env.create_global_ref(&local);
        let delete_local = env.delete_local_ref(&local);

        match (cloned, delete_local) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(err), _) => Err(err),
            (Ok(_), Err(err)) => Err(err),
        }
    }

    /// Delete this global reference explicitly.
    #[inline]
    pub fn delete(self, env: &Env<'_>) -> Result<()> {
        env.delete_global_ref(self)
    }
}

impl ToGlobalRefSource for GlobalRef {
    fn to_global_ref(&self, env: &Env<'_>) -> Result<GlobalRef> {
        self.clone_ref(env)
    }
}

impl WeakRef {
    /// Upgrade this weak handle to a thread-local [`AniRef`], if the target is
    /// still alive.
    #[inline]
    pub fn upgrade<'env>(&self, env: &Env<'env>) -> Result<Option<AniRef<'env>>> {
        env.upgrade_weak_ref(self)
    }

    /// Check whether this weak handle can still be upgraded.
    ///
    /// Any temporary local reference created during the check is deleted before
    /// returning, so this helper does not accidentally keep the object alive.
    pub fn is_alive(&self, env: &Env<'_>) -> Result<bool> {
        match self.upgrade(env)? {
            Some(local) => {
                env.delete_local_ref(&local)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Check whether the weak target has already been released.
    #[inline]
    pub fn is_released(&self, env: &Env<'_>) -> Result<bool> {
        self.is_alive(env).map(|alive| !alive)
    }

    /// Delete this weak reference explicitly.
    #[inline]
    pub fn delete(self, env: &Env<'_>) -> Result<()> {
        env.delete_weak_ref(self)
    }
}

// ============================================================================
// Ref<AniObject> - Object Reference
// ============================================================================

impl Ref<AniObject<'static>> {
    /// Borrow the reference as a local `AniObject`
    ///
    /// The returned object is a fresh local reference valid for the current
    /// native call scope.
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
    pub fn borrow<'env>(&self, env: &Env<'env>) -> AniObject<'env> {
        env.local_object_from_global_ref(self.as_global_ref())
            .expect("Ref<AniObject>::borrow failed to materialize local object")
    }

    /// Borrow the reference as a local `AniRef`
    #[inline]
    pub fn borrow_as_ref<'env>(&self, env: &Env<'env>) -> AniRef<'env> {
        env.local_ref_from_global_ref(self.as_global_ref())
            .expect("Ref<AniObject>::borrow_as_ref failed to materialize local ref")
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

impl TypeInfo for GlobalRef {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_ref"
    }
}

impl TypeInfo for WeakRef {
    fn type_signature() -> &'static str {
        "Lstd/core/WeakRef;"
    }

    fn ani_c_type() -> &'static str {
        "ani_wref"
    }
}

// ============================================================================
// FromAni Implementation
// ============================================================================

impl<'env> FromAni<'env> for Ref<AniObject<'static>> {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Object value is null"));
        }

        // Create a global reference from the local reference
        let ani_ref = unsafe { AniRef::from_raw(value as sys::ani_ref) };
        let global_ref = env.create_global_ref(&ani_ref)?;

        Ok(Ref::managed(env.get_vm()?, global_ref))
    }
}

impl<'env> FromAni<'env> for GlobalRef {
    type Input = sys::ani_ref;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "GlobalRef value is null"));
        }
        Ok(unsafe { GlobalRef::from_raw(value) })
    }
}

impl<'env> FromAni<'env> for WeakRef {
    type Input = sys::ani_wref;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "WeakRef value is null"));
        }
        Ok(unsafe { WeakRef::from_raw(value) })
    }
}

// ============================================================================
// ToAni Implementation
// ============================================================================

impl<'env> ToAni<'env> for Ref<AniObject<'static>> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let local = env.local_object_from_global_ref(self.as_global_ref())?;
        Ok(local.as_raw())
    }
}

impl<'env> ToAni<'env> for &Ref<AniObject<'static>> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let local = env.local_object_from_global_ref(self.as_global_ref())?;
        Ok(local.as_raw())
    }
}

impl<'env> ToAni<'env> for GlobalRef {
    type Output = sys::ani_ref;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.as_raw())
    }
}

impl<'env> ToAni<'env> for &GlobalRef {
    type Output = sys::ani_ref;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.as_raw())
    }
}

impl<'env> ToAni<'env> for WeakRef {
    type Output = sys::ani_wref;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.as_raw())
    }
}

impl<'env> ToAni<'env> for &WeakRef {
    type Output = sys::ani_wref;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.as_raw())
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
        let ani_ref = env.local_ref_from_global_ref(self.as_global_ref())?;
        let new_global = env.create_global_ref(&ani_ref)?;
        Ok(Ref::managed(env.get_vm()?, new_global))
    }
}

impl<T> ToGlobalRefSource for Ref<T> {
    fn to_global_ref(&self, env: &Env<'_>) -> Result<GlobalRef> {
        self.as_global_ref().clone_ref(env)
    }
}

// ============================================================================
// Debug Implementation
// ============================================================================

impl<T> std::fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ref")
            .field("raw", &self.inner.as_ref().map(GlobalRef::as_raw))
            .finish()
    }
}

impl<T> Drop for Ref<T> {
    fn drop(&mut self) {
        let Some(vm) = self.vm.as_ref() else {
            return;
        };
        let Some(global) = self.inner.take() else {
            return;
        };

        let _ = vm.with_attached(|env| env.delete_global_ref(global));
    }
}

// ============================================================================
// RefContainer - Async-safe local reference container
// ============================================================================

/// Convert a thread-affine ANI local handle into an owned [`GlobalRef`].
///
/// This is primarily used by [`RefContainer`] and async wrapper generation to
/// capture local ANI values on the caller thread, then materialize them again
/// on a runtime worker thread.
pub trait ToGlobalRefSource {
    /// Promote the local handle into an owned [`GlobalRef`].
    fn to_global_ref(&self, env: &Env<'_>) -> Result<GlobalRef>;
}

/// Restore a thread-affine ANI local handle from a [`GlobalRef`].
///
/// Implementations are provided for ANI local-reference wrapper types such as
/// [`AniRef`], [`AniObject`], [`AniClass`], [`AniString`], arrays, and other
/// reference-backed runtime handles.
pub trait FromGlobalRef<'env>: Sized {
    /// Materialize the local handle on the current thread.
    fn from_global_ref(env: &Env<'env>, global: &GlobalRef) -> Result<Self>;
}

macro_rules! impl_global_ref_bridge_for_ref_type {
    ($ty:ident, $raw:ty) => {
        impl<'env> ToGlobalRefSource for $ty<'env> {
            fn to_global_ref(&self, env: &Env<'_>) -> Result<GlobalRef> {
                let value = unsafe { AniRef::from_raw(self.as_raw() as sys::ani_ref) };
                env.create_global_ref(&value)
            }
        }

        impl<'env> FromGlobalRef<'env> for $ty<'env> {
            fn from_global_ref(env: &Env<'env>, global: &GlobalRef) -> Result<Self> {
                let local = env.local_ref_from_global_ref(global)?;
                Ok(unsafe { $ty::from_raw(local.as_raw() as $raw) })
            }
        }
    };
}

impl_global_ref_bridge_for_ref_type!(AniRef, sys::ani_ref);
impl_global_ref_bridge_for_ref_type!(AniObject, sys::ani_object);
impl_global_ref_bridge_for_ref_type!(AniClass, sys::ani_class);
impl_global_ref_bridge_for_ref_type!(AniType, sys::ani_type);
impl_global_ref_bridge_for_ref_type!(AniModule, sys::ani_module);
impl_global_ref_bridge_for_ref_type!(AniNamespace, sys::ani_namespace);
impl_global_ref_bridge_for_ref_type!(AniString, sys::ani_string);
impl_global_ref_bridge_for_ref_type!(AniEnum, sys::ani_enum);
impl_global_ref_bridge_for_ref_type!(AniError, sys::ani_error);
impl_global_ref_bridge_for_ref_type!(AniFnObject, sys::ani_fn_object);
impl_global_ref_bridge_for_ref_type!(AniArray, sys::ani_array);
impl_global_ref_bridge_for_ref_type!(AniArrayInt, sys::ani_fixedarray_int);
impl_global_ref_bridge_for_ref_type!(AniArrayLong, sys::ani_fixedarray_long);
impl_global_ref_bridge_for_ref_type!(AniArrayDouble, sys::ani_fixedarray_double);
impl_global_ref_bridge_for_ref_type!(AniArrayRef, sys::ani_array);
impl_global_ref_bridge_for_ref_type!(AniFixedArray, sys::ani_fixedarray);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayBoolean, sys::ani_fixedarray_boolean);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayChar, sys::ani_fixedarray_char);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayByte, sys::ani_fixedarray_byte);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayShort, sys::ani_fixedarray_short);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayInt, sys::ani_fixedarray_int);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayLong, sys::ani_fixedarray_long);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayFloat, sys::ani_fixedarray_float);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayDouble, sys::ani_fixedarray_double);
impl_global_ref_bridge_for_ref_type!(AniFixedArrayRef, sys::ani_fixedarray_ref);
impl_global_ref_bridge_for_ref_type!(AniArrayBuffer, sys::ani_arraybuffer);
impl_global_ref_bridge_for_ref_type!(AniEnumItem, sys::ani_enum_item);
impl_global_ref_bridge_for_ref_type!(AniTupleValue, sys::ani_tuple_value);

impl<'env> ToGlobalRefSource for AnyValue<'env> {
    fn to_global_ref(&self, env: &Env<'_>) -> Result<GlobalRef> {
        env.create_global_ref(self.as_ref())
    }
}

impl<'env> FromGlobalRef<'env> for AnyValue<'env> {
    fn from_global_ref(env: &Env<'env>, global: &GlobalRef) -> Result<Self> {
        let local = env.local_ref_from_global_ref(global)?;
        Ok(Self::from_ref(local))
    }
}

/// Owned global-reference container for async tasks.
///
/// `RefContainer` captures a thread-affine ANI local handle as a [`GlobalRef`]
/// together with its owning [`AniVm`]. When the container is dropped, it
/// reattaches to the VM if needed and deletes the global reference
/// automatically. This provides a napi-rs-style "ref container" building block
/// for async workflows.
pub struct RefContainer {
    vm: AniVm,
    inner: Option<GlobalRef>,
}

impl RefContainer {
    /// Create a new container from a local ANI handle.
    pub fn new<'env, T>(env: &Env<'env>, value: &T) -> Result<Self>
    where
        T: ToGlobalRefSource,
    {
        let vm = env.get_vm()?;
        let inner = value.to_global_ref(env)?;
        Ok(Self {
            vm,
            inner: Some(inner),
        })
    }

    /// Create a container from an existing [`GlobalRef`].
    pub fn from_global_ref(vm: AniVm, inner: GlobalRef) -> Self {
        Self {
            vm,
            inner: Some(inner),
        }
    }

    /// Borrow the owned [`GlobalRef`].
    pub fn as_global_ref(&self) -> Option<&GlobalRef> {
        self.inner.as_ref()
    }

    /// Materialize the requested local ANI handle on the current thread.
    pub fn to_local<'env, T>(&self, env: &Env<'env>) -> Result<T>
    where
        T: FromGlobalRef<'env>,
    {
        let global = self
            .inner
            .as_ref()
            .ok_or_else(|| Error::new(Status::InvalidArgs, "RefContainer has no global ref"))?;
        T::from_global_ref(env, global)
    }

    /// Create a second container pointing at the same object.
    pub fn clone_container(&self, env: &Env<'_>) -> Result<Self> {
        let local = self.to_local::<AniRef<'_>>(env)?;
        let cloned = env.create_global_ref(&local)?;
        Ok(Self {
            vm: env.get_vm()?,
            inner: Some(cloned),
        })
    }

    /// Consume the container and return the owned [`GlobalRef`] without
    /// deleting it on drop.
    pub fn into_global_ref(mut self) -> GlobalRef {
        self.inner
            .take()
            .expect("RefContainer should always contain a global ref before into_global_ref")
    }
}

impl std::fmt::Debug for RefContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefContainer")
            .field("raw", &self.inner.as_ref().map(GlobalRef::as_raw))
            .finish()
    }
}

impl Drop for RefContainer {
    fn drop(&mut self) {
        let Some(global) = self.inner.take() else {
            return;
        };

        if let Ok(guard) = self.vm.attach_current_thread_scoped() {
            let _ = guard.env().delete_global_ref(global);
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::NonNull;

    #[test]
    fn test_ref_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Ref<AniObject<'static>>>();
        assert_send_sync::<GlobalRef>();
        assert_send_sync::<WeakRef>();
        assert_send_sync::<RefContainer>();
    }

    #[test]
    fn test_global_and_weak_ref_type_info() {
        assert_eq!(GlobalRef::type_signature(), "Lstd/core/Object;");
        assert_eq!(GlobalRef::ani_c_type(), "ani_ref");
        assert_eq!(WeakRef::type_signature(), "Lstd/core/WeakRef;");
        assert_eq!(WeakRef::ani_c_type(), "ani_wref");
    }

    #[test]
    fn test_ref_container_trait_support_covers_common_local_handles() {
        fn assert_to_global<T: ToGlobalRefSource>() {}
        fn assert_from_global<'env, T: FromGlobalRef<'env>>() {}

        assert_to_global::<GlobalRef>();
        assert_to_global::<AniRef<'static>>();
        assert_to_global::<AniObject<'static>>();
        assert_to_global::<AniClass<'static>>();
        assert_to_global::<AniString<'static>>();
        assert_to_global::<AniArrayBuffer<'static>>();
        assert_to_global::<AniFixedArrayInt<'static>>();
        assert_to_global::<AnyValue<'static>>();
        assert_to_global::<Ref<AniObject<'static>>>();

        assert_from_global::<AniRef<'static>>();
        assert_from_global::<AniObject<'static>>();
        assert_from_global::<AniClass<'static>>();
        assert_from_global::<AniString<'static>>();
        assert_from_global::<AniArrayBuffer<'static>>();
        assert_from_global::<AniFixedArrayInt<'static>>();
        assert_from_global::<AnyValue<'static>>();
    }

    #[test]
    fn test_ref_container_into_global_ref_preserves_raw() {
        let vm = unsafe { AniVm::from_raw_unchecked(NonNull::<sys::ani_vm>::dangling().as_ptr()) };
        let raw = NonNull::<std::ffi::c_void>::dangling().as_ptr() as sys::ani_ref;
        let global = unsafe { GlobalRef::from_raw(raw) };
        let container = RefContainer::from_global_ref(vm, global);
        let global = container.into_global_ref();
        assert_eq!(global.as_raw(), raw);
    }

    #[test]
    fn test_global_and_weak_ref_helper_methods_compile() {
        fn compile<'env>(env: &Env<'env>) {
            let raw = NonNull::<std::ffi::c_void>::dangling().as_ptr() as sys::ani_ref;
            let global = unsafe { GlobalRef::from_raw(raw) };
            let weak = unsafe { WeakRef::from_raw(raw as sys::ani_wref) };

            let _ = global.to_local(env);
            let _ = unsafe { GlobalRef::from_raw(raw) }.to_object(env);
            let _ = unsafe { GlobalRef::from_raw(raw) }.to_class(env);
            let _ = unsafe { GlobalRef::from_raw(raw) }.clone_ref(env);
            let _ = unsafe { GlobalRef::from_raw(raw) }.delete(env);

            let _ = weak.upgrade(env);
            let _ = unsafe { WeakRef::from_raw(raw as sys::ani_wref) }.is_alive(env);
            let _ = unsafe { WeakRef::from_raw(raw as sys::ani_wref) }.is_released(env);
            let _ = unsafe { WeakRef::from_raw(raw as sys::ani_wref) }.delete(env);
        }

        let _ = compile;
    }
}
