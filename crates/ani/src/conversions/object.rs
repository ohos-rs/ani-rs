//! Object Type Conversion
//!
//! Implements conversion between Rust custom structs and ANI object types

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;
use crate::types::*;

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// AniObject Conversion
// ============================================================================

impl<'env> TypeInfo for AniObject<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ToAni<'env> for AniObject<'env> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniObject<'env> {
    type Input = sys::ani_object;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "object"),
            ));
        }
        Ok(unsafe { AniObject::from_raw(value) })
    }
}

// ============================================================================
// AniClass Conversion
// ============================================================================

impl<'env> TypeInfo for AniClass<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/Class;"
    }
    fn ani_c_type() -> &'static str {
        "ani_class"
    }
}

impl<'env> ToAni<'env> for AniClass<'env> {
    type Output = sys::ani_class;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniClass<'env> {
    type Input = sys::ani_class;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "class"),
            ));
        }
        Ok(unsafe { AniClass::from_raw(value) })
    }
}

// ============================================================================
// AniString Conversion
// ============================================================================

impl<'env> TypeInfo for AniString<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for AniString<'env> {
    type Output = sys::ani_string;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniString<'env> {
    type Input = sys::ani_string;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "string"),
            ));
        }
        Ok(unsafe { AniString::from_raw(value) })
    }
}

// ============================================================================
// Native Pointer Wrapper
// ============================================================================

/// Native pointer wrapper for passing Rust object pointers in ANI
#[repr(transparent)]
pub struct NativePointer<T> {
    ptr: i64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> NativePointer<T> {
    /// Create NativePointer from Box
    pub fn from_box(boxed: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(boxed) as i64,
            _marker: std::marker::PhantomData,
        }
    }

    /// Convert back to Box (consumes NativePointer)
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr as *mut T) }
    }

    /// Get reference
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn as_ref(&self) -> &T {
        unsafe { &*(self.ptr as *const T) }
    }

    /// Get mutable reference
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.ptr as *mut T) }
    }

    /// Get raw pointer value (i64)
    pub fn as_raw(&self) -> i64 {
        self.ptr
    }

    /// Create from raw pointer value
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn from_raw(ptr: i64) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> TypeInfo for NativePointer<T> {
    fn type_signature() -> &'static str {
        "J"
    } // passed as long
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
}

impl<'env, T> ToAni<'env> for NativePointer<T> {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.ptr)
    }
}

impl<'env, T> FromAni<'env> for NativePointer<T> {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(unsafe { NativePointer::from_raw(value) })
    }
}

// ============================================================================
// Generic Object Conversion Helper
// ============================================================================

/// Object conversion helper trait
pub trait ObjectConversion<'env> {
    /// Convert from AniObject
    fn from_ani_object(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self>
    where
        Self: Sized;

    /// Convert to AniObject
    fn to_ani_object(&self, env: &Env<'env>) -> Result<AniObject<'env>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_pointer() {
        let boxed = Box::new(42i32);
        let ptr = NativePointer::from_box(boxed);

        unsafe {
            assert_eq!(*ptr.as_ref(), 42);
            let recovered = ptr.into_box();
            assert_eq!(*recovered, 42);
        }
    }

    #[test]
    fn test_object_type_signature() {
        assert_eq!(<AniObject>::type_signature(), "Lstd/core/Object;");
        assert_eq!(<AniClass>::type_signature(), "Lstd/core/Class;");
    }
}
