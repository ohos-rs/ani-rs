use crate::{
    objects::{AObject, AObjectRef},
    sys::{aobject, aobjectArray},
};

use super::AsAArrayRaw;

/// Lifetime'd representation of a [`aobjectArray`] which wraps a [`AObject`] reference
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AObjectArray<'local>(AObject<'local>);

impl<'local> AsRef<AObjectArray<'local>> for AObjectArray<'local> {
    fn as_ref(&self) -> &AObjectArray<'local> {
        self
    }
}

impl<'local> AsRef<AObject<'local>> for AObjectArray<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for AObjectArray<'local> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<AObjectArray<'local>> for AObject<'local> {
    fn from(other: AObjectArray) -> AObject {
        other.0
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local> From<AObject<'local>> for AObjectArray<'local> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local, 'obj_ref> From<&'obj_ref AObject<'local>> for &'obj_ref AObjectArray<'local> {
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `AObjectArray` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const AObjectArray<'local>) }
    }
}

unsafe impl<'local> AsAArrayRaw<'local> for AObjectArray<'local> {}

impl AObjectArray<'_> {
    /// Creates a [`AObjectArray`] that wraps the given `raw` [`aobjectArray`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw ANI local reference.
    /// * There must not be any other `AObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub const unsafe fn from_raw(raw: aobjectArray) -> Self {
        Self(AObject::from_raw(raw as aobject))
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> aobjectArray {
        self.0.into_raw() as aobjectArray
    }
}

impl AObjectRef for AObjectArray<'_> {
    type Kind<'env> = AObjectArray<'env>;
    type GlobalKind = AObjectArray<'static>;

    fn as_raw(&self) -> aobject {
        self.0.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        AObjectArray::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        AObjectArray::from_raw(global_ref)
    }
}


