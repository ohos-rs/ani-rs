//! Object Type Conversion
//!
//! Implements conversion between Rust custom structs and ANI object types.

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::*;

use super::traits::{FromAni, ToAni, TypeInfo};

macro_rules! impl_ref_handle_conversion {
    ($ty:ident, $raw:ty, $sig:literal, $ani_c:literal, $name:literal) => {
        impl<'env> TypeInfo for $ty<'env> {
            fn type_signature() -> &'static str {
                $sig
            }

            fn ani_c_type() -> &'static str {
                $ani_c
            }
        }

        impl<'env> ToAni<'env> for $ty<'env> {
            type Output = $raw;

            fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
                Ok(self.into_raw())
            }
        }

        impl<'env> FromAni<'env> for $ty<'env> {
            type Input = $raw;

            unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
                if value.is_null() {
                    return Err(Error::new(
                        crate::error::Status::InvalidArgs,
                        format!("Null pointer: {}", $name),
                    ));
                }
                Ok(unsafe { $ty::from_raw(value) })
            }
        }
    };
}

macro_rules! impl_opaque_handle_conversion {
    ($ty:ident, $raw:ty, $sig:literal, $ani_c:literal, $name:literal) => {
        impl TypeInfo for $ty {
            fn type_signature() -> &'static str {
                $sig
            }

            fn ani_c_type() -> &'static str {
                $ani_c
            }
        }

        impl<'env> ToAni<'env> for $ty {
            type Output = $raw;

            fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
                Ok(self.as_raw())
            }
        }

        impl<'env> FromAni<'env> for $ty {
            type Input = $raw;

            unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
                if value.is_null() {
                    return Err(Error::new(
                        crate::error::Status::InvalidArgs,
                        format!("Null pointer: {}", $name),
                    ));
                }
                Ok(unsafe { $ty::from_raw(value) })
            }
        }
    };
}

// ============================================================================
// Generic custom-object field access
// ============================================================================

/// Named field access used by `#[ani(object)]` generated structs.
///
/// `#[ani(object)]` follows nominal ArkTS class semantics. Field conversion is
/// performed through ANI object field APIs instead of dynamic `Any` access.
pub trait ObjectField<'env>: Sized {
    /// Read a named field from an ANI object.
    fn get_named_field(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self>;

    /// Write a named field to an ANI object.
    fn set_named_field(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()>;
}

/// Named property access used by `#[ani(object)]` field-level property metadata.
pub trait ObjectProperty<'env>: Sized {
    /// Read a named property from an ANI object.
    fn get_named_property(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self>;

    /// Write a named property to an ANI object.
    fn set_named_property(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()>;
}

/// Persist a Rust object back into an existing ArkTS object instance.
///
/// This is used by `#[ani]` impl methods with `&mut self` receivers: the
/// wrapper reconstructs a typed Rust value from `this`, invokes the Rust
/// method, then writes the updated fields back to the original ArkTS object.
pub trait WriteBackToAniObject<'env>: Sized {
    /// Consume the Rust value and write its fields into `obj`.
    fn write_back_to_ani_object(self, env: &Env<'env>, obj: &AniObject<'_>) -> Result<()>;
}

trait IntoFieldRef<'env> {
    fn into_field_ref(self) -> AniRef<'env>;
}

trait FromFieldRefInput<'env>: Sized {
    fn from_field_ref(value: AniRef<'env>) -> Self;
}

impl<'env> IntoFieldRef<'env> for AniRef<'env> {
    fn into_field_ref(self) -> AniRef<'env> {
        self
    }
}

impl<'env> IntoFieldRef<'env> for AniObject<'env> {
    fn into_field_ref(self) -> AniRef<'env> {
        unsafe { AniRef::from_raw(self.into_raw() as sys::ani_ref) }
    }
}

impl<'env> IntoFieldRef<'env> for AniString<'env> {
    fn into_field_ref(self) -> AniRef<'env> {
        unsafe { AniRef::from_raw(self.into_raw() as sys::ani_ref) }
    }
}

impl<'env> IntoFieldRef<'env> for sys::ani_ref {
    fn into_field_ref(self) -> AniRef<'env> {
        unsafe { AniRef::from_raw(self) }
    }
}

impl<'env> FromFieldRefInput<'env> for AniRef<'env> {
    fn from_field_ref(value: AniRef<'env>) -> Self {
        value
    }
}

impl<'env> FromFieldRefInput<'env> for AniObject<'env> {
    fn from_field_ref(value: AniRef<'env>) -> Self {
        unsafe { AniObject::from_raw(value.into_raw() as sys::ani_object) }
    }
}

impl<'env> FromFieldRefInput<'env> for AniString<'env> {
    fn from_field_ref(value: AniRef<'env>) -> Self {
        unsafe { AniString::from_raw(value.into_raw() as sys::ani_string) }
    }
}

impl<'env> FromFieldRefInput<'env> for sys::ani_ref {
    fn from_field_ref(value: AniRef<'env>) -> Self {
        value.into_raw()
    }
}

macro_rules! impl_object_member_primitive {
    ($ty:ty, $field_getter:ident, $field_setter:ident, $property_getter:ident, $property_setter:ident) => {
        impl<'env> ObjectField<'env> for $ty {
            fn get_named_field(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
                env.$field_getter(obj, name)
            }

            fn set_named_field(
                self,
                env: &Env<'env>,
                obj: &AniObject<'_>,
                name: &str,
            ) -> Result<()> {
                env.$field_setter(obj, name, self)
            }
        }

        impl<'env> ObjectProperty<'env> for $ty {
            fn get_named_property(
                env: &Env<'env>,
                obj: &AniObject<'_>,
                name: &str,
            ) -> Result<Self> {
                env.$property_getter(obj, name)
            }

            fn set_named_property(
                self,
                env: &Env<'env>,
                obj: &AniObject<'_>,
                name: &str,
            ) -> Result<()> {
                env.$property_setter(obj, name, self)
            }
        }
    };
}

impl_object_member_primitive!(
    bool,
    get_field_by_name_boolean,
    set_field_by_name_boolean,
    get_property_by_name_boolean,
    set_property_by_name_boolean
);
impl_object_member_primitive!(
    i8,
    get_field_by_name_byte,
    set_field_by_name_byte,
    get_property_by_name_byte,
    set_property_by_name_byte
);
impl_object_member_primitive!(
    i16,
    get_field_by_name_short,
    set_field_by_name_short,
    get_property_by_name_short,
    set_property_by_name_short
);
impl_object_member_primitive!(
    u16,
    get_field_by_name_char,
    set_field_by_name_char,
    get_property_by_name_char,
    set_property_by_name_char
);
impl_object_member_primitive!(
    i32,
    get_field_by_name_int,
    set_field_by_name_int,
    get_property_by_name_int,
    set_property_by_name_int
);
impl_object_member_primitive!(
    i64,
    get_field_by_name_long,
    set_field_by_name_long,
    get_property_by_name_long,
    set_property_by_name_long
);
impl_object_member_primitive!(
    f32,
    get_field_by_name_float,
    set_field_by_name_float,
    get_property_by_name_float,
    set_property_by_name_float
);
impl_object_member_primitive!(
    f64,
    get_field_by_name_double,
    set_field_by_name_double,
    get_property_by_name_double,
    set_property_by_name_double
);

impl<'env> ObjectField<'env> for u8 {
    fn get_named_field(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = i16::get_named_field(env, obj, name)?;
        u8::try_from(value).map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("short field {name}={value} does not fit in u8"),
            )
        })
    }

    fn set_named_field(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        i16::from(self).set_named_field(env, obj, name)
    }
}

impl<'env> ObjectProperty<'env> for u8 {
    fn get_named_property(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = i16::get_named_property(env, obj, name)?;
        u8::try_from(value).map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("short property {name}={value} does not fit in u8"),
            )
        })
    }

    fn set_named_property(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        i16::from(self).set_named_property(env, obj, name)
    }
}

impl<'env> ObjectField<'env> for u32 {
    fn get_named_field(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = i64::get_named_field(env, obj, name)?;
        u32::try_from(value).map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("long field {name}={value} does not fit in u32"),
            )
        })
    }

    fn set_named_field(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        i64::from(self).set_named_field(env, obj, name)
    }
}

impl<'env> ObjectProperty<'env> for u32 {
    fn get_named_property(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = i64::get_named_property(env, obj, name)?;
        u32::try_from(value).map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("long property {name}={value} does not fit in u32"),
            )
        })
    }

    fn set_named_property(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        i64::from(self).set_named_property(env, obj, name)
    }
}

impl<'env, T> ObjectField<'env> for T
where
    T: ToAni<'env> + FromAni<'env>,
    <T as ToAni<'env>>::Output: IntoFieldRef<'env>,
    <T as FromAni<'env>>::Input: FromFieldRefInput<'env>,
{
    fn get_named_field(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = env.get_field_by_name_ref(obj, name)?;
        unsafe {
            T::from_ani(
                env,
                <<T as FromAni<'env>>::Input as FromFieldRefInput<'env>>::from_field_ref(value),
            )
        }
    }

    fn set_named_field(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        let value = self.to_ani(env)?.into_field_ref();
        env.set_field_by_name_ref(obj, name, &value)
    }
}

impl<'env, T> ObjectProperty<'env> for T
where
    T: ToAni<'env> + FromAni<'env>,
    <T as ToAni<'env>>::Output: IntoFieldRef<'env>,
    <T as FromAni<'env>>::Input: FromFieldRefInput<'env>,
{
    fn get_named_property(env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<Self> {
        let value = env.get_property_by_name_ref(obj, name)?;
        unsafe {
            T::from_ani(
                env,
                <<T as FromAni<'env>>::Input as FromFieldRefInput<'env>>::from_field_ref(value),
            )
        }
    }

    fn set_named_property(self, env: &Env<'env>, obj: &AniObject<'_>, name: &str) -> Result<()> {
        let value = self.to_ani(env)?.into_field_ref();
        env.set_property_by_name_ref(obj, name, &value)
    }
}

// ============================================================================
// AniObject Conversion
// ============================================================================

impl<'env> TypeInfo for AniObject<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ToAni<'env> for AniObject<'env> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniObject<'env> {
    type Input = sys::ani_object;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "object"),
            ));
        }
        Ok(unsafe { AniObject::from_raw(value) })
    }
}

impl_ref_handle_conversion!(AniRef, sys::ani_ref, "Lstd/core/Object;", "ani_ref", "ref");

// ============================================================================
// AniClass Conversion
// ============================================================================

impl<'env> TypeInfo for AniClass<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/Class;"
    }
    fn ani_c_type() -> &'static str {
        "ani_class"
    }
}

impl<'env> ToAni<'env> for AniClass<'env> {
    type Output = sys::ani_class;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniClass<'env> {
    type Input = sys::ani_class;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "class"),
            ));
        }
        Ok(unsafe { AniClass::from_raw(value) })
    }
}

impl_ref_handle_conversion!(
    AniType,
    sys::ani_type,
    "Lstd/core/Object;",
    "ani_type",
    "type"
);

impl_ref_handle_conversion!(
    AniModule,
    sys::ani_module,
    "Lstd/core/Object;",
    "ani_module",
    "module"
);

impl_ref_handle_conversion!(
    AniNamespace,
    sys::ani_namespace,
    "Lstd/core/Object;",
    "ani_namespace",
    "namespace"
);

// ============================================================================
// AniString Conversion
// ============================================================================

impl<'env> TypeInfo for AniString<'env> {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for AniString<'env> {
    type Output = sys::ani_string;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.into_raw())
    }
}

impl<'env> FromAni<'env> for AniString<'env> {
    type Input = sys::ani_string;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                format!("Null pointer: {}", "string"),
            ));
        }
        Ok(unsafe { AniString::from_raw(value) })
    }
}

impl_ref_handle_conversion!(
    AniEnum,
    sys::ani_enum,
    "Lstd/core/Object;",
    "ani_enum",
    "enum"
);

impl_ref_handle_conversion!(
    AniError,
    sys::ani_error,
    "Lstd/core/Object;",
    "ani_error",
    "error"
);

impl_ref_handle_conversion!(
    AniEnumItem,
    sys::ani_enum_item,
    "Lstd/core/Object;",
    "ani_enum_item",
    "enum_item"
);

impl_ref_handle_conversion!(
    AniTupleValue,
    sys::ani_tuple_value,
    "Lstd/core/Object;",
    "ani_tuple_value",
    "tuple_value"
);

impl_ref_handle_conversion!(
    AniArray,
    sys::ani_array,
    "A{C{std.core.Object}}",
    "ani_array",
    "array"
);

impl_ref_handle_conversion!(
    AniArrayRef,
    sys::ani_array,
    "A{C{std.core.Object}}",
    "ani_array",
    "array_ref"
);

impl_ref_handle_conversion!(
    AniArrayInt,
    sys::ani_fixedarray_int,
    "A{i}",
    "ani_fixedarray_int",
    "array_int"
);

impl_ref_handle_conversion!(
    AniArrayLong,
    sys::ani_fixedarray_long,
    "A{l}",
    "ani_fixedarray_long",
    "array_long"
);

impl_ref_handle_conversion!(
    AniArrayDouble,
    sys::ani_fixedarray_double,
    "A{d}",
    "ani_fixedarray_double",
    "array_double"
);

impl_ref_handle_conversion!(
    AniFixedArray,
    sys::ani_fixedarray,
    "A{C{std.core.Object}}",
    "ani_fixedarray",
    "fixed_array"
);

impl_ref_handle_conversion!(
    AniFixedArrayBoolean,
    sys::ani_fixedarray_boolean,
    "A{z}",
    "ani_fixedarray_boolean",
    "fixed_array_boolean"
);

impl_ref_handle_conversion!(
    AniFixedArrayChar,
    sys::ani_fixedarray_char,
    "A{c}",
    "ani_fixedarray_char",
    "fixed_array_char"
);

impl_ref_handle_conversion!(
    AniFixedArrayByte,
    sys::ani_fixedarray_byte,
    "A{b}",
    "ani_fixedarray_byte",
    "fixed_array_byte"
);

impl_ref_handle_conversion!(
    AniFixedArrayShort,
    sys::ani_fixedarray_short,
    "A{s}",
    "ani_fixedarray_short",
    "fixed_array_short"
);

impl_ref_handle_conversion!(
    AniFixedArrayInt,
    sys::ani_fixedarray_int,
    "A{i}",
    "ani_fixedarray_int",
    "fixed_array_int"
);

impl_ref_handle_conversion!(
    AniFixedArrayLong,
    sys::ani_fixedarray_long,
    "A{l}",
    "ani_fixedarray_long",
    "fixed_array_long"
);

impl_ref_handle_conversion!(
    AniFixedArrayFloat,
    sys::ani_fixedarray_float,
    "A{f}",
    "ani_fixedarray_float",
    "fixed_array_float"
);

impl_ref_handle_conversion!(
    AniFixedArrayDouble,
    sys::ani_fixedarray_double,
    "A{d}",
    "ani_fixedarray_double",
    "fixed_array_double"
);

impl_ref_handle_conversion!(
    AniFixedArrayRef,
    sys::ani_fixedarray_ref,
    "A{C{std.core.Object}}",
    "ani_fixedarray_ref",
    "fixed_array_ref"
);

impl_opaque_handle_conversion!(
    AniMethod,
    sys::ani_method,
    "Lstd/core/Object;",
    "ani_method",
    "method"
);

impl_opaque_handle_conversion!(
    AniStaticMethod,
    sys::ani_static_method,
    "Lstd/core/Object;",
    "ani_static_method",
    "static_method"
);

impl_opaque_handle_conversion!(
    AniField,
    sys::ani_field,
    "Lstd/core/Object;",
    "ani_field",
    "field"
);

impl_opaque_handle_conversion!(
    AniStaticField,
    sys::ani_static_field,
    "Lstd/core/Object;",
    "ani_static_field",
    "static_field"
);

impl_opaque_handle_conversion!(
    AniFunction,
    sys::ani_function,
    "Lstd/core/Object;",
    "ani_function",
    "function"
);

impl_opaque_handle_conversion!(
    AniVariable,
    sys::ani_variable,
    "Lstd/core/Object;",
    "ani_variable",
    "variable"
);

impl_opaque_handle_conversion!(
    AniResolver,
    sys::ani_resolver,
    "Lstd/core/Object;",
    "ani_resolver",
    "resolver"
);

// ============================================================================
// Native Pointer Wrapper
// ============================================================================

/// Native pointer wrapper for passing Rust object pointers in ANI
#[repr(transparent)]
pub struct NativePointer<T> {
    ptr: i64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> NativePointer<T> {
    /// Create NativePointer from Box
    pub fn from_box(boxed: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(boxed) as i64,
            _marker: std::marker::PhantomData,
        }
    }

    /// Convert back to Box (consumes NativePointer)
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr as *mut T) }
    }

    /// Get reference
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn as_ref(&self) -> &T {
        unsafe { &*(self.ptr as *const T) }
    }

    /// Get mutable reference
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.ptr as *mut T) }
    }

    /// Get raw pointer value (i64)
    pub fn as_raw(&self) -> i64 {
        self.ptr
    }

    /// Create from raw pointer value
    ///
    /// # Safety
    ///
    /// Caller must ensure the pointer is valid and the type is correct
    pub unsafe fn from_raw(ptr: i64) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> TypeInfo for NativePointer<T> {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
}

impl<'env, T> ToAni<'env> for NativePointer<T> {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.ptr)
    }
}

impl<'env, T> FromAni<'env> for NativePointer<T> {
    type Input = sys::ani_long;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(unsafe { NativePointer::from_raw(value) })
    }
}

// ============================================================================
// Generic Object Conversion Helper
// ============================================================================

/// Object conversion helper trait
pub trait ObjectConversion<'env> {
    /// Convert from AniObject
    fn from_ani_object(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self>
    where
        Self: Sized;

    /// Convert to AniObject
    fn to_ani_object(&self, env: &Env<'env>) -> Result<AniObject<'env>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_pointer() {
        let boxed = Box::new(42i32);
        let ptr = NativePointer::from_box(boxed);

        unsafe {
            assert_eq!(*ptr.as_ref(), 42);
            let recovered = ptr.into_box();
            assert_eq!(*recovered, 42);
        }
    }

    #[test]
    fn test_object_type_signature() {
        assert_eq!(<AniObject>::type_signature(), "Lstd/core/Object;");
        assert_eq!(<AniClass>::type_signature(), "Lstd/core/Class;");
    }

    #[test]
    fn test_array_handle_type_signatures() {
        assert_eq!(<AniArray>::type_signature(), "A{C{std.core.Object}}");
        assert_eq!(<AniArrayRef>::type_signature(), "A{C{std.core.Object}}");
        assert_eq!(<AniFixedArray>::type_signature(), "A{C{std.core.Object}}");
        assert_eq!(
            <AniFixedArrayRef>::type_signature(),
            "A{C{std.core.Object}}"
        );
        assert_eq!(<AniFixedArrayInt>::type_signature(), "A{i}");
        assert_eq!(<AniFixedArrayBoolean>::type_signature(), "A{z}");
    }

    #[test]
    fn test_array_handle_ani_c_types() {
        assert_eq!(<AniArray>::ani_c_type(), "ani_array");
        assert_eq!(<AniArrayRef>::ani_c_type(), "ani_array");
        assert_eq!(<AniFixedArray>::ani_c_type(), "ani_fixedarray");
        assert_eq!(<AniFixedArrayRef>::ani_c_type(), "ani_fixedarray_ref");
        assert_eq!(<AniArrayInt>::ani_c_type(), "ani_fixedarray_int");
        assert_eq!(<AniFixedArrayDouble>::ani_c_type(), "ani_fixedarray_double");
    }
}
