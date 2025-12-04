use std::marker::PhantomData;

use crate::{
    objects::{AObject, AObjectRef},
    sys::{aarray, aobject},
};

use super::TypeArray;

#[cfg(doc)]
use crate::ANIEnv;

/// Lifetime'd representation of a [`jarray`] which wraps a [`AObject`] reference
///
/// This is a wrapper type for a [`AObject`] local reference that's used to
/// differentiate VM array types.
#[repr(transparent)]
#[derive(Debug)]
pub struct APrimitiveArray<'local, T: TypeArray> {
    obj: AObject<'local>,
    lifetime: PhantomData<(&'local (), T)>,
}

impl<'local, T: TypeArray> AsRef<APrimitiveArray<'local, T>> for APrimitiveArray<'local, T> {
    fn as_ref(&self) -> &APrimitiveArray<'local, T> {
        self
    }
}

impl<'local, T: TypeArray> AsMut<APrimitiveArray<'local, T>> for APrimitiveArray<'local, T> {
    fn as_mut(&mut self) -> &mut APrimitiveArray<'local, T> {
        self
    }
}

impl<'local, T: TypeArray> AsRef<AObject<'local>> for APrimitiveArray<'local, T> {
    fn as_ref(&self) -> &AObject<'local> {
        &self.obj
    }
}

impl<'local, T: TypeArray> ::std::ops::Deref for APrimitiveArray<'local, T> {
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<'local, T: TypeArray> From<APrimitiveArray<'local, T>> for AObject<'local> {
    fn from(other: APrimitiveArray<'local, T>) -> AObject<'local> {
        other.obj
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local, T: TypeArray> From<AObject<'local>> for APrimitiveArray<'local, T> {
    fn from(other: AObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

/// This conversion assumes that the `AObject` is a pointer to a class object.
impl<'local, 'obj_ref, T: TypeArray> From<&'obj_ref AObject<'local>>
    for &'obj_ref APrimitiveArray<'local, T>
{
    fn from(other: &'obj_ref AObject<'local>) -> Self {
        // Safety: `APrimitiveArray` is `repr(transparent)` around `AObject`.
        unsafe { &*(other as *const AObject<'local> as *const APrimitiveArray<'local, T>) }
    }
}

impl<T: TypeArray> std::default::Default for APrimitiveArray<'_, T> {
    fn default() -> Self {
        Self {
            obj: AObject::null(),
            lifetime: PhantomData,
        }
    }
}

impl<T: TypeArray> APrimitiveArray<'_, T> {
    /// Creates a [`APrimitiveArray`] that wraps the given `raw` [`aarray`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw ANI local reference.
    /// * There must not be any other `AObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub const unsafe fn from_raw(raw: aarray) -> Self {
        Self {
            obj: AObject::from_raw(raw as aobject),
            lifetime: PhantomData,
        }
    }

    /// Unwrap to the raw ani type.
    pub const fn into_raw(self) -> aarray {
        self.obj.into_raw() as aarray
    }
}

/// Lifetime'd representation of a boolean array which wraps a [`AObject`] reference
pub type ABooleanArray<'local> = APrimitiveArray<'local, crate::sys::aboolean>;

/// Lifetime'd representation of a byte array which wraps a [`AObject`] reference
pub type AByteArray<'local> = APrimitiveArray<'local, crate::sys::abyte>;

/// Lifetime'd representation of a char array which wraps a [`AObject`] reference
pub type ACharArray<'local> = APrimitiveArray<'local, crate::sys::achar>;

/// Lifetime'd representation of a short array which wraps a [`AObject`] reference
pub type AShortArray<'local> = APrimitiveArray<'local, crate::sys::ashort>;

/// Lifetime'd representation of an int array which wraps a [`AObject`] reference
pub type AIntArray<'local> = APrimitiveArray<'local, crate::sys::aint>;

/// Lifetime'd representation of a long array which wraps a [`AObject`] reference
pub type ALongArray<'local> = APrimitiveArray<'local, crate::sys::along>;

/// Lifetime'd representation of a float array which wraps a [`AObject`] reference
pub type AFloatArray<'local> = APrimitiveArray<'local, crate::sys::afloat>;

/// Lifetime'd representation of a double array which wraps a [`AObject`] reference
pub type ADoubleArray<'local> = APrimitiveArray<'local, crate::sys::adouble>;

/// Trait to access the raw `aarray` pointer for types that wrap an array reference
///
/// # Safety
///
/// Implementing this trait will allow a type to be passed to [`ANIEnv::get_array_length()`]
/// or other ANI APIs that only work with a valid reference to an array (or `null`)
///
pub unsafe trait AsAArrayRaw<'local>: AsRef<AObject<'local>> {
    /// Returns the raw ANI pointer as a `aarray`
    fn as_aarray_raw(&self) -> aarray {
        self.as_ref().as_raw() as aarray
    }
}

unsafe impl<'local, T: TypeArray> AsAArrayRaw<'local> for APrimitiveArray<'local, T> {}

impl<T: TypeArray> AObjectRef for APrimitiveArray<'_, T> {
    type Kind<'env> = APrimitiveArray<'env, T>;
    type GlobalKind = APrimitiveArray<'static, T>;

    fn as_raw(&self) -> aobject {
        self.obj.as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        APrimitiveArray::from_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        APrimitiveArray::from_raw(global_ref)
    }
}


