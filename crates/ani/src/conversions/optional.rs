//! Option Type Conversion
//!
//! Implements conversion between Rust Option types and ANI nullable types
//! - Option<T> <-> nullable T (null | T)

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::*;

use super::boxed::Boxable;
use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// TypeInfo for Option<T>
// ============================================================================

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn type_signature() -> &'static str {
        // Option types need to use boxed types
        // e.g., Option<i32> -> Lstd/core/Int;
        T::type_signature()
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

// ============================================================================
// Conversion for Option<primitive types> - requires boxing
// ============================================================================

impl<'env> ToAni<'env> for Option<i32> {
    type Output = sys::ani_ref;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let boxed = value.box_value(env)?;
                Ok(boxed.as_raw() as sys::ani_ref)
            }
            None => Ok(std::ptr::null_mut()),
        }
    }
}

impl<'env> FromAni<'env> for Option<i32> {
    type Input = sys::ani_ref;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            // Unbox to get the value
            let obj = unsafe { AniObject::from_raw(value as sys::ani_object) };
            let unboxed = env.call_method_by_name_int(&obj, "unboxed", Some(":I"))?;
            Ok(Some(unboxed))
        }
    }
}

impl<'env> ToAni<'env> for Option<i64> {
    type Output = sys::ani_ref;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let boxed = value.box_value(env)?;
                Ok(boxed.as_raw() as sys::ani_ref)
            }
            None => Ok(std::ptr::null_mut()),
        }
    }
}

impl<'env> FromAni<'env> for Option<i64> {
    type Input = sys::ani_ref;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            let obj = unsafe { AniObject::from_raw(value as sys::ani_object) };
            let unboxed = env.call_method_by_name_long(&obj, "unboxed", Some(":J"))?;
            Ok(Some(unboxed))
        }
    }
}

impl<'env> ToAni<'env> for Option<f64> {
    type Output = sys::ani_ref;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let boxed = value.box_value(env)?;
                Ok(boxed.as_raw() as sys::ani_ref)
            }
            None => Ok(std::ptr::null_mut()),
        }
    }
}

impl<'env> FromAni<'env> for Option<f64> {
    type Input = sys::ani_ref;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            let obj = unsafe { AniObject::from_raw(value as sys::ani_object) };
            let unboxed = env.call_method_by_name_double(&obj, "unboxed", Some(":D"))?;
            Ok(Some(unboxed))
        }
    }
}

impl<'env> ToAni<'env> for Option<bool> {
    type Output = sys::ani_ref;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let boxed = value.box_value(env)?;
                Ok(boxed.as_raw() as sys::ani_ref)
            }
            None => Ok(std::ptr::null_mut()),
        }
    }
}

impl<'env> FromAni<'env> for Option<bool> {
    type Input = sys::ani_ref;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            let obj = unsafe { AniObject::from_raw(value as sys::ani_object) };
            let unboxed = env.call_method_by_name_boolean(&obj, "unboxed", Some(":Z"))?;
            Ok(Some(unboxed))
        }
    }
}

// ============================================================================
// Option<String> - strings can directly use null
// ============================================================================

impl<'env> ToAni<'env> for Option<String> {
    type Output = sys::ani_string;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(s) => {
                let ani_str = env.create_string(&s)?;
                Ok(ani_str.into_raw())
            }
            None => Ok(std::ptr::null_mut()),
        }
    }
}

impl<'env> FromAni<'env> for Option<String> {
    type Input = sys::ani_string;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            let ani_str = unsafe { AniString::from_raw(value) };
            let s = env.get_string(&ani_str)?;
            Ok(Some(s))
        }
    }
}

// ============================================================================
// Generic Option<T> implementation (for object types)
// ============================================================================

/// General Option conversion helper
pub struct OptionHelper;

impl OptionHelper {
    /// Check if ANI reference is null
    pub fn is_null(value: sys::ani_ref) -> bool {
        value.is_null()
    }

    /// Convert null to None
    pub fn null_to_none<T>(value: sys::ani_ref) -> Option<sys::ani_ref> {
        if value.is_null() { None } else { Some(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_type_signature() {
        assert_eq!(<Option<i32>>::type_signature(), "I");
        assert_eq!(<Option<String>>::type_signature(), "Lstd/core/String;");
    }

    #[test]
    fn test_option_helper() {
        assert!(OptionHelper::is_null(std::ptr::null_mut()));
        assert!(!OptionHelper::is_null(1 as sys::ani_ref));
    }
}
