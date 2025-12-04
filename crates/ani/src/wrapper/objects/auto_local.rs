use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
};

use crate::{objects::JObject, sys::jobject, anienv::ANIEnv};

use super::JObjectRef;

/// Auto-delete wrapper for local refs.
///
/// Anything passed to a foreign method _and_ returned from ANI methods is considered a local ref
/// unless it is specified otherwise.
/// These refs are automatically deleted once the foreign method exits, but it's possible that
/// they may reach the VM-imposed limit before that happens.
///
/// This wrapper provides automatic local ref deletion when it goes out of
/// scope.
#[derive(Debug)]
pub struct AutoLocal<'local, T>
where
    T: Into<JObject<'local>>,
{
    obj: ManuallyDrop<T>,
    env: ANIEnv<'local>,
}

impl<'local, T> AutoLocal<'local, T>
where
    // Note that this bound prevents `AutoLocal` from wrapping a `GlobalRef`, which implements
    // `AsRef<JObject<'static>>` but *not* `Into<JObject<'static>>`. This is good, because trying
    // to delete a global reference as though it were local would cause undefined behavior.
    T: Into<JObject<'local>>,
{
    /// Creates a new auto-delete wrapper for a local ref.
    ///
    /// Once this wrapper goes out of scope, the `delete_local_ref` will be
    /// called on the object. While wrapped, the object can be accessed via
    /// the `Deref` impl.
    pub fn new(obj: T, env: &ANIEnv<'local>) -> Self {
        // Safety: The cloned `ANIEnv` will not be used to create any local references, only to
        // delete one.
        let env = unsafe { env.unsafe_clone() };

        AutoLocal {
            obj: ManuallyDrop::new(obj),
            env,
        }
    }

    /// Forget the wrapper, returning the original object.
    ///
    /// This prevents `delete_local_ref` from being called when the `AutoLocal`
    /// gets
    /// dropped. You must either remember to delete the local ref manually, or
    /// be
    /// ok with it getting deleted once the foreign method returns.
    pub fn forget(self) -> T {
        // We need to move `self.obj` out of `self`. Normally that's trivial, but moving out of a
        // type with a `Drop` implementation is not allowed. We'll have to do it manually (and
        // carefully) with `unsafe`.
        //
        // This could be done without `unsafe` by adding `where T: Default` and using
        // `std::mem::replace` to extract `self.obj`, but doing it this way avoids unnecessarily
        // running the drop routine on `self`.

        // Before we mutilate `self`, make sure its drop code will not be automatically run. That
        // would cause undefined behavior.
        let mut self_md = ManuallyDrop::new(self);

        unsafe {
            // Drop the `ANIEnv` in place. As of this writing, that's a no-op, but if `ANIEnv`
            // gains any drop code in the future, this will run it.
            //
            // Safety: The `&mut` proves that `self_md.env` is valid and not aliased. It is not
            // accessed again after this point. It is wrapped inside `ManuallyDrop`, and will
            // therefore not be dropped twice.
            ptr::drop_in_place(&mut self_md.env);

            // Move `obj` out of `self` and return it.
            //
            // Safety: The `&mut` proves that `self_md.obj` is valid and not aliased. It is not
            // accessed again after this point. It is wrapped inside `ManuallyDrop`, and will
            // therefore not be dropped after it is moved.
            ptr::read(&*self_md.obj)
        }
    }
}

impl<'local, T> Drop for AutoLocal<'local, T>
where
    T: Into<JObject<'local>>,
{
    fn drop(&mut self) {
        // Extract the local reference from `self.obj` so that we can delete it.
        //
        // This is needed because it is not allowed to move out of `self` during drop. A safe
        // alternative would be to wrap `self.obj` in `Option`, but that would incur a run-time
        // performance penalty from constantly checking if it's `None`.
        //
        // Safety: `self.obj` is not used again after this `take` call.
        let obj = unsafe { ManuallyDrop::take(&mut self.obj) };

        self.env.delete_local_ref(obj);
    }
}

impl<'local, T, U> AsRef<U> for AutoLocal<'local, T>
where
    T: AsRef<U> + Into<JObject<'local>>,
{
    fn as_ref(&self) -> &U {
        self.obj.as_ref()
    }
}

impl<'local, T, U> AsMut<U> for AutoLocal<'local, T>
where
    T: AsMut<U> + Into<JObject<'local>>,
{
    fn as_mut(&mut self) -> &mut U {
        self.obj.as_mut()
    }
}

impl<'local, T> Deref for AutoLocal<'local, T>
where
    T: Into<JObject<'local>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<'local, T> DerefMut for AutoLocal<'local, T>
where
    T: Into<JObject<'local>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl<'local, T> JObjectRef for AutoLocal<'local, T>
where
    T: JObjectRef + Into<JObject<'local>>,
{
    type Kind<'env> = T::Kind<'env>;
    type GlobalKind = T::GlobalKind;

    fn as_raw(&self) -> jobject {
        self.obj.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: jobject) -> Self::Kind<'env> {
        T::from_local_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: jobject) -> Self::GlobalKind {
        T::from_global_raw(global_ref)
    }
}
