//! Either Type - Support for ANI Union Types
//!
//! Provides `Either` types to represent ArkTS union types like `string | int`.
//! In ANI, all union types are mapped to `Lstd/core/Object;` (ani_object),
//! and we need to use `Object_InstanceOf` to determine the actual type at runtime.
//!
//! # Examples
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! // Handle string | int union type
//! fn handle_union(value: Either<String, i32>) -> String {
//!     match value {
//!         Either::A(s) => format!("String: {}", s),
//!         Either::B(i) => format!("Int: {}", i),
//!     }
//! }
//! ```

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::*;

use super::boxed::{Boxable, Unboxable};
use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// Traits for Either Conversion
// ============================================================================

/// Trait for validating if an ANI object can be converted to a specific type.
///
/// This is used by Either to try each variant in order. Types that want to be
/// used in Either must implement this trait to provide runtime type checking.
pub trait ValidateFromAni<'env> {
    /// Check if the given ani_object can be converted to this type
    ///
    /// # Safety
    ///
    /// `value` must be a live ANI reference that belongs to the VM associated
    /// with `env`, or a null reference accepted by the implementation.
    unsafe fn validate(env: &Env<'env>, value: sys::ani_object) -> bool;
}

/// Trait for converting from ANI object (ani_object) to Rust type in Either context.
///
/// This is different from `FromAni` because:
/// - Input is always `ani_object` (the union type representation)
/// - Handles unboxing for primitive types
///
/// For types implementing `Unboxable`, this trait is automatically implemented.
pub trait FromAniObject<'env>: Sized {
    /// Convert from ani_object to Self
    ///
    /// # Safety
    ///
    /// `value` must be a live ANI object reference that belongs to the VM
    /// associated with `env` and must satisfy this type's runtime validation.
    unsafe fn from_ani_object(env: &Env<'env>, value: sys::ani_object) -> Result<Self>;
}

/// Trait for converting Rust type to ANI object (ani_object) in Either context.
///
/// This is different from `ToAni` because:
/// - Output is always `ani_object` (the union type representation)
/// - Handles boxing for primitive types
///
/// For types implementing `Boxable`, this trait is automatically implemented.
pub trait ToAniObject<'env> {
    /// Convert Self to ani_object
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object>;
}

// ============================================================================
// Blanket implementations - Auto-implement for Boxable/Unboxable types
// ============================================================================

/// Blanket implementation: any type that implements `Unboxable` automatically
/// implements `FromAniObject` by unboxing from the boxed object.
impl<'env, T> FromAniObject<'env> for T
where
    T: Unboxable<'env>,
{
    unsafe fn from_ani_object(env: &Env<'env>, value: sys::ani_object) -> Result<Self> {
        let obj = unsafe { AniObject::from_raw(value) };
        T::unbox(env, &obj)
    }
}

/// Blanket implementation: any type that implements `Boxable` automatically
/// implements `ToAniObject` by boxing to a boxed object.
impl<'env, T> ToAniObject<'env> for T
where
    T: Boxable<'env, Boxed = AniObject<'env>>,
{
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object> {
        Ok(self.box_value(env)?.into_raw())
    }
}

// ============================================================================
// Either Macro - Generate Either types with all trait implementations
// ============================================================================

/// Macro to generate Either types with N variants
macro_rules! either_n {
    ($either_name:ident, $($variant:ident),+ $(,)?) => {
        /// Either type for representing a union of types.
        ///
        /// Maps to `Lstd/core/Object;` in ANI (ani_object).
        #[derive(Debug, Clone, PartialEq)]
        pub enum $either_name<$($variant),+> {
            $(
                /// Variant $variant
                $variant($variant),
            )+
        }

        // TypeInfo implementation - all union types map to Object
        impl<$($variant),+> TypeInfo for $either_name<$($variant),+> {
            fn type_signature() -> &'static str {
                "Lstd/core/Object;"
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        // FromAni implementation - try each variant in order
        impl<'env, $($variant),+> FromAni<'env> for $either_name<$($variant),+>
        where
            $(
                $variant: ValidateFromAni<'env> + FromAniObject<'env>,
            )+
        {
            type Input = sys::ani_object;

            unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
                // Try each variant in order - validation first, then conversion.
                // This allows explicit `Null` / `Undefined` variants to participate
                // instead of being rejected up front as a raw nullish reference.
                $(
                    if unsafe { $variant::validate(env, value) } {
                        if let Ok(v) = unsafe { $variant::from_ani_object(env, value) } {
                            return Ok(Self::$variant(v));
                        }
                    }
                )+

                Err(Error::new(
                    Status::InvalidType,
                    concat!(
                        "Object does not match any variant of ",
                        stringify!($either_name)
                    )
                ))
            }
        }

        // ToAni implementation - convert the contained value
        impl<'env, $($variant),+> ToAni<'env> for $either_name<$($variant),+>
        where
            $(
                $variant: ToAniObject<'env>,
            )+
        {
            type Output = sys::ani_object;

            fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
                match self {
                    $(
                        Self::$variant(v) => v.to_ani_object(env),
                    )+
                }
            }
        }
    };
}

// ============================================================================
// Generate Either types (2-26 variants)
// ============================================================================

either_n!(Either, A, B);
either_n!(Either3, A, B, C);
either_n!(Either4, A, B, C, D);
either_n!(Either5, A, B, C, D, E);
either_n!(Either6, A, B, C, D, E, F);
either_n!(Either7, A, B, C, D, E, F, G);
either_n!(Either8, A, B, C, D, E, F, G, H);
either_n!(Either9, A, B, C, D, E, F, G, H, I);
either_n!(Either10, A, B, C, D, E, F, G, H, I, J);
either_n!(Either11, A, B, C, D, E, F, G, H, I, J, K);
either_n!(Either12, A, B, C, D, E, F, G, H, I, J, K, L);
either_n!(Either13, A, B, C, D, E, F, G, H, I, J, K, L, M);
either_n!(Either14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
either_n!(Either15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
either_n!(Either16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
either_n!(Either17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
either_n!(
    Either18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R
);
either_n!(
    Either19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S
);
either_n!(
    Either20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
either_n!(
    Either21, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
either_n!(
    Either22, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
either_n!(
    Either23, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
either_n!(
    Either24, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
either_n!(
    Either25, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
either_n!(
    Either26, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

// ============================================================================
// Convenience methods for Either<A, B>
// ============================================================================

impl<A, B> Either<A, B> {
    /// Check if this is variant A
    pub fn is_a(&self) -> bool {
        matches!(self, Either::A(_))
    }

    /// Check if this is variant B
    pub fn is_b(&self) -> bool {
        matches!(self, Either::B(_))
    }

    /// Get variant A if present
    pub fn as_a(&self) -> Option<&A> {
        match self {
            Either::A(a) => Some(a),
            Either::B(_) => None,
        }
    }

    /// Get variant B if present
    pub fn as_b(&self) -> Option<&B> {
        match self {
            Either::A(_) => None,
            Either::B(b) => Some(b),
        }
    }

    /// Convert to variant A, consuming self
    pub fn into_a(self) -> Option<A> {
        match self {
            Either::A(a) => Some(a),
            Either::B(_) => None,
        }
    }

    /// Convert to variant B, consuming self
    pub fn into_b(self) -> Option<B> {
        match self {
            Either::A(_) => None,
            Either::B(b) => Some(b),
        }
    }
}

// ============================================================================
// ValidateFromAni implementations for common types
// ============================================================================

impl<'env> ValidateFromAni<'env> for String {
    unsafe fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
        if value.is_null() {
            return false;
        }

        let obj = unsafe { AniObject::from_raw(value) };

        if let Ok(cls) = env.find_class("std.core.String")
            && env.object_instance_of(&obj, &cls).unwrap_or(false)
        {
            return true;
        }

        // Primitive wrappers should not be treated as string union variants.
        for numeric_cls in [
            "std.core.Boolean",
            "std.core.Byte",
            "std.core.Short",
            "std.core.Char",
            "std.core.Int",
            "std.core.Long",
            "std.core.Float",
            "std.core.Double",
        ] {
            if let Ok(cls) = env.find_class(numeric_cls)
                && env.object_instance_of(&obj, &cls).unwrap_or(false)
            {
                return false;
            }
        }

        let string_ref = unsafe { AniString::from_raw(value as sys::ani_string) };
        env.get_string(&string_ref).is_ok()
    }
}

/// Macro to implement ValidateFromAni for boxed primitive types
macro_rules! impl_validate_for_boxed {
    ($rust_type:ty, $class_descriptor:expr) => {
        impl<'env> ValidateFromAni<'env> for $rust_type {
            unsafe fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
                if let Ok(cls) = env.find_class($class_descriptor) {
                    let obj = unsafe { AniObject::from_raw(value) };
                    env.object_instance_of(&obj, &cls).unwrap_or(false)
                } else {
                    false
                }
            }
        }
    };
}

impl_validate_for_boxed!(bool, "std.core.Boolean");
impl_validate_for_boxed!(i8, "std.core.Byte");
impl_validate_for_boxed!(i16, "std.core.Short");
impl_validate_for_boxed!(u16, "std.core.Char");
impl_validate_for_boxed!(i32, "std.core.Int");
impl_validate_for_boxed!(i64, "std.core.Long");
impl_validate_for_boxed!(f32, "std.core.Float");
impl_validate_for_boxed!(f64, "std.core.Double");

// ============================================================================
// String conversion for Either (not Boxable, needs manual impl)
// ============================================================================

impl<'env> FromAniObject<'env> for String {
    unsafe fn from_ani_object(env: &Env<'env>, value: sys::ani_object) -> Result<Self> {
        // String object can be cast to ani_string directly
        let str_ref = unsafe { AniString::from_raw(value as sys::ani_string) };
        env.get_string(&str_ref)
    }
}

impl<'env> ToAniObject<'env> for String {
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object> {
        let ani_str = env.create_string(&self)?;
        Ok(ani_str.into_raw() as sys::ani_object)
    }
}

impl<'env> ToAniObject<'env> for &str {
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object> {
        let ani_str = env.create_string(self)?;
        Ok(ani_str.into_raw() as sys::ani_object)
    }
}
