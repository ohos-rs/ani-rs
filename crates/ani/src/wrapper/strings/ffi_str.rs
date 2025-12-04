use std::{
    borrow::{Borrow, Cow},
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// An owned null-terminated string encoded in UTF-8.
///
/// This type is intended for constructing ANI strings from Rust code.
pub struct ANIString {
    internal: CString,
}

/// A borrowed null-terminated string encoded in UTF-8.
///
/// Instances of `ANIStr` cannot be created directly, but can be borrowed
/// from [`ANIString`].
#[repr(transparent)]
pub struct ANIStr {
    internal: CStr,
}

impl ::std::ops::Deref for ANIString {
    type Target = ANIStr;

    fn deref(&self) -> &Self::Target {
        unsafe { ANIStr::from_ptr(self.internal.as_ptr()) }
    }
}

impl<T> From<T> for ANIString
where
    T: AsRef<str>,
{
    fn from(other: T) -> Self {
        let bytes = other.as_ref().as_bytes().to_vec();
        ANIString {
            internal: unsafe { CString::from_vec_unchecked(bytes) },
        }
    }
}

impl From<ANIString> for CString {
    fn from(string: ANIString) -> Self {
        string.into_cstring()
    }
}

impl<'str_ref> From<&'str_ref ANIStr> for Cow<'str_ref, str> {
    fn from(other: &'str_ref ANIStr) -> Cow<'str_ref, str> {
        let bytes = other.as_cstr().to_bytes();
        String::from_utf8_lossy(bytes)
    }
}

impl<'str_ref> From<&'str_ref ANIStr> for &'str_ref CStr {
    fn from(value: &'str_ref ANIStr) -> Self {
        &value.internal
    }
}

impl From<ANIString> for String {
    fn from(other: ANIString) -> String {
        other.to_str().into_owned()
    }
}

impl ANIString {
    /// Converts a Rust string into an ANI-compatible string.
    pub fn new(string: impl AsRef<str>) -> Self {
        string.into()
    }

    /// Converts a `CString` into an `ANIString`.
    ///
    /// # Safety
    ///
    /// The `string` must be valid UTF-8.
    pub unsafe fn from_cstring(string: CString) -> Self {
        Self { internal: string }
    }

    /// Converts an `ANIString` into a `CString`.
    pub fn into_cstring(self) -> CString {
        self.internal
    }

    /// Borrows this `ANIString` as a `&ANIStr`.
    pub fn borrowed(&self) -> &ANIStr {
        self
    }
}

impl ANIStr {
    /// Constructs a reference to an `ANIStr` from a pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid, non-null pointer to a null-terminated
    /// UTF-8 string, and must not be mutated or become invalid during the
    /// lifetime `'a`.
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a ANIStr {
        &*(CStr::from_ptr(ptr) as *const CStr as *const ANIStr)
    }

    /// Returns a pointer to the string.
    pub fn as_ptr(&self) -> *const c_char {
        self.as_cstr().as_ptr()
    }

    /// Returns a `CStr` view of the string.
    pub fn as_cstr(&self) -> &CStr {
        &self.internal
    }

    /// Converts this string to a Rust string.
    pub fn to_str(&'_ self) -> Cow<'_, str> {
        self.into()
    }
}

impl Borrow<ANIStr> for ANIString {
    fn borrow(&self) -> &ANIStr {
        self
    }
}

impl std::borrow::ToOwned for ANIStr {
    type Owned = ANIString;

    fn to_owned(&self) -> ANIString {
        ANIString {
            internal: CString::from(self.as_cstr()),
        }
    }
}

impl AsRef<ANIStr> for ANIStr {
    fn as_ref(&self) -> &ANIStr {
        self
    }
}

impl AsRef<ANIStr> for ANIString {
    fn as_ref(&self) -> &ANIStr {
        self
    }
}

