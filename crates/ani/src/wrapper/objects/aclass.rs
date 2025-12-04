use crate::{
    objects::AObject,
    sys::{aclass, aobject},
};

use super::AObjectRef;

/// Lifetime'd representation of a `jclass`. Just a `AObject` wrapped in a new
/// class.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AClass<'local>(AObject<'local>);

impl<'local> AsRef<AClass<'local>> for AClass<'local> {
    fn as_ref(&self) -> &AClass<'local> {
        self
    }
}

impl<'local> AsRef<AObject<'local>> for AClass<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for AClass<'local> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<AClass<'local>> for AObject<'local> {
    fn from(other: AClass) -> AObject {
        other.0
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local> From<AObject<'local>> for AClass<'local> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local, 'obj_ref> From<&'obj_ref AObject<'local>> for &'obj_ref AClass<'local> {
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `AClass` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const AClass<'local>) }
    }
}

impl AClass<'_> {
    /// Creates a [`AClass`] that wraps the given `raw` [`aclass`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw ANI local reference.
    /// * There must not be any other `AObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub const unsafe fn from_raw(raw: aclass) -> Self {
        Self(AObject::from_raw(raw as aobject))
    }

    /// Returns the raw ANI pointer.
    pub const fn as_raw(&self) -> aclass {
        self.0.as_raw() as aclass
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> aclass {
        self.0.into_raw() as aclass
    }
}

impl AObjectRef for AClass<'_> {
    type Kind<'env> = AClass<'env>;
    type GlobalKind = AClass<'static>;

    fn as_raw(&self) -> aobject {
        self.0.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        AClass::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        AClass::from_raw(global_ref)
    }
}


