//! Core Conversion Traits
//!
//! Defines core traits for conversion between Rust and ANI types

use crate::env::Env;
use crate::error::Result;

// ============================================================================
// Type Info Trait
// ============================================================================

/// Type info trait for getting ANI signatures
pub trait TypeInfo {
    /// Get the ANI mangling signature for this type
    ///
    /// Primitive type signatures:
    /// - Z: boolean
    /// - B: byte (i8)
    /// - C: char (u16)
    /// - S: short (i16)
    /// - I: int (i32)
    /// - J: long (i64)
    /// - F: float (f32)
    /// - D: double (f64)
    /// - V: void
    /// - Lxxx; reference types
    fn type_signature() -> &'static str;

    /// Get the ANI C type name for this type
    fn ani_c_type() -> &'static str;

    /// Whether this is a primitive type (no boxing required)
    fn is_primitive() -> bool {
        false
    }
}

// ============================================================================
// ToAni Trait - Rust to ANI Conversion
// ============================================================================

/// Convert Rust type to ANI type
///
/// # Example
///
/// ```ignore
/// let rust_string = "Hello".to_string();
/// let ani_string = rust_string.to_ani(&env)?;
/// ```
pub trait ToAni<'env> {
    /// Output ANI type
    type Output;

    /// Perform conversion
    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output>;
}

/// Fast conversion trait for simple types that don't need env
pub trait ToAniDirect {
    /// Output ANI type
    type Output;

    /// Direct conversion (no environment needed)
    fn to_ani_direct(self) -> Self::Output;
}

// ============================================================================
// FromAni Trait - ANI to Rust Conversion
// ============================================================================

/// Convert ANI type to Rust type
///
/// # Example
///
/// ```ignore
/// let rust_string = String::from_ani(&env, ani_string)?;
/// ```
pub trait FromAni<'env>: Sized {
    /// Input ANI type
    type Input;

    /// Perform conversion
    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self>;
}

/// Fast conversion trait for simple types that don't need env
pub trait FromAniDirect: Sized {
    /// Input ANI type
    type Input;

    /// Direct conversion (no environment needed)
    fn from_ani_direct(value: Self::Input) -> Self;
}

// ============================================================================
// Signature Generation Helpers
// ============================================================================

/// Signature arguments trait
pub trait SignatureArgs {
    /// Get the signature for arguments part
    fn args_signature() -> String;
}

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

impl<A: TypeInfo, B: TypeInfo, C: TypeInfo, D: TypeInfo, E: TypeInfo> SignatureArgs
    for (A, B, C, D, E)
{
    fn args_signature() -> String {
        format!(
            "{}{}{}{}{}",
            A::type_signature(),
            B::type_signature(),
            C::type_signature(),
            D::type_signature(),
            E::type_signature()
        )
    }
}

impl<A: TypeInfo, B: TypeInfo, C: TypeInfo, D: TypeInfo, E: TypeInfo, F: TypeInfo> SignatureArgs
    for (A, B, C, D, E, F)
{
    fn args_signature() -> String {
        format!(
            "{}{}{}{}{}{}",
            A::type_signature(),
            B::type_signature(),
            C::type_signature(),
            D::type_signature(),
            E::type_signature(),
            F::type_signature()
        )
    }
}

// ============================================================================
// Convenience Macros
// ============================================================================

/// Macro for generating method signatures
#[macro_export]
macro_rules! ani_signature {
    // 无参数返回 void
    (() -> ()) => { ":V" };

    // 无参数有返回值
    (() -> $ret:ty) => {
        concat!(":", <$ret as $crate::conversions::TypeInfo>::type_signature())
    };

    // 有参数返回 void
    (($($arg:ty),+) -> ()) => {
        concat!(
            $(<$arg as $crate::conversions::TypeInfo>::type_signature()),+,
            ":V"
        )
    };

    // 有参数有返回值
    (($($arg:ty),+) -> $ret:ty) => {
        concat!(
            $(<$arg as $crate::conversions::TypeInfo>::type_signature()),+,
            ":",
            <$ret as $crate::conversions::TypeInfo>::type_signature()
        )
    };
}
