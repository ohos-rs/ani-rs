//! ANI Environment Wrapper
//!
//! Provides safe wrapper for ANI environment

use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

use crate::error::{BusinessError, Error, Result, Status, check_status};
use crate::sys;
use crate::types::*;

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
    // 使用 *const () 使 Env 非 Send/Sync
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
        let mut version: u32 = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GetVersion.unwrap())(self.raw, &mut version);
            check_status(status)?;
        }
        Ok(version)
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
        let mut result: sys::ani_class = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.FindClass.unwrap())(self.raw, c_descriptor.as_ptr(), &mut result);
            check_status(status)?;
            Ok(AniClass::from_raw(result))
        }
    }

    /// Find module
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Module descriptor, e.g., "Lmy_module;"
    pub fn find_module(&self, descriptor: &str) -> Result<AniModule<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid module descriptor"))?;
        let mut result: sys::ani_module = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.FindModule.unwrap())(self.raw, c_descriptor.as_ptr(), &mut result);
            check_status(status)?;
            Ok(AniModule::from_raw(result))
        }
    }

    /// Find namespace
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Namespace descriptor, e.g., "Lmodule/MyNamespace;"
    pub fn find_namespace(&self, descriptor: &str) -> Result<AniNamespace<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid namespace descriptor"))?;
        let mut result: sys::ani_namespace = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.FindNamespace.unwrap())(self.raw, c_descriptor.as_ptr(), &mut result);
            check_status(status)?;
            Ok(AniNamespace::from_raw(result))
        }
    }

    /// Find enum
    pub fn find_enum(&self, descriptor: &str) -> Result<AniEnum<'local>> {
        let c_descriptor = CString::new(descriptor)
            .map_err(|_| Error::new(Status::Error, "Invalid enum descriptor"))?;
        let mut result: sys::ani_enum = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.FindEnum.unwrap())(self.raw, c_descriptor.as_ptr(), &mut result);
            check_status(status)?;
            Ok(AniEnum::from_raw(result))
        }
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
        let mut result: sys::ani_method = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.Class_FindMethod.unwrap())(
                self.raw,
                class.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(AniMethod::from_raw(result))
        }
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
        let mut result: sys::ani_static_method = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.Class_FindStaticMethod.unwrap())(
                self.raw,
                class.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(AniStaticMethod::from_raw(result))
        }
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
        let mut result: sys::ani_field = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.Class_FindField.unwrap())(
                self.raw,
                class.as_raw(),
                c_name.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(AniField::from_raw(result))
        }
    }

    // ========================================================================
    // String Operations
    // ========================================================================

    /// Create new ANI string
    pub fn create_string(&self, s: &str) -> Result<AniString<'local>> {
        let bytes = s.as_bytes();
        let mut result: sys::ani_string = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.String_NewUTF8.unwrap())(
                self.raw,
                bytes.as_ptr() as *const i8,
                bytes.len(),
                &mut result,
            );
            check_status(status)?;
            Ok(AniString::from_raw(result))
        }
    }

    /// Get ANI string content
    pub fn get_string(&self, string: &AniString<'_>) -> Result<String> {
        // First get the string size
        let mut size: sys::ani_size = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.String_GetUTF8Size.unwrap())(self.raw, string.as_raw(), &mut size);
            check_status(status)?;
        }

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
        let mut result: sys::ani_object = ptr::null_mut();

        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_New_A.unwrap())(
                self.raw,
                class.as_raw(),
                constructor.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(AniObject::from_raw(result))
        }
    }

    /// Check if object is instance of specified type
    pub fn is_instance_of(&self, obj: &AniObject<'_>, class: &AniClass<'_>) -> Result<bool> {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_InstanceOf.unwrap())(
                self.raw,
                obj.as_raw(),
                class.as_raw(),
                &mut result,
            );
            check_status(status)?;
            Ok(result != 0)
        }
    }

    /// Get object type
    pub fn get_object_type(&self, obj: &AniObject<'_>) -> Result<AniType<'local>> {
        let mut result: sys::ani_type = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_GetType.unwrap())(self.raw, obj.as_raw(), &mut result);
            check_status(status)?;
            Ok(AniType::from_raw(result))
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Void_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                args.as_ptr(),
            );
            check_status(status)
        }
    }

    /// Call method returning int
    pub fn call_int_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i32> {
        let mut result: sys::ani_int = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Int_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(result)
        }
    }

    /// Call a method returning long
    pub fn call_long_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i64> {
        let mut result: sys::ani_long = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Long_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(result)
        }
    }

    /// Call a method returning double
    pub fn call_double_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<f64> {
        let mut result: sys::ani_double = 0.0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Double_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(result)
        }
    }

    /// Call a method returning boolean
    pub fn call_boolean_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<bool> {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Boolean_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(result != 0)
        }
    }

    /// Call a method returning object reference
    pub fn call_ref_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Ref_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                &mut result,
                args.as_ptr(),
            );
            check_status(status)?;
            Ok(AniRef::from_raw(result))
        }
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

        let mut result: sys::ani_int = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Int_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_long = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Long_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_double = 0.0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Double_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_float = 0.0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Float_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Boolean_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result != 0)
        }
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

        let mut result: sys::ani_byte = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Byte_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_short = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Short_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        let mut result: sys::ani_char = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Char_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                &mut result,
                ptr::null(), // No arguments
            );
            check_status(status)?;
            Ok(result)
        }
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

        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Void_A.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                ptr::null(), // No arguments
            );
            check_status(status)
        }
    }

    /// Call a void method (using method handle)
    pub fn call_method_void(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethod_Void_A.unwrap())(
                self.raw,
                obj.as_raw(),
                method.as_raw(),
                args.as_ptr(),
            );
            check_status(status)
        }
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
        let mut result: sys::ani_int = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_GetPropertyByName_Int.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(result)
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_SetPropertyByName_Int.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                value,
            );
            check_status(status)
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Class_BindNativeMethods.unwrap())(
                self.raw,
                class.as_raw(),
                methods.as_ptr(),
                methods.len(),
            );
            check_status(status)
        }
    }

    /// Bind native functions to a namespace
    pub fn bind_namespace_native_functions(
        &self,
        namespace: &AniNamespace<'_>,
        functions: &[sys::ani_native_function],
    ) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Namespace_BindNativeFunctions.unwrap())(
                self.raw,
                namespace.as_raw(),
                functions.as_ptr(),
                functions.len(),
            );
            check_status(status)
        }
    }

    /// Bind native functions to a module
    pub fn bind_module_native_functions(
        &self,
        module: &AniModule<'_>,
        functions: &[sys::ani_native_function],
    ) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Module_BindNativeFunctions.unwrap())(
                self.raw,
                module.as_raw(),
                functions.as_ptr(),
                functions.len(),
            );
            check_status(status)
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.ResetError.unwrap())(self.raw);
            check_status(status)
        }
    }

    /// Describe current exception (print stack trace)
    pub fn describe_exception(&self) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.DescribeError.unwrap())(self.raw);
            check_status(status)
        }
    }

    /// Throw an ANI error object
    ///
    /// This method throws an existing ANI error object (from `get_unhandled_error` or similar).
    pub fn throw_error(&self, error: &AniError<'_>) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.ThrowError.unwrap())(self.raw, error.as_raw());
            check_status(status)
        }
    }

    // ========================================================================
    // Reference Management
    // ========================================================================

    /// Create a global reference
    pub fn create_global_ref<'a>(&self, obj: &AniRef<'a>) -> Result<GlobalRef> {
        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GlobalReference_Create.unwrap())(self.raw, obj.as_raw(), &mut result);
            check_status(status)?;
            Ok(GlobalRef::from_raw(result))
        }
    }

    /// 删除全局引用
    pub fn delete_global_ref(&self, gref: GlobalRef) -> Result<()> {
        unsafe {
            let api = &*(*self.raw);
            let status = (api.GlobalReference_Delete.unwrap())(self.raw, gref.as_raw());
            check_status(status)
        }
    }

    /// 检查引用是否为 null
    pub fn is_null(&self, obj: &AniRef<'_>) -> Result<bool> {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Reference_IsNull.unwrap())(self.raw, obj.as_raw(), &mut result);
            check_status(status)?;
            Ok(result != 0)
        }
    }

    /// 检查引用是否为 undefined
    pub fn is_undefined(&self, obj: &AniRef<'_>) -> Result<bool> {
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Reference_IsUndefined.unwrap())(self.raw, obj.as_raw(), &mut result);
            check_status(status)?;
            Ok(result != 0)
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
        let mut result: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status =
                (api.Object_InstanceOf.unwrap())(self.raw, obj.as_raw(), cls.as_raw(), &mut result);
            check_status(status)?;
            Ok(result != 0)
        }
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

        let mut result: sys::ani_ref = ptr::null_mut();
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_CallMethodByName_Ref.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
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
        let mut result: sys::ani_double = 0.0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_GetPropertyByName_Double.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                &mut result,
            );
            check_status(status)?;
            Ok(result)
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.Object_SetPropertyByName_Double.unwrap())(
                self.raw,
                obj.as_raw(),
                c_name.as_ptr(),
                value,
            );
            check_status(status)
        }
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
        let mut has_error: sys::ani_boolean = 0;
        unsafe {
            let api = &*(*self.raw);
            let status = (api.ExistUnhandledError.unwrap())(self.raw, &mut has_error);
            check_status(status)?;
        }
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
        unsafe {
            let api = &*(*self.raw);
            let status = (api.ResetError.unwrap())(self.raw);
            check_status(status)
        }
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
        unsafe {
            let api = &*(*self.raw);
            let _ = (api.Abort.unwrap())(self.raw, c_message.as_ptr());
        }
        // Abort should never return, but in case it does
        std::process::abort()
    }
}
