//! ANI Type Wrappers
//!
//! Provides safe wrappers for ANI types

use crate::sys;
use std::marker::PhantomData;

/// Basic reference type macro
macro_rules! define_ref_type {
    (
        $(#[$meta:meta])*
        $name:ident, $raw_type:ty
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        pub struct $name<'local> {
            raw: $raw_type,
            _marker: PhantomData<&'local ()>,
        }

        impl<'local> $name<'local> {
            /// Create from raw pointer
            ///
            /// # Safety
            ///
            /// Caller must ensure the pointer is valid
            #[inline]
            pub unsafe fn from_raw(raw: $raw_type) -> Self {
                Self {
                    raw,
                    _marker: PhantomData,
                }
            }

            /// Get raw pointer
            #[inline]
            pub fn as_raw(&self) -> $raw_type {
                self.raw
            }

            /// Convert to raw pointer, giving up ownership
            #[inline]
            pub fn into_raw(self) -> $raw_type {
                self.raw
            }

            /// Check if null
            #[inline]
            pub fn is_null(&self) -> bool {
                self.raw.is_null()
            }
        }
    };
}

/// Basic non-reference type macro (e.g., method, field)
macro_rules! define_opaque_type {
    (
        $(#[$meta:meta])*
        $name:ident, $raw_type:ty
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $name {
            raw: $raw_type,
        }

        impl $name {
            /// Create from raw pointer
            ///
            /// # Safety
            ///
            /// Caller must ensure the pointer is valid
            #[inline]
            pub unsafe fn from_raw(raw: $raw_type) -> Self {
                Self { raw }
            }

            /// Get raw pointer
            #[inline]
            pub fn as_raw(&self) -> $raw_type {
                self.raw
            }

            /// Check if null
            #[inline]
            pub fn is_null(&self) -> bool {
                self.raw.is_null()
            }
        }
    };
}

// ============================================================================
// Reference Types
// ============================================================================

define_ref_type!(
    /// ANI reference type
    AniRef, sys::ani_ref
);

define_ref_type!(
    /// ANI object type
    AniObject, sys::ani_object
);

define_ref_type!(
    /// ANI class type
    AniClass, sys::ani_class
);

define_ref_type!(
    /// ANI type (base type for all types)
    AniType, sys::ani_type
);

define_ref_type!(
    /// ANI module type
    AniModule, sys::ani_module
);

define_ref_type!(
    /// ANI namespace type
    AniNamespace, sys::ani_namespace
);

define_ref_type!(
    /// ANI string type
    AniString, sys::ani_string
);

define_ref_type!(
    /// ANI enum type
    AniEnum, sys::ani_enum
);

define_ref_type!(
    /// ANI error type
    AniError, sys::ani_error
);

define_ref_type!(
    /// ANI function object type
    AniFnObject, sys::ani_fn_object
);

define_ref_type!(
    /// ANI array type (generic)
    AniArray, sys::ani_array
);

define_ref_type!(
    /// ANI int array type
    AniArrayInt, sys::ani_fixedarray_int
);

define_ref_type!(
    /// ANI long array type
    AniArrayLong, sys::ani_fixedarray_long
);

define_ref_type!(
    /// ANI double array type
    AniArrayDouble, sys::ani_fixedarray_double
);

define_ref_type!(
    /// ANI reference array type
    AniArrayRef, sys::ani_array
);

define_ref_type!(
    /// ANI fixed array type (base)
    AniFixedArray, sys::ani_fixedarray
);

define_ref_type!(
    /// ANI fixed boolean array type
    AniFixedArrayBoolean, sys::ani_fixedarray_boolean
);

define_ref_type!(
    /// ANI fixed char array type
    AniFixedArrayChar, sys::ani_fixedarray_char
);

define_ref_type!(
    /// ANI fixed byte array type
    AniFixedArrayByte, sys::ani_fixedarray_byte
);

define_ref_type!(
    /// ANI fixed short array type
    AniFixedArrayShort, sys::ani_fixedarray_short
);

define_ref_type!(
    /// ANI fixed int array type
    AniFixedArrayInt, sys::ani_fixedarray_int
);

define_ref_type!(
    /// ANI fixed long array type
    AniFixedArrayLong, sys::ani_fixedarray_long
);

define_ref_type!(
    /// ANI fixed float array type
    AniFixedArrayFloat, sys::ani_fixedarray_float
);

define_ref_type!(
    /// ANI fixed double array type
    AniFixedArrayDouble, sys::ani_fixedarray_double
);

define_ref_type!(
    /// ANI fixed reference array type
    AniFixedArrayRef, sys::ani_fixedarray_ref
);

define_ref_type!(
    /// ANI ArrayBuffer type
    AniArrayBuffer, sys::ani_arraybuffer
);

define_ref_type!(
    /// ANI enum item type
    AniEnumItem, sys::ani_enum_item
);

define_ref_type!(
    /// ANI tuple value type
    AniTupleValue, sys::ani_tuple_value
);

// ============================================================================
// Opaque Types (methods, fields, etc.)
// ============================================================================

define_opaque_type!(
    /// ANI method type
    AniMethod, sys::ani_method
);

define_opaque_type!(
    /// ANI static method type
    AniStaticMethod, sys::ani_static_method
);

define_opaque_type!(
    /// ANI field type
    AniField, sys::ani_field
);

define_opaque_type!(
    /// ANI static field type
    AniStaticField, sys::ani_static_field
);

define_opaque_type!(
    /// ANI function type
    AniFunction, sys::ani_function
);

define_opaque_type!(
    /// ANI variable type
    AniVariable, sys::ani_variable
);

// ============================================================================
// Promise Types
// ============================================================================

define_opaque_type!(
    /// ANI resolver type for Promise resolution/rejection
    ///
    /// The resolver is used to resolve or reject a Promise that was created
    /// along with it. Once a Promise is resolved or rejected, the resolver
    /// is automatically freed.
    AniResolver, sys::ani_resolver
);

// ============================================================================
// Global References
// ============================================================================

/// ANI global reference
///
/// Global references are not limited by local reference frames and must be manually released
#[repr(transparent)]
pub struct GlobalRef {
    raw: sys::ani_ref,
}

impl GlobalRef {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid
    #[inline]
    pub unsafe fn from_raw(raw: sys::ani_ref) -> Self {
        Self { raw }
    }

    /// Get raw pointer
    #[inline]
    pub fn as_raw(&self) -> sys::ani_ref {
        self.raw
    }
}

// GlobalRef can be transferred between threads
unsafe impl Send for GlobalRef {}
unsafe impl Sync for GlobalRef {}

// ============================================================================
// Weak References
// ============================================================================

/// ANI weak reference
#[repr(transparent)]
pub struct WeakRef {
    raw: sys::ani_wref,
}

impl WeakRef {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid
    #[inline]
    pub unsafe fn from_raw(raw: sys::ani_wref) -> Self {
        Self { raw }
    }

    /// Get raw pointer
    #[inline]
    pub fn as_raw(&self) -> sys::ani_wref {
        self.raw
    }
}

unsafe impl Send for WeakRef {}
unsafe impl Sync for WeakRef {}

// ============================================================================
// Type Conversions
// ============================================================================

impl<'local> From<AniObject<'local>> for AniRef<'local> {
    fn from(obj: AniObject<'local>) -> Self {
        unsafe { AniRef::from_raw(obj.as_raw()) }
    }
}

impl<'local> From<AniString<'local>> for AniRef<'local> {
    fn from(s: AniString<'local>) -> Self {
        unsafe { AniRef::from_raw(s.as_raw()) }
    }
}

impl<'local> From<AniClass<'local>> for AniRef<'local> {
    fn from(cls: AniClass<'local>) -> Self {
        unsafe { AniRef::from_raw(cls.as_raw()) }
    }
}

impl<'local> From<AniString<'local>> for AniObject<'local> {
    fn from(s: AniString<'local>) -> Self {
        unsafe { AniObject::from_raw(s.as_raw()) }
    }
}

impl<'local> From<AniClass<'local>> for AniType<'local> {
    fn from(cls: AniClass<'local>) -> Self {
        unsafe { AniType::from_raw(cls.as_raw()) }
    }
}

// ============================================================================
// ani_value Helper Functions
// ============================================================================

/// Create ani_value containing byte value
#[inline]
pub fn ani_value_byte(v: i8) -> sys::ani_value {
    sys::ani_value { b: v }
}

/// Create ani_value containing short value
#[inline]
pub fn ani_value_short(v: i16) -> sys::ani_value {
    sys::ani_value { s: v }
}

/// Create ani_value containing char value
#[inline]
pub fn ani_value_char(v: u16) -> sys::ani_value {
    sys::ani_value { c: v }
}

/// Create ani_value containing int value
#[inline]
pub fn ani_value_int(v: i32) -> sys::ani_value {
    sys::ani_value { i: v }
}

/// Create ani_value containing long value
#[inline]
pub fn ani_value_long(v: i64) -> sys::ani_value {
    sys::ani_value { l: v }
}

/// Create ani_value containing float value
#[inline]
pub fn ani_value_float(v: f32) -> sys::ani_value {
    sys::ani_value { f: v }
}

/// Create ani_value containing double value
#[inline]
pub fn ani_value_double(v: f64) -> sys::ani_value {
    sys::ani_value { d: v }
}

/// Create ani_value containing boolean value
#[inline]
pub fn ani_value_boolean(v: bool) -> sys::ani_value {
    sys::ani_value {
        z: if v { 1 } else { 0 },
    }
}

/// Create ani_value containing reference
#[inline]
pub fn ani_value_ref(r: sys::ani_ref) -> sys::ani_value {
    sys::ani_value { r }
}

// ============================================================================
// Native Function Helpers
// ============================================================================

/// Create ani_native_function struct
///
/// # Safety
///
/// Caller must ensure name and signature are valid C strings,
/// and pointer points to a function with the correct signature
#[inline]
pub const fn native_function(
    name: &'static str,
    signature: &'static str,
    pointer: *const std::ffi::c_void,
) -> sys::ani_native_function {
    sys::ani_native_function {
        name: name.as_ptr() as *const std::ffi::c_char,
        signature: signature.as_ptr() as *const std::ffi::c_char,
        pointer,
    }
}

/// Create ani_native_function with null terminator
#[macro_export]
macro_rules! ani_native_fn {
    ($name:expr, $sig:expr, $fn:expr) => {
        $crate::sys::ani_native_function {
            name: concat!($name, "\0").as_ptr() as *const std::os::raw::c_char,
            signature: concat!($sig, "\0").as_ptr() as *const std::os::raw::c_char,
            pointer: $fn as *const std::os::raw::c_void,
        }
    };
}
