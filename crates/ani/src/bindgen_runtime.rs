//! Type Conversion Runtime Support
//!
//! Provides automatic conversion between Rust types and ANI types

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::*;

// ============================================================================
// Type Info Trait
// ============================================================================

/// Type info trait for getting ANI signature of a type
pub trait TypeInfo {
    /// Get ANI mangling signature for this type
    fn type_signature() -> &'static str;

    /// Get ANI C type name for this type
    fn ani_c_type() -> &'static str;
}

// Implement TypeInfo for primitive types
impl TypeInfo for bool {
    fn type_signature() -> &'static str {
        "Z"
    }
    fn ani_c_type() -> &'static str {
        "ani_boolean"
    }
}

impl TypeInfo for i8 {
    fn type_signature() -> &'static str {
        "B"
    }
    fn ani_c_type() -> &'static str {
        "ani_byte"
    }
}

impl TypeInfo for u16 {
    fn type_signature() -> &'static str {
        "C"
    }
    fn ani_c_type() -> &'static str {
        "ani_char"
    }
}

impl TypeInfo for i16 {
    fn type_signature() -> &'static str {
        "S"
    }
    fn ani_c_type() -> &'static str {
        "ani_short"
    }
}

impl TypeInfo for i32 {
    fn type_signature() -> &'static str {
        "I"
    }
    fn ani_c_type() -> &'static str {
        "ani_int"
    }
}

impl TypeInfo for i64 {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
}

impl TypeInfo for f32 {
    fn type_signature() -> &'static str {
        "F"
    }
    fn ani_c_type() -> &'static str {
        "ani_float"
    }
}

impl TypeInfo for f64 {
    fn type_signature() -> &'static str {
        "D"
    }
    fn ani_c_type() -> &'static str {
        "ani_double"
    }
}

impl TypeInfo for () {
    fn type_signature() -> &'static str {
        "V"
    }
    fn ani_c_type() -> &'static str {
        "void"
    }
}

impl TypeInfo for String {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl TypeInfo for &str {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }
    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn type_signature() -> &'static str {
        // Optional types use boxed type or keep original type
        T::type_signature()
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn type_signature() -> &'static str {
        // Simplified implementation, should generate based on T
        "[Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_array"
    }
}

impl TypeInfo for crate::conversions::ArrayBuffer {
    fn type_signature() -> &'static str {
        "Lescompat/ArrayBuffer;"
    }
    fn ani_c_type() -> &'static str {
        "ani_arraybuffer"
    }
}

// ============================================================================
// ToAni Trait - Rust to ANI Conversion
// ============================================================================

/// Convert Rust type to ANI type
pub trait ToAni<'env> {
    /// Output ANI type
    type Output;

    /// Perform conversion
    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output>;
}

// Basic type implementations (direct mapping)
impl<'env> ToAni<'env> for bool {
    type Output = sys::ani_boolean;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(if self { 1 } else { 0 })
    }
}

impl<'env> ToAni<'env> for i8 {
    type Output = sys::ani_byte;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for i16 {
    type Output = sys::ani_short;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for u16 {
    type Output = sys::ani_char;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for i32 {
    type Output = sys::ani_int;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for i64 {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for f32 {
    type Output = sys::ani_float;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for f64 {
    type Output = sys::ani_double;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for String {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(&self)
    }
}

impl<'env> ToAni<'env> for &str {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(self)
    }
}

impl<'env> ToAni<'env> for () {
    type Output = ();

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(())
    }
}

// Option type implementation
impl<'env, T: ToAni<'env>> ToAni<'env> for Option<T> {
    type Output = sys::ani_ref;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        match self {
            Some(value) => {
                let _converted = value.to_ani(env)?;
                // TODO: Return converted reference
                Ok(std::ptr::null_mut())
            }
            None => {
                // Return null or undefined
                Ok(std::ptr::null_mut())
            }
        }
    }
}

// ============================================================================
// FromAni Trait - ANI to Rust Conversion
// ============================================================================

/// Convert ANI type to Rust type
pub trait FromAni<'env>: Sized {
    /// Input ANI type
    type Input;

    /// Perform conversion
    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self>;
}

// Primitive type implementations
impl<'env> FromAni<'env> for bool {
    type Input = sys::ani_boolean;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value != 0)
    }
}

impl<'env> FromAni<'env> for i8 {
    type Input = sys::ani_byte;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for i16 {
    type Input = sys::ani_short;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for u16 {
    type Input = sys::ani_char;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for i32 {
    type Input = sys::ani_int;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for i64 {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for f32 {
    type Input = sys::ani_float;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for f64 {
    type Input = sys::ani_double;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for String {
    type Input = AniString<'env>;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        env.get_string(&value)
    }
}

// ============================================================================
// Boxing/Unboxing Support
// ============================================================================

/// Boxable trait - box a primitive type into an object
pub trait Boxable<'env> {
    /// Boxed type
    type Boxed;

    /// Box class descriptor
    fn box_class_descriptor() -> &'static str;

    /// Box the value
    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed>;
}

impl<'env> Boxable<'env> for i32 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Int"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, "i:")?;
        let args = [ani_value_int(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Boxable<'env> for i64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Long"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, "l:")?;
        let args = [ani_value_long(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Boxable<'env> for f64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Double"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, "d:")?;
        let args = [ani_value_double(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Boxable<'env> for bool {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Boolean"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, "z:")?;
        let args = [ani_value_boolean(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

/// Unboxable trait - extract primitive type value from object
pub trait Unboxable<'env>: Sized {
    /// Unbox from boxed object
    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self>;
}

impl<'env> Unboxable<'env> for i32 {
    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_int(obj, "unboxed", Some(":I"))
    }
}

// ============================================================================
// Signature Generation Helpers
// ============================================================================

/// Generate function signature
pub fn generate_signature<Args, Ret>() -> String
where
    Args: SignatureArgs,
    Ret: TypeInfo,
{
    let mut sig = Args::args_signature();
    sig.push(':');
    sig.push_str(Ret::type_signature());
    sig
}

/// Signature arguments trait
pub trait SignatureArgs {
    /// Get the signature for arguments part
    fn args_signature() -> String;
}

// Implement signatures for tuples
impl SignatureArgs for () {
    fn args_signature() -> String {
        String::new()
    }
}

impl<A: TypeInfo> SignatureArgs for (A,) {
    fn args_signature() -> String {
        A::type_signature().to_string()
    }
}

impl<A: TypeInfo, B: TypeInfo> SignatureArgs for (A, B) {
    fn args_signature() -> String {
        format!("{}{}", A::type_signature(), B::type_signature())
    }
}

impl<A: TypeInfo, B: TypeInfo, C: TypeInfo> SignatureArgs for (A, B, C) {
    fn args_signature() -> String {
        format!(
            "{}{}{}",
            A::type_signature(),
            B::type_signature(),
            C::type_signature()
        )
    }
}

impl<A: TypeInfo, B: TypeInfo, C: TypeInfo, D: TypeInfo> SignatureArgs for (A, B, C, D) {
    fn args_signature() -> String {
        format!(
            "{}{}{}{}",
            A::type_signature(),
            B::type_signature(),
            C::type_signature(),
            D::type_signature()
        )
    }
}

// ============================================================================
// Convenience Macros
// ============================================================================

/// Macro for generating method signatures
#[macro_export]
macro_rules! signature {
    // No parameters, returns void
    (() -> ()) => { ":V" };

    // No parameters, has return value
    (() -> $ret:ty) => {
        concat!(":", <$ret as $crate::bindgen_runtime::TypeInfo>::type_signature())
    };

    // Has parameters, returns void
    (($($arg:ty),+) -> ()) => {
        concat!(
            $(<$arg as $crate::bindgen_runtime::TypeInfo>::type_signature()),+,
            ":V"
        )
    };

    // Has parameters, has return value
    (($($arg:ty),+) -> $ret:ty) => {
        concat!(
            $(<$arg as $crate::bindgen_runtime::TypeInfo>::type_signature()),+,
            ":",
            <$ret as $crate::bindgen_runtime::TypeInfo>::type_signature()
        )
    };
}
