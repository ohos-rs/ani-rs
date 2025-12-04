#![allow(missing_docs)]

use std::char::{CharTryFromError, DecodeUtf16Error};

use thiserror::Error;

use crate::sys;
use crate::wrapper::signature::TypeSignature;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("ANI VM singleton uninitialized")]
    UninitializedAniVM,
    #[error("Invalid AniValue type cast: {0}. Actual type: {1}")]
    WrongAniValueType(&'static str, &'static str),
    #[error("Invalid constructor return type (must be void)")]
    InvalidCtorReturn,
    #[error("Invalid number or type of arguments passed to method: {0}")]
    InvalidArgList(TypeSignature),
    #[error("Object behind weak reference freed")]
    ObjectFreed,
    #[error("Method not found: {name} {sig}")]
    MethodNotFound { name: String, sig: String },
    #[error("Field not found: {name} {sig}")]
    FieldNotFound { name: String, sig: String },
    #[error("ANI exception was thrown")]
    AniException,
    #[error("ANIEnv null method pointer for {0}")]
    ANIEnvMethodNotFound(&'static str),
    #[error("Null pointer in {0}")]
    NullPtr(&'static str),
    #[error("Null pointer deref in {0}")]
    NullDeref(&'static str),
    #[error("Mutex already locked")]
    TryLock,
    #[error("ANI VM null method pointer for {0}")]
    AniVMMethodNotFound(&'static str),
    #[error("Field already set: {0}")]
    FieldAlreadySet(String),
    #[error("Throw failed with status {0}")]
    ThrowFailed(u32),
    #[error("Parse failed for input: {0}")]
    ParseFailed(String),
    #[error("ANI call failed")]
    AniCall(#[source] AniError),

    #[error("A char has the value 0x{char:x}; it is part of a UTF-16 surrogate pair and cannot be converted to a Rust `char` by itself", char = source.unpaired_surrogate())]
    InvalidUtf16 {
        #[source]
        source: DecodeUtf16Error,
    },

    #[error("An int has the value 0x{char:x}, which is not a valid UTF-32 unit; cannot convert it to a Rust `char`")]
    InvalidUtf32 {
        char: sys::ani_int,
        #[source]
        source: CharTryFromError,
    },

    #[error("This ANI virtual machine version is not supported")]
    UnsupportedVersion,

    // Keep JNI compatibility aliases
    #[error("JavaException (JNI compat)")]
    JavaException,
    #[error("JNIEnv method not found: {0}")]
    JNIEnvMethodNotFound(&'static str),
    #[error("JNI call failed")]
    JniCall(#[source] AniError),
    #[error("Invalid JValue type cast: {0}. Actual type: {1}")]
    WrongJValueType(&'static str, &'static str),
    #[error("JavaVM method not found: {0}")]
    JavaVMMethodNotFound(&'static str),
}

#[derive(Debug, Error)]
pub enum AniError {
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid arguments")]
    InvalidArgs,
    #[error("Invalid type")]
    InvalidType,
    #[error("Invalid descriptor")]
    InvalidDescriptor,
    #[error("Incorrect reference")]
    IncorrectRef,
    #[error("Pending error")]
    PendingError,
    #[error("Not found")]
    NotFound,
    #[error("Already bound")]
    AlreadyBound,
    #[error("Out of references")]
    OutOfRef,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Out of range")]
    OutOfRange,
    #[error("Buffer too small")]
    BufferTooSmall,
    #[error("Invalid version")]
    InvalidVersion,
    #[error("Ambiguous")]
    Ambiguous,
    #[error("Error status {0}")]
    Other(sys::ani_status),
}

// Alias for JNI compatibility
pub type JniError = AniError;

impl<T> From<::std::sync::TryLockError<T>> for Error {
    fn from(_: ::std::sync::TryLockError<T>) -> Self {
        Error::TryLock
    }
}

pub fn ani_status_to_error(status: sys::ani_status) -> AniError {
    match status {
        sys::ani_status_ANI_ERROR => AniError::Unknown,
        sys::ani_status_ANI_INVALID_ARGS => AniError::InvalidArgs,
        sys::ani_status_ANI_INVALID_TYPE => AniError::InvalidType,
        sys::ani_status_ANI_INVALID_DESCRIPTOR => AniError::InvalidDescriptor,
        sys::ani_status_ANI_INCORRECT_REF => AniError::IncorrectRef,
        sys::ani_status_ANI_PENDING_ERROR => AniError::PendingError,
        sys::ani_status_ANI_NOT_FOUND => AniError::NotFound,
        sys::ani_status_ANI_ALREADY_BINDED => AniError::AlreadyBound,
        sys::ani_status_ANI_OUT_OF_REF => AniError::OutOfRef,
        sys::ani_status_ANI_OUT_OF_MEMORY => AniError::OutOfMemory,
        sys::ani_status_ANI_OUT_OF_RANGE => AniError::OutOfRange,
        sys::ani_status_ANI_BUFFER_TO_SMALL => AniError::BufferTooSmall,
        sys::ani_status_ANI_INVALID_VERSION => AniError::InvalidVersion,
        sys::ani_status_ANI_AMBIGUOUS => AniError::Ambiguous,
        _ => AniError::Other(status),
    }
}

pub fn ani_status_to_result(status: sys::ani_status) -> Result<()> {
    if status == sys::ani_status_ANI_OK {
        Ok(())
    } else {
        Err(Error::AniCall(ani_status_to_error(status)))
    }
}

// JNI compatibility function
pub fn jni_error_code_to_result(code: sys::jint) -> Result<()> {
    match code {
        sys::JNI_OK => Ok(()),
        sys::JNI_ERR => Err(AniError::Unknown),
        sys::JNI_EDETACHED => Err(AniError::Unknown),
        sys::JNI_EVERSION => Err(AniError::InvalidVersion),
        sys::JNI_ENOMEM => Err(AniError::OutOfMemory),
        sys::JNI_EEXIST => Err(AniError::AlreadyBound),
        sys::JNI_EINVAL => Err(AniError::InvalidArgs),
        _ => Err(AniError::Other(code as sys::ani_status)),
    }
    .map_err(Error::AniCall)
}

pub struct Exception {
    pub class: String,
    pub msg: String,
}

pub trait ToException {
    fn to_exception(&self) -> Exception;
}

/// Error that occurred while starting the ANI VM.
#[cfg(feature = "invocation")]
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StartVmError {
    #[error("Couldn't load the ANI VM shared library ({0}): {1}")]
    LoadError(String, #[source] libloading::Error),

    #[error("{0}")]
    Create(
        #[from]
        #[source]
        Error,
    ),
}

#[cfg(feature = "invocation")]
pub type StartVmResult<T> = std::result::Result<T, StartVmError>;

// JNI compatibility alias
#[cfg(feature = "invocation")]
pub type StartJvmError = StartVmError;
#[cfg(feature = "invocation")]
pub type StartJvmResult<T> = StartVmResult<T>;

#[derive(Debug, Error)]
#[error("The code point U+{char_as_u32:X} {char:?} cannot be converted to a char, because it is not representable as a single UTF-16 unit.", char_as_u32 = u32::from(*char))]
pub struct CharToJavaError {
    pub char: char,
}
