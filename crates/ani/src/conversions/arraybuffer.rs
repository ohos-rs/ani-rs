//! ArrayBuffer Type Conversion
//!
//! Align with napi-rs: **keep the ANI object alive** when we hold an ArrayBuffer from ANI, so it is
//! not freed (use GlobalReference like napi-rs `napi_create_reference`). Prefer Rust borrowing
//! where possible to avoid ref APIs in the hot path.
//!
//! - **[`ArrayBufferSlice<'env>`]** — zero-copy borrowed view. Argument type when you only read in
//!   the current scope; lifetime `'env` = caller keeps object alive. Only `ArrayBuffer_GetInfo`, no ref.
//! - **[`ArrayBuffer`]** — owned. From ANI: hold **global ref** + data pointer (zero-copy, object not freed
//!   until we drop). From Rust: `Vec<u8>`. ToAni: CreateArrayBuffer + copy; the returned ANI ArrayBuffer
//!   is kept by the caller (ANI/ArkTS), so it is **not released** until the caller releases it.
//!
//! ## Usage
//!
//! ```ignore
//! pub fn sum_bytes(buf: ArrayBufferSlice<'_>) -> u64 { buf.iter().map(|&b| b as u64).sum() }
//! pub fn take_buffer(buf: ArrayBuffer) -> usize { buf.len() }  // holds ref, buffer not freed
//! let owned = slice.to_owned();
//! ```
//!

use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::{ani_api, ani_api_raw, ani_call_2ret};

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// ArrayBuffer - Owned Buffer Type
// ============================================================================

/// A buffer that owns its data and can be converted to/from ANI ArrayBuffer.
///
/// This type is used when you need to own the data and potentially transfer it
/// across async boundaries. When converting to ANI, data is copied into a new
/// ArrayBuffer managed by the ANI runtime.
///
/// # Memory Model
///
/// When converting from ANI (`FromAni`):
/// - Data is copied from the ANI ArrayBuffer into a Rust-owned Vec<u8>
/// - The Rust side owns the memory and will free it when dropped
///
/// When converting from ANI (`FromAni`): we hold a **global reference** so the ANI object is not freed
/// (like napi-rs Buffer with `napi_create_reference`). When converting to ANI (`ToAni`): we create
/// a new buffer; the caller keeps it so it is not freed.
///
/// Backing: either Rust-owned `Vec<u8>` or ANI-backed (global ref + pointer, zero-copy).
#[derive(Debug)]
pub struct ArrayBuffer {
    backing: ArrayBufferBacking,
}

#[derive(Debug)]
enum ArrayBufferBacking {
    Owned(Vec<u8>),
    /// Hold global ref so ANI object is not freed; data pointer is shared (zero-copy).
    AniRef {
        global_ref: sys::ani_ref,
        env: *mut sys::ani_env,
        ptr: *const u8,
        len: usize,
    },
}

impl ArrayBuffer {
    /// Create a new ArrayBuffer with the specified data.
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            backing: ArrayBufferBacking::Owned(data),
        }
    }

    /// Create with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            backing: ArrayBufferBacking::Owned(Vec::with_capacity(capacity)),
        }
    }

    /// Create with the given size, filled with zeros.
    #[inline]
    pub fn zeroed(size: usize) -> Self {
        Self {
            backing: ArrayBufferBacking::Owned(vec![0u8; size]),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match &self.backing {
            ArrayBufferBacking::Owned(v) => v.len(),
            ArrayBufferBacking::AniRef { len, .. } => *len,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match &self.backing {
            ArrayBufferBacking::Owned(v) => v.as_slice(),
            ArrayBufferBacking::AniRef { ptr, len, .. } => {
                if *len == 0 {
                    &[]
                } else {
                    unsafe { slice::from_raw_parts(*ptr, *len) }
                }
            }
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match &mut self.backing {
            ArrayBufferBacking::Owned(v) => v.as_mut_slice(),
            ArrayBufferBacking::AniRef { .. } => {
                panic!("as_mut_slice not supported for ANI-backed ArrayBuffer")
            }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        match &self.backing {
            ArrayBufferBacking::Owned(v) => v.as_ptr(),
            ArrayBufferBacking::AniRef { ptr, .. } => *ptr,
        }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match &mut self.backing {
            ArrayBufferBacking::Owned(v) => v.as_mut_ptr(),
            ArrayBufferBacking::AniRef { .. } => {
                panic!("as_mut_ptr not supported for ANI-backed ArrayBuffer")
            }
        }
    }

    /// Consume and return bytes. Copies if ANI-backed (then releases the ref).
    #[inline]
    pub fn into_vec(mut self) -> Vec<u8> {
        let backing = std::mem::replace(&mut self.backing, ArrayBufferBacking::Owned(Vec::new()));
        match backing {
            ArrayBufferBacking::Owned(v) => v,
            ArrayBufferBacking::AniRef { ptr, len, .. } => {
                if len == 0 {
                    Vec::new()
                } else {
                    unsafe { slice::from_raw_parts(ptr, len) }.to_vec()
                }
            }
        }
    }

    /// Create from raw parts (copies into owned buffer).
    #[inline]
    pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
        Self::new(unsafe { slice::from_raw_parts(ptr, len) }.to_vec())
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        let backing = std::mem::replace(&mut self.backing, ArrayBufferBacking::Owned(Vec::new()));
        if let ArrayBufferBacking::AniRef {
            global_ref,
            env,
            ..
        } = backing
        {
            if !env.is_null() && !global_ref.is_null() {
                let api = ani_api_raw!(env);
                if let Some(delete_fn) = api.GlobalReference_Delete {
                    let _ = unsafe { delete_fn(env, global_ref) };
                }
            }
        }
    }
}

impl Clone for ArrayBuffer {
    fn clone(&self) -> Self {
        Self::new(self.as_slice().to_vec())
    }
}

impl PartialEq for ArrayBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for ArrayBuffer {}

impl Default for ArrayBuffer {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl From<Vec<u8>> for ArrayBuffer {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}

impl From<&[u8]> for ArrayBuffer {
    fn from(data: &[u8]) -> Self {
        Self::new(data.to_vec())
    }
}

impl<const N: usize> From<[u8; N]> for ArrayBuffer {
    fn from(data: [u8; N]) -> Self {
        Self::new(data.to_vec())
    }
}

impl From<ArrayBuffer> for Vec<u8> {
    fn from(buffer: ArrayBuffer) -> Self {
        buffer.into_vec()
    }
}

impl AsRef<[u8]> for ArrayBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for ArrayBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl Deref for ArrayBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for ArrayBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

// ============================================================================
// TypeInfo Implementation
// ============================================================================

impl TypeInfo for ArrayBuffer {
    fn type_signature() -> &'static str {
        "Lescompat/ArrayBuffer;"
    }

    fn ani_c_type() -> &'static str {
        "ani_arraybuffer"
    }
}

// ============================================================================
// ToAni Implementation - ArrayBuffer -> ani_arraybuffer (copy)
// ============================================================================
//
// Lifetime (align with napi-rs semantics):
// - We create a **new** ANI ArrayBuffer via CreateArrayBuffer (runtime allocates; we copy
//   data into it). The returned handle is passed back to the ANI/ArkTS caller.
// - The **caller** holds the reference to that ArrayBuffer, so the ANI runtime will **not**
//   free it until the caller releases it. No extra reference or finalizer on our side.
// - Our Rust ArrayBuffer is consumed (or copied from for &ArrayBuffer); we do not hold
//   the ANI object, so we never free it. After to_ani, the corresponding ANI ArrayBuffer
//   is owned by the runtime and the caller—it will not be released prematurely.
//
// (napi-rs uses napi_create_external_buffer + finalizer to transfer ownership to V8 so
// the JS buffer is not freed until GC; we have no CreateArrayBufferExternal, so we use
// copy and the returned value is kept by the caller the same way.)

impl<'env> ToAni<'env> for ArrayBuffer {
    type Output = sys::ani_arraybuffer;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let len = self.len();
        let copy = self.as_slice().to_vec();
        drop(self);

        let (data_ptr, arraybuffer): (*mut std::ffi::c_void, sys::ani_arraybuffer) = ani_call_2ret!(
            env,
            CreateArrayBuffer,
            *mut std::ffi::c_void,
            sys::ani_arraybuffer,
            ptr::null_mut(),
            ptr::null_mut(),
            len
        )?;

        if !copy.is_empty() && !data_ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(copy.as_ptr(), data_ptr as *mut u8, copy.len());
            }
        }

        Ok(arraybuffer)
    }
}

impl<'env> ToAni<'env> for &ArrayBuffer {
    type Output = sys::ani_arraybuffer;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let (data_ptr, arraybuffer): (*mut std::ffi::c_void, sys::ani_arraybuffer) = ani_call_2ret!(
            env,
            CreateArrayBuffer,
            *mut std::ffi::c_void,
            sys::ani_arraybuffer,
            ptr::null_mut(),
            ptr::null_mut(),
            self.len()
        )?;

        if !self.is_empty() && !data_ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.as_ptr(), data_ptr as *mut u8, self.len());
            }
        }

        Ok(arraybuffer)
    }
}

// ============================================================================
// FromAni Implementation - ani_arraybuffer -> ArrayBuffer
// ============================================================================

impl<'env> FromAni<'env> for ArrayBuffer {
    type Input = sys::ani_arraybuffer;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Ok(ArrayBuffer::new(Vec::new()));
        }

        let api = ani_api!(env);

        // Like napi-rs Buffer: keep object alive with global ref so it is not freed.
        if let (Some(create_global), Some(delete_fn)) =
            (api.GlobalReference_Create, api.GlobalReference_Delete)
        {
            let mut global_ref: sys::ani_ref = ptr::null_mut();
            let status = unsafe { create_global(env.as_raw(), value as sys::ani_ref, &mut global_ref) };

            if status == sys::ani_status_ANI_OK && !global_ref.is_null() {
                if let Ok((data_ptr, length)) = ani_call_2ret!(
                    env,
                    ArrayBuffer_GetInfo,
                    *mut std::ffi::c_void,
                    usize,
                    ptr::null_mut(),
                    0,
                    value
                ) {
                    let data_ptr: *mut std::ffi::c_void = data_ptr;
                    let ptr = if data_ptr.is_null() || length == 0 {
                        ptr::null()
                    } else {
                        data_ptr as *const u8
                    };
                    return Ok(ArrayBuffer {
                        backing: ArrayBufferBacking::AniRef {
                            global_ref,
                            env: env.as_raw(),
                            ptr,
                            len: length,
                        },
                    });
                }
                let _ = unsafe { delete_fn(env.as_raw(), global_ref) };
            }
        }

        // Fallback: copy (no ref APIs or creation failed).
        let (data_ptr, length): (*mut std::ffi::c_void, usize) = ani_call_2ret!(
            env,
            ArrayBuffer_GetInfo,
            *mut std::ffi::c_void,
            usize,
            ptr::null_mut(),
            0,
            value
        )?;

        if data_ptr.is_null() || length == 0 {
            return Ok(ArrayBuffer::new(Vec::new()));
        }

        let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, length).to_vec() };
        Ok(ArrayBuffer::new(data))
    }
}

// ============================================================================
// ArrayBufferSlice - Zero-copy borrowed view (for sync contexts)
// ============================================================================

/// Zero-copy borrowed view of an ANI ArrayBuffer (Rust lifetime only, no ANI ref APIs).
///
/// Use as argument type when you only read in the current scope; the lifetime `'env` expresses
/// that the caller keeps the object alive. Only calls `ArrayBuffer_GetInfo` — no GlobalReference.
/// Call [`.to_owned()`](ArrayBufferSlice::to_owned) when you need an owned [`ArrayBuffer`] (copies).
///
/// # Example
///
/// ```ignore
/// fn sum_bytes(buf: ArrayBufferSlice<'_>) -> u64 { buf.iter().map(|&b| b as u64).sum() }
/// ```
pub struct ArrayBufferSlice<'env> {
    data: &'env [u8],
    raw: sys::ani_arraybuffer,
}

impl<'env> ArrayBufferSlice<'env> {
    /// Create a new ArrayBufferSlice from an ANI arraybuffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `buffer` is a valid ANI arraybuffer and
    /// that it remains valid for the lifetime `'env`.
    pub unsafe fn from_raw(env: &Env<'env>, buffer: sys::ani_arraybuffer) -> Result<Self> {
        if buffer.is_null() {
            return Ok(Self {
                data: &[],
                raw: buffer,
            });
        }

        let (data_ptr, length): (*mut std::ffi::c_void, usize) = ani_call_2ret!(
            env,
            ArrayBuffer_GetInfo,
            *mut std::ffi::c_void,
            usize,
            ptr::null_mut(),
            0,
            buffer
        )?;

        let data = if data_ptr.is_null() || length == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(data_ptr as *const u8, length) }
        };

        Ok(Self { data, raw: buffer })
    }

    /// Get the length of the buffer in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the raw ANI arraybuffer handle.
    #[inline]
    pub fn as_raw(&self) -> sys::ani_arraybuffer {
        self.raw
    }

    /// Copy the data into an owned ArrayBuffer.
    ///
    /// Use this when you need to keep the data beyond the current scope.
    #[inline]
    pub fn to_owned(&self) -> ArrayBuffer {
        ArrayBuffer::new(self.data.to_vec())
    }
}

impl<'env> AsRef<[u8]> for ArrayBufferSlice<'env> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'env> Deref for ArrayBufferSlice<'env> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'env> FromAni<'env> for ArrayBufferSlice<'env> {
    type Input = sys::ani_arraybuffer;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe { Self::from_raw(env, value) }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arraybuffer_creation() {
        let buffer = ArrayBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[4], 5);
    }

    #[test]
    fn test_arraybuffer_from_slice() {
        let data: &[u8] = &[1, 2, 3];
        let buffer = ArrayBuffer::from(data);
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_arraybuffer_from_array() {
        let buffer = ArrayBuffer::from([1u8, 2, 3, 4]);
        assert_eq!(buffer.len(), 4);
    }

    #[test]
    fn test_arraybuffer_zeroed() {
        let buffer = ArrayBuffer::zeroed(10);
        assert_eq!(buffer.len(), 10);
        assert!(buffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_arraybuffer_into_vec() {
        let buffer = ArrayBuffer::new(vec![1, 2, 3]);
        let data: Vec<u8> = buffer.into_vec();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn test_arraybuffer_deref() {
        let buffer = ArrayBuffer::new(vec![1, 2, 3, 4, 5]);
        let sum: u8 = buffer.iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_arraybuffer_deref_mut() {
        let mut buffer = ArrayBuffer::new(vec![1, 2, 3]);
        buffer[0] = 10;
        assert_eq!(buffer[0], 10);
    }

    #[test]
    fn test_arraybuffer_type_signature() {
        assert_eq!(ArrayBuffer::type_signature(), "Lescompat/ArrayBuffer;");
        assert_eq!(ArrayBuffer::ani_c_type(), "ani_arraybuffer");
    }

    #[test]
    fn test_arraybuffer_empty() {
        let buffer = ArrayBuffer::default();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_arraybuffer_clone() {
        let buffer1 = ArrayBuffer::new(vec![1, 2, 3]);
        let buffer2 = buffer1.clone();
        assert_eq!(buffer1, buffer2);
    }
}
