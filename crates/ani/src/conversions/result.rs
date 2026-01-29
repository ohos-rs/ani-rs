//! Result Type Conversion
//!
//! Implements Rust Result type to ANI conversion
//! - Result<T, E> -> T (on success) or throws exception (on failure)

use std::fmt::Debug;

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;

use super::traits::{ToAni, TypeInfo};

// ============================================================================
// TypeInfo for Result<T, E>
// ============================================================================

impl<T: TypeInfo, E> TypeInfo for std::result::Result<T, E> {
    fn type_signature() -> &'static str {
        // Result type returns T's signature (exceptions handled via error mechanism)
        T::type_signature()
    }
    fn ani_c_type() -> &'static str {
        T::ani_c_type()
    }
}

// ============================================================================
// Result<T, E> to ANI Conversion
// ============================================================================

impl<'env, T, E> ToAni<'env> for std::result::Result<T, E>
where
    T: ToAni<'env>,
    E: Debug,
{
    type Output = T::Output;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Ok(value) => value.to_ani(env),
            Err(e) => {
                // Throw ANI exception
                let error_msg = format!("{:?}", e);
                let _ = throw_error(env, &error_msg);
                Err(Error::Exception(error_msg))
            }
        }
    }
}

// ============================================================================
// Error Handling Helper Functions
// ============================================================================

/// Throw an ANI error
pub fn throw_error(env: &Env<'_>, message: &str) -> Result<()> {
    unsafe {
        let api = &*(*env.as_raw());

        // Create error message string
        let ani_msg = env.create_string(message)?;

        // Find Error class
        let error_class = env.find_class("Lstd/core/Error;")?;

        // Find constructor
        let ctor = env.find_constructor(&error_class, "Lstd/core/String;:V")?;

        // Create error object
        let args = [crate::types::ani_value_ref(ani_msg.as_raw() as sys::ani_ref)];
        let error_obj = env.new_object(&error_class, &ctor, &args[..])?;

        // Throw exception
        let status = (api.ThrowError.unwrap())(env.as_raw(), error_obj.as_raw() as sys::ani_error);

        if status != sys::ani_status_ANI_OK {
            return Err(Error::Status(crate::error::Status::from(status)));
        }

        Ok(())
    }
}

/// Throw a type error
pub fn throw_type_error(env: &Env<'_>, expected: &str, got: &str) -> Result<()> {
    let message = format!("Type error: expected {}, got {}", expected, got);
    throw_error(env, &message)
}

/// Throw a null pointer error
pub fn throw_null_error(env: &Env<'_>, name: &str) -> Result<()> {
    let message = format!("Null pointer error: {} is null", name);
    throw_error(env, &message)
}

/// Check if there is a pending exception
pub fn check_exception(env: &Env<'_>) -> bool {
    unsafe {
        let api = &*(*env.as_raw());
        let mut has_error: sys::ani_boolean = 0;
        let status = (api.ExistUnhandledError.unwrap())(env.as_raw(), &mut has_error);
        status == sys::ani_status_ANI_OK && has_error != 0
    }
}

/// Clear pending exception
pub fn clear_exception(env: &Env<'_>) -> Result<()> {
    unsafe {
        let api = &*(*env.as_raw());
        let status = (api.ResetError.unwrap())(env.as_raw());
        if status != sys::ani_status_ANI_OK {
            return Err(Error::Status(crate::error::Status::from(status)));
        }
        Ok(())
    }
}

/// Get current exception
pub fn get_exception(env: &Env<'_>) -> Option<sys::ani_error> {
    unsafe {
        let api = &*(*env.as_raw());
        let mut error: sys::ani_error = std::ptr::null_mut();
        let status = (api.GetUnhandledError.unwrap())(env.as_raw(), &mut error);
        if status == sys::ani_status_ANI_OK && !error.is_null() {
            Some(error)
        } else {
            None
        }
    }
}

// ============================================================================
// Rust Error to ANI Error Conversion
// ============================================================================

/// ANI throwable error trait
pub trait AniThrowable {
    /// Throw this error as an ANI exception
    fn throw(&self, env: &Env<'_>) -> Result<()>;
}

impl<E: Debug> AniThrowable for E {
    fn throw(&self, env: &Env<'_>) -> Result<()> {
        throw_error(env, &format!("{:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_type_signature() {
        assert_eq!(<std::result::Result<i32, String>>::type_signature(), "I");
        assert_eq!(
            <std::result::Result<String, std::io::Error>>::type_signature(),
            "Lstd/core/String;"
        );
    }
}
