//! ANI Error Handling
//!
//! Provides unified error types and result types

use std::fmt;

/// ANI Status Code
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
    AlreadyBinded = 8,
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
}

impl From<crate::sys::ani_status> for Status {
    fn from(status: crate::sys::ani_status) -> Self {
        match status {
            0 => Status::Ok,
            1 => Status::Error,
            2 => Status::InvalidArgs,
            3 => Status::InvalidType,
            4 => Status::InvalidDescriptor,
            5 => Status::IncorrectRef,
            6 => Status::PendingError,
            7 => Status::NotFound,
            8 => Status::AlreadyBinded,
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

impl From<Status> for crate::sys::ani_status {
    fn from(status: Status) -> Self {
        status as crate::sys::ani_status
    }
}

/// ANI Error Type
#[derive(Debug)]
pub enum Error {
    /// ANI status error
    Status(Status),
    /// Null pointer error
    NullPointer(&'static str),
    /// Type conversion error
    TypeConversion(String),
    /// ANI exception was thrown
    Exception(String),
    /// UTF-8 encoding error
    Utf8Error(std::str::Utf8Error),
    /// Custom error
    Custom(String),
    /// Class not found
    ClassNotFound(String),
    /// Method not found
    MethodNotFound(String),
    /// Field not found
    FieldNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Status(s) => write!(f, "ANI status error: {:?}", s),
            Error::NullPointer(name) => write!(f, "Null pointer: {}", name),
            Error::TypeConversion(msg) => write!(f, "Type conversion error: {}", msg),
            Error::Exception(msg) => write!(f, "ANI exception: {}", msg),
            Error::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::ClassNotFound(name) => write!(f, "Class not found: {}", name),
            Error::MethodNotFound(name) => write!(f, "Method not found: {}", name),
            Error::FieldNotFound(name) => write!(f, "Field not found: {}", name),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}

impl From<Status> for Error {
    fn from(s: Status) -> Self {
        Error::Status(s)
    }
}

impl From<crate::sys::ani_status> for Error {
    fn from(s: crate::sys::ani_status) -> Self {
        Error::Status(Status::from(s))
    }
}

/// ANI result type
pub type Result<T> = std::result::Result<T, Error>;

/// Convert ANI status code to Result
#[inline]
pub fn check_status(status: crate::sys::ani_status) -> Result<()> {
    let s = Status::from(status);
    if s == Status::Ok {
        Ok(())
    } else {
        Err(Error::Status(s))
    }
}

/// Check if pointer is null
#[inline]
pub fn check_ptr<T>(ptr: *mut T, name: &'static str) -> Result<*mut T> {
    if ptr.is_null() {
        Err(Error::NullPointer(name))
    } else {
        Ok(ptr)
    }
}
