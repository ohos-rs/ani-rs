//! ANI Environment Wrapper
//!
//! Provides safe wrapper for ANI environment

use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::slice;

use crate::conversions::{Deferred, PromiseRaw};
use crate::error::{BusinessError, Error, Result, Status, check_status};
use crate::sys;
use crate::types::*;
use crate::vm::AniVm;

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
    ($env:expr) => {{ unsafe { &*(*$env) } }};
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

/// RAII guard for a local reference scope created by
/// [`Env::create_local_scope`].
///
/// When dropped, this guard automatically calls `DestroyLocalScope`.
pub struct LocalScopeGuard<'local> {
    raw_env: *mut sys::ani_env,
    active: bool,
    _marker: PhantomData<(&'local (), *const ())>,
}

impl<'local> LocalScopeGuard<'local> {
    #[inline]
    fn new(raw_env: *mut sys::ani_env) -> Self {
        Self {
            raw_env,
            active: true,
            _marker: PhantomData,
        }
    }

    /// Closes the local scope immediately.
    pub fn close(mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }

        let status = unsafe {
            let api = &*(*self.raw_env);
            (api.DestroyLocalScope.unwrap())(self.raw_env)
        };
        self.active = false;
        check_status(status)
    }
}

impl Drop for LocalScopeGuard<'_> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }

        let _ = unsafe {
            let api = &*(*self.raw_env);
            check_status((api.DestroyLocalScope.unwrap())(self.raw_env))
        };
        self.active = false;
    }
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

    /// Get VM handle associated with this environment
    pub fn get_vm(&self) -> Result<AniVm> {
        let raw_vm = ani_call_ret!(self, GetVM, *mut sys::ani_vm, ptr::null_mut())?;
        unsafe { AniVm::from_raw(raw_vm) }
    }

    #[inline]
    fn value_args_ptr(args: &[sys::ani_value]) -> *const sys::ani_value {
        if args.is_empty() {
            ptr::NonNull::<sys::ani_value>::dangling().as_ptr() as *const sys::ani_value
        } else {
            args.as_ptr()
        }
    }

    #[inline]
    fn ref_args_to_raw(args: &[AniRef<'_>]) -> Vec<sys::ani_ref> {
        args.iter().map(|arg| arg.as_raw()).collect()
    }

    // ========================================================================
    // Class and Module Operations
    // ========================================================================

    /// Find class
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Class name, e.g., "std.core.String"
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

    /// Find enum item by name.
    pub fn get_enum_item_by_name(
        &self,
        enm: &AniEnum<'_>,
        name: &str,
    ) -> Result<AniEnumItem<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid enum item name"))?;
        ani_call_wrap!(
            self,
            Enum_GetEnumItemByName,
            sys::ani_enum_item,
            AniEnumItem,
            enm.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Find enum item by index.
    pub fn get_enum_item_by_index(
        &self,
        enm: &AniEnum<'_>,
        index: usize,
    ) -> Result<AniEnumItem<'local>> {
        ani_call_wrap!(
            self,
            Enum_GetEnumItemByIndex,
            sys::ani_enum_item,
            AniEnumItem,
            enm.as_raw(),
            index
        )
    }

    /// Get enum object of enum item.
    pub fn get_enum_of_item(&self, item: &AniEnumItem<'_>) -> Result<AniEnum<'local>> {
        ani_call_wrap!(
            self,
            EnumItem_GetEnum,
            sys::ani_enum,
            AniEnum,
            item.as_raw()
        )
    }

    /// Get integer value of enum item.
    pub fn get_enum_item_value_int(&self, item: &AniEnumItem<'_>) -> Result<i32> {
        ani_call_ret!(self, EnumItem_GetValue_Int, sys::ani_int, 0, item.as_raw())
    }

    /// Get string value of enum item.
    pub fn get_enum_item_value_string(&self, item: &AniEnumItem<'_>) -> Result<AniString<'local>> {
        ani_call_wrap!(
            self,
            EnumItem_GetValue_String,
            sys::ani_string,
            AniString,
            item.as_raw()
        )
    }

    /// Get name of enum item.
    pub fn get_enum_item_name(&self, item: &AniEnumItem<'_>) -> Result<AniString<'local>> {
        ani_call_wrap!(
            self,
            EnumItem_GetName,
            sys::ani_string,
            AniString,
            item.as_raw()
        )
    }

    /// Get index of enum item.
    pub fn get_enum_item_index(&self, item: &AniEnumItem<'_>) -> Result<usize> {
        ani_call_ret!(self, EnumItem_GetIndex, sys::ani_size, 0, item.as_raw())
    }

    /// Find a function in a module by name and signature.
    pub fn find_module_function(
        &self,
        module: &AniModule<'_>,
        name: &str,
        signature: &str,
    ) -> Result<AniFunction> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid function name"))?;
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid function signature"))?;
        ani_call_wrap!(
            self,
            Module_FindFunction,
            sys::ani_function,
            AniFunction,
            module.as_raw(),
            c_name.as_ptr(),
            c_sig.as_ptr()
        )
    }

    /// Find a variable in a module by name.
    pub fn find_module_variable(&self, module: &AniModule<'_>, name: &str) -> Result<AniVariable> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid variable name"))?;
        ani_call_wrap!(
            self,
            Module_FindVariable,
            sys::ani_variable,
            AniVariable,
            module.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Find a function in a namespace by name and signature.
    pub fn find_namespace_function(
        &self,
        namespace: &AniNamespace<'_>,
        name: &str,
        signature: &str,
    ) -> Result<AniFunction> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid function name"))?;
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid function signature"))?;
        ani_call_wrap!(
            self,
            Namespace_FindFunction,
            sys::ani_function,
            AniFunction,
            namespace.as_raw(),
            c_name.as_ptr(),
            c_sig.as_ptr()
        )
    }

    /// Find a variable in a namespace by name.
    pub fn find_namespace_variable(
        &self,
        namespace: &AniNamespace<'_>,
        name: &str,
    ) -> Result<AniVariable> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid variable name"))?;
        ani_call_wrap!(
            self,
            Namespace_FindVariable,
            sys::ani_variable,
            AniVariable,
            namespace.as_raw(),
            c_name.as_ptr()
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

    /// Find class static field.
    pub fn find_static_field(&self, class: &AniClass<'_>, name: &str) -> Result<AniStaticField> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_wrap!(
            self,
            Class_FindStaticField,
            sys::ani_static_field,
            AniStaticField,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Find getter method by property name.
    pub fn find_getter(&self, class: &AniClass<'_>, name: &str) -> Result<AniMethod> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid getter name"))?;
        ani_call_wrap!(
            self,
            Class_FindGetter,
            sys::ani_method,
            AniMethod,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Find setter method by property name.
    pub fn find_setter(&self, class: &AniClass<'_>, name: &str) -> Result<AniMethod> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid setter name"))?;
        ani_call_wrap!(
            self,
            Class_FindSetter,
            sys::ani_method,
            AniMethod,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Find indexable getter method by signature.
    pub fn find_indexable_getter(
        &self,
        class: &AniClass<'_>,
        signature: &str,
    ) -> Result<AniMethod> {
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid indexable getter signature"))?;
        ani_call_wrap!(
            self,
            Class_FindIndexableGetter,
            sys::ani_method,
            AniMethod,
            class.as_raw(),
            c_sig.as_ptr()
        )
    }

    /// Find indexable setter method by signature.
    pub fn find_indexable_setter(
        &self,
        class: &AniClass<'_>,
        signature: &str,
    ) -> Result<AniMethod> {
        let c_sig = CString::new(signature)
            .map_err(|_| Error::new(Status::Error, "Invalid indexable setter signature"))?;
        ani_call_wrap!(
            self,
            Class_FindIndexableSetter,
            sys::ani_method,
            AniMethod,
            class.as_raw(),
            c_sig.as_ptr()
        )
    }

    /// Find class iterator method.
    pub fn find_iterator(&self, class: &AniClass<'_>) -> Result<AniMethod> {
        ani_call_wrap!(
            self,
            Class_FindIterator,
            sys::ani_method,
            AniMethod,
            class.as_raw()
        )
    }

    /// Get static `bool` field value.
    pub fn get_static_field_boolean(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Class_GetStaticField_Boolean,
            sys::ani_boolean,
            0,
            class.as_raw(),
            field.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Get static `char` field value.
    pub fn get_static_field_char(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<sys::ani_char> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Char,
            sys::ani_char,
            0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `i8` field value.
    pub fn get_static_field_byte(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<i8> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Byte,
            sys::ani_byte,
            0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `i16` field value.
    pub fn get_static_field_short(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<i16> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Short,
            sys::ani_short,
            0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `i32` field value.
    pub fn get_static_field_int(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<i32> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Int,
            sys::ani_int,
            0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `i64` field value.
    pub fn get_static_field_long(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<i64> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Long,
            sys::ani_long,
            0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `f32` field value.
    pub fn get_static_field_float(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<f32> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Float,
            sys::ani_float,
            0.0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static `f64` field value.
    pub fn get_static_field_double(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<f64> {
        ani_call_ret!(
            self,
            Class_GetStaticField_Double,
            sys::ani_double,
            0.0,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Get static reference field value.
    pub fn get_static_field_ref(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
    ) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            Class_GetStaticField_Ref,
            sys::ani_ref,
            AniRef,
            class.as_raw(),
            field.as_raw()
        )
    }

    /// Set static `bool` field value.
    pub fn set_static_field_boolean(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: bool,
    ) -> Result<()> {
        let value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            Class_SetStaticField_Boolean,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `char` field value.
    pub fn set_static_field_char(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: sys::ani_char,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Char,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `i8` field value.
    pub fn set_static_field_byte(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: i8,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Byte,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `i16` field value.
    pub fn set_static_field_short(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: i16,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Short,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `i32` field value.
    pub fn set_static_field_int(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: i32,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Int,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `i64` field value.
    pub fn set_static_field_long(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: i64,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Long,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `f32` field value.
    pub fn set_static_field_float(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: f32,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Float,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static `f64` field value.
    pub fn set_static_field_double(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: f64,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Double,
            class.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Set static reference field value.
    pub fn set_static_field_ref(
        &self,
        class: &AniClass<'_>,
        field: &AniStaticField,
        value: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(
            self,
            Class_SetStaticField_Ref,
            class.as_raw(),
            field.as_raw(),
            value.as_raw()
        )
    }

    /// Get static `bool` field value by field name.
    pub fn get_static_field_by_name_boolean(
        &self,
        class: &AniClass<'_>,
        name: &str,
    ) -> Result<bool> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        let value = ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Boolean,
            sys::ani_boolean,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )?;
        Ok(value != 0)
    }

    /// Get static `char` field value by field name.
    pub fn get_static_field_by_name_char(
        &self,
        class: &AniClass<'_>,
        name: &str,
    ) -> Result<sys::ani_char> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Char,
            sys::ani_char,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `i8` field value by field name.
    pub fn get_static_field_by_name_byte(&self, class: &AniClass<'_>, name: &str) -> Result<i8> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Byte,
            sys::ani_byte,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `i16` field value by field name.
    pub fn get_static_field_by_name_short(&self, class: &AniClass<'_>, name: &str) -> Result<i16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Short,
            sys::ani_short,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `i32` field value by field name.
    pub fn get_static_field_by_name_int(&self, class: &AniClass<'_>, name: &str) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Int,
            sys::ani_int,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `i64` field value by field name.
    pub fn get_static_field_by_name_long(&self, class: &AniClass<'_>, name: &str) -> Result<i64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Long,
            sys::ani_long,
            0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `f32` field value by field name.
    pub fn get_static_field_by_name_float(&self, class: &AniClass<'_>, name: &str) -> Result<f32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Float,
            sys::ani_float,
            0.0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static `f64` field value by field name.
    pub fn get_static_field_by_name_double(&self, class: &AniClass<'_>, name: &str) -> Result<f64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Class_GetStaticFieldByName_Double,
            sys::ani_double,
            0.0,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Get static reference field value by field name.
    pub fn get_static_field_by_name_ref(
        &self,
        class: &AniClass<'_>,
        name: &str,
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_wrap!(
            self,
            Class_GetStaticFieldByName_Ref,
            sys::ani_ref,
            AniRef,
            class.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set static `bool` field value by field name.
    pub fn set_static_field_by_name_boolean(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: bool,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Boolean,
            class.as_raw(),
            c_name.as_ptr(),
            if value { 1 } else { 0 }
        )
    }

    /// Set static `char` field value by field name.
    pub fn set_static_field_by_name_char(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: sys::ani_char,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Char,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `i8` field value by field name.
    pub fn set_static_field_by_name_byte(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: i8,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Byte,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `i16` field value by field name.
    pub fn set_static_field_by_name_short(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: i16,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Short,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `i32` field value by field name.
    pub fn set_static_field_by_name_int(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: i32,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Int,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `i64` field value by field name.
    pub fn set_static_field_by_name_long(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: i64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Long,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `f32` field value by field name.
    pub fn set_static_field_by_name_float(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: f32,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Float,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static `f64` field value by field name.
    pub fn set_static_field_by_name_double(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: f64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Double,
            class.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Set static reference field value by field name.
    pub fn set_static_field_by_name_ref(
        &self,
        class: &AniClass<'_>,
        name: &str,
        value: &AniRef<'_>,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Class_SetStaticFieldByName_Ref,
            class.as_raw(),
            c_name.as_ptr(),
            value.as_raw()
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

    /// Create ANI string from UTF-16 code units.
    pub fn create_string_utf16(&self, utf16: &[u16]) -> Result<AniString<'local>> {
        ani_call_wrap!(
            self,
            String_NewUTF16,
            sys::ani_string,
            AniString,
            utf16.as_ptr(),
            utf16.len()
        )
    }

    /// Get ANI string UTF-16 content.
    pub fn get_string_utf16(&self, string: &AniString<'_>) -> Result<Vec<u16>> {
        let size = ani_call_ret!(self, String_GetUTF16Size, sys::ani_size, 0, string.as_raw())?;
        let mut buffer = vec![0u16; size + 1];
        let written = ani_call_ret!(
            self,
            String_GetUTF16,
            sys::ani_size,
            0,
            string.as_raw(),
            buffer.as_mut_ptr(),
            buffer.len()
        )?;
        buffer.truncate(written);
        Ok(buffer)
    }

    /// Get UTF-16 substring by offset and size.
    pub fn get_string_utf16_substring(
        &self,
        string: &AniString<'_>,
        offset: usize,
        size: usize,
    ) -> Result<Vec<u16>> {
        let mut buffer = vec![0u16; size + 1];
        let written = ani_call_ret!(
            self,
            String_GetUTF16SubString,
            sys::ani_size,
            0,
            string.as_raw(),
            offset,
            size,
            buffer.as_mut_ptr(),
            buffer.len()
        )?;
        buffer.truncate(written);
        Ok(buffer)
    }

    /// Get UTF-8 substring by offset and size.
    pub fn get_string_utf8_substring(
        &self,
        string: &AniString<'_>,
        offset: usize,
        size: usize,
    ) -> Result<String> {
        let mut buffer = vec![0u8; size.saturating_mul(4).saturating_add(1)];
        let written = ani_call_ret!(
            self,
            String_GetUTF8SubString,
            sys::ani_size,
            0,
            string.as_raw(),
            offset,
            size,
            buffer.as_mut_ptr() as *mut i8,
            buffer.len()
        )?;
        buffer.truncate(written);
        String::from_utf8(buffer)
            .map_err(|e| Error::new(Status::Error, format!("Invalid UTF-8 substring: {}", e)))
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

    /// Call a method returning `char`.
    pub fn call_char_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<sys::ani_char> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Char_A,
            sys::ani_char,
            0,
            obj.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning `i8`.
    pub fn call_byte_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i8> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Byte_A,
            sys::ani_byte,
            0,
            obj.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning `i16`.
    pub fn call_short_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<i16> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Short_A,
            sys::ani_short,
            0,
            obj.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning `f32`.
    pub fn call_float_method(
        &self,
        obj: &AniObject<'_>,
        method: &AniMethod,
        args: &[sys::ani_value],
    ) -> Result<f32> {
        ani_call_method_ret!(
            self,
            Object_CallMethod_Float_A,
            sys::ani_float,
            0.0,
            obj.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
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

    /// Call a static method returning `bool`.
    pub fn call_static_method_boolean(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<bool> {
        let result = ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Boolean_A,
            sys::ani_boolean,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )?;
        Ok(result != 0)
    }

    /// Call a static method returning `char`.
    pub fn call_static_method_char(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<sys::ani_char> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Char_A,
            sys::ani_char,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `i8`.
    pub fn call_static_method_byte(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<i8> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Byte_A,
            sys::ani_byte,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `i16`.
    pub fn call_static_method_short(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<i16> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Short_A,
            sys::ani_short,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `i32`.
    pub fn call_static_method_int(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<i32> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Int_A,
            sys::ani_int,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `i64`.
    pub fn call_static_method_long(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<i64> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Long_A,
            sys::ani_long,
            0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `f32`.
    pub fn call_static_method_float(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<f32> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Float_A,
            sys::ani_float,
            0.0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning `f64`.
    pub fn call_static_method_double(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<f64> {
        ani_call_method_ret!(
            self,
            Class_CallStaticMethod_Double_A,
            sys::ani_double,
            0.0,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning reference value.
    pub fn call_static_method_ref(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        ani_call_method_wrap!(
            self,
            Class_CallStaticMethod_Ref_A,
            sys::ani_ref,
            AniRef,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method with `void` return type.
    pub fn call_static_method_void(
        &self,
        class: &AniClass<'_>,
        method: &AniStaticMethod,
        args: &[sys::ani_value],
    ) -> Result<()> {
        ani_call!(
            self,
            Class_CallStaticMethod_Void_A,
            class.as_raw(),
            method.as_raw(),
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning int by name with `ani_value` arguments.
    pub fn call_static_method_by_name_int_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<i32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Int_A,
            sys::ani_int,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning int by name.
    pub fn call_static_method_by_name_int(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i32> {
        self.call_static_method_by_name_int_with_args(class, name, signature, &[])
    }

    /// Call a static method returning long by name with `ani_value` arguments.
    pub fn call_static_method_by_name_long_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<i64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Long_A,
            sys::ani_long,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning long by name.
    pub fn call_static_method_by_name_long(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i64> {
        self.call_static_method_by_name_long_with_args(class, name, signature, &[])
    }

    /// Call a static method returning double by name with `ani_value` arguments.
    pub fn call_static_method_by_name_double_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<f64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Double_A,
            sys::ani_double,
            0.0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning double by name.
    pub fn call_static_method_by_name_double(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f64> {
        self.call_static_method_by_name_double_with_args(class, name, signature, &[])
    }

    /// Call a static method returning float by name with `ani_value` arguments.
    pub fn call_static_method_by_name_float_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<f32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Float_A,
            sys::ani_float,
            0.0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning float by name.
    pub fn call_static_method_by_name_float(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f32> {
        self.call_static_method_by_name_float_with_args(class, name, signature, &[])
    }

    /// Call a static method returning boolean by name with `ani_value` arguments.
    pub fn call_static_method_by_name_boolean_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<bool> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        let result = ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Boolean_A,
            sys::ani_boolean,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )?;
        Ok(result != 0)
    }

    /// Call a static method returning boolean by name.
    pub fn call_static_method_by_name_boolean(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<bool> {
        self.call_static_method_by_name_boolean_with_args(class, name, signature, &[])
    }

    /// Call a static method returning byte by name with `ani_value` arguments.
    pub fn call_static_method_by_name_byte_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<i8> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Byte_A,
            sys::ani_byte,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning byte by name.
    pub fn call_static_method_by_name_byte(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i8> {
        self.call_static_method_by_name_byte_with_args(class, name, signature, &[])
    }

    /// Call a static method returning short by name with `ani_value` arguments.
    pub fn call_static_method_by_name_short_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<i16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Short_A,
            sys::ani_short,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning short by name.
    pub fn call_static_method_by_name_short(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i16> {
        self.call_static_method_by_name_short_with_args(class, name, signature, &[])
    }

    /// Call a static method returning char by name with `ani_value` arguments.
    pub fn call_static_method_by_name_char_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<u16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call_by_name_ret!(
            self,
            Class_CallStaticMethodByName_Char_A,
            sys::ani_char,
            0,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method returning char by name.
    pub fn call_static_method_by_name_char(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<u16> {
        self.call_static_method_by_name_char_with_args(class, name, signature, &[])
    }

    /// Call a static method with `void` return type by name with `ani_value` arguments.
    pub fn call_static_method_by_name_void_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        ani_call!(
            self,
            Class_CallStaticMethodByName_Void_A,
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )
    }

    /// Call a static method with `void` return type by name.
    pub fn call_static_method_by_name_void(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<()> {
        self.call_static_method_by_name_void_with_args(class, name, signature, &[])
    }

    /// Call a static method returning reference value by name with `ani_value` arguments.
    pub fn call_static_method_by_name_ref_with_args(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        let result = ani_call_ret_mid!(
            self,
            Class_CallStaticMethodByName_Ref_A,
            sys::ani_ref,
            ptr::null_mut(),
            class.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )?;
        Ok(unsafe { AniRef::from_raw(result) })
    }

    /// Call a static method returning reference value by name.
    pub fn call_static_method_by_name_ref(
        &self,
        class: &AniClass<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<AniRef<'local>> {
        self.call_static_method_by_name_ref_with_args(class, name, signature, &[])
    }

    // ========================================================================
    // Call Method by Name (Simplified API)
    // ========================================================================

    /// Call a method returning int by name with `ani_value` arguments.
    pub fn call_method_by_name_int_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning int by name.
    pub fn call_method_by_name_int(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i32> {
        self.call_method_by_name_int_with_args(obj, name, signature, &[])
    }

    /// Call a method returning long by name with `ani_value` arguments.
    pub fn call_method_by_name_long_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning long by name.
    pub fn call_method_by_name_long(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i64> {
        self.call_method_by_name_long_with_args(obj, name, signature, &[])
    }

    /// Call a method returning double by name with `ani_value` arguments.
    pub fn call_method_by_name_double_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning double by name.
    pub fn call_method_by_name_double(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f64> {
        self.call_method_by_name_double_with_args(obj, name, signature, &[])
    }

    /// Call a method returning float by name with `ani_value` arguments.
    pub fn call_method_by_name_float_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning float by name.
    pub fn call_method_by_name_float(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<f32> {
        self.call_method_by_name_float_with_args(obj, name, signature, &[])
    }

    /// Call a method returning boolean by name with `ani_value` arguments.
    pub fn call_method_by_name_boolean_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )?;
        Ok(result != 0)
    }

    /// Call a method returning boolean by name.
    pub fn call_method_by_name_boolean(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<bool> {
        self.call_method_by_name_boolean_with_args(obj, name, signature, &[])
    }

    /// Call a method returning byte by name with `ani_value` arguments.
    pub fn call_method_by_name_byte_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning byte by name.
    pub fn call_method_by_name_byte(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i8> {
        self.call_method_by_name_byte_with_args(obj, name, signature, &[])
    }

    /// Call a method returning short by name with `ani_value` arguments.
    pub fn call_method_by_name_short_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning short by name.
    pub fn call_method_by_name_short(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<i16> {
        self.call_method_by_name_short_with_args(obj, name, signature, &[])
    }

    /// Call a method returning char by name with `ani_value` arguments.
    pub fn call_method_by_name_char_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a method returning char by name.
    pub fn call_method_by_name_char(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<u16> {
        self.call_method_by_name_char_with_args(obj, name, signature, &[])
    }

    /// Call a void method by name with `ani_value` arguments.
    pub fn call_method_by_name_void_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
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
            Self::value_args_ptr(args)
        )
    }

    /// Call a void method by name.
    pub fn call_method_by_name_void(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<()> {
        self.call_method_by_name_void_with_args(obj, name, signature, &[])
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
        ani_call_ret!(
            self,
            Object_GetField_Int,
            sys::ani_int,
            0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set int type field value
    pub fn set_field_int(&self, obj: &AniObject<'_>, field: &AniField, value: i32) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Int,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get boolean type field value.
    pub fn get_field_boolean(&self, obj: &AniObject<'_>, field: &AniField) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Object_GetField_Boolean,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            field.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Set boolean type field value.
    pub fn set_field_boolean(
        &self,
        obj: &AniObject<'_>,
        field: &AniField,
        value: bool,
    ) -> Result<()> {
        let value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            Object_SetField_Boolean,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get char type field value.
    pub fn get_field_char(&self, obj: &AniObject<'_>, field: &AniField) -> Result<sys::ani_char> {
        ani_call_ret!(
            self,
            Object_GetField_Char,
            sys::ani_char,
            0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set char type field value.
    pub fn set_field_char(
        &self,
        obj: &AniObject<'_>,
        field: &AniField,
        value: sys::ani_char,
    ) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Char,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get byte type field value.
    pub fn get_field_byte(&self, obj: &AniObject<'_>, field: &AniField) -> Result<i8> {
        ani_call_ret!(
            self,
            Object_GetField_Byte,
            sys::ani_byte,
            0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set byte type field value.
    pub fn set_field_byte(&self, obj: &AniObject<'_>, field: &AniField, value: i8) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Byte,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get short type field value.
    pub fn get_field_short(&self, obj: &AniObject<'_>, field: &AniField) -> Result<i16> {
        ani_call_ret!(
            self,
            Object_GetField_Short,
            sys::ani_short,
            0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set short type field value.
    pub fn set_field_short(&self, obj: &AniObject<'_>, field: &AniField, value: i16) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Short,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get long type field value.
    pub fn get_field_long(&self, obj: &AniObject<'_>, field: &AniField) -> Result<i64> {
        ani_call_ret!(
            self,
            Object_GetField_Long,
            sys::ani_long,
            0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set long type field value.
    pub fn set_field_long(&self, obj: &AniObject<'_>, field: &AniField, value: i64) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Long,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get float type field value.
    pub fn get_field_float(&self, obj: &AniObject<'_>, field: &AniField) -> Result<f32> {
        ani_call_ret!(
            self,
            Object_GetField_Float,
            sys::ani_float,
            0.0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set float type field value.
    pub fn set_field_float(&self, obj: &AniObject<'_>, field: &AniField, value: f32) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Float,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get double type field value.
    pub fn get_field_double(&self, obj: &AniObject<'_>, field: &AniField) -> Result<f64> {
        ani_call_ret!(
            self,
            Object_GetField_Double,
            sys::ani_double,
            0.0,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set double type field value.
    pub fn set_field_double(
        &self,
        obj: &AniObject<'_>,
        field: &AniField,
        value: f64,
    ) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Double,
            obj.as_raw(),
            field.as_raw(),
            value
        )
    }

    /// Get reference type field value.
    pub fn get_field_ref(&self, obj: &AniObject<'_>, field: &AniField) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            Object_GetField_Ref,
            sys::ani_ref,
            AniRef,
            obj.as_raw(),
            field.as_raw()
        )
    }

    /// Set reference type field value.
    pub fn set_field_ref(
        &self,
        obj: &AniObject<'_>,
        field: &AniField,
        value: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(
            self,
            Object_SetField_Ref,
            obj.as_raw(),
            field.as_raw(),
            value.as_raw()
        )
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
        ani_call!(
            self,
            Object_SetFieldByName_Int,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get boolean type field value by name.
    pub fn get_field_by_name_boolean(&self, obj: &AniObject<'_>, name: &str) -> Result<bool> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        let result = ani_call_ret!(
            self,
            Object_GetFieldByName_Boolean,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )?;
        Ok(result != 0)
    }

    /// Set boolean type field value by name.
    pub fn set_field_by_name_boolean(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: bool,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        let value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            Object_SetFieldByName_Boolean,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get char type field value by name.
    pub fn get_field_by_name_char(&self, obj: &AniObject<'_>, name: &str) -> Result<sys::ani_char> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Char,
            sys::ani_char,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set char type field value by name.
    pub fn set_field_by_name_char(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: sys::ani_char,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Char,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get byte type field value by name.
    pub fn get_field_by_name_byte(&self, obj: &AniObject<'_>, name: &str) -> Result<i8> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Byte,
            sys::ani_byte,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set byte type field value by name.
    pub fn set_field_by_name_byte(&self, obj: &AniObject<'_>, name: &str, value: i8) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Byte,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get short type field value by name.
    pub fn get_field_by_name_short(&self, obj: &AniObject<'_>, name: &str) -> Result<i16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Short,
            sys::ani_short,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set short type field value by name.
    pub fn set_field_by_name_short(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i16,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Short,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get long type field value by name.
    pub fn get_field_by_name_long(&self, obj: &AniObject<'_>, name: &str) -> Result<i64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Long,
            sys::ani_long,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set long type field value by name.
    pub fn set_field_by_name_long(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Long,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get float type field value by name.
    pub fn get_field_by_name_float(&self, obj: &AniObject<'_>, name: &str) -> Result<f32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Float,
            sys::ani_float,
            0.0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set float type field value by name.
    pub fn set_field_by_name_float(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: f32,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Float,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get double type field value by name.
    pub fn get_field_by_name_double(&self, obj: &AniObject<'_>, name: &str) -> Result<f64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_ret!(
            self,
            Object_GetFieldByName_Double,
            sys::ani_double,
            0.0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set double type field value by name.
    pub fn set_field_by_name_double(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: f64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Double,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get reference type field value by name.
    pub fn get_field_by_name_ref(&self, obj: &AniObject<'_>, name: &str) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call_wrap!(
            self,
            Object_GetFieldByName_Ref,
            sys::ani_ref,
            AniRef,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set reference type field value by name.
    pub fn set_field_by_name_ref(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: &AniRef<'_>,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid field name"))?;
        ani_call!(
            self,
            Object_SetFieldByName_Ref,
            obj.as_raw(),
            c_name.as_ptr(),
            value.as_raw()
        )
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

    /// Get boolean type property value by name.
    pub fn get_property_by_name_boolean(&self, obj: &AniObject<'_>, name: &str) -> Result<bool> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        let result = ani_call_ret!(
            self,
            Object_GetPropertyByName_Boolean,
            sys::ani_boolean,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )?;
        Ok(result != 0)
    }

    /// Set boolean type property value by name.
    pub fn set_property_by_name_boolean(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: bool,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        let value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            Object_SetPropertyByName_Boolean,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get char type property value by name.
    pub fn get_property_by_name_char(
        &self,
        obj: &AniObject<'_>,
        name: &str,
    ) -> Result<sys::ani_char> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Char,
            sys::ani_char,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set char type property value by name.
    pub fn set_property_by_name_char(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: sys::ani_char,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Char,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get byte type property value by name.
    pub fn get_property_by_name_byte(&self, obj: &AniObject<'_>, name: &str) -> Result<i8> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Byte,
            sys::ani_byte,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set byte type property value by name.
    pub fn set_property_by_name_byte(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i8,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Byte,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get short type property value by name.
    pub fn get_property_by_name_short(&self, obj: &AniObject<'_>, name: &str) -> Result<i16> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Short,
            sys::ani_short,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set short type property value by name.
    pub fn set_property_by_name_short(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i16,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Short,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get long type property value by name.
    pub fn get_property_by_name_long(&self, obj: &AniObject<'_>, name: &str) -> Result<i64> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Long,
            sys::ani_long,
            0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set long type property value by name.
    pub fn set_property_by_name_long(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: i64,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Long,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get float type property value by name.
    pub fn get_property_by_name_float(&self, obj: &AniObject<'_>, name: &str) -> Result<f32> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_ret!(
            self,
            Object_GetPropertyByName_Float,
            sys::ani_float,
            0.0,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set float type property value by name.
    pub fn set_property_by_name_float(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: f32,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Float,
            obj.as_raw(),
            c_name.as_ptr(),
            value
        )
    }

    /// Get reference type property value by name.
    pub fn get_property_by_name_ref(
        &self,
        obj: &AniObject<'_>,
        name: &str,
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_wrap!(
            self,
            Object_GetPropertyByName_Ref,
            sys::ani_ref,
            AniRef,
            obj.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set reference type property value by name.
    pub fn set_property_by_name_ref(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        value: &AniRef<'_>,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Object_SetPropertyByName_Ref,
            obj.as_raw(),
            c_name.as_ptr(),
            value.as_raw()
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

    /// Bind static native methods to a class.
    pub fn bind_class_static_native_methods(
        &self,
        class: &AniClass<'_>,
        methods: &[sys::ani_native_function],
    ) -> Result<()> {
        ani_call!(
            self,
            Class_BindStaticNativeMethods,
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
    // Function and Variable Operations
    // ========================================================================

    /// Call a module/namespace function and return `bool`.
    pub fn call_function_boolean(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<bool> {
        let mut result: sys::ani_boolean = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Boolean_A.unwrap())(
                self.raw,
                function.as_raw(),
                &mut result,
                args_ptr,
            )
        };
        check_status(status)?;
        Ok(result != 0)
    }

    /// Call a module/namespace function and return `ani_char`.
    pub fn call_function_char(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<sys::ani_char> {
        let mut result: sys::ani_char = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Char_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `i8`.
    pub fn call_function_byte(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<i8> {
        let mut result: sys::ani_byte = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Byte_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `i16`.
    pub fn call_function_short(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<i16> {
        let mut result: sys::ani_short = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Short_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `i32`.
    pub fn call_function_int(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<i32> {
        let mut result: sys::ani_int = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Int_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `i64`.
    pub fn call_function_long(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<i64> {
        let mut result: sys::ani_long = 0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Long_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `f32`.
    pub fn call_function_float(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<f32> {
        let mut result: sys::ani_float = 0.0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Float_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return `f64`.
    pub fn call_function_double(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<f64> {
        let mut result: sys::ani_double = 0.0;
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Double_A.unwrap())(
                self.raw,
                function.as_raw(),
                &mut result,
                args_ptr,
            )
        };
        check_status(status)?;
        Ok(result)
    }

    /// Call a module/namespace function and return reference value.
    pub fn call_function_ref(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        let mut result: sys::ani_ref = ptr::null_mut();
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Ref_A.unwrap())(self.raw, function.as_raw(), &mut result, args_ptr)
        };
        check_status(status)?;
        Ok(unsafe { AniRef::from_raw(result) })
    }

    /// Call a module/namespace function with `void` return.
    pub fn call_function_void(
        &self,
        function: &AniFunction,
        args: &[sys::ani_value],
    ) -> Result<()> {
        let args_ptr = Self::value_args_ptr(args);
        let status = unsafe {
            let api = &*(*self.raw);
            (api.Function_Call_Void_A.unwrap())(self.raw, function.as_raw(), args_ptr)
        };
        check_status(status)
    }

    /// Set `bool` value to a variable.
    pub fn set_variable_boolean(&self, variable: &AniVariable, value: bool) -> Result<()> {
        let raw_value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            Variable_SetValue_Boolean,
            variable.as_raw(),
            raw_value
        )
    }

    /// Set `ani_char` value to a variable.
    pub fn set_variable_char(&self, variable: &AniVariable, value: sys::ani_char) -> Result<()> {
        ani_call!(self, Variable_SetValue_Char, variable.as_raw(), value)
    }

    /// Set `i8` value to a variable.
    pub fn set_variable_byte(&self, variable: &AniVariable, value: i8) -> Result<()> {
        ani_call!(self, Variable_SetValue_Byte, variable.as_raw(), value)
    }

    /// Set `i16` value to a variable.
    pub fn set_variable_short(&self, variable: &AniVariable, value: i16) -> Result<()> {
        ani_call!(self, Variable_SetValue_Short, variable.as_raw(), value)
    }

    /// Set `i32` value to a variable.
    pub fn set_variable_int(&self, variable: &AniVariable, value: i32) -> Result<()> {
        ani_call!(self, Variable_SetValue_Int, variable.as_raw(), value)
    }

    /// Set `i64` value to a variable.
    pub fn set_variable_long(&self, variable: &AniVariable, value: i64) -> Result<()> {
        ani_call!(self, Variable_SetValue_Long, variable.as_raw(), value)
    }

    /// Set `f32` value to a variable.
    pub fn set_variable_float(&self, variable: &AniVariable, value: f32) -> Result<()> {
        ani_call!(self, Variable_SetValue_Float, variable.as_raw(), value)
    }

    /// Set `f64` value to a variable.
    pub fn set_variable_double(&self, variable: &AniVariable, value: f64) -> Result<()> {
        ani_call!(self, Variable_SetValue_Double, variable.as_raw(), value)
    }

    /// Set reference value to a variable.
    pub fn set_variable_ref(&self, variable: &AniVariable, value: &AniRef<'_>) -> Result<()> {
        ani_call!(
            self,
            Variable_SetValue_Ref,
            variable.as_raw(),
            value.as_raw()
        )
    }

    /// Get `bool` value from a variable.
    pub fn get_variable_boolean(&self, variable: &AniVariable) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Variable_GetValue_Boolean,
            sys::ani_boolean,
            0,
            variable.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Get `ani_char` value from a variable.
    pub fn get_variable_char(&self, variable: &AniVariable) -> Result<sys::ani_char> {
        ani_call_ret!(
            self,
            Variable_GetValue_Char,
            sys::ani_char,
            0,
            variable.as_raw()
        )
    }

    /// Get `i8` value from a variable.
    pub fn get_variable_byte(&self, variable: &AniVariable) -> Result<i8> {
        ani_call_ret!(
            self,
            Variable_GetValue_Byte,
            sys::ani_byte,
            0,
            variable.as_raw()
        )
    }

    /// Get `i16` value from a variable.
    pub fn get_variable_short(&self, variable: &AniVariable) -> Result<i16> {
        ani_call_ret!(
            self,
            Variable_GetValue_Short,
            sys::ani_short,
            0,
            variable.as_raw()
        )
    }

    /// Get `i32` value from a variable.
    pub fn get_variable_int(&self, variable: &AniVariable) -> Result<i32> {
        ani_call_ret!(
            self,
            Variable_GetValue_Int,
            sys::ani_int,
            0,
            variable.as_raw()
        )
    }

    /// Get `i64` value from a variable.
    pub fn get_variable_long(&self, variable: &AniVariable) -> Result<i64> {
        ani_call_ret!(
            self,
            Variable_GetValue_Long,
            sys::ani_long,
            0,
            variable.as_raw()
        )
    }

    /// Get `f32` value from a variable.
    pub fn get_variable_float(&self, variable: &AniVariable) -> Result<f32> {
        ani_call_ret!(
            self,
            Variable_GetValue_Float,
            sys::ani_float,
            0.0,
            variable.as_raw()
        )
    }

    /// Get `f64` value from a variable.
    pub fn get_variable_double(&self, variable: &AniVariable) -> Result<f64> {
        ani_call_ret!(
            self,
            Variable_GetValue_Double,
            sys::ani_double,
            0.0,
            variable.as_raw()
        )
    }

    /// Get reference value from a variable.
    pub fn get_variable_ref(&self, variable: &AniVariable) -> Result<AniRef<'local>> {
        let result = ani_call_ret!(
            self,
            Variable_GetValue_Ref,
            sys::ani_ref,
            ptr::null_mut(),
            variable.as_raw()
        )?;
        Ok(unsafe { AniRef::from_raw(result) })
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

    /// Delete a local reference explicitly.
    pub fn delete_local_ref(&self, local_ref: &AniRef<'_>) -> Result<()> {
        ani_call!(self, Reference_Delete, local_ref.as_raw())
    }

    /// Ensure enough local reference slots are available.
    pub fn ensure_enough_references(&self, nr_refs: usize) -> Result<()> {
        ani_call!(self, EnsureEnoughReferences, nr_refs)
    }

    /// Create a local reference scope and return a RAII guard.
    pub fn create_local_scope(&self, nr_refs: usize) -> Result<LocalScopeGuard<'local>> {
        ani_call!(self, CreateLocalScope, nr_refs)?;
        Ok(LocalScopeGuard::new(self.raw))
    }

    /// Create an escape local scope.
    pub fn create_escape_local_scope(&self, nr_refs: usize) -> Result<()> {
        ani_call!(self, CreateEscapeLocalScope, nr_refs)
    }

    /// Destroy an escape local scope and return the escaped reference.
    pub fn destroy_escape_local_scope(&self, reference: &AniRef<'_>) -> Result<AniRef<'local>> {
        let escaped = ani_call_ret!(
            self,
            DestroyEscapeLocalScope,
            sys::ani_ref,
            ptr::null_mut(),
            reference.as_raw()
        )?;
        Ok(unsafe { AniRef::from_raw(escaped) })
    }

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

    /// Materialize a thread-local reference from a global reference.
    ///
    /// ANI does not currently expose a dedicated `GlobalReference_GetReference`
    /// API, so we bridge through a temporary weak reference on the current
    /// thread and immediately upgrade it back to a local reference.
    pub fn local_ref_from_global_ref(&self, gref: &GlobalRef) -> Result<AniRef<'local>> {
        let source = unsafe { AniRef::from_raw(gref.as_raw()) };
        let weak = self.create_weak_ref(&source)?;
        let upgraded = self.upgrade_weak_ref(&weak);
        let delete_result = self.delete_weak_ref(weak);

        match (upgraded, delete_result) {
            (Ok(Some(local)), Ok(())) => Ok(local),
            (Ok(None), Ok(())) => Err(Error::new(
                Status::NotFound,
                "Global reference target was released",
            )),
            (Ok(_), Err(err)) => Err(err),
            (Err(err), _) => Err(err),
        }
    }

    /// Materialize a thread-local object from a global reference.
    pub fn local_object_from_global_ref(&self, gref: &GlobalRef) -> Result<AniObject<'local>> {
        let local = self.local_ref_from_global_ref(gref)?;
        Ok(unsafe { AniObject::from_raw(local.as_raw() as sys::ani_object) })
    }

    /// Materialize a thread-local class from a global reference.
    pub fn local_class_from_global_ref(&self, gref: &GlobalRef) -> Result<AniClass<'local>> {
        let local = self.local_ref_from_global_ref(gref)?;
        Ok(unsafe { AniClass::from_raw(local.as_raw() as sys::ani_class) })
    }

    /// Create a weak reference from a local/global reference.
    pub fn create_weak_ref<'a>(&self, obj: &AniRef<'a>) -> Result<WeakRef> {
        ani_call_wrap!(
            self,
            WeakReference_Create,
            sys::ani_wref,
            WeakRef,
            obj.as_raw()
        )
    }

    /// Delete a weak reference.
    pub fn delete_weak_ref(&self, wref: WeakRef) -> Result<()> {
        ani_call!(self, WeakReference_Delete, wref.as_raw())
    }

    /// Upgrade a weak reference.
    ///
    /// Returns `Ok(None)` when the referenced object has been released.
    pub fn upgrade_weak_ref(&self, wref: &WeakRef) -> Result<Option<AniRef<'local>>> {
        let (released, value) = ani_call_2ret!(
            self,
            WeakReference_GetReference,
            sys::ani_boolean,
            sys::ani_ref,
            0,
            ptr::null_mut(),
            wref.as_raw()
        )?;

        if released != 0 || value.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { AniRef::from_raw(value) }))
        }
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

    /// Check if the reference is nullish (`null` or `undefined`).
    pub fn is_nullish(&self, obj: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Reference_IsNullishValue,
            sys::ani_boolean,
            0,
            obj.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Compare two references using ANI equality semantics.
    pub fn reference_equals(&self, lhs: &AniRef<'_>, rhs: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Reference_Equals,
            sys::ani_boolean,
            0,
            lhs.as_raw(),
            rhs.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Compare two references using strict equality semantics.
    pub fn reference_strict_equals(&self, lhs: &AniRef<'_>, rhs: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Reference_StrictEquals,
            sys::ani_boolean,
            0,
            lhs.as_raw(),
            rhs.as_raw()
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

    /// Create a generic reference array with an optional initial element.
    pub fn create_array(
        &self,
        length: usize,
        initial_element: Option<&AniRef<'_>>,
    ) -> Result<AniArray<'local>> {
        let initial = initial_element
            .map(|r| r.as_raw())
            .unwrap_or(ptr::null_mut());
        ani_call_wrap!(self, Array_New, sys::ani_array, AniArray, length, initial)
    }

    /// Set array element at `index`.
    pub fn set_array_element(
        &self,
        array: &AniArray<'_>,
        index: usize,
        value: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(self, Array_Set, array.as_raw(), index, value.as_raw())
    }

    /// Get array element at `index`.
    pub fn get_array_element(&self, array: &AniArray<'_>, index: usize) -> Result<AniRef<'local>> {
        let element = ani_call_ret!(
            self,
            Array_Get,
            sys::ani_ref,
            ptr::null_mut(),
            array.as_raw(),
            index
        )?;
        Ok(unsafe { AniRef::from_raw(element) })
    }

    /// Push an element to the end of the array.
    pub fn push_array_element(&self, array: &AniArray<'_>, value: &AniRef<'_>) -> Result<()> {
        ani_call!(self, Array_Push, array.as_raw(), value.as_raw())
    }

    /// Pop an element from the end of the array.
    pub fn pop_array_element(&self, array: &AniArray<'_>) -> Result<AniRef<'local>> {
        let element = ani_call_ret!(
            self,
            Array_Pop,
            sys::ani_ref,
            ptr::null_mut(),
            array.as_raw()
        )?;
        Ok(unsafe { AniRef::from_raw(element) })
    }

    /// Get fixed array length.
    pub fn get_fixed_array_length(&self, array: &AniFixedArray<'_>) -> Result<usize> {
        ani_call_ret!(self, FixedArray_GetLength, sys::ani_size, 0, array.as_raw())
    }

    /// Create fixed boolean array.
    pub fn create_fixed_array_boolean(
        &self,
        length: usize,
    ) -> Result<AniFixedArrayBoolean<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Boolean,
            sys::ani_fixedarray_boolean,
            AniFixedArrayBoolean,
            length
        )
    }

    /// Create fixed char array.
    pub fn create_fixed_array_char(&self, length: usize) -> Result<AniFixedArrayChar<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Char,
            sys::ani_fixedarray_char,
            AniFixedArrayChar,
            length
        )
    }

    /// Create fixed byte array.
    pub fn create_fixed_array_byte(&self, length: usize) -> Result<AniFixedArrayByte<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Byte,
            sys::ani_fixedarray_byte,
            AniFixedArrayByte,
            length
        )
    }

    /// Create fixed short array.
    pub fn create_fixed_array_short(&self, length: usize) -> Result<AniFixedArrayShort<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Short,
            sys::ani_fixedarray_short,
            AniFixedArrayShort,
            length
        )
    }

    /// Create fixed int array.
    pub fn create_fixed_array_int(&self, length: usize) -> Result<AniFixedArrayInt<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Int,
            sys::ani_fixedarray_int,
            AniFixedArrayInt,
            length
        )
    }

    /// Create fixed long array.
    pub fn create_fixed_array_long(&self, length: usize) -> Result<AniFixedArrayLong<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Long,
            sys::ani_fixedarray_long,
            AniFixedArrayLong,
            length
        )
    }

    /// Create fixed float array.
    pub fn create_fixed_array_float(&self, length: usize) -> Result<AniFixedArrayFloat<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Float,
            sys::ani_fixedarray_float,
            AniFixedArrayFloat,
            length
        )
    }

    /// Create fixed double array.
    pub fn create_fixed_array_double(&self, length: usize) -> Result<AniFixedArrayDouble<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Double,
            sys::ani_fixedarray_double,
            AniFixedArrayDouble,
            length
        )
    }

    /// Get boolean region from fixed array.
    pub fn get_fixed_array_region_boolean(
        &self,
        array: &AniFixedArrayBoolean<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<bool>> {
        let mut raw = vec![0 as sys::ani_boolean; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Boolean,
            array.as_raw(),
            offset,
            length,
            raw.as_mut_ptr()
        )?;
        Ok(raw.into_iter().map(|v| v != 0).collect())
    }

    /// Set boolean region to fixed array.
    pub fn set_fixed_array_region_boolean(
        &self,
        array: &AniFixedArrayBoolean<'_>,
        offset: usize,
        values: &[bool],
    ) -> Result<()> {
        let raw: Vec<sys::ani_boolean> = values.iter().map(|v| if *v { 1 } else { 0 }).collect();
        ani_call!(
            self,
            FixedArray_SetRegion_Boolean,
            array.as_raw(),
            offset,
            raw.len(),
            raw.as_ptr()
        )
    }

    /// Get char region from fixed array.
    pub fn get_fixed_array_region_char(
        &self,
        array: &AniFixedArrayChar<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<sys::ani_char>> {
        let mut buffer = vec![0 as sys::ani_char; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Char,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set char region to fixed array.
    pub fn set_fixed_array_region_char(
        &self,
        array: &AniFixedArrayChar<'_>,
        offset: usize,
        values: &[sys::ani_char],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Char,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get byte region from fixed array.
    pub fn get_fixed_array_region_byte(
        &self,
        array: &AniFixedArrayByte<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<i8>> {
        let mut buffer = vec![0_i8; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Byte,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set byte region to fixed array.
    pub fn set_fixed_array_region_byte(
        &self,
        array: &AniFixedArrayByte<'_>,
        offset: usize,
        values: &[i8],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Byte,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get short region from fixed array.
    pub fn get_fixed_array_region_short(
        &self,
        array: &AniFixedArrayShort<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<i16>> {
        let mut buffer = vec![0_i16; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Short,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set short region to fixed array.
    pub fn set_fixed_array_region_short(
        &self,
        array: &AniFixedArrayShort<'_>,
        offset: usize,
        values: &[i16],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Short,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get int region from fixed array.
    pub fn get_fixed_array_region_int(
        &self,
        array: &AniFixedArrayInt<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<i32>> {
        let mut buffer = vec![0_i32; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Int,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set int region to fixed array.
    pub fn set_fixed_array_region_int(
        &self,
        array: &AniFixedArrayInt<'_>,
        offset: usize,
        values: &[i32],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Int,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get long region from fixed array.
    pub fn get_fixed_array_region_long(
        &self,
        array: &AniFixedArrayLong<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<i64>> {
        let mut buffer = vec![0_i64; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Long,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set long region to fixed array.
    pub fn set_fixed_array_region_long(
        &self,
        array: &AniFixedArrayLong<'_>,
        offset: usize,
        values: &[i64],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Long,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get float region from fixed array.
    pub fn get_fixed_array_region_float(
        &self,
        array: &AniFixedArrayFloat<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<f32>> {
        let mut buffer = vec![0_f32; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Float,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set float region to fixed array.
    pub fn set_fixed_array_region_float(
        &self,
        array: &AniFixedArrayFloat<'_>,
        offset: usize,
        values: &[f32],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Float,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Get double region from fixed array.
    pub fn get_fixed_array_region_double(
        &self,
        array: &AniFixedArrayDouble<'_>,
        offset: usize,
        length: usize,
    ) -> Result<Vec<f64>> {
        let mut buffer = vec![0_f64; length];
        ani_call!(
            self,
            FixedArray_GetRegion_Double,
            array.as_raw(),
            offset,
            length,
            buffer.as_mut_ptr()
        )?;
        Ok(buffer)
    }

    /// Set double region to fixed array.
    pub fn set_fixed_array_region_double(
        &self,
        array: &AniFixedArrayDouble<'_>,
        offset: usize,
        values: &[f64],
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_SetRegion_Double,
            array.as_raw(),
            offset,
            values.len(),
            values.as_ptr()
        )
    }

    /// Create fixed reference array.
    pub fn create_fixed_array_ref(
        &self,
        element_type: &AniType<'_>,
        length: usize,
        initial_element: Option<&AniRef<'_>>,
    ) -> Result<AniFixedArrayRef<'local>> {
        let initial = initial_element
            .map(|r| r.as_raw())
            .unwrap_or(ptr::null_mut());
        ani_call_wrap!(
            self,
            FixedArray_New_Ref,
            sys::ani_fixedarray_ref,
            AniFixedArrayRef,
            element_type.as_raw(),
            length,
            initial
        )
    }

    /// Set reference element of fixed reference array.
    pub fn set_fixed_array_ref(
        &self,
        array: &AniFixedArrayRef<'_>,
        index: usize,
        value: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(
            self,
            FixedArray_Set_Ref,
            array.as_raw(),
            index,
            value.as_raw()
        )
    }

    /// Get reference element of fixed reference array.
    pub fn get_fixed_array_ref(
        &self,
        array: &AniFixedArrayRef<'_>,
        index: usize,
    ) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_Get_Ref,
            sys::ani_ref,
            AniRef,
            array.as_raw(),
            index
        )
    }

    /// Get tuple item count.
    pub fn get_tuple_number_of_items(&self, tuple: &AniTupleValue<'_>) -> Result<usize> {
        ani_call_ret!(
            self,
            TupleValue_GetNumberOfItems,
            sys::ani_size,
            0,
            tuple.as_raw()
        )
    }

    /// Get tuple boolean item.
    pub fn get_tuple_item_boolean(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<bool> {
        let value = ani_call_ret!(
            self,
            TupleValue_GetItem_Boolean,
            sys::ani_boolean,
            0,
            tuple.as_raw(),
            index
        )?;
        Ok(value != 0)
    }

    /// Get tuple char item.
    pub fn get_tuple_item_char(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
    ) -> Result<sys::ani_char> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Char,
            sys::ani_char,
            0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple byte item.
    pub fn get_tuple_item_byte(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<i8> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Byte,
            sys::ani_byte,
            0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple short item.
    pub fn get_tuple_item_short(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<i16> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Short,
            sys::ani_short,
            0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple int item.
    pub fn get_tuple_item_int(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<i32> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Int,
            sys::ani_int,
            0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple long item.
    pub fn get_tuple_item_long(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<i64> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Long,
            sys::ani_long,
            0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple float item.
    pub fn get_tuple_item_float(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<f32> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Float,
            sys::ani_float,
            0.0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple double item.
    pub fn get_tuple_item_double(&self, tuple: &AniTupleValue<'_>, index: usize) -> Result<f64> {
        ani_call_ret!(
            self,
            TupleValue_GetItem_Double,
            sys::ani_double,
            0.0,
            tuple.as_raw(),
            index
        )
    }

    /// Get tuple reference item.
    pub fn get_tuple_item_ref(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
    ) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            TupleValue_GetItem_Ref,
            sys::ani_ref,
            AniRef,
            tuple.as_raw(),
            index
        )
    }

    /// Set tuple boolean item.
    pub fn set_tuple_item_boolean(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: bool,
    ) -> Result<()> {
        let value: sys::ani_boolean = if value { 1 } else { 0 };
        ani_call!(
            self,
            TupleValue_SetItem_Boolean,
            tuple.as_raw(),
            index,
            value
        )
    }

    /// Set tuple char item.
    pub fn set_tuple_item_char(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: sys::ani_char,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Char, tuple.as_raw(), index, value)
    }

    /// Set tuple byte item.
    pub fn set_tuple_item_byte(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: i8,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Byte, tuple.as_raw(), index, value)
    }

    /// Set tuple short item.
    pub fn set_tuple_item_short(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: i16,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Short, tuple.as_raw(), index, value)
    }

    /// Set tuple int item.
    pub fn set_tuple_item_int(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: i32,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Int, tuple.as_raw(), index, value)
    }

    /// Set tuple long item.
    pub fn set_tuple_item_long(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: i64,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Long, tuple.as_raw(), index, value)
    }

    /// Set tuple float item.
    pub fn set_tuple_item_float(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: f32,
    ) -> Result<()> {
        ani_call!(self, TupleValue_SetItem_Float, tuple.as_raw(), index, value)
    }

    /// Set tuple double item.
    pub fn set_tuple_item_double(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: f64,
    ) -> Result<()> {
        ani_call!(
            self,
            TupleValue_SetItem_Double,
            tuple.as_raw(),
            index,
            value
        )
    }

    /// Set tuple reference item.
    pub fn set_tuple_item_ref(
        &self,
        tuple: &AniTupleValue<'_>,
        index: usize,
        value: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(
            self,
            TupleValue_SetItem_Ref,
            tuple.as_raw(),
            index,
            value.as_raw()
        )
    }

    /// Check dynamic value instance relationship.
    pub fn any_instance_of(&self, value: &AniRef<'_>, ty: &AniRef<'_>) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Any_InstanceOf,
            sys::ani_boolean,
            0,
            value.as_raw(),
            ty.as_raw()
        )?;
        Ok(result != 0)
    }

    /// Get dynamic property by name.
    pub fn any_get_property(&self, value: &AniRef<'_>, name: &str) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call_wrap!(
            self,
            Any_GetProperty,
            sys::ani_ref,
            AniRef,
            value.as_raw(),
            c_name.as_ptr()
        )
    }

    /// Set dynamic property by name.
    pub fn any_set_property(
        &self,
        value: &AniRef<'_>,
        name: &str,
        property: &AniRef<'_>,
    ) -> Result<()> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid property name"))?;
        ani_call!(
            self,
            Any_SetProperty,
            value.as_raw(),
            c_name.as_ptr(),
            property.as_raw()
        )
    }

    /// Get dynamic value by index.
    pub fn any_get_by_index(&self, value: &AniRef<'_>, index: usize) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            Any_GetByIndex,
            sys::ani_ref,
            AniRef,
            value.as_raw(),
            index
        )
    }

    /// Set dynamic value by index.
    pub fn any_set_by_index(
        &self,
        value: &AniRef<'_>,
        index: usize,
        item: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(self, Any_SetByIndex, value.as_raw(), index, item.as_raw())
    }

    /// Get dynamic value by dynamic key.
    pub fn any_get_by_value(&self, value: &AniRef<'_>, key: &AniRef<'_>) -> Result<AniRef<'local>> {
        ani_call_wrap!(
            self,
            Any_GetByValue,
            sys::ani_ref,
            AniRef,
            value.as_raw(),
            key.as_raw()
        )
    }

    /// Set dynamic value by dynamic key.
    pub fn any_set_by_value(
        &self,
        value: &AniRef<'_>,
        key: &AniRef<'_>,
        item: &AniRef<'_>,
    ) -> Result<()> {
        ani_call!(
            self,
            Any_SetByValue,
            value.as_raw(),
            key.as_raw(),
            item.as_raw()
        )
    }

    /// Call dynamic function.
    pub fn any_call(&self, func: &AniRef<'_>, args: &[AniRef<'_>]) -> Result<AniRef<'local>> {
        let mut raw_args = Self::ref_args_to_raw(args);
        let raw_ptr = if raw_args.is_empty() {
            ptr::null_mut()
        } else {
            raw_args.as_mut_ptr()
        };
        ani_call_wrap!(
            self,
            Any_Call,
            sys::ani_ref,
            AniRef,
            func.as_raw(),
            raw_args.len(),
            raw_ptr
        )
    }

    /// Call dynamic method by name.
    pub fn any_call_method(
        &self,
        this_ref: &AniRef<'_>,
        name: &str,
        args: &[AniRef<'_>],
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let mut raw_args = Self::ref_args_to_raw(args);
        let raw_ptr = if raw_args.is_empty() {
            ptr::null_mut()
        } else {
            raw_args.as_mut_ptr()
        };
        ani_call_wrap!(
            self,
            Any_CallMethod,
            sys::ani_ref,
            AniRef,
            this_ref.as_raw(),
            c_name.as_ptr(),
            raw_args.len(),
            raw_ptr
        )
    }

    /// Construct dynamic object via ctor reference.
    pub fn any_new(&self, ctor: &AniRef<'_>, args: &[AniRef<'_>]) -> Result<AniRef<'local>> {
        let mut raw_args = Self::ref_args_to_raw(args);
        let raw_ptr = if raw_args.is_empty() {
            ptr::null_mut()
        } else {
            raw_args.as_mut_ptr()
        };
        ani_call_wrap!(
            self,
            Any_New,
            sys::ani_ref,
            AniRef,
            ctor.as_raw(),
            raw_args.len(),
            raw_ptr
        )
    }

    /// Create an int array
    pub fn create_int_array(&self, length: usize) -> Result<AniArrayInt<'local>> {
        ani_call_wrap!(
            self,
            FixedArray_New_Int,
            sys::ani_fixedarray_int,
            AniArrayInt,
            length
        )
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

    /// Get the direct super class of a type.
    pub fn get_super_class(&self, ty: &AniType<'_>) -> Result<AniClass<'local>> {
        ani_call_wrap!(
            self,
            Type_GetSuperClass,
            sys::ani_class,
            AniClass,
            ty.as_raw()
        )
    }

    /// Check whether `to_type` can be assigned from `from_type`.
    pub fn is_assignable_from(
        &self,
        from_type: &AniType<'_>,
        to_type: &AniType<'_>,
    ) -> Result<bool> {
        let result = ani_call_ret!(
            self,
            Type_IsAssignableFrom,
            sys::ani_boolean,
            0,
            from_type.as_raw(),
            to_type.as_raw()
        )?;
        Ok(result != 0)
    }

    // ========================================================================
    // Call Method by Name Returning Reference
    // ========================================================================

    /// Call a method returning reference by name with `ani_value` arguments.
    pub fn call_method_by_name_ref_with_args(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
        args: &[sys::ani_value],
    ) -> Result<AniRef<'local>> {
        let c_name =
            CString::new(name).map_err(|_| Error::new(Status::Error, "Invalid method name"))?;
        let c_sig = signature.map(|s| CString::new(s).ok()).flatten();
        let sig_ptr = c_sig.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());

        let result = ani_call_ret_mid!(
            self,
            Object_CallMethodByName_Ref_A,
            sys::ani_ref,
            ptr::null_mut(),
            obj.as_raw(),
            c_name.as_ptr(),
            sig_ptr,
            Self::value_args_ptr(args)
        )?;
        Ok(unsafe { AniRef::from_raw(result) })
    }

    /// Call a method returning reference by name.
    pub fn call_method_by_name_ref(
        &self,
        obj: &AniObject<'_>,
        name: &str,
        signature: Option<&str>,
    ) -> Result<AniRef<'local>> {
        self.call_method_by_name_ref_with_args(obj, name, signature, &[])
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
        Ok((unsafe { AniResolver::from_raw(resolver) }, unsafe {
            AniObject::from_raw(promise)
        }))
    }

    /// Create a new Promise together with a typed [`Deferred<T>`] facade.
    ///
    /// This bridges the low-level `promise_new()` API into the higher-level
    /// `PromiseRaw<T> + Deferred<T>` model used by `ani::conversions`.
    pub fn promise_new_typed<T>(&self) -> Result<(Deferred<T>, PromiseRaw<'local, T>)> {
        let (resolver, promise) = self.promise_new()?;
        Ok((Deferred::from_resolver(resolver), unsafe {
            PromiseRaw::from_raw(promise.into_raw())
        }))
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
        let error = crate::conversions::create_promise_error(self, message)?;
        ani_call!(
            self,
            PromiseResolver_Reject,
            resolver.as_raw(),
            error.as_raw()
        )
    }
}
