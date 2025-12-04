use crate::{
    objects::AObject,
    sys::{aobject, athrowable},
};

use super::AObjectRef;

/// Lifetime'd representation of a `jthrowable`. Just a `AObject` wrapped in a
/// new class.
#[repr(transparent)]
#[derive(Default)]
pub struct AThrowable<'local>(AObject<'local>);

impl<'local> AsRef<AThrowable<'local>> for AThrowable<'local> {
    fn as_ref(&self) -> &AThrowable<'local> {
        self
    }
}

impl<'local> AsRef<AObject<'local>> for AThrowable<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for AThrowable<'local> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<AThrowable<'local>> for AObject<'local> {
    fn from(other: AThrowable) -> AObject {
        other.0
    }
}

impl<'local> From<AObject<'local>> for AThrowable<'local> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

impl<'local, 'obj_ref> From<&'obj_ref AObject<'local>> for &'obj_ref AThrowable<'local> {
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `AThrowable` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const AThrowable<'local>) }
    }
}

impl AThrowable<'_> {
    /// Creates a [`AThrowable`] that wraps the given `raw` [`athrowable`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw ANI local reference.
    /// * There must not be any other `AObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub const unsafe fn from_raw(raw: athrowable) -> Self {
        Self(AObject::from_raw(raw as aobject))
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> athrowable {
        self.0.into_raw() as athrowable
    }
}

impl AObjectRef for AThrowable<'_> {
    type Kind<'env> = AThrowable<'env>;
    type GlobalKind = AThrowable<'static>;

    fn as_raw(&self) -> aobject {
        self.0.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        AThrowable::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        AThrowable::from_raw(global_ref)
    }
}


