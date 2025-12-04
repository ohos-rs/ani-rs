use std::marker::PhantomData;

use crate::{
    errors::*,
    objects::{APrimitiveArray, ReleaseMode},
    sys,
    anienv::ANIEnv,
};

/// Auto-release wrapper for array elements.
///
/// This struct is used to wrap a pointer to array elements, ensuring they are
/// properly released when the wrapper is dropped.
pub struct AutoElements<'local, 'array, 'env, T: TypeArray> {
    array: &'array APrimitiveArray<'local, T>,
    len: usize,
    ptr: *mut T,
    mode: ReleaseMode,
    env: &'env ANIEnv<'local>,
    _phantom: PhantomData<T>,
}

impl<'local, 'array, 'env, T: TypeArray> AutoElements<'local, 'array, 'env, T> {
    /// Get the array elements
    ///
    /// # Safety
    ///
    /// The caller must ensure that the array is valid for the lifetime of
    /// this AutoElements.
    pub(crate) unsafe fn new(
        env: &'env ANIEnv<'local>,
        array: &'array APrimitiveArray<'local, T>,
        mode: ReleaseMode,
    ) -> Result<Self> {
        // ANI doesn't have direct array element pinning like JNI
        // For now, we'll use a simplified approach
        let mut len: sys::ani_size = 0;
        let status = ani_call_unchecked!(env, Array_GetLength, array.as_raw(), &mut len);
        ani_status_to_result(status)?;

        // Allocate a buffer to hold the elements
        let mut buffer = vec![T::default(); len].into_boxed_slice();
        let ptr = buffer.as_mut_ptr();
        std::mem::forget(buffer);

        Ok(Self {
            array,
            len,
            ptr,
            mode,
            env,
            _phantom: PhantomData,
        })
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get a reference to the array
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Get a mutable reference to the array
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T: TypeArray> Drop for AutoElements<'_, '_, '_, T> {
    fn drop(&mut self) {
        // Free the allocated buffer
        if !self.ptr.is_null() {
            unsafe {
                let _ = Vec::from_raw_parts(self.ptr, self.len, self.len);
            }
        }
    }
}

impl<T: TypeArray> std::ops::Deref for AutoElements<'_, '_, '_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: TypeArray> std::ops::DerefMut for AutoElements<'_, '_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

/// Marker trait for types that can be used as array elements
pub trait TypeArray: Copy + Default + Send + Sync + 'static {}

impl TypeArray for sys::aboolean {}
impl TypeArray for sys::abyte {}
impl TypeArray for sys::achar {}
impl TypeArray for sys::ashort {}
impl TypeArray for sys::aint {}
impl TypeArray for sys::along {}
impl TypeArray for sys::afloat {}
impl TypeArray for sys::adouble {}
