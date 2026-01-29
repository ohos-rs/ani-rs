//! ANI Environment Wrapper
//!
//! Provides safe wrapper for ANI environment

use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use crate::error::{BusinessError, Error, Result, Status, check_status};
use crate::sys;
use crate::types::*;

// ============================================================================
// ANI API Call Macros
// ============================================================================

/// Call ANI API function without return value
///
/// Usage: `ani_call!(self, FunctionName, arg1, arg2, ...)`
macro_rules! ani_call {
    ($self:ident, $func:ident $(, $arg:expr)*) => {{
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw $(, $arg)*);
            check_status(status)
        }
    }};
}

/// Call ANI API function with primitive return value (result at end)
///
/// Usage: `ani_call_ret!(self, FunctionName, result_type, default_value, arg1, arg2, ...)`
macro_rules! ani_call_ret {
    ($self:ident, $func:ident, $ret_ty:ty, $default:expr $(, $arg:expr)*) => {{
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw $(, $arg)*, &mut result);
            check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API function with primitive return value (result before last arg)
///
/// For APIs like Object_CallMethod_Int_A where signature is (env, obj, method, &result, args)
macro_rules! ani_call_method_ret {
    ($self:ident, $func:ident, $ret_ty:ty, $default:expr, $obj:expr, $method:expr, $args:expr) => {{
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw, $obj, $method, &mut result, $args);
            check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
    }};
}

/// Call ANI API function with wrapped return value (result at end)
///
/// Usage: `ani_call_wrap!(self, FunctionName, sys_type, WrapperType, arg1, arg2, ...)`
macro_rules! ani_call_wrap {
    ($self:ident, $func:ident, $sys_ty:ty, $wrap_ty:ident $(, $arg:expr)*) => {{
        let mut result: $sys_ty = ptr::null_mut();
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw $(, $arg)*, &mut result);
            check_status(status)?;
            Ok($wrap_ty::from_raw(result))
        }
    }};
}

/// Call ANI API function with wrapped return value (result before last arg)
///
/// For APIs like Object_CallMethod_Ref_A where signature is (env, obj, method, &result, args)
macro_rules! ani_call_method_wrap {
    ($self:ident, $func:ident, $sys_ty:ty, $wrap_ty:ident, $obj:expr, $method:expr, $args:expr) => {{
        let mut result: $sys_ty = ptr::null_mut();
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw, $obj, $method, &mut result, $args);
            check_status(status)?;
            Ok($wrap_ty::from_raw(result))
        }
    }};
}

/// Call ANI API "by name" function with primitive return value
///
/// For APIs like Object_CallMethodByName_Int_A where signature is (env, obj, name, sig, &result, args)
macro_rules! ani_call_by_name_ret {
    ($self:ident, $func:ident, $ret_ty:ty, $default:expr, $obj:expr, $name:expr, $sig:expr, $args:expr) => {{
        let mut result: $ret_ty = $default;
        unsafe {
            let api = &*(*$self.raw);
            let status = (api.$func.unwrap())($self.raw, $obj, $name, $sig, &mut result, $args);
            check_status(status)?;
        }
        Result::<$ret_ty>::Ok(result)
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
        let mut written: sys::ani_size = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.String_GetUTF8.unwrap())(
                self.raw,
                string.as_raw(),
                buffer.as_mut_ptr() as *mut i8,
                size + 1,
                &mut written,
            );
            check_status(status)?;
        }

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
        let mut result: sys::ani_int = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_GetField_Int.unwrap())(
                self.raw,
                obj.as_raw(),
                field.as_raw(),
                &mut result,
            );
            check_status(status)?;
            Ok(result)
        }
    }

    /// Set int type field value
    pub fn set_field_int(&self, obj: &AniObject<'_>, field: &AniField, value: i32) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status =
                (api.Object_SetField_Int.unwrap())(self.raw, obj.as_raw(), field.as_raw(), value);
            check_status(status)
        }
    }

    /// Get int type field value by name
    pub fn get_field_by_name_int(&self, obj: &AniObject<'_>, name: &str) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        let mut result: sys::ani_int = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_GetFieldByName_Int.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(result)
        }
    }

    /// Set int type field value by name
    pub fn set_field_by_name_int(&self, obj: &AniObject<'_>, name: &str, value: i32) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_SetFieldByName_Int.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                value,
            );
            check_status(status)
        }
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
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let _ = (api.ExistUnhandledError.unwrap())(self.raw, &mut result);
            result != 0
        }
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
        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GetNull.unwrap())(self.raw, &mut result);
            check_status(status)?;
            Ok(result as sys::ani_object)
        }
    }

    /// Get the undefined object reference
    pub fn get_undefined_object(&self) -> Result<sys::ani_object> {
        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GetUndefined.unwrap())(self.raw, &mut result);
            check_status(status)?;
            Ok(result as sys::ani_object)
        }
    }

    // ========================================================================
    // Array Operations
    // ========================================================================

    /// Create an int array
    pub fn create_int_array(&self, length: usize) -> Result<AniArrayInt<'local>> {
        let mut result: sys::ani_array_int = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Array_New_Int.unwrap())(self.raw, length, &mut result);
            check_status(status)?;
            Ok(AniArrayInt::from_raw(result))
        }
    }

    /// Get array length
    pub fn get_array_length(&self, array: &AniArray<'_>) -> Result<usize> {
        let mut result: sys::ani_size = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Array_GetLength.unwrap())(self.raw, array.as_raw(), &mut result);
            check_status(status)?;
            Ok(result)
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

        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Ref.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                sig_ptr,
                &mut result,
                arg,
            );
            check_status(status)?;
            Ok(AniRef::from_raw(result))
        }
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

        let mut result: sys::ani_error = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GetUnhandledError.unwrap())(self.raw, &mut result);
            check_status(status)?;
            if result.is_null() {
                Ok(None)
            } else {
                Ok(Some(AniError::from_raw(result)))
            }
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
        let mut resolver: sys::ani_resolver = ptr::null_mut();
        let mut promise: sys::ani_object = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Promise_New.unwrap())(self.raw, &mut resolver, &mut promise);
            check_status(status)?;
            Ok((
                AniResolver::from_raw(resolver),
                AniObject::from_raw(promise),
            ))
        }
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
        // Create an error string and use it to reject
        // Note: In ANI, we can cast a string to ani_error for rejection
        let error_string = self.create_string(message)?;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.PromiseResolver_Reject.unwrap())(
                self.raw,
                resolver.as_raw(),
                error_string.as_raw() as sys::ani_error,
            );
            check_status(status)
        }
    }
}
