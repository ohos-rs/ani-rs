use crate::{objects::AObject, sys::aobject};

/// Lifetime'd representation of a `jobject` that is an instance of the
/// ByteBuffer class. Just a `AObject` wrapped in a new class.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AByteBuffer<'local>(AObject<'local>);

impl<'local> AsRef<AByteBuffer<'local>> for AByteBuffer<'local> {
    fn as_ref(&self) -> &AByteBuffer<'local> {
        self
    }
}

impl<'local> AsRef<AObject<'local>> for AByteBuffer<'local> {
    fn as_ref(&self) -> &AObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for AByteBuffer<'local> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<AByteBuffer<'local>> for AObject<'local> {
    fn from(other: AByteBuffer) -> AObject {
        other.0
    }
}

impl<'local> From<AObject<'local>> for AByteBuffer<'local> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

impl<'local, 'obj_ref> From<&'obj_ref AObject<'local>> for &'obj_ref AByteBuffer<'local> {
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `AByteBuffer` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const AByteBuffer<'local>) }
    }
}

impl AByteBuffer<'_> {
    /// Creates a [`AByteBuffer`] that wraps the given `raw` [`aobject`]
    ///
    /// # Safety
    /// No runtime check is made to verify that the given [`aobject`] is an instance of
    /// a `ByteBuffer`.
    pub const unsafe fn from_raw(raw: aobject) -> Self {
        Self(AObject::from_raw(raw as aobject))
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> aobject {
        self.0.into_raw() as aobject
    }
}


