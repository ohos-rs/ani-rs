//! Result Type Conversion
//!
//! Implements Rust Result type to ANI conversion
//! - Result<T, E> -> T (on success) or throws exception (on failure)

use std::fmt::Debug;

use crate::env::Env;
use crate::error::{AniErrorPayload, Error, Result};
use crate::sys;
use crate::{ani_call, ani_call_ret_result};

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
    E: AniErrorPayload,
{
    type Output = T::Output;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Ok(value) => value.to_ani(env),
            Err(e) => {
                let error_msg = e.to_string();
                if let Ok(error) = crate::error::payload_to_ani_error(env, &e) {
                    let _ = crate::ani_call!(env, ThrowError, error.into_raw());
                }
                Err(Error::new(crate::error::Status::PendingError, error_msg))
            }
        }
    }
}

// ============================================================================
// Error Handling Helper Functions
// ============================================================================

/// Throw an ANI error
pub fn throw_error(env: &Env<'_>, message: &str) -> Result<()> {
    let error = crate::error::Error::new(crate::error::Status::Error, message);
    let business_error = crate::error::BusinessError::from(error);
    unsafe { business_error.throw_into(env.as_raw()) };
    Ok(())
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
    ani_call_ret_result!(env, ExistUnhandledError, sys::ani_boolean, 0)
        .map(|r| r != 0)
        .unwrap_or(false)
}

/// Clear pending exception
pub fn clear_exception(env: &Env<'_>) -> Result<()> {
    ani_call!(env, ResetError)
}

/// Get current exception
pub fn get_exception(env: &Env<'_>) -> Option<sys::ani_error> {
    ani_call_ret_result!(env, GetUnhandledError, sys::ani_error, std::ptr::null_mut())
        .ok()
        .filter(|p| !p.is_null())
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
