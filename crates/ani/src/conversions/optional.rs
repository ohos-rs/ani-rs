//! Option Type Conversion
//!
//! Implements conversion between Rust Option types and ANI nullable types
//! - Option<T> <-> nullable T (null | T)
//!
//! In ANI, nullable values are represented as union object references at the
//! ABI boundary. Even `Option<String>` and boxed primitive options flow through
//! `ani_object` when exposed from native functions.

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::*;

use super::boxed::{Boxable, Unboxable};
use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// TypeInfo for Option<T>
// ============================================================================

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn type_signature() -> &'static str {
        // Option types use the inner type's signature
        // For primitive types, this will be their boxed form when used as Option
        T::type_signature()
    }

    fn ani_c_type() -> &'static str {
        // Nullable unions are represented as ANI objects.
        "ani_object"
    }
}

// ============================================================================
// Generic Option<T> for Boxable/Unboxable types (primitives)
// ============================================================================

/// Marker trait for primitive types that need boxing when used in Option
pub trait OptionalPrimitive<'env>:
    Boxable<'env, Boxed = AniObject<'env>> + Unboxable<'env>
{
}

// Implement OptionalPrimitive for all types that are both Boxable and Unboxable
impl<'env> OptionalPrimitive<'env> for bool {}
impl<'env> OptionalPrimitive<'env> for i8 {}
impl<'env> OptionalPrimitive<'env> for i16 {}
impl<'env> OptionalPrimitive<'env> for u16 {}
impl<'env> OptionalPrimitive<'env> for i32 {}
impl<'env> OptionalPrimitive<'env> for i64 {}
impl<'env> OptionalPrimitive<'env> for f32 {}
impl<'env> OptionalPrimitive<'env> for f64 {}

fn is_option_none_ref(env: &Env<'_>, value: sys::ani_object) -> bool {
    if value.is_null() {
        return true;
    }
    let value_ref = unsafe { AniRef::from_raw(value as sys::ani_ref) };
    env.is_null(&value_ref).unwrap_or(false)
}

impl<'env, T> ToAni<'env> for Option<T>
where
    T: OptionalPrimitive<'env>,
{
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let boxed = value.box_value(env)?;
                Ok(boxed.into_raw())
            }
            None => env.get_null_object(),
        }
    }
}

impl<'env, T> FromAni<'env> for Option<T>
where
    T: OptionalPrimitive<'env>,
{
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if is_option_none_ref(env, value) {
            Ok(None)
        } else {
            let obj = unsafe { AniObject::from_raw(value) };
            let unboxed = T::unbox(env, &obj)?;
            Ok(Some(unboxed))
        }
    }
}

// ============================================================================
// Option<String> - strings can directly use null
// ============================================================================

impl<'env> ToAni<'env> for Option<String> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(s) => {
                let ani_str = env.create_string(&s)?;
                Ok(ani_str.into_raw() as sys::ani_object)
            }
            None => env.get_null_object(),
        }
    }
}

impl<'env> FromAni<'env> for Option<String> {
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if is_option_none_ref(env, value) {
            Ok(None)
        } else {
            let ani_str = unsafe { AniString::from_raw(value as sys::ani_string) };
            let s = env.get_string(&ani_str)?;
            Ok(Some(s))
        }
    }
}

// ============================================================================
// Option<AniObject> - objects can directly use null
// ============================================================================

impl<'env> ToAni<'env> for Option<AniObject<'env>> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(obj) => Ok(obj.as_raw()),
            None => env.get_null_object(),
        }
    }
}

impl<'env> FromAni<'env> for Option<AniObject<'env>> {
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if is_option_none_ref(env, value) {
            Ok(None)
        } else {
            Ok(Some(unsafe { AniObject::from_raw(value) }))
        }
    }
}

// ============================================================================
// General Option conversion helper
// ============================================================================

/// General Option conversion helper utilities
pub struct OptionHelper;

impl OptionHelper {
    /// Check if ANI reference is null
    #[inline]
    pub fn is_null(value: sys::ani_ref) -> bool {
        value.is_null()
    }

    /// Convert null to None, non-null to Some
    #[inline]
    pub fn null_to_none(value: sys::ani_ref) -> Option<sys::ani_ref> {
        if value.is_null() { None } else { Some(value) }
    }

    /// Convert Option to nullable pointer
    #[inline]
    pub fn option_to_nullable<T>(opt: Option<*mut T>) -> *mut T {
        opt.unwrap_or(std::ptr::null_mut())
    }
}

// ============================================================================
// Macro for extending Option support to custom types
// ============================================================================

/// Macro to implement Option<T> support for reference types
///
/// Use this macro to add Option support for custom types that implement
/// ToAni and FromAni with reference-based outputs.
///
/// # Example
///
/// ```ignore
/// impl_option_for_ref_type!(MyCustomType, sys::ani_object);
/// ```
#[macro_export]
macro_rules! impl_option_for_ref_type {
    ($ty:ty, $ani_ty:ty) => {
        impl<'env> $crate::bindgen_runtime::ToAni<'env> for Option<$ty>
        where
            $ty: $crate::bindgen_runtime::ToAni<'env, Output = $ani_ty>,
        {
            type Output = $ani_ty;

            fn to_ani(self, env: &$crate::env::Env<'env>) -> $crate::error::Result<Self::Output> {
                match self {
                    Some(value) => value.to_ani(env),
                    None => Ok(std::ptr::null_mut()),
                }
            }
        }

        impl<'env> $crate::bindgen_runtime::FromAni<'env> for Option<$ty>
        where
            $ty: $crate::bindgen_runtime::FromAni<'env, Input = $ani_ty>,
        {
            type Input = $ani_ty;

            fn from_ani(
                env: &$crate::env::Env<'env>,
                value: Self::Input,
            ) -> $crate::error::Result<Self> {
                if value.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(<$ty>::from_ani(env, value)?))
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_type_signature() {
        assert_eq!(<Option<i32>>::type_signature(), "I");
        assert_eq!(<Option<i64>>::type_signature(), "J");
        assert_eq!(<Option<f64>>::type_signature(), "D");
        assert_eq!(<Option<bool>>::type_signature(), "Z");
        assert_eq!(<Option<String>>::type_signature(), "Lstd/core/String;");
    }

    #[test]
    fn test_option_ani_c_type() {
        assert_eq!(<Option<i32>>::ani_c_type(), "ani_object");
        assert_eq!(<Option<String>>::ani_c_type(), "ani_object");
    }

    #[test]
    fn test_option_helper() {
        assert!(OptionHelper::is_null(std::ptr::null_mut()));
        assert!(!OptionHelper::is_null(1 as sys::ani_ref));

        assert!(OptionHelper::null_to_none(std::ptr::null_mut()).is_none());
        assert!(OptionHelper::null_to_none(1 as sys::ani_ref).is_some());
    }

    #[test]
    fn test_option_to_nullable() {
        let some_ptr: Option<*mut i32> = Some(1 as *mut i32);
        let none_ptr: Option<*mut i32> = None;

        assert!(!OptionHelper::option_to_nullable(some_ptr).is_null());
        assert!(OptionHelper::option_to_nullable(none_ptr).is_null());
    }
}
