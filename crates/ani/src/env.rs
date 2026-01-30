//! ANI Environment Wrapper
//!
//! Provides safe wrapper for ANI environment

use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::slice;

use crate::error::{BusinessError, Error, Result, Status};
use crate::sys;
use crate::types::*;

// ============================================================================
// ANI API Call Macros (__ani_interaction_api)
// ============================================================================
//
// All ANI env API calls go through these macros. Usage: pass env (or self) as first arg.
// Example: ani_call!(env, ThrowError, error.as_raw()), ani_call_ret!(self, GetVersion, u32, 0)

/// Call ANI API function without return value.
/// Usage: `ani_call!($env, FunctionName, arg1, arg2, ...)`
#[macro_export]
macro_rules! ani_call {
    ($env:expr, $func:ident $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw $(, $arg)*);
            $crate::error::check_status(status)
        }
    }};
}

/// Call ANI API function with primitive return value (result at end).
/// Usage: `ani_call_ret!($env, FunctionName, result_type, default_value, arg1, arg2, ...)`
#[macro_export]
macro_rules! ani_call_ret {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw $(, $arg)*, &mut result);
            $crate::error::check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API function with two return values (two out params at end).
/// Usage: `ani_call_2ret!($env, FunctionName, ret1_ty, ret2_ty, default1, default2, arg1, ...)`
#[macro_export]
macro_rules! ani_call_2ret {
    ($env:expr, $func:ident, $ret1_ty:ty, $ret2_ty:ty, $default1:expr, $default2:expr $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        let mut result1: $ret1_ty = $default1;
        let mut result2: $ret2_ty = $default2;
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw $(, $arg)*, &mut result1, &mut result2);
            $crate::error::check_status(status)?;
        }
        Result::<($ret1_ty, $ret2_ty)>::Ok((result1, result2))
    }};
}

/// Call ANI API that returns status only (no out param). Use when you need to do work after the call (e.g. copy into buffer).
/// Usage: `ani_call_status!($env, FunctionName, arg1, ...)` returns `Result<()>`.
#[macro_export]
macro_rules! ani_call_status {
    ($env:expr, $func:ident $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw $(, $arg)*);
            $crate::error::check_status(status)
        }
    }};
}

/// Call ANI API function with primitive return value (result before last arg).
/// For APIs like Object_CallMethod_Int_A where signature is (env, obj, method, &result, args).
#[macro_export]
macro_rules! ani_call_method_ret {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr, $obj:expr, $method:expr, $args:expr) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw, $obj, $method, &mut result, $args);
            $crate::error::check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API function with wrapped return value (result at end).
/// Usage: `ani_call_wrap!($env, FunctionName, sys_type, WrapperType, arg1, arg2, ...)`
#[macro_export]
macro_rules! ani_call_wrap {
    ($env:expr, $func:ident, $sys_ty:ty, $wrap_ty:ident $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        let mut result: $sys_ty = ::std::ptr::null_mut();
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw $(, $arg)*, &mut result);
            $crate::error::check_status(status)?;
            Ok($wrap_ty::from_raw(result))
        }
    }};
}

/// Call ANI API function with wrapped return value (result before last arg).
/// For APIs like Object_CallMethod_Ref_A where signature is (env, obj, method, &result, args).
#[macro_export]
macro_rules! ani_call_method_wrap {
    ($env:expr, $func:ident, $sys_ty:ty, $wrap_ty:ident, $obj:expr, $method:expr, $args:expr) => {{
        let raw = $env.as_raw();
        let mut result: $sys_ty = ::std::ptr::null_mut();
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw, $obj, $method, &mut result, $args);
            $crate::error::check_status(status)?;
            Ok($wrap_ty::from_raw(result))
        }
    }};
}

/// Call ANI API "by name" function with primitive return value.
/// For APIs like Object_CallMethodByName_Int_A where signature is (env, obj, name, sig, &result, args).
#[macro_export]
macro_rules! ani_call_by_name_ret {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr, $obj:expr, $name:expr, $sig:expr, $args:expr) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw, $obj, $name, $sig, &mut result, $args);
            $crate::error::check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API with single return value; returns Result<T> without propagating (?). Use in fn that returns bool/Option.
/// Usage: `ani_call_ret_result!($env, Func, ret_ty, default, arg1, ...)` then e.g. `.map(|r| r != 0).unwrap_or(false)`.
#[macro_export]
macro_rules! ani_call_ret_result {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr $(, $arg:expr)*) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        let status = unsafe {
            let api = &*(*raw);
            (api.$func.unwrap())(raw $(, $arg)*, &mut result)
        };
        match $crate::error::check_status(status) {
            Ok(()) => Ok(result),
            Err(e) => Err(e),
        }
    }};
}

/// Call ANI API with result in the middle: (env, a, b, c, &mut result, d). For Object_CallMethodByName_Ref.
/// Usage: `ani_call_ret_mid!($env, Func, ret_ty, default, a, b, c, d)`.
#[macro_export]
macro_rules! ani_call_ret_mid {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*raw);
            let status = (api.$func.unwrap())(raw, $a, $b, $c, &mut result, $d);
            $crate::error::check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API with result before last arg: (env, a, b, &mut result, c). For Object_New variadic.
/// Usage: `ani_call_ret_before_last!($env, Func, ret_ty, default, a, b, c)`.
#[macro_export]
macro_rules! ani_call_ret_before_last {
    ($env:expr, $func:ident, $ret_ty:ty, $default:expr, $a:expr, $b:expr, $c:expr) => {{
        let raw = $env.as_raw();
        let mut result: $ret_ty = $default;
        let status = unsafe {
            let api = &*(*raw);
            (api.$func.unwrap())(raw, $a, $b, &mut result, $c)
        };
        match $crate::error::check_status(status) {
            Ok(()) => Ok(result),
            Err(e) => Err(e),
        }
    }};
}

/// Get __ani_interaction_api from env (expression with .as_raw(), e.g. &Env).
/// Usage: `let api = ani_api!(env);` then e.g. `api.GlobalReference_Delete`
#[macro_export]
macro_rules! ani_api {
    ($env:expr) => {{
        let raw = $env.as_raw();
        unsafe { &*(*raw) }
    }};
}

/// Get __ani_interaction_api from raw env pointer (*mut ani_env). Use in Drop or unsafe blocks.
/// Usage: `let api = ani_api_raw!(env);`
/// Get __ani_interaction_api from raw env pointer (*mut ani_env). Use in Drop or unsafe blocks.
#[macro_export]
macro_rules! ani_api_raw {
    ($env:expr) => {{
        unsafe { &*(*$env) }
    }};
}

/// ANI Environment Wrapper
///
/// This is the main interface for interacting with ANI VM. All ANI operations go through this struct.
///
/// # Lifetime
///
/// The `'local` lifetime represents the validity period of local references created in this environment.
/// When the environment is released, all associated local references become invalid.
///
/// # Thread Safety
///
/// Env is not Send or Sync because ANI environment can only be used in the thread that created it.
#[repr(transparent)]
pub struct Env<'local> {
    raw: *mut sys::ani_env,
    // Use *const () to make Env non-Send/Sync
    _marker: PhantomData<(&'local (), *const ())>,
}

impl<'local> Env<'local> {
    /// Create Env from raw pointer
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and remains valid for the lifetime of Env
    #[inline]
    pub unsafe fn from_raw(raw: *mut sys::ani_env) -> Result<Self> {
        if raw.is_null() {
            return Err(Error::new(
                Status::InvalidArgs,
                format!("Null pointer: {}", "ani_env"),
            ));
        }
        Ok(Self {
            raw,
            _marker: PhantomData,
        })
    }

    /// Create Env from raw pointer (without null check)
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is non-null and valid
    #[inline]
    pub unsafe fn from_raw_unchecked(raw: *mut sys::ani_env) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get raw pointer
    #[inline]
    pub fn as_raw(&self) -> *mut sys::ani_env {
        self.raw
    }

    /// Get ANI version
    pub fn get_version(&self) -> Result<u32> {
        ani_call_ret!(self, GetVersion, u32, 0)
    }

    // ========================================================================
    // Class and Module Operations
    // ========================================================================

    /// Find class
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Class descriptor, e.g., "Lstd/core/String;"
    pub fn find_class(&self, descriptor: &str) -> Result<AniClass<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid class descriptor"))?;
        ani_call_wrap!(
            self,
            FindClass,
            sys::ani_class,
            AniClass,
            c_descriptor.as_ptr()
        )
    }

    /// Find module
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Module descriptor, e.g., "Lmy_module;"
    pub fn find_module(&self, descriptor: &str) -> Result<AniModule<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid module descriptor"))?;
        ani_call_wrap!(
            self,
            FindModule,
            sys::ani_module,
            AniModule,
            c_descriptor.as_ptr()
        )
    }

    /// Find namespace
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Namespace descriptor, e.g., "Lmodule/MyNamespace;"
    pub fn find_namespace(&self, descriptor: &str) -> Result<AniNamespace<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid namespace descriptor"))?;
        ani_call_wrap!(
            self,
            FindNamespace,
            sys::ani_namespace,
            AniNamespace,
            c_descriptor.as_ptr()
        )
    }

    /// Find enum
    pub fn find_enum(&self, descriptor: &str) -> Result<AniEnum<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid enum descriptor"))?;
        ani_call_wrap!(
            self,
            FindEnum,
            sys::ani_enum,
            AniEnum,
            c_descriptor.as_ptr()
        )
    }

    // ========================================================================
    // Class Method Operations
    // ========================================================================

    /// Find class instance method
    ///
    /// # Arguments
    ///
    /// * `class` - Class
    /// * `name` - Method name
    /// * `signature` - Method signature, e.g., "II:I"
    pub fn find_method(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: &str,
    ) -> Result<AniMethod> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid method signature"))?;
        ani_call_wrap!(
            self,
            Class_FindMethod,
            sys::ani_method,
            AniMethod,
            class.as_raw(),
            c_name.as_ptr(),
            c_sig.as_ptr()
        )
    }

    /// Find class static method
    pub fn find_static_method(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: &str,
    ) -> Result<AniStaticMethod> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid method signature"))?;
        ani_call_wrap!(
            self,
            Class_FindStaticMethod,
            sys::ani_static_method,
            AniStaticMethod,
            class.as_raw(),
            c_name.as_ptr(),
            c_sig.as_ptr()
        )
    }

    /// Find constructor
    pub fn find_constructor(&self, class: &AniClass<'_>, signature: &str) -> Result<AniMethod> {
        self.find_method(class, "<ctor>", signature)
    }

    // ========================================================================
    // Field Operations
    // ========================================================================

    /// Find class field
    pub fn find_field(&self, class: &AniClass<'_>, name: &str) -> Result<AniField> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_wrap!(
            self,
            Class_FindField,
            sys::ani_field,
            AniField,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    // ========================================================================
    // String Operations
    // ========================================================================

    /// Create new ANI string
    pub fn create_string(&self, s: &str) -> Result<AniString<'local>> {
        let bytes = s.as_bytes();
        ani_call_wrap!(
            self,
            String_NewUTF8,
            sys::ani_string,
            AniString,
            bytes.as_ptr() as *const i8,
            bytes.len()
        )
    }

    /// Get ANI string content
    pub fn get_string(&self, string: &AniString<'_>) -> Result<String> {
        // First get the string size
        let size = ani_call_ret!(self, String_GetUTF8Size, sys::ani_size, 0, string.as_raw())?;

        // Allocate buffer and get string content
        let mut buffer = vec![0u8; size + 1];
        let written = ani_call_ret!(
            self,
            String_GetUTF8,
            sys::ani_size,
            0,
            string.as_raw(),
            buffer.as_mut_ptr() as *mut i8,
            size + 1
        )?;

        buffer.truncate(written);
        String::from_utf8(buffer)
            .map_err(|e| Error::new(Status::Error, format!("Invalid UTF-8: {}", e)))
    }

    // ========================================================================
    // Object Operations
    // ========================================================================

    /// Create new object
    pub fn new_object(
        &self,
        class: &AniClass<'_>,
        constructor: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<AniObject<'local>> {
        ani_call_method_wrap!(
            self,
            Object_New_A,
            sys::ani_object,
            AniObject,
            class.as_raw(),
            constructor.as_raw(),
            args.as_ptr()
        )
    }

    /// Check if object is instance of specified type
    pub fn is_instance_of(&self, obj: &AniObject<'_>, class: &AniClass<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Object_InstanceOf,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            class.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Get object type
    pub fn get_object_type(&self, obj: &AniObject<'_>) -> Result<AniType<'local>> {
        ani_call_wrap!(self, Object_GetType, sys::ani_type, AniType, obj.as_raw())
    }

    // ========================================================================
    // Method Invocation
    // ========================================================================

    /// Call method returning void
    pub fn call_void_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<()> {
        ani_call!(
            self,
            Object_CallMethod_Void_A,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    /// Call method returning int
    pub fn call_int_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i32> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Int_A,
            sys::ani_int,
            0,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    /// Call a method returning long
    pub fn call_long_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i64> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Long_A,
            sys::ani_long,
            0,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    /// Call a method returning double
    pub fn call_double_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<f64> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Double_A,
            sys::ani_double,
            0.0,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    /// Call a method returning boolean
    pub fn call_boolean_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<bool> {
        let result = ani_call_method_ret!(
            self,
            Object_CallMethod_Boolean_A,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )?;
        Ok(result != 0)
    }

    /// Call a method returning object reference
    pub fn call_ref_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        ani_call_method_wrap!(
            self,
            Object_CallMethod_Ref_A,
            sys::ani_ref,
            AniRef,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    // ========================================================================
    // Call Method by Name (Simplified API)
    // ========================================================================

    /// Call a method returning int by name
    pub fn call_method_by_name_int(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Int_A,
            sys::ani_int,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning long by name
    pub fn call_method_by_name_long(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Long_A,
            sys::ani_long,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning double by name
    pub fn call_method_by_name_double(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Double_A,
            sys::ani_double,
            0.0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning float by name
    pub fn call_method_by_name_float(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Float_A,
            sys::ani_float,
            0.0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning boolean by name
    pub fn call_method_by_name_boolean(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<bool> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        let result = ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Boolean_A,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )?;
        Ok(result != 0)
    }

    /// Call a method returning byte by name
    pub fn call_method_by_name_byte(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i8> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Byte_A,
            sys::ani_byte,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning short by name
    pub fn call_method_by_name_short(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Short_A,
            sys::ani_short,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a method returning char by name
    pub fn call_method_by_name_char(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<u16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Object_CallMethodByName_Char_A,
            sys::ani_char,
            0,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null()
        )
    }

    /// Call a void method by name
    pub fn call_method_by_name_void(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call!(
            self,
            Object_CallMethodByName_Void_A,
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            ptr::null::<sys::ani_value>()
        )
    }

    /// Call a void method (using method handle)
    pub fn call_method_void(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<()> {
        ani_call!(
            self,
            Object_CallMethod_Void_A,
            obj.as_raw(),
            method.as_raw(),
            args.as_ptr()
        )
    }

    // ========================================================================
    // Field Access
    // ========================================================================

    /// Get int type field value
    pub fn get_field_int(&self, obj: &AniObject<'_>, field: &AniField) -> Result<i32> {
        ani_call_ret!(self, Object_GetField_Int, sys::ani_int, 0, obj.as_raw(), field.as_raw())
    }

    /// Set int type field value
    pub fn set_field_int(&self, obj: &AniObject<'_>, field: &AniField, value: i32) -> Result<()> {
        ani_call!(self, Object_SetField_Int, obj.as_raw(), field.as_raw(), value)
    }

    /// Get int type field value by name
    pub fn get_field_by_name_int(&self, obj: &AniObject<'_>, name: &str) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Int,
            sys::ani_int,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set int type field value by name
    pub fn set_field_by_name_int(&self, obj: &AniObject<'_>, name: &str, value: i32) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(self, Object_SetFieldByName_Int, obj.as_raw(), c_name.as_ptr(), value)
    }

    // ========================================================================
    // Property Access
    // ========================================================================

    /// Get int type property value by name
    pub fn get_property_by_name_int(&self, obj: &AniObject<'_>, name: &str) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Int,
            sys::ani_int,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set int type property value by name
    pub fn set_property_by_name_int(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i32,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Int,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    // ========================================================================
    // Native Method Binding
    // ========================================================================

    /// Bind native methods to a class
    pub fn bind_class_native_methods(
        &self,
        class: &AniClass<'_>,
        methods: &[sys::ani_native_function],
    ) -> Result<()> {
        ani_call!(
            self,
            Class_BindNativeMethods,
            class.as_raw(),
            methods.as_ptr(),
            methods.len()
        )
    }

    /// Bind native functions to a namespace
    pub fn bind_namespace_native_functions(
        &self,
        namespace: &AniNamespace<'_>,
        functions: &[sys::ani_native_function],
    ) -> Result<()> {
        ani_call!(
            self,
            Namespace_BindNativeFunctions,
            namespace.as_raw(),
            functions.as_ptr(),
            functions.len()
        )
    }

    /// Bind native functions to a module
    pub fn bind_module_native_functions(
        &self,
        module: &AniModule<'_>,
        functions: &[sys::ani_native_function],
    ) -> Result<()> {
        ani_call!(
            self,
            Module_BindNativeFunctions,
            module.as_raw(),
            functions.as_ptr(),
            functions.len()
        )
    }

    // ========================================================================
    // Exception Handling
    // ========================================================================

    /// Check if there is an unhandled exception
    pub fn has_exception(&self) -> bool {
        ani_call_ret_result!(self, ExistUnhandledError, sys::ani_boolean, 0)
            .map(|r| r != 0)
            .unwrap_or(false)
    }

    /// Clear current exception
    pub fn clear_exception(&self) -> Result<()> {
        ani_call!(self, ResetError)
    }

    /// Describe current exception (print stack trace)
    pub fn describe_exception(&self) -> Result<()> {
        ani_call!(self, DescribeError)
    }

    /// Throw an ANI error object
    ///
    /// This method throws an existing ANI error object (from `get_unhandled_error` or similar).
    pub fn throw_error(&self, error: &AniError<'_>) -> Result<()> {
        ani_call!(self, ThrowError, error.as_raw())
    }

    // ========================================================================
    // Reference Management
    // ========================================================================

    /// Create a global reference
    pub fn create_global_ref<'a>(&self, obj: &AniRef<'a>) -> Result<GlobalRef> {
        ani_call_wrap!(
            self,
            GlobalReference_Create,
            sys::ani_ref,
            GlobalRef,
            obj.as_raw()
        )
    }

    /// Delete a global reference
    pub fn delete_global_ref(&self, gref: GlobalRef) -> Result<()> {
        ani_call!(self, GlobalReference_Delete, gref.as_raw())
    }

    /// Check if the reference is null
    pub fn is_null(&self, obj: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(self, Reference_IsNull, sys::ani_boolean, 0, obj.as_raw())?;
        Ok(result != 0)
    }

    /// Check if the reference is undefined
    pub fn is_undefined(&self, obj: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Reference_IsUndefined,
            sys::ani_boolean,
            0,
            obj.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Get the null object reference
    pub fn get_null_object(&self) -> Result<sys::ani_object> {
        let r = ani_call_ret!(self, GetNull, sys::ani_ref, ptr::null_mut())?;
        Ok(r as sys::ani_object)
    }

    /// Get the undefined object reference
    pub fn get_undefined_object(&self) -> Result<sys::ani_object> {
        let r = ani_call_ret!(self, GetUndefined, sys::ani_ref, ptr::null_mut())?;
        Ok(r as sys::ani_object)
    }

    // ========================================================================
    // Array Operations
    // ========================================================================

    /// Create an int array
    pub fn create_int_array(&self, length: usize) -> Result<AniArrayInt<'local>> {
        ani_call_wrap!(self, Array_New_Int, sys::ani_array_int, AniArrayInt, length)
    }

    /// Get array length
    pub fn get_array_length(&self, array: &AniArray<'_>) -> Result<usize> {
        ani_call_ret!(self, Array_GetLength, sys::ani_size, 0, array.as_raw())
    }

    // ========================================================================
    // ArrayBuffer Operations
    // ========================================================================

    /// Create a new ArrayBuffer with the specified size.
    ///
    /// The buffer is initialized with unspecified values. Use `create_arraybuffer_zeroed`
    /// if you need zero-initialized data.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the buffer in bytes
    ///
    /// # Returns
    ///
    /// Returns a tuple of (data_ptr, arraybuffer) where:
    /// - `data_ptr` is a pointer to the buffer's data that can be used to write data
    /// - `arraybuffer` is the ANI ArrayBuffer wrapper
    ///
    /// # Example
    ///
    /// ```ignore
    /// let (data_ptr, buffer) = env.create_arraybuffer(1024)?;
    /// unsafe {
    ///     // Write data directly to the buffer
    ///     std::ptr::write(data_ptr as *mut u32, 42);
    /// }
    /// ```
    pub fn create_arraybuffer(
        &self,
        size: usize,
    ) -> Result<(*mut std::ffi::c_void, AniArrayBuffer<'local>)> {
        let (data_ptr, arraybuffer): (*mut std::ffi::c_void, sys::ani_arraybuffer) = ani_call_2ret!(
            self,
            CreateArrayBuffer,
            *mut std::ffi::c_void,
            sys::ani_arraybuffer,
            ptr::null_mut(),
            ptr::null_mut(),
            size
        )?;
        Ok((data_ptr, unsafe { AniArrayBuffer::from_raw(arraybuffer) }))
    }

    /// Create a new ArrayBuffer initialized with the provided data.
    ///
    /// This creates a new ANI ArrayBuffer and copies the provided data into it.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to copy into the ArrayBuffer
    ///
    /// # Example
    ///
    /// ```ignore
    /// let buffer = env.create_arraybuffer_with_data(&[1, 2, 3, 4, 5])?;
    /// ```
    pub fn create_arraybuffer_with_data(&self, data: &[u8]) -> Result<AniArrayBuffer<'local>> {
        let (data_ptr, buffer) = self.create_arraybuffer(data.len())?;

        if !data.is_empty() && !data_ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), data_ptr as *mut u8, data.len());
            }
        }

        Ok(buffer)
    }

    /// Get information about an ArrayBuffer.
    ///
    /// Returns the data pointer and length of the ArrayBuffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The ArrayBuffer to get information about
    ///
    /// # Returns
    ///
    /// Returns a tuple of (data_ptr, length) where:
    /// - `data_ptr` is a pointer to the buffer's data
    /// - `length` is the size of the buffer in bytes
    ///
    /// # Example
    ///
    /// ```ignore
    /// let (data_ptr, len) = env.get_arraybuffer_info(&buffer)?;
    /// let data = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, len) };
    /// ```
    pub fn get_arraybuffer_info(
        &self,
        buffer: &AniArrayBuffer<'_>,
    ) -> Result<(*mut std::ffi::c_void, usize)> {
        ani_call_2ret!(
            self,
            ArrayBuffer_GetInfo,
            *mut std::ffi::c_void,
            usize,
            ptr::null_mut(),
            0,
            buffer.as_raw()
        )
    }

    /// Read the contents of an ArrayBuffer into a Vec<u8>.
    ///
    /// This creates a copy of the ArrayBuffer data.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The ArrayBuffer to read from
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data: Vec<u8> = env.read_arraybuffer(&buffer)?;
    /// ```
    pub fn read_arraybuffer(&self, buffer: &AniArrayBuffer<'_>) -> Result<Vec<u8>> {
        let (data_ptr, length) = self.get_arraybuffer_info(buffer)?;

        if data_ptr.is_null() || length == 0 {
            return Ok(Vec::new());
        }

        unsafe {
            let data = slice::from_raw_parts(data_ptr as *const u8, length).to_vec();
            Ok(data)
        }
    }

    // ========================================================================
    // Type Checking
    // ========================================================================

    /// Check if object is an instance of the specified class
    pub fn object_instance_of(&self, obj: &AniObject<'_>, cls: &AniClass<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Object_InstanceOf,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            cls.as_raw()
        )?;
        Ok(result != 0)
    }

    // ========================================================================
    // Call Method by Name Returning Reference
    // ========================================================================

    /// Call a method returning reference by name
    pub fn call_method_by_name_ref(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        arg: sys::ani_int,
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());

        let result = ani_call_ret_mid!(
            self,
            Object_CallMethodByName_Ref,
            sys::ani_ref,
            ptr::null_mut(),
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            arg
        )?;
        Ok(unsafe { AniRef::from_raw(result) })
    }

    // ========================================================================
    // Property Access (double)
    // ========================================================================

    /// Get double type property value by name
    pub fn get_property_by_name_double(&self, obj: &AniObject<'_>, name: &str) -> Result<f64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Double,
            sys::ani_double,
            0.0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set double type property value by name
    pub fn set_property_by_name_double(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: f64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Double,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    // ========================================================================
    // Error Handling (High-Level)
    // ========================================================================

    /// Throw an error with a message into the ANI environment
    ///
    /// This creates an Error object and throws it as an exception.
    /// Unlike [`throw_error`](Self::throw_error), this method takes a message string
    /// and creates the error object automatically.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ani::prelude::*;
    ///
    /// fn may_throw(env: &Env) -> Result<()> {
    ///     if some_condition {
    ///         env.throw_error_message("Something went wrong")?;
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn throw_error_message(&self, message: &str) -> Result<()> {
        let err = Error::new(Status::Error, message);
        let biz_err = BusinessError::from(err);
        unsafe { biz_err.throw_into(self.raw) };
        Ok(())
    }

    /// Throw a type error into the ANI environment
    ///
    /// This creates a BusinessError with type error status.
    pub fn throw_type_error(&self, message: &str) -> Result<()> {
        let err = Error::new(Status::InvalidType, message);
        let biz_err = BusinessError::from(err);
        unsafe { biz_err.throw_into(self.raw) };
        Ok(())
    }

    /// Throw a range error into the ANI environment
    ///
    /// This creates a BusinessError with range error status.
    pub fn throw_range_error(&self, message: &str) -> Result<()> {
        let err = Error::new(Status::OutOfRange, message);
        let biz_err = BusinessError::from(err);
        unsafe { biz_err.throw_into(self.raw) };
        Ok(())
    }

    /// Throw any Error as an ANI BusinessError
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ani::prelude::*;
    ///
    /// let err = Error::new(Status::InvalidArgs, "Bad input");
    /// env.throw(err)?;
    /// ```
    pub fn throw<S: AsRef<str> + std::fmt::Debug>(&self, error: Error<S>) -> Result<()> {
        let biz_err = BusinessError::from(Error::new(Status::GenericFailure, error.to_string()));
        unsafe { biz_err.throw_into(self.raw) };
        Ok(())
    }

    /// Check if there's an unhandled error/exception pending
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if env.exist_unhandled_error()? {
    ///     // Handle the error
    ///     env.reset_error()?;
    /// }
    /// ```
    pub fn exist_unhandled_error(&self) -> Result<bool> {
        let has_error = ani_call_ret!(self, ExistUnhandledError, sys::ani_boolean, 0)?;
        Ok(has_error != 0)
    }

    /// Get the unhandled error if one exists
    ///
    /// Returns `None` if there is no pending error.
    pub fn get_unhandled_error(&self) -> Result<Option<AniError<'local>>> {
        if !self.exist_unhandled_error()? {
            return Ok(None);
        }
        let result = ani_call_ret!(self, GetUnhandledError, sys::ani_error, ptr::null_mut())?;
        if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { AniError::from_raw(result) }))
        }
    }

    /// Clear/reset the pending error state
    ///
    /// This clears any pending exception so that subsequent ANI calls can proceed.
    pub fn reset_error(&self) -> Result<()> {
        ani_call!(self, ResetError)
    }

    /// Abort the process with an error message
    ///
    /// This is for unrecoverable errors that should terminate the process.
    ///
    /// # Safety
    ///
    /// This function will terminate the process and never return.
    pub fn abort(&self, message: &str) -> ! {
        let c_message = CString::new(message).unwrap_or_else(|_| CString::new("Abort").unwrap());
        let _ = ani_call!(self, Abort, c_message.as_ptr());
        // Abort should never return, but in case it does
        std::process::abort()
    }

    // ========================================================================
    // Promise Operations (Low-level API)
    // ========================================================================
    //
    // For a higher-level Promise API, consider using `PromiseRaw` and `Deferred`
    // from the `ani::conversions` module:
    //
    // ```rust,ignore
    // use ani::conversions::{PromiseRaw, Deferred};
    //
    // // Immediately resolve a promise
    // let promise = PromiseRaw::resolve_int(&env, 42)?;
    //
    // // Create a deferred promise for later resolution
    // let (deferred, promise) = PromiseRaw::deferred(&env)?;
    // deferred.resolve_string(&env, "done")?;
    // ```

    /// Create a new Promise with its resolver (low-level API)
    ///
    /// Returns a tuple of (resolver, promise). The resolver is used to either
    /// resolve or reject the promise. Once resolved or rejected, the resolver
    /// is automatically freed.
    ///
    /// **Note:** For a higher-level API, consider using [`PromiseRaw`](crate::conversions::PromiseRaw)
    /// and [`Deferred`](crate::conversions::Deferred) from the `ani::conversions` module.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ani::prelude::*;
    ///
    /// fn create_async_task(env: &Env) -> Result<AniObject> {
    ///     let (resolver, promise) = env.promise_new()?;
    ///
    ///     // In a real scenario, you'd spawn a thread or async task
    ///     // that eventually calls resolve or reject
    ///     let result = env.create_string("done")?;
    ///     env.promise_resolve(&resolver, &result.into())?;
    ///
    ///     Ok(promise)
    /// }
    /// ```
    pub fn promise_new(&self) -> Result<(AniResolver, AniObject<'local>)> {
        let (resolver, promise) = ani_call_2ret!(
            self,
            Promise_New,
            sys::ani_resolver,
            sys::ani_object,
            ptr::null_mut(),
            ptr::null_mut()
        )?;
        Ok((
            unsafe { AniResolver::from_raw(resolver) },
            unsafe { AniObject::from_raw(promise) },
        ))
    }

    /// Resolve a Promise with a value
    ///
    /// This resolves the promise associated with the given resolver and queues
    /// any `then` callbacks. The resolver is freed after this call.
    ///
    /// # Arguments
    ///
    /// * `resolver` - The resolver for the promise to resolve
    /// * `value` - The value to resolve the promise with
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let (resolver, promise) = env.promise_new()?;
    /// let result = env.create_string("success")?;
    /// env.promise_resolve(&resolver, &result.into())?;
    /// ```
    pub fn promise_resolve(&self, resolver: &AniResolver, value: &AniRef<'_>) -> Result<()> {
        ani_call!(
            self,
            PromiseResolver_Resolve,
            resolver.as_raw(),
            value.as_raw()
        )
    }

    /// Reject a Promise with an error
    ///
    /// This rejects the promise associated with the given resolver and queues
    /// any `catch` callbacks. The resolver is freed after this call.
    ///
    /// # Arguments
    ///
    /// * `resolver` - The resolver for the promise to reject
    /// * `error` - The error to reject the promise with
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let (resolver, promise) = env.promise_new()?;
    /// let error = env.create_error("Something went wrong")?;
    /// env.promise_reject(&resolver, &error)?;
    /// ```
    pub fn promise_reject(&self, resolver: &AniResolver, error: &AniError<'_>) -> Result<()> {
        ani_call!(
            self,
            PromiseResolver_Reject,
            resolver.as_raw(),
            error.as_raw()
        )
    }

    /// Reject a Promise with a string message
    ///
    /// Convenience method that creates an error from a message and rejects the promise.
    ///
    /// # Arguments
    ///
    /// * `resolver` - The resolver for the promise to reject
    /// * `message` - The error message
    pub fn promise_reject_with_message(&self, resolver: &AniResolver, message: &str) -> Result<()> {
        let error_string = self.create_string(message)?;
        ani_call!(
            self,
            PromiseResolver_Reject,
            resolver.as_raw(),
            error_string.as_raw() as sys::ani_error
        )
    }
}
