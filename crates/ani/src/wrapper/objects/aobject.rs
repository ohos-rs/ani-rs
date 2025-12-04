use std::marker::PhantomData;

use crate::sys::aobject;

#[cfg(doc)]
use crate::objects::GlobalRef;

use super::AObjectRef;

/// Wrapper around [`sys::jobject`] that adds a lifetime to ensure that
/// the underlying ANI pointer won't be accessible to safe Rust code if the
/// object reference is released.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take a `jobject`.
///
/// Most other types in the `objects` module deref to this, as they do in the C
/// representation.
///
/// The lifetime `'local` represents the local reference frame that this
/// reference belongs to. See the [`ANIEnv`] documentation for more information
/// about local reference frames. If `'local` is `'static`, then this reference
/// does not belong to a local reference frame, that is, it is either null or a
/// [global reference][GlobalRef].
///
/// Note that an *owned* `AObject` is always a local reference and will never
/// have the `'static` lifetime. [`GlobalRef`] does implement
/// <code>[AsRef]&lt;AObject&lt;'static>></code>, but this only yields a
/// *borrowed* `&AObject<'static>`, never an owned `AObject<'static>`.
///
/// Local references belong to a single thread and are not safe to share across
/// threads. This type implements [`Send`] and [`Sync`] if and only if the
/// lifetime `'local` is `'static`.
#[repr(transparent)]
#[derive(Debug)]
pub struct AObject<'local> {
    internal: aobject,
    lifetime: PhantomData<&'local ()>,
}

unsafe impl Send for AObject<'static> {}
unsafe impl Sync for AObject<'static> {}

impl<'local> AsRef<AObject<'local>> for AObject<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> AsMut<AObject<'local>> for AObject<'local> {
    fn as_mut(&mut self) -> &mut AObject<'local> {
        self
    }
}

impl ::std::ops::Deref for AObject<'_> {
    type Target = aobject;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl AObject<'_> {
    /// Creates a [`AObject`] that wraps the given `raw` [`aobject`]
    ///
    /// # Safety
    ///
    /// * `raw` must be a valid raw ANI reference (or `null`).
    /// * There must not be any other `AObject` representing the same reference.
    /// * If `raw` represents a local reference then the `'local` lifetime must
    ///   not outlive the ANI stack frame that the local reference was created in.
    /// * Only global, weak global and `null` references may use a `'static` lifetime.
    pub const unsafe fn from_raw(raw: aobject) -> Self {
        Self {
            internal: raw,
            lifetime: PhantomData,
        }
    }

    /// Returns the raw ANI pointer.
    pub const fn as_raw(&self) -> aobject {
        self.internal
    }

    /// Unwrap to the internal ani type.
    pub const fn into_raw(self) -> aobject {
        self.internal
    }

    /// Creates a new null reference.
    ///
    /// Null references are always valid and do not belong to a local reference frame. Therefore,
    /// the returned `AObject` always has the `'static` lifetime.
    pub const fn null() -> AObject<'static> {
        unsafe { AObject::from_raw(std::ptr::null_mut() as aobject) }
    }
}

impl std::default::Default for AObject<'_> {
    fn default() -> Self {
        Self::null()
    }
}

impl AObjectRef for AObject<'_> {
    type Kind<'env> = AObject<'env>;
    type GlobalKind = AObject<'static>;

    fn as_raw(&self) -> aobject {
        self.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        AObject::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        AObject::from_raw(global_ref)
    }
}


