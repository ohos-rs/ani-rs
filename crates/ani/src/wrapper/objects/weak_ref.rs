use std::ops::Deref;

use log::{debug, warn};

use crate::{
    errors::{ani_status_to_result, Error, Result},
    objects::{GlobalRef, AObject},
    sys, sys::aobject, anienv::ANIEnv, ANIVersion, AniVM,
};

use super::AObjectRef;

// Note: `WeakRef` must not implement `Into<AObject>`! If it did, then it would be possible to
// wrap it in `AutoLocal`, which would cause undefined behavior upon drop as a result of calling
// the wrong function to delete the reference.

/// A global reference to an object that does *not* prevent it from being
/// garbage collected.
///
/// <dfn>Weak global references</dfn> have the same properties as [ordinary
/// "strong" global references][GlobalRef], with one exception: a weak global
/// reference does not prevent the referenced object from being garbage
/// collected. In other words, the object can be garbage collected even if
/// there is a weak global reference to it.
///
///
/// # Upgrading
///
/// Because the object referred to by a weak global reference may be
/// garbage collected at any moment, it cannot be directly used (such as
/// calling methods on the referenced object). Instead, it must first be
/// <dfn>upgraded</dfn> to a local or strong global reference, using the
/// [`WeakRef::upgrade_local`] or [`WeakRef::upgrade_global`] method,
/// respectively.
///
/// Both upgrade methods return an [`Option`]. If, when the upgrade method is
/// called, the object has not yet been garbage collected, then the
/// `Option` will be [`Some`] containing a newly created strong reference that
/// can be used as normal. If not, the `Option` will be [`None`].
///
/// Upgrading a weak global reference does not delete it. It is only deleted
/// when the `WeakRef` is dropped, and it can be upgraded more than once.
///
///
/// # Creating and Deleting
///
/// To create a weak global reference, use the [`ANIEnv::new_weak_ref`] method.
/// To delete it, simply drop the `WeakRef` (but be sure to do so on an attached
/// thread if possible; see the warning below).
///
/// It is also possible to create a new weak global reference from an
/// existing one. To do that, use the [`WeakRef::clone_in_vm`] method.
///
///
/// # Warning: Drop On an Attached Thread If Possible
///
/// When a `WeakRef` is dropped, a call is made to delete the weak global
/// reference. If this frequently happens on a thread that is not already
/// attached to the VM, the thread will be temporarily attached,
/// causing a severe performance penalty.
///
/// To avoid this performance penalty, ensure that `WeakRef`s are only dropped
/// on a thread that is already attached (or never dropped at all).
///
/// In the event that a weak reference is dropped on an unattached thread, a
/// message is [logged][log] at [`log::Level::Warn`].
#[repr(transparent)]
#[derive(Debug)]
pub struct WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    obj: T,
}

unsafe impl<T> Send for WeakRef<T> where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync
{
}

unsafe impl<T> Sync for WeakRef<T> where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync
{
}

impl<T> Default for WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    fn default() -> Self {
        Self::null()
    }
}

impl<T, U> AsRef<U> for WeakRef<T>
where
    T: AsRef<U>
        + Into<AObject<'static>>
        + AsRef<AObject<'static>>
        + Default
        + AObjectRef
        + Send
        + Sync,
{
    fn as_ref(&self) -> &U {
        self.obj.as_ref()
    }
}

impl<T> Deref for WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<T> WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    /// Creates a new auto-delete wrapper for the `'static` weak global reference
    ///
    /// Note: It's more likely that you want to look at the [`ANIEnv::new_weak_ref`] API instead
    /// of this, since you can't get `'static` reference types through safe APIs.
    ///
    /// # Safety
    ///
    /// If the given reference is non-null, it must represent a weak global reference.
    pub unsafe fn new(env: &ANIEnv, obj: T) -> Self {
        // Guarantee that the `AniVM::singleton()` is initialized for the `Drop` implementation
        let _vm = env.get_ani_vm();
        Self { obj }
    }

    /// Creates a [`WeakRef`] wrapper for a `null` reference
    ///
    /// This is equivalent [`WeakRef::default()`]
    ///
    /// A `null` [`WeakRef`] acts as-if the object has been garbage collected
    /// ([`Self::is_garbage_collected()`] will return `true`).
    pub fn null() -> Self {
        Self { obj: T::default() }
    }

    /// Returns the raw weak reference.
    pub fn as_raw(&self) -> sys::aobject {
        self.obj.as_raw()
    }

    /// Creates a new local reference to this object.
    ///
    /// This returns `None` if the object has already been garbage collected, otherwise it returns
    /// `Some(new_local_reference)`.
    ///
    /// If this method returns `Some(r)`, it is guaranteed that the object will not be garbage
    /// collected at least until `r` is deleted or becomes invalid.
    pub fn upgrade_local<'local>(
        &self,
        env: &mut ANIEnv<'local>,
    ) -> Result<Option<T::Kind<'local>>> {
        match env.new_local_ref(self) {
            Ok(local_ref) => Ok(Some(local_ref)),
            Err(Error::ObjectFreed) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Creates a new strong global reference to this object.
    ///
    /// This returns `None` if the object has already been garbage collected, otherwise it returns
    /// `Some(new_local_reference)`.
    ///
    /// If this method returns `Some(r)`, it is guaranteed that the object will not be garbage
    /// collected at least until `r` is deleted or becomes invalid.
    pub fn upgrade_global(&self, env: &ANIEnv) -> Result<Option<GlobalRef<T::GlobalKind>>> {
        match env.new_global_ref(self) {
            Err(Error::ObjectFreed) => Ok(None),
            Err(err) => Err(err),
            Ok(global_ref) => Ok(Some(global_ref)),
        }
    }

    /// Checks if the object referred to by this `WeakRef` has been garbage collected.
    ///
    /// Note that garbage collection can happen at any moment, so a return of `Ok(true)` from this
    /// method does not guarantee that [`WeakRef::upgrade_local`] or [`WeakRef::upgrade_global`]
    /// will succeed.
    pub fn is_garbage_collected(&self, env: &ANIEnv) -> bool {
        env.is_same_object(self, AObject::null())
    }

    /// Creates a new weak reference to the same object that this one refers to.
    ///
    /// This method returns `None` if the object has already been garbage collected.
    pub fn clone_in_vm(&self, env: &mut ANIEnv<'_>) -> Result<Option<WeakRef<T::GlobalKind>>> {
        match env.new_weak_ref(self) {
            Err(Error::ObjectFreed) => Ok(None),
            Err(err) => Err(err),
            Ok(weak_ref) => Ok(Some(weak_ref)),
        }
    }
}

impl<T> Drop for WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    fn drop(&mut self) {
        let obj = std::mem::take(&mut self.obj);

        // It's redundant to explicitly delete a null pointer and we don't
        // assume that a AniVM has been initialized if we only wrap a 'static null pointer
        if !obj.is_null() {
            fn drop_impl(env: &ANIEnv, raw: sys::aobject) -> Result<()> {
                unsafe {
                    let status = ani_call_unchecked!(env, Reference_Delete, raw);
                    ani_status_to_result(status)
                }
            }

            // Panic: If we have a non-null reference, we know AniVM::singleton() must have been
            // initialized (and can't return an error) because ::new() takes a ANIEnv reference.
            let vm = AniVM::singleton().expect("AniVM singleton uninitialized");

            let res = match unsafe { vm.get_env(ANIVersion::V1) } {
                Ok(env) => drop_impl(&env, obj.as_raw()),
                Err(_) => {
                    warn!("Dropping a WeakRef in a detached thread. Fix your code if this message appears frequently (see the WeakRef docs).");
                    vm.attach_current_thread()
                        .and_then(|env| drop_impl(&env, obj.as_raw()))
                }
            };

            if let Err(err) = res {
                debug!("error dropping weak ref: {:#?}", err);
            }
        }
    }
}

impl<T> AObjectRef for WeakRef<T>
where
    T: Into<AObject<'static>> + AsRef<AObject<'static>> + Default + AObjectRef + Send + Sync,
{
    type Kind<'env> = T::Kind<'env>;
    type GlobalKind = T::GlobalKind;

    fn as_raw(&self) -> aobject {
        self.obj.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        T::from_local_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        T::from_global_raw(global_ref)
    }
}
