//! ANI Error Handling
//!
//! Provides unified error types and result types that support custom business errors.
//!
//! # Usage
//!
//! ## Basic Usage with Status
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! fn may_fail() -> Result<i32> {
//!     Err(Error::new(Status::InvalidArgs, "Invalid arguments"))
//! }
//! ```
//!
//! ## Custom Error Types
//!
//! You can define custom error types that implement `AsRef<str>`:
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! pub enum MyError {
//!     InvalidInput,
//!     NotFound,
//!     Internal(String),
//! }
//!
//! impl AsRef<str> for MyError {
//!     fn as_ref(&self) -> &str {
//!         match self {
//!             MyError::InvalidInput => "InvalidInput",
//!             MyError::NotFound => "NotFound",
//!             MyError::Internal(_) => "InternalError",
//!         }
//!     }
//! }
//!
//! fn custom_error() -> std::result::Result<(), Error<MyError>> {
//!     Err(Error::new(MyError::InvalidInput, "The input is invalid"))
//! }
//! ```
//!
//! ## Using anyhow (with `error_anyhow` feature)
//!
//! When the `error_anyhow` feature is enabled, `anyhow::Error` can be automatically
//! converted to ANI errors:
//!
//! ```rust,ignore
//! use ani::prelude::*;
//! use anyhow::Context;
//!
//! fn may_fail() -> ani::error::Result<()> {
//!     let result = std::fs::read_to_string("config.json")
//!         .context("Failed to read config")?;  // anyhow::Error -> ani::Error
//!     Ok(())
//! }
//! ```

use std::fmt;

use crate::env::Env;
use crate::sys;

// ============================================================================
// Status Code
// ============================================================================

/// ANI Status Code
///
/// Standard status codes returned by ANI API calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    /// Success
    Ok = 0,
    /// General error
    Error = 1,
    /// Invalid arguments
    InvalidArgs = 2,
    /// Invalid type
    InvalidType = 3,
    /// Invalid descriptor
    InvalidDescriptor = 4,
    /// Incorrect reference
    IncorrectRef = 5,
    /// Pending error exists
    PendingError = 6,
    /// Not found
    NotFound = 7,
    /// Already bound
    AlreadyBound = 8,
    /// Reference limit exceeded
    OutOfRef = 9,
    /// Out of memory
    OutOfMemory = 10,
    /// Out of range
    OutOfRange = 11,
    /// Buffer too small
    BufferTooSmall = 12,
    /// Invalid version
    InvalidVersion = 13,
    /// Ambiguous
    Ambiguous = 14,
    /// Generic failure for custom errors
    GenericFailure = 100,
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Status::Ok => "Ok",
            Status::Error => "Error",
            Status::InvalidArgs => "InvalidArgs",
            Status::InvalidType => "InvalidType",
            Status::InvalidDescriptor => "InvalidDescriptor",
            Status::IncorrectRef => "IncorrectRef",
            Status::PendingError => "PendingError",
            Status::NotFound => "NotFound",
            Status::AlreadyBound => "AlreadyBound",
            Status::OutOfRef => "OutOfRef",
            Status::OutOfMemory => "OutOfMemory",
            Status::OutOfRange => "OutOfRange",
            Status::BufferTooSmall => "BufferTooSmall",
            Status::InvalidVersion => "InvalidVersion",
            Status::Ambiguous => "Ambiguous",
            Status::GenericFailure => "GenericFailure",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::GenericFailure
    }
}

impl From<sys::ani_status> for Status {
    fn from(status: sys::ani_status) -> Self {
        match status {
            0 => Status::Ok,
            1 => Status::Error,
            2 => Status::InvalidArgs,
            3 => Status::InvalidType,
            4 => Status::InvalidDescriptor,
            5 => Status::IncorrectRef,
            6 => Status::PendingError,
            7 => Status::NotFound,
            8 => Status::AlreadyBound,
            9 => Status::OutOfRef,
            10 => Status::OutOfMemory,
            11 => Status::OutOfRange,
            12 => Status::BufferTooSmall,
            13 => Status::InvalidVersion,
            14 => Status::Ambiguous,
            _ => Status::Error,
        }
    }
}

impl From<Status> for sys::ani_status {
    fn from(status: Status) -> Self {
        match status {
            Status::Ok => 0,
            Status::Error => 1,
            Status::InvalidArgs => 2,
            Status::InvalidType => 3,
            Status::InvalidDescriptor => 4,
            Status::IncorrectRef => 5,
            Status::PendingError => 6,
            Status::NotFound => 7,
            Status::AlreadyBound => 8,
            Status::OutOfRef => 9,
            Status::OutOfMemory => 10,
            Status::OutOfRange => 11,
            Status::BufferTooSmall => 12,
            Status::InvalidVersion => 13,
            Status::Ambiguous => 14,
            Status::GenericFailure => 1, // Maps to generic error
        }
    }
}

// ============================================================================
// Error Type
// ============================================================================

/// ANI Error Type
///
/// Generic over the status type `S`, which must implement `AsRef<str>`.
/// This allows custom error types to be used as status codes.
///
/// # Type Parameters
///
/// * `S` - The status type, defaults to [`Status`]. Can be any type implementing `AsRef<str>`.
///
/// # Examples
///
/// ```rust
/// use ani::error::{Error, Status};
///
/// // Using default Status
/// let err = Error::new(Status::InvalidArgs, "Invalid argument");
///
/// // Using custom status type
/// enum MyStatus {
///     CustomError,
/// }
///
/// impl AsRef<str> for MyStatus {
///     fn as_ref(&self) -> &str {
///         match self {
///             MyStatus::CustomError => "CustomError",
///         }
///     }
/// }
///
/// let custom_err: Error<MyStatus> = Error::new(MyStatus::CustomError, "Something went wrong");
/// ```
#[derive(Debug)]
pub struct Error<S: AsRef<str> = Status> {
    /// The status/error code
    pub status: S,
    /// Human-readable error message
    pub reason: String,
    /// Optional cause of this error
    pub cause: Option<Box<Error<Status>>>,
}

impl<S: AsRef<str>> Error<S> {
    /// Create a new error with status and reason
    ///
    /// # Arguments
    ///
    /// * `status` - The error status code
    /// * `reason` - Human-readable error message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ani::error::{Error, Status};
    ///
    /// let err = Error::new(Status::NotFound, "Resource not found");
    /// ```
    pub fn new<R: Into<String>>(status: S, reason: R) -> Self {
        Self {
            status,
            reason: reason.into(),
            cause: None,
        }
    }

    /// Create an error from just a reason message, using default status
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ani::error::{Error, Status};
    ///
    /// let err: Error<Status> = Error::from_reason("Something went wrong");
    /// ```
    pub fn from_reason<R: Into<String>>(reason: R) -> Self
    where
        S: Default,
    {
        Self {
            status: S::default(),
            reason: reason.into(),
            cause: None,
        }
    }

    /// Set the cause of this error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ani::error::{Error, Status};
    ///
    /// let mut err = Error::new(Status::Error, "Outer error");
    /// err.set_cause(Error::new(Status::NotFound, "Inner error"));
    /// ```
    pub fn set_cause(&mut self, cause: Error<Status>) {
        self.cause = Some(Box::new(cause));
    }

    /// Create an error with a cause
    pub fn with_cause<R: Into<String>>(status: S, reason: R, cause: Error<Status>) -> Self {
        Self {
            status,
            reason: reason.into(),
            cause: Some(Box::new(cause)),
        }
    }

    /// Get the status code as a string reference
    pub fn status_str(&self) -> &str {
        self.status.as_ref()
    }
}

impl<S: AsRef<str> + fmt::Debug> fmt::Display for Error<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status.as_ref(), self.reason)?;
        if let Some(ref cause) = self.cause {
            write!(f, "\n  Caused by: {}", cause)?;
        }
        Ok(())
    }
}

impl<S: AsRef<str> + fmt::Debug> std::error::Error for Error<S> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e as &dyn std::error::Error)
    }
}

// Convenience methods for common operations
impl Error<Status> {
    /// Create an error from ANI status code
    pub fn from_status(status: Status) -> Self {
        Self::new(status, format!("ANI error: {}", status))
    }
}

// ============================================================================
// Common Error Conversions
// ============================================================================

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::new(Status::Error, format!("UTF-8 error: {}", e))
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Self {
        Error::new(Status::InvalidArgs, format!("Null byte in string: {}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(Status::Error, format!("IO error: {}", e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::new(Status::Error, format!("UTF-8 conversion error: {}", e))
    }
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::from_status(status)
    }
}

impl From<sys::ani_status> for Error {
    fn from(status: sys::ani_status) -> Self {
        Error::from_status(Status::from(status))
    }
}

// ============================================================================
// anyhow Integration (with error_anyhow feature)
// ============================================================================

#[cfg(feature = "error_anyhow")]
impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        // Include the full error chain in the message
        Error::new(Status::GenericFailure, format!("{:?}", e))
    }
}

// ============================================================================
// Result Type
// ============================================================================

/// ANI Result type
///
/// Alias for `std::result::Result<T, Error<S>>`.
/// Defaults to using [`Status`] as the error status type.
pub type Result<T, S = Status> = std::result::Result<T, Error<S>>;

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert ANI status code to Result
///
/// # Examples
///
/// ```rust,ignore
/// use ani::error::check_status;
/// use ani::sys;
///
/// fn call_ani_api(status: sys::ani_status) -> ani::error::Result<()> {
///     check_status(status)
/// }
/// ```
#[inline]
pub fn check_status(status: sys::ani_status) -> Result<()> {
    let s = Status::from(status);
    if s == Status::Ok {
        Ok(())
    } else {
        Err(Error::from_status(s))
    }
}

/// Check if pointer is null
#[inline]
pub fn check_ptr<T>(ptr: *mut T, name: &'static str) -> Result<*mut T> {
    if ptr.is_null() {
        Err(Error::new(
            Status::InvalidArgs,
            format!("Null pointer: {}", name),
        ))
    } else {
        Ok(ptr)
    }
}

// ============================================================================
// BusinessError - ANI Error Wrapper
// ============================================================================

/// ANI Business Error wrapper
///
/// Wraps an [`Error`] and provides methods to throw it into the ANI environment.
/// This corresponds to the `escompat.BusinessError` class in ANI.
///
/// # Examples
///
/// ```rust,ignore
/// use ani::error::{Error, Status, BusinessError};
///
/// let err = Error::new(Status::InvalidArgs, "Invalid argument");
/// let biz_err = BusinessError::from(err);
///
/// // In a function that has access to env:
/// // unsafe { biz_err.throw_into(env) };
/// ```
pub struct BusinessError<S: AsRef<str> = Status>(pub Error<S>);

impl<S: AsRef<str>> From<Error<S>> for BusinessError<S> {
    fn from(err: Error<S>) -> Self {
        BusinessError(err)
    }
}

impl<S: AsRef<str>> BusinessError<S> {
    /// Create a new BusinessError
    pub fn new<R: Into<String>>(status: S, reason: R) -> Self {
        BusinessError(Error::new(status, reason))
    }

    /// Get a reference to the inner error
    pub fn inner(&self) -> &Error<S> {
        &self.0
    }

    /// Throw this error into the ANI environment
    ///
    /// # Safety
    ///
    /// The env pointer must be valid.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let err = Error::new(Status::InvalidArgs, "Bad input");
    /// let biz_err = BusinessError::from(err);
    /// unsafe { biz_err.throw_into(env) };
    /// ```
    pub unsafe fn throw_into(self, env: *mut sys::ani_env) {
        if env.is_null() {
            return;
        }
        let env_ref = unsafe { Env::from_raw_unchecked(env) };
        let fallback_message = format!("[{}] {}", self.0.status.as_ref(), self.0.reason);
        let has_error =
            crate::ani_call_ret_result!(env_ref, ExistUnhandledError, sys::ani_boolean, 0)
                .map(|r| r != 0)
                .unwrap_or(false);
        if has_error {
            return;
        }
        if let Some(error) = unsafe { self.create_error_object(env) } {
            let _ = crate::ani_call!(env_ref, ThrowError, error);
            return;
        }
        if let Ok(error_string) = env_ref.create_string(&fallback_message) {
            let _ = crate::ani_call!(
                env_ref,
                ThrowError,
                error_string.into_raw() as sys::ani_error
            );
        }
    }

    /// Create an ANI BusinessError object
    ///
    /// # Safety
    ///
    /// The env pointer must be valid.
    unsafe fn create_error_object(self, env: *mut sys::ani_env) -> Option<sys::ani_error> {
        let env_ref = unsafe { Env::from_raw_unchecked(env) };
        let message = format!("[{}] {}", self.0.status.as_ref(), self.0.reason);
        let code = match self.0.status.as_ref() {
            "InvalidType" => 401,
            "OutOfRange" => 10200001,
            _ => 1,
        };

        // Current OpenHarmony runtimes expose the ECMAScript-compatible
        // throwable as `std.core.Error`. Its constructor takes the message and
        // an optional ErrorOptions value; `undefined` is the canonical value
        // when no options are supplied.
        if let Ok(err_cls) = env_ref.find_class("std.core.Error")
            && let Ok(err_ctor) =
                env_ref.find_constructor(&err_cls, "C{std.core.String}C{std.core.ErrorOptions}:")
            && let Ok(text) = env_ref.create_string(&message)
            && let Ok(undefined) = env_ref.get_undefined_object()
        {
            let args = [
                crate::types::ani_value_ref(text.as_raw() as sys::ani_ref),
                crate::types::ani_value_ref(undefined as sys::ani_ref),
            ];
            if let Ok(err_obj) = env_ref.new_object(&err_cls, &err_ctor, &args) {
                let _ = env_ref.set_property_by_name_int(&err_obj, "code", code);
                return Some(err_obj.into_raw() as sys::ani_error);
            }
        }

        // Keep compatibility with older runtimes that exposed only the
        // no-argument BusinessError/escompat.Error constructors.
        for (class_name, error_name) in [
            ("@ohos.base.BusinessError", "BusinessError"),
            ("escompat.Error", "Error"),
        ] {
            let err_cls = match env_ref.find_class(class_name) {
                Ok(cls) => cls,
                Err(_) => continue,
            };
            let err_ctor = match env_ref.find_constructor(&err_cls, ":") {
                Ok(ctor) => ctor,
                Err(_) => continue,
            };
            let err_obj = match env_ref.new_object(&err_cls, &err_ctor, &[]) {
                Ok(obj) => obj,
                Err(_) => continue,
            };

            let name = match env_ref.create_string(error_name) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let text = match env_ref.create_string(&message) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let name_ref =
                unsafe { crate::types::AniRef::from_raw(name.into_raw() as sys::ani_ref) };
            let text_ref =
                unsafe { crate::types::AniRef::from_raw(text.into_raw() as sys::ani_ref) };

            let _ = env_ref.set_property_by_name_ref(&err_obj, "name", &name_ref);
            let _ = env_ref.set_property_by_name_ref(&err_obj, "message", &text_ref);
            let _ = env_ref.set_property_by_name_int(&err_obj, "code", code);
            return Some(err_obj.into_raw() as sys::ani_error);
        }

        None
    }
}

impl BusinessError<Status> {
    /// Create a BusinessError from just a reason message
    pub fn from_reason<R: Into<String>>(reason: R) -> Self {
        BusinessError(Error::from_reason(reason))
    }
}

// Type aliases for backward compatibility
/// Alias for BusinessError (deprecated, use BusinessError instead)
#[deprecated(since = "0.1.0", note = "Use BusinessError instead")]
pub type JsError<S = Status> = BusinessError<S>;
/// Alias for BusinessError (deprecated, use BusinessError instead)
#[deprecated(since = "0.1.0", note = "Use BusinessError instead")]
pub type JsTypeError<S = Status> = BusinessError<S>;
/// Alias for BusinessError (deprecated, use BusinessError instead)
#[deprecated(since = "0.1.0", note = "Use BusinessError instead")]
pub type JsRangeError<S = Status> = BusinessError<S>;

// ============================================================================
// Macros
// ============================================================================

/// Check ANI status and return error if not OK
///
/// # Examples
///
/// ```rust,ignore
/// ani_check!(api.SomeCall(env, arg));
/// ```
#[macro_export]
macro_rules! ani_check {
    ($expr:expr) => {{
        let status = $expr;
        $crate::error::check_status(status)?;
    }};
    ($expr:expr, $msg:expr) => {{
        let status = $expr;
        let s = $crate::error::Status::from(status);
        if s != $crate::error::Status::Ok {
            return Err($crate::error::Error::new(s, $msg));
        }
    }};
}

/// Create an error with formatted message
///
/// # Examples
///
/// ```rust
/// use ani::ani_error;
/// use ani::error::Status;
///
/// let err = ani_error!(Status::NotFound, "Resource {} not found", "user");
/// ```
#[macro_export]
macro_rules! ani_error {
    ($status:expr, $($msg:tt)*) => {
        $crate::error::Error::new($status, format!($($msg)*))
    };
}

/// Bail out with an error
///
/// # Examples
///
/// ```rust,ignore
/// use ani::ani_bail;
/// use ani::error::Status;
///
/// fn may_fail() -> ani::error::Result<()> {
///     ani_bail!(Status::NotFound, "Not found");
/// }
/// ```
#[macro_export]
macro_rules! ani_bail {
    ($status:expr, $($msg:tt)*) => {
        return Err($crate::ani_error!($status, $($msg)*))
    };
}
