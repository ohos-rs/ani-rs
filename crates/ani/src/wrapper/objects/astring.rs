use crate::{
    objects::AObject,
    sys::{aobject, astring},
};

use super::AObjectRef;

/// Lifetime'd representation of a `jstring`. Just a `AObject` wrapped in a new
/// class.
#[repr(transparent)]
#[derive(Default)]
pub struct AString<'local>(AObject<'local>);

impl<'local> AsRef<AString<'local>> for AString<'local> {
    fn as_ref(&self) -> &AString<'local> {
        self
    }
}

impl<'local> AsRef<AObject<'local>> for AString<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for AString<'local> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<AString<'local>> for AObject<'local> {
    fn from(other: AString) -> AObject {
        other.0
    }
}

impl<'local> From<AObject<'local>> for AString<'local> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

impl<'local, 'obj_ref> From<&'obj_ref AObject<'local>> for &'obj_ref AString<'local> {
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `AString` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const AString<'local>) }
    }
}

impl AString<'_> {
    /// Creates a [`AString`] that wraps the given `raw` [`astring`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw ANI local reference.
    /// * There must not be any other `AObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub const unsafe fn from_raw(raw: astring) -> Self {
        Self(AObject::from_raw(raw as aobject))
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> astring {
        self.0.into_raw() as astring
    }
}

impl AObjectRef for AString<'_> {
    type Kind<'env> = AString<'env>;
    type GlobalKind = AString<'static>;

    fn as_raw(&self) -> aobject {
        self.0.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        AString::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        AString::from_raw(global_ref)
    }
}


