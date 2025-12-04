use std::{
    marker::PhantomData,
    os::raw::c_char,
    ptr,
};

use crate::{
    errors::*,
    objects::{
        AutoLocal, GlobalRef, JClass, JObject, JString, JThrowable, WeakRef,
    },
    sys::{self, ani_ref, jobject},
    ANIVersion, AniVM,
};

use super::objects::JObjectRef;

/// FFI-compatible ANIEnv struct. This is where most of the
/// interaction with the ANI VM happens. All methods on this object are wrappers
/// around ANI functions.
///
/// # Exception handling
///
/// Since we're calling into the VM, many methods also have the
/// potential to cause an exception to get thrown. If this is the case, an `Err`
/// result will be returned with the error kind `AniException`. Note that this
/// will _not_ clear the exception - it's up to the caller to decide whether to
/// do so or to let it continue being thrown.
///
/// # References and Lifetimes
///
/// Interactions with objects happen through <dfn>references</dfn>, either local
/// or global, represented by [`JObject`] and [`GlobalRef`] respectively.
///
/// <dfn>Global references</dfn> exist until deleted. Deletion occurs when the `GlobalRef` is
/// dropped.
///
/// <dfn>Local references</dfn> belong to a local reference frame, and exist until
/// deleted or until the local reference frame is exited.
#[repr(transparent)]
#[derive(Debug)]
pub struct ANIEnv<'local> {
    internal: *mut sys::ani_env,
    lifetime: PhantomData<&'local ()>,
}

impl ANIEnv<'_> {
    /// Create an ANIEnv from a raw pointer.
    ///
    /// # Safety
    ///
    /// Expects a valid pointer retrieved from the `AttachCurrentThread` or similar function.
    pub unsafe fn from_raw(ptr: *mut sys::ani_env) -> Result<Self> {
        null_check!(ptr, "from_raw ptr argument")?;
        Ok(Self::from_raw_unchecked(ptr))
    }

    /// Create an ANIEnv from a raw pointer without checking if it's null.
    ///
    /// # Safety
    ///
    /// Expects a valid, non-null pointer.
    pub unsafe fn from_raw_unchecked(ptr: *mut sys::ani_env) -> Self {
        ANIEnv {
            internal: ptr,
            lifetime: PhantomData,
        }
    }

    /// Get the raw ANIEnv pointer
    pub fn get_raw(&self) -> *mut sys::ani_env {
        self.internal
    }

    /// Creates a clone of this ANIEnv that has the same lifetime.
    ///
    /// # Safety
    ///
    /// This is unsafe because it creates a second reference to the same environment,
    /// which could lead to use-after-free if not careful.
    pub unsafe fn unsafe_clone(&self) -> Self {
        Self {
            internal: self.internal,
            lifetime: PhantomData,
        }
    }

    /// Get the AniVM interface
    pub fn get_ani_vm(&self) -> AniVM {
        AniVM::from_env(self)
    }

    /// Get the ANI version
    pub fn get_version(&self) -> Result<ANIVersion> {
        let mut version: u32 = 0;
        unsafe {
            let status = ani_call_unchecked!(self, GetVersion, &mut version);
            ani_status_to_result(status)?;
        }
        Ok(ANIVersion::from(version))
    }

    /// Check if there is an unhandled error
    pub fn exception_check(&self) -> bool {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let _ = ani_call_unchecked!(self, ExistUnhandledError, &mut result);
        }
        result != sys::ANI_FALSE as sys::ani_boolean
    }

    /// Reset/clear the current error state
    pub fn exception_clear(&self) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, ResetError);
            ani_status_to_result(status)
        }
    }

    /// Describe the current error (prints stack trace)
    pub fn exception_describe(&self) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, DescribeError);
            ani_status_to_result(status)
        }
    }

    /// Throw an error
    pub fn throw(&self, err: &JThrowable) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, ThrowError, err.as_raw());
            ani_status_to_result(status)
        }
    }

    /// Find a class by descriptor
    ///
    /// # Arguments
    ///
    /// * `name` - The class descriptor (e.g., "Lstd/core/String;")
    pub fn find_class<'local>(&mut self, name: &str) -> Result<JClass<'local>> {
        let c_name = std::ffi::CString::new(name).map_err(|_| Error::NullPtr("class name"))?;
        let mut result: sys::ani_class = ptr::null_mut();
        unsafe {
            let status = ani_call_unchecked!(self, FindClass, c_name.as_ptr(), &mut result);
            ani_status_to_result(status)?;
            Ok(JClass::from_raw(result))
        }
    }

    /// Create a new string from a Rust string
    pub fn new_string<'local>(&mut self, s: &str) -> Result<JString<'local>> {
        let utf8_bytes = s.as_bytes();
        let mut result: sys::ani_string = ptr::null_mut();
        unsafe {
            let status = ani_call_unchecked!(
                self,
                String_NewUTF8,
                utf8_bytes.as_ptr() as *const c_char,
                utf8_bytes.len(),
                &mut result
            );
            ani_status_to_result(status)?;
            Ok(JString::from_raw(result))
        }
    }

    /// Get the string contents as UTF-8
    pub fn get_string(&self, string: &JString) -> Result<String> {
        // First get the size
        let mut size: sys::ani_size = 0;
        unsafe {
            let status = ani_call_unchecked!(self, String_GetUTF8Size, string.as_raw(), &mut size);
            ani_status_to_result(status)?;
        }

        // Allocate buffer and get string
        let mut buffer = vec![0u8; size + 1];
        let mut written: sys::ani_size = 0;
        unsafe {
            let status = ani_call_unchecked!(
                self,
                String_GetUTF8,
                string.as_raw(),
                buffer.as_mut_ptr() as *mut c_char,
                size + 1,
                &mut written
            );
            ani_status_to_result(status)?;
        }

        buffer.truncate(written);
        String::from_utf8(buffer).map_err(|_| Error::NullPtr("invalid UTF-8"))
    }

    /// Delete a local reference
    pub fn delete_local_ref<'a, O>(&self, obj: O)
    where
        O: Into<JObject<'a>>,
    {
        let raw = obj.into().into_raw();
        if !raw.is_null() {
            unsafe {
                let _ = ani_call_unchecked!(self, Reference_Delete, raw);
            }
        }
    }

    /// Create a new local reference to an object
    pub fn new_local_ref<'local, T>(&mut self, obj: &T) -> Result<T::Kind<'local>>
    where
        T: JObjectRef,
    {
        let raw = obj.as_raw();
        if raw.is_null() {
            return Err(Error::ObjectFreed);
        }
        // In ANI, we don't have a direct NewLocalRef, so we just return the object as-is
        // The reference management is handled differently in ANI
        Ok(unsafe { T::from_local_raw(raw) })
    }

    /// Create a new global reference to an object
    pub fn new_global_ref<T>(&self, obj: &T) -> Result<GlobalRef<T::GlobalKind>>
    where
        T: JObjectRef,
    {
        let raw = obj.as_raw();
        if raw.is_null() {
            return Err(Error::ObjectFreed);
        }
        // In ANI, global references work differently
        // For now, we just wrap the raw pointer
        Ok(unsafe { GlobalRef::new(self, T::from_global_raw(raw)) })
    }

    /// Create a new weak reference to an object
    pub fn new_weak_ref<T>(&mut self, obj: &T) -> Result<WeakRef<T::GlobalKind>>
    where
        T: JObjectRef,
    {
        let raw = obj.as_raw();
        if raw.is_null() {
            return Err(Error::ObjectFreed);
        }
        Ok(unsafe { WeakRef::new(self, T::from_global_raw(raw)) })
    }

    /// Check if two references refer to the same object
    pub fn is_same_object<'other_local_1, 'other_local_2, O, T>(
        &self,
        ref1: O,
        ref2: T,
    ) -> bool
    where
        O: AsRef<JObject<'other_local_1>>,
        T: AsRef<JObject<'other_local_2>>,
    {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let status = ani_call_unchecked!(
                self,
                Reference_StrictEquals,
                ref1.as_ref().as_raw(),
                ref2.as_ref().as_raw(),
                &mut result
            );
            if status != sys::ani_status_ANI_OK {
                return false;
            }
        }
        result != sys::ANI_FALSE as sys::ani_boolean
    }

    /// Create an AutoLocal wrapper for automatic local reference deletion
    pub fn auto_local<'local, T>(&'local self, obj: T) -> AutoLocal<'local, T>
    where
        T: Into<JObject<'local>>,
    {
        AutoLocal::new(obj, self)
    }

    /// Ensure enough local references can be created
    pub fn ensure_local_capacity(&self, capacity: usize) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, EnsureEnoughReferences, capacity);
            ani_status_to_result(status)
        }
    }

    /// Push a new local reference frame
    pub fn push_local_frame(&self, capacity: usize) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, CreateLocalScope, capacity);
            ani_status_to_result(status)
        }
    }

    /// Pop a local reference frame  
    pub fn pop_local_frame(&self) -> Result<()> {
        unsafe {
            let status = ani_call_unchecked!(self, DestroyLocalScope);
            ani_status_to_result(status)
        }
    }
}

// JNI compatibility alias
pub type JNIEnv<'local> = ANIEnv<'local>;

