//! String Type Conversion
//!
//! Implements conversion between Rust string types and ANI string types
//! - String <-> ani_string
//! - &str -> ani_string
//! - Cow<str> <-> ani_string

use std::borrow::Cow;
use std::ffi::CString;

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;
use crate::types::AniString;

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// String - Lstd/core/String;
// ============================================================================

impl TypeInfo for String {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for String {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(&self)
    }
}

impl<'env> FromAni<'env> for String {
    type Input = AniString<'env>;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        env.get_string(&value)
    }
}

// ============================================================================
// &str - Lstd/core/String;
// ============================================================================

impl TypeInfo for &str {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env, 'a> ToAni<'env> for &'a str {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(self)
    }
}

// ============================================================================
// Cow<str> - Lstd/core/String;
// ============================================================================

impl<'a> TypeInfo for Cow<'a, str> {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env, 'a> ToAni<'env> for Cow<'a, str> {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(&self)
    }
}

impl<'env> FromAni<'env> for Cow<'static, str> {
    type Input = AniString<'env>;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let s = env.get_string(&value)?;
        Ok(Cow::Owned(s))
    }
}

// ============================================================================
// Box<str> - Lstd/core/String;
// ============================================================================

impl TypeInfo for Box<str> {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for Box<str> {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(&self)
    }
}

impl<'env> FromAni<'env> for Box<str> {
    type Input = AniString<'env>;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let s = env.get_string(&value)?;
        Ok(s.into_boxed_str())
    }
}

// ============================================================================
// CString - for FFI
// ============================================================================

impl TypeInfo for CString {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for CString {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let s = self.to_string_lossy();
        env.create_string(&s)
    }
}

impl<'env> FromAni<'env> for CString {
    type Input = AniString<'env>;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let s = env.get_string(&value)?;
        CString::new(s).map_err(|_| {
            Error::new(
                crate::error::Status::InvalidType,
                "Invalid string for CString",
            )
        })
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get Rust String from raw ANI string pointer
///
/// # Safety
///
/// Caller must ensure env and string pointers are valid
pub unsafe fn string_from_raw(env: *mut sys::ani_env, string: sys::ani_string) -> Result<String> {
    if env.is_null() || string.is_null() {
        return Err(Error::new(
            crate::error::Status::InvalidArgs,
            format!("Null pointer: {}", "string"),
        ));
    }

    unsafe {
        let api = &*(*env);

        // Get string length
        let mut len: usize = 0;
        let status = (api.String_GetUTF8Size.unwrap())(env, string, &mut len);
        if status != sys::ani_status_ANI_OK {
            return Err(Error::from_status(crate::error::Status::from(status)));
        }

        // Allocate buffer and get content
        let mut buffer = vec![0u8; len + 1];
        let mut chars_copied: usize = 0;

        let status = (api.String_GetUTF8.unwrap())(
            env,
            string,
            buffer.as_mut_ptr() as *mut i8,
            len + 1,
            &mut chars_copied,
        );

        if status != sys::ani_status_ANI_OK {
            return Err(Error::from_status(crate::error::Status::from(status)));
        }

        buffer.truncate(chars_copied);
        String::from_utf8(buffer)
            .map_err(|_| Error::new(crate::error::Status::InvalidType, "Invalid UTF-8 string"))
    }
}

/// Create ANI string
///
/// # Safety
///
/// Caller must ensure env pointer is valid
pub unsafe fn string_to_raw(env: *mut sys::ani_env, s: &str) -> Result<sys::ani_string> {
    if env.is_null() {
        return Err(Error::new(
            crate::error::Status::InvalidArgs,
            format!("Null pointer: {}", "env"),
        ));
    }

    unsafe {
        let api = &*(*env);
        let c_str = CString::new(s)
            .map_err(|_| Error::new(crate::error::Status::InvalidType, "Invalid string"))?;

        let mut result: sys::ani_string = std::ptr::null_mut();
        let status = (api.String_NewUTF8.unwrap())(env, c_str.as_ptr(), s.len(), &mut result);

        if status != sys::ani_status_ANI_OK {
            return Err(Error::from_status(crate::error::Status::from(status)));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_type_signature() {
        assert_eq!(String::type_signature(), "Lstd/core/String;");
        assert_eq!(<&str>::type_signature(), "Lstd/core/String;");
    }
}
