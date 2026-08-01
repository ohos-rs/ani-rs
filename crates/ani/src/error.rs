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
//! You can define custom error types that implement [`AniErrorPayload`]:
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! #[derive(Debug)]
//! pub struct MyError(String);
//!
//! impl std::fmt::Display for MyError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         f.write_str(&self.0)
//!     }
//! }
//!
//! impl AniErrorPayload for MyError {
//!     fn ani_status(&self) -> &str { "MyError" }
//!     fn ani_code(&self) -> i32 { 70001 }
//!     fn ani_message(&self) -> &str { &self.0 }
//! }
//!
//! fn custom_error() -> std::result::Result<(), MyError> {
//!     Err(MyError("The input is invalid".into()))
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

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use crate::conversions::{ArrayBuffer, Boxable, FromAni, RefContainer, ToAni, Unboxable};
use crate::env::Env;
use crate::sys;
use crate::types::{AniError, AniObject, AniRef, AniString};

/// Recursive, runtime-independent value used for custom error metadata.
///
/// Unlike a string-only error map this preserves the application schema across
/// Promise, task, threadsafe-function, and stream boundaries.
#[derive(Debug, Clone, PartialEq)]
pub enum AniErrorValue {
    /// ArkTS `null`.
    Null,
    /// Boolean property.
    Bool(bool),
    /// Signed integral property.
    Integer(i64),
    /// Floating-point property.
    Number(f64),
    /// UTF-8 string property.
    String(String),
    /// Ordered structured object/Record.
    Object(BTreeMap<String, AniErrorValue>),
    /// Structured array.
    Array(Vec<AniErrorValue>),
    /// Binary ArrayBuffer payload.
    Binary(Vec<u8>),
}

impl From<bool> for AniErrorValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for AniErrorValue {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for AniErrorValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for AniErrorValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for AniErrorValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for AniErrorValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl AniErrorValue {
    /// Materialize this owned value in the current ANI scope.
    pub fn to_ani_ref<'env>(&self, env: &Env<'env>) -> Result<AniRef<'env>> {
        fn object_ref(object: AniObject<'_>) -> AniRef<'_> {
            unsafe { AniRef::from_raw(object.into_raw() as sys::ani_ref) }
        }

        match self {
            Self::Null => Ok(unsafe { AniRef::from_raw(env.get_null_object()? as sys::ani_ref) }),
            Self::Bool(value) => (*value).box_value(env).map(object_ref),
            Self::Integer(value) => (*value).box_value(env).map(object_ref),
            Self::Number(value) => (*value).box_value(env).map(object_ref),
            Self::String(value) => env
                .create_string(value)
                .map(|value| unsafe { AniRef::from_raw(value.into_raw() as sys::ani_ref) }),
            Self::Array(values) => values
                .iter()
                .map(|value| value.to_ani_ref(env))
                .collect::<Result<Vec<_>>>()?
                .to_ani(env)
                .map(|value| unsafe { AniRef::from_raw(value as sys::ani_ref) }),
            Self::Object(values) => values
                .iter()
                .map(|(key, value)| value.to_ani_ref(env).map(|value| (key.clone(), value)))
                .collect::<Result<HashMap<_, _>>>()?
                .to_ani(env)
                .map(object_ref),
            Self::Binary(value) => ArrayBuffer::from(value.as_slice())
                .to_ani(env)
                .map(|value| unsafe { AniRef::from_raw(value as sys::ani_ref) }),
        }
    }

    /// Decode a structured ArkTS value without relying on JSON serialization.
    pub fn from_ani_ref(env: &Env<'_>, value: &AniRef<'_>) -> Result<Self> {
        if env.is_nullish(value)? {
            return Ok(Self::Null);
        }
        let object = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
        let instance_of = |descriptor: &str| {
            env.find_class(descriptor)
                .and_then(|class| env.object_instance_of(&object, &class))
                .unwrap_or(false)
        };
        if instance_of("std.core.String") {
            let string = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
            return env.get_string(&string).map(Self::String);
        }
        if instance_of("std.core.Boolean") {
            return bool::unbox(env, &object).map(Self::Bool);
        }
        if instance_of("std.core.Int") {
            return i32::unbox(env, &object).map(|value| Self::Integer(value as i64));
        }
        if instance_of("std.core.Long") {
            return i64::unbox(env, &object).map(Self::Integer);
        }
        if instance_of("std.core.Float") {
            return f32::unbox(env, &object).map(|value| Self::Number(value as f64));
        }
        if instance_of("std.core.Double") {
            return f64::unbox(env, &object).map(Self::Number);
        }
        if instance_of("std.core.Array") {
            let values =
                unsafe { Vec::<AniRef<'_>>::from_ani(env, value.as_raw() as sys::ani_array) }?;
            return values
                .iter()
                .map(|value| Self::from_ani_ref(env, value))
                .collect::<Result<Vec<_>>>()
                .map(Self::Array);
        }
        if instance_of("std.core.Record") {
            let values = unsafe { HashMap::<String, AniRef<'_>>::from_ani(env, object.as_raw()) }?;
            return values
                .iter()
                .map(|(key, value)| {
                    Self::from_ani_ref(env, value).map(|value| (key.clone(), value))
                })
                .collect::<Result<BTreeMap<_, _>>>()
                .map(Self::Object);
        }
        if instance_of("escompat.ArrayBuffer") || instance_of("std.core.ArrayBuffer") {
            let buffer =
                unsafe { ArrayBuffer::from_ani(env, value.as_raw() as sys::ani_arraybuffer) }?;
            return Ok(Self::Binary(buffer.as_ref().to_vec()));
        }
        Err(Error::new(
            Status::InvalidType,
            "error property must be null, bool, number, string, Record, Array, or ArrayBuffer",
        ))
    }
}

// ============================================================================
// Status Code
// ============================================================================

/// ANI Status Code
///
/// Standard status codes returned by ANI API calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[derive(Default)]
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
    #[default]
    GenericFailure = 100,
    /// A bounded callback/task queue has reached capacity.
    QueueFull = 101,
    /// A callback/task handle is closing or already closed.
    Closing = 102,
    /// An asynchronous operation was cancelled.
    Cancelled = 103,
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
            Status::QueueFull => "QueueFull",
            Status::Closing => "Closing",
            Status::Cancelled => "Cancelled",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
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
            Status::QueueFull | Status::Closing | Status::Cancelled => 1,
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
#[non_exhaustive]
pub struct Error<S: AsRef<str> = Status> {
    /// The status/error code
    pub status: S,
    /// Human-readable error message
    pub reason: String,
    /// Optional dynamic status name received across a type-erased boundary.
    pub status_name: Option<String>,
    /// Optional application-defined numeric BusinessError code.
    pub code: Option<i32>,
    /// Additional string metadata copied to the ArkTS error object.
    pub metadata: BTreeMap<String, String>,
    /// Extensible payload is kept out of line so the error remains cheap to
    /// return through the crate-wide `Result` ABI.
    extension: Box<ErrorExtension>,
    /// Optional cause of this error
    pub cause: Option<Box<Error<Status>>>,
}

#[derive(Debug, Default)]
struct ErrorExtension {
    properties: BTreeMap<String, AniErrorValue>,
    stack: Option<String>,
    raw_rejection: Option<RefContainer>,
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
            status_name: None,
            code: None,
            metadata: BTreeMap::new(),
            extension: Box::default(),
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
            status_name: None,
            code: None,
            metadata: BTreeMap::new(),
            extension: Box::default(),
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
            status_name: None,
            code: None,
            metadata: BTreeMap::new(),
            extension: Box::default(),
            cause: Some(Box::new(cause)),
        }
    }

    /// Override the stable numeric BusinessError code.
    pub fn with_code(mut self, code: i32) -> Self {
        self.code = Some(code);
        self
    }

    /// Preserve a dynamic status identifier received from ArkTS.
    pub fn with_status_name(mut self, status: impl Into<String>) -> Self {
        self.status_name = Some(status.into());
        self
    }

    /// Attach application metadata to the ArkTS error object.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Attach typed application metadata to the ArkTS error object.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<AniErrorValue>,
    ) -> Self {
        self.extension.properties.insert(key.into(), value.into());
        self
    }

    /// Preserve or override an ArkTS-compatible stack trace.
    pub fn with_stack(mut self, stack: impl Into<String>) -> Self {
        self.extension.stack = Some(stack.into());
        self
    }

    /// Typed application properties retained across asynchronous boundaries.
    pub fn properties(&self) -> &BTreeMap<String, AniErrorValue> {
        &self.extension.properties
    }

    /// Preserved ArkTS stack trace, if one was available.
    pub fn stack(&self) -> Option<&str> {
        self.extension.stack.as_deref()
    }

    pub(crate) fn insert_property(&mut self, key: String, value: AniErrorValue) {
        self.extension.properties.insert(key, value);
    }

    pub(crate) fn set_stack(&mut self, stack: Option<String>) {
        self.extension.stack = stack;
    }

    pub(crate) fn preserve_rejection(&mut self, rejection: Option<RefContainer>) {
        self.extension.raw_rejection = rejection;
    }

    /// Get the status code as a string reference
    pub fn status_str(&self) -> &str {
        self.status_name
            .as_deref()
            .unwrap_or_else(|| self.status.as_ref())
    }
}

/// Extensible error contract shared by sync exports, async functions, tasks,
/// Promise rejection, and streams.
///
/// Custom application errors can implement this trait directly; ani-rs never
/// requires conversion to its built-in [`Status`] enum or to a string.
pub trait AniErrorPayload: fmt::Debug + fmt::Display + Send + Sync + 'static {
    /// Stable status identifier exposed as ArkTS `Error.name` and as the
    /// structured context's `status` field.
    fn ani_status(&self) -> &str {
        "GenericFailure"
    }

    /// Numeric BusinessError code. Applications may define their own range.
    fn ani_code(&self) -> i32 {
        business_error_code(self.ani_status())
    }

    /// Human-readable message.
    fn ani_message(&self) -> &str;

    /// Optional structured cause.
    fn ani_cause(&self) -> Option<&dyn AniErrorPayload> {
        None
    }

    /// Visit extra string properties to attach to the ArkTS error.
    fn visit_ani_metadata(&self, _visitor: &mut dyn FnMut(&str, &str)) {}

    /// Visit typed properties. Custom errors can expose arbitrary nested
    /// structured metadata without first serializing it to strings.
    fn visit_ani_properties(&self, _visitor: &mut dyn FnMut(&str, &AniErrorValue)) {}

    /// Optional preserved stack trace.
    fn ani_stack(&self) -> Option<&str> {
        None
    }

    /// Object-safe materializer hook for application-defined ArkTS Error
    /// subclasses or a previously retained rejection object.
    fn materialize_ani_error<'env>(&self, _env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        Ok(None)
    }
}

impl<S> AniErrorPayload for Error<S>
where
    S: AsRef<str> + fmt::Debug + Send + Sync + 'static,
{
    fn ani_status(&self) -> &str {
        self.status_str()
    }

    fn ani_code(&self) -> i32 {
        self.code
            .unwrap_or_else(|| business_error_code(self.status_str()))
    }

    fn ani_message(&self) -> &str {
        &self.reason
    }

    fn ani_cause(&self) -> Option<&dyn AniErrorPayload> {
        self.cause
            .as_deref()
            .map(|cause| cause as &dyn AniErrorPayload)
    }

    fn visit_ani_metadata(&self, visitor: &mut dyn FnMut(&str, &str)) {
        for (key, value) in &self.metadata {
            visitor(key, value);
        }
    }

    fn visit_ani_properties(&self, visitor: &mut dyn FnMut(&str, &AniErrorValue)) {
        for (key, value) in &self.extension.properties {
            visitor(key, value);
        }
    }

    fn ani_stack(&self) -> Option<&str> {
        self.extension.stack.as_deref()
    }

    fn materialize_ani_error<'env>(&self, env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        self.extension
            .raw_rejection
            .as_ref()
            .map(|rejection| rejection.to_local::<AniError<'env>>(env))
            .transpose()
    }
}

/// Type-erased error retained across an asynchronous scheduler boundary.
pub type DynAniError = Box<dyn AniErrorPayload>;

/// Exact ArkTS rejection retained for cancellation/iterator `throw()` paths.
#[derive(Debug)]
pub struct PreservedArktsError {
    rejection: RefContainer,
    message: String,
}

impl PreservedArktsError {
    /// Promote an ArkTS rejection object to a VM-owned global reference.
    pub fn new(env: &Env<'_>, rejection: &AniRef<'_>) -> Result<Self> {
        let object = unsafe { AniObject::from_raw(rejection.as_raw() as sys::ani_object) };
        let message = env
            .get_property_by_name_ref(&object, "message")
            .and_then(|message| {
                let message = unsafe { AniString::from_raw(message.as_raw() as sys::ani_string) };
                env.get_string(&message)
            })
            .unwrap_or_else(|_| "ArkTS async iterator was thrown".to_string());
        Ok(Self {
            rejection: RefContainer::new(env, rejection)?,
            message,
        })
    }
}

impl fmt::Display for PreservedArktsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl AniErrorPayload for PreservedArktsError {
    fn ani_status(&self) -> &str {
        "ArktsRejection"
    }

    fn ani_message(&self) -> &str {
        &self.message
    }

    fn materialize_ani_error<'env>(&self, env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        self.rejection.to_local(env).map(Some)
    }
}

impl<T> AniErrorPayload for Box<T>
where
    T: AniErrorPayload + ?Sized,
{
    fn ani_status(&self) -> &str {
        (**self).ani_status()
    }
    fn ani_code(&self) -> i32 {
        (**self).ani_code()
    }
    fn ani_message(&self) -> &str {
        (**self).ani_message()
    }
    fn ani_cause(&self) -> Option<&dyn AniErrorPayload> {
        (**self).ani_cause()
    }
    fn visit_ani_metadata(&self, visitor: &mut dyn FnMut(&str, &str)) {
        (**self).visit_ani_metadata(visitor);
    }
    fn visit_ani_properties(&self, visitor: &mut dyn FnMut(&str, &AniErrorValue)) {
        (**self).visit_ani_properties(visitor);
    }
    fn ani_stack(&self) -> Option<&str> {
        (**self).ani_stack()
    }
    fn materialize_ani_error<'env>(&self, env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        (**self).materialize_ani_error(env)
    }
}

impl<T> AniErrorPayload for std::sync::Arc<T>
where
    T: AniErrorPayload + ?Sized,
{
    fn ani_status(&self) -> &str {
        (**self).ani_status()
    }
    fn ani_code(&self) -> i32 {
        (**self).ani_code()
    }
    fn ani_message(&self) -> &str {
        (**self).ani_message()
    }
    fn ani_cause(&self) -> Option<&dyn AniErrorPayload> {
        (**self).ani_cause()
    }
    fn visit_ani_metadata(&self, visitor: &mut dyn FnMut(&str, &str)) {
        (**self).visit_ani_metadata(visitor);
    }
    fn visit_ani_properties(&self, visitor: &mut dyn FnMut(&str, &AniErrorValue)) {
        (**self).visit_ani_properties(visitor);
    }
    fn ani_stack(&self) -> Option<&str> {
        (**self).ani_stack()
    }
    fn materialize_ani_error<'env>(&self, env: &Env<'env>) -> Result<Option<AniError<'env>>> {
        (**self).materialize_ani_error(env)
    }
}

/// Stable numeric BusinessError code for a framework status name.
pub fn business_error_code(status: &str) -> i32 {
    match status {
        "InvalidArgs" | "InvalidType" => 401,
        "NotFound" => 404,
        "OutOfMemory" => 100001,
        "QueueFull" => 100002,
        "Closing" => 100003,
        "Cancelled" => 100004,
        "OutOfRange" => 10200001,
        _ => 1,
    }
}

impl<S: AsRef<str> + fmt::Debug> fmt::Display for Error<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status_str(), self.reason)?;
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
    pub unsafe fn throw_into(self, env: *mut sys::ani_env)
    where
        S: fmt::Debug + Send + Sync + 'static,
    {
        unsafe { throw_error_payload(env, &self.0) };
    }

    /// Create an ANI BusinessError object
    ///
    /// # Safety
    ///
    /// The env pointer must be valid.
    unsafe fn create_error_object(&self, env: *mut sys::ani_env) -> Option<sys::ani_error>
    where
        S: fmt::Debug + Send + Sync + 'static,
    {
        let env_ref = unsafe { Env::from_raw_unchecked(env) };
        create_error_payload(&env_ref, &self.0)
    }

    /// Materialize this error for Promise rejection or explicit propagation.
    pub fn into_ani_error<'env>(&self, env: &Env<'env>) -> Result<AniError<'env>>
    where
        S: fmt::Debug + Send + Sync + 'static,
    {
        let raw = unsafe { self.create_error_object(env.as_raw()) }.ok_or_else(|| {
            Error::new(
                Status::GenericFailure,
                "failed to create ArkTS BusinessError",
            )
        })?;
        Ok(unsafe { AniError::from_raw(raw) })
    }
}

/// Throw any structured error payload into an ANI environment without first
/// converting it to ani-rs's built-in [`Error`] type.
///
/// # Safety
///
/// `env` must point to a valid environment for the current thread.
pub unsafe fn throw_error_payload(env: *mut sys::ani_env, payload: &dyn AniErrorPayload) {
    if env.is_null() {
        return;
    }
    let env_ref = unsafe { Env::from_raw_unchecked(env) };
    let fallback_message = format!("[{}] {}", payload.ani_status(), payload.ani_message());
    let has_error = crate::ani_call_ret_result!(env_ref, ExistUnhandledError, sys::ani_boolean, 0)
        .map(|r| r != 0)
        .unwrap_or(false);
    if has_error {
        return;
    }
    if let Some(error) = create_error_payload(&env_ref, payload) {
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

/// Materialize any custom async/sync error payload without erasing it.
pub fn payload_to_ani_error<'env>(
    env: &Env<'env>,
    payload: &dyn AniErrorPayload,
) -> Result<AniError<'env>> {
    if let Some(error) = payload.materialize_ani_error(env)? {
        return Ok(error);
    }
    let raw = create_error_payload(env, payload).ok_or_else(|| {
        Error::new(
            Status::GenericFailure,
            "failed to create ArkTS BusinessError",
        )
    })?;
    Ok(unsafe { AniError::from_raw(raw) })
}

fn set_error_metadata(
    env: &Env<'_>,
    object: &crate::types::AniObject<'_>,
    status: &str,
    message: &str,
    code: i32,
) {
    if let Ok(name) = env.create_string(status) {
        let name = unsafe { AniRef::from_raw(name.into_raw() as sys::ani_ref) };
        let _ = env.set_property_by_name_ref(object, "name", &name);
    }
    if let Ok(text) = env.create_string(message) {
        let text = unsafe { AniRef::from_raw(text.into_raw() as sys::ani_ref) };
        let _ = env.set_property_by_name_ref(object, "message", &text);
    }
    let _ = env.set_property_by_name_int(object, "code", code);
}

fn attach_error_context(
    env: &Env<'_>,
    object: &crate::types::AniObject<'_>,
    payload: &dyn AniErrorPayload,
) {
    let mut metadata = BTreeMap::<String, AniErrorValue>::new();
    payload.visit_ani_metadata(&mut |key, value| {
        metadata.insert(key.to_string(), AniErrorValue::String(value.to_string()));
    });
    payload.visit_ani_properties(&mut |key, value| {
        metadata.insert(key.to_string(), value.clone());
    });
    let metadata_values = metadata
        .iter()
        .map(|(key, value)| value.to_ani_ref(env).map(|value| (key.clone(), value)))
        .collect::<Result<HashMap<_, _>>>();
    let Ok(metadata_values) = metadata_values else {
        return;
    };
    let Ok(metadata_object) = metadata_values.to_ani(env) else {
        return;
    };

    for (key, value) in metadata {
        if let Ok(value) = value.to_ani_ref(env) {
            let _ = env.set_property_by_name_ref(object, &key, &value);
        }
    }
    let Ok(status) = env.create_string(payload.ani_status()) else {
        return;
    };
    let mut context = HashMap::<String, AniRef<'_>>::new();
    context.insert("status".to_string(), unsafe {
        AniRef::from_raw(status.into_raw() as sys::ani_ref)
    });
    context.insert("metadata".to_string(), unsafe {
        AniRef::from_raw(metadata_object.into_raw() as sys::ani_ref)
    });
    if let Some(cause) = payload.ani_cause()
        && let Some(cause) = create_error_payload(env, cause)
    {
        context.insert("cause".to_string(), unsafe {
            AniRef::from_raw(cause as sys::ani_ref)
        });
    }
    if let Ok(context) = context.to_ani(env) {
        let context = unsafe { AniRef::from_raw(context.into_raw() as sys::ani_ref) };
        let _ = env.set_property_by_name_ref(object, "cause", &context);
    }
    if let Some(stack) = payload.ani_stack()
        && let Ok(stack) = env.create_string(stack)
    {
        let stack = unsafe { AniRef::from_raw(stack.into_raw() as sys::ani_ref) };
        let _ = env.set_property_by_name_ref(object, "stack", &stack);
    }
}

fn create_error_payload(
    env_ref: &Env<'_>,
    payload: &dyn AniErrorPayload,
) -> Option<sys::ani_error> {
    if let Ok(Some(error)) = payload.materialize_ani_error(env_ref) {
        return Some(error.into_raw());
    }
    let status = payload.ani_status();
    let message = payload.ani_message();
    let code = payload.ani_code();

    // Current OpenHarmony runtimes expose the ECMAScript-compatible
    // throwable as `std.core.Error`. Its constructor takes the message and
    // an optional ErrorOptions value; `undefined` is the canonical value
    // when no options are supplied.
    if let Ok(err_cls) = env_ref.find_class("std.core.Error")
        && let Ok(err_ctor) =
            env_ref.find_constructor(&err_cls, "C{std.core.String}C{std.core.ErrorOptions}:")
        && let Ok(text) = env_ref.create_string(message)
        && let Ok(undefined) = env_ref.get_undefined_object()
    {
        let args = [
            crate::types::ani_value_ref(text.as_raw() as sys::ani_ref),
            crate::types::ani_value_ref(undefined as sys::ani_ref),
        ];
        if let Ok(err_obj) = env_ref.new_object(&err_cls, &err_ctor, &args) {
            set_error_metadata(env_ref, &err_obj, status, message, code);
            attach_error_context(env_ref, &err_obj, payload);
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

        let _ = error_name;
        set_error_metadata(env_ref, &err_obj, status, message, code);
        attach_error_context(env_ref, &err_obj, payload);
        return Some(err_obj.into_raw() as sys::ani_error);
    }

    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_status_and_metadata_survive_type_erasure() {
        let error = Error::new(Status::GenericFailure, "remote")
            .with_status_name("RemoteDomainFailure")
            .with_code(74001)
            .with_metadata("operation", "read")
            .with_property("retryable", false)
            .with_property(
                "details",
                AniErrorValue::Object(BTreeMap::from([
                    (
                        "labels".to_string(),
                        AniErrorValue::Array(vec!["remote".into(), "async".into()]),
                    ),
                    ("attempt".to_string(), AniErrorValue::Integer(2)),
                ])),
            )
            .with_property("binary", AniErrorValue::Binary(vec![1, 2, 3]))
            .with_stack("RemoteError: remote\n  at asyncCall");
        let erased: DynAniError = Box::new(error);
        assert_eq!(erased.ani_status(), "RemoteDomainFailure");
        assert_eq!(erased.ani_code(), 74001);
        let mut metadata = BTreeMap::new();
        erased.visit_ani_metadata(&mut |key, value| {
            metadata.insert(key.to_string(), value.to_string());
        });
        assert_eq!(metadata.get("operation").map(String::as_str), Some("read"));
        let mut properties = BTreeMap::new();
        erased.visit_ani_properties(&mut |key, value| {
            properties.insert(key.to_string(), value.clone());
        });
        assert_eq!(
            properties.get("retryable"),
            Some(&AniErrorValue::Bool(false))
        );
        assert_eq!(
            properties.get("binary"),
            Some(&AniErrorValue::Binary(vec![1, 2, 3]))
        );
        assert!(matches!(
            properties.get("details"),
            Some(AniErrorValue::Object(_))
        ));
        assert!(erased.ani_stack().unwrap().contains("asyncCall"));
    }
}
