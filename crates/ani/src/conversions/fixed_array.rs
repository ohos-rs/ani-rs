//! FixedArray conversions between Rust and ANI.
//!
//! This module provides Rust-side wrapper types for ANI fixed arrays, following
//! the existing `ToAni`/`FromAni` conversion style.

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::{
    AniFixedArray, AniFixedArrayBoolean, AniFixedArrayByte, AniFixedArrayChar, AniFixedArrayDouble,
    AniFixedArrayFloat, AniFixedArrayInt, AniFixedArrayLong, AniFixedArrayShort,
};

use super::traits::{FromAni, ToAni, TypeInfo};

macro_rules! define_fixed_array_wrapper {
    ($(#[$meta:meta])* $name:ident, $elem:ty) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name(Vec<$elem>);

        impl $name {
            /// Create a wrapper from a vector.
            #[inline]
            pub fn new(data: Vec<$elem>) -> Self {
                Self(data)
            }

            /// Number of elements.
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns true if the wrapper has no elements.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Borrow the inner slice.
            #[inline]
            pub fn as_slice(&self) -> &[$elem] {
                self.0.as_slice()
            }

            /// Consume into the inner vector.
            #[inline]
            pub fn into_vec(self) -> Vec<$elem> {
                self.0
            }
        }

        impl From<Vec<$elem>> for $name {
            #[inline]
            fn from(data: Vec<$elem>) -> Self {
                Self::new(data)
            }
        }

        impl<const N: usize> From<[$elem; N]> for $name {
            #[inline]
            fn from(data: [$elem; N]) -> Self {
                Self::new(data.to_vec())
            }
        }

        impl From<$name> for Vec<$elem> {
            #[inline]
            fn from(value: $name) -> Self {
                value.into_vec()
            }
        }

        impl AsRef<[$elem]> for $name {
            #[inline]
            fn as_ref(&self) -> &[$elem] {
                self.as_slice()
            }
        }
    };
}

define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed boolean arrays.
    FixedBooleanArray,
    bool
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed char arrays (`u16`).
    FixedCharArray,
    u16
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed byte arrays (`i8`).
    FixedByteArray,
    i8
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed short arrays (`i16`).
    FixedShortArray,
    i16
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed int arrays (`i32`).
    FixedIntArray,
    i32
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed long arrays (`i64`).
    FixedLongArray,
    i64
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed float arrays (`f32`).
    FixedFloatArray,
    f32
);
define_fixed_array_wrapper!(
    /// Rust-side wrapper for ANI fixed double arrays (`f64`).
    FixedDoubleArray,
    f64
);

#[inline]
fn fixed_array_len<'env>(env: &Env<'env>, array: sys::ani_fixedarray) -> Result<usize> {
    let base = unsafe { AniFixedArray::from_raw(array) };
    env.get_fixed_array_length(&base)
}

macro_rules! impl_fixed_array_conversion {
    (
        $wrapper:ident,
        $elem:ty,
        $ani_raw:ty,
        $ani_handle:ident,
        $create:ident,
        $set_region:ident,
        $get_region:ident,
        $ani_c:literal
    ) => {
        impl TypeInfo for $wrapper {
            fn type_signature() -> &'static str {
                "Lstd/core/Object;"
            }

            fn ani_c_type() -> &'static str {
                $ani_c
            }
        }

        impl<'env> ToAni<'env> for $wrapper {
            type Output = $ani_raw;

            fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
                let handle = env.$create(self.0.len())?;
                if !self.0.is_empty() {
                    env.$set_region(&handle, 0, self.0.as_slice())?;
                }
                Ok(handle.into_raw())
            }
        }

        impl<'env> FromAni<'env> for $wrapper {
            type Input = $ani_raw;

            fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
                let len = fixed_array_len(env, value as sys::ani_fixedarray)?;
                let handle = unsafe { $ani_handle::from_raw(value) };
                let data: Vec<$elem> = env.$get_region(&handle, 0, len)?;
                Ok(Self(data))
            }
        }
    };
}

impl_fixed_array_conversion!(
    FixedBooleanArray,
    bool,
    sys::ani_fixedarray_boolean,
    AniFixedArrayBoolean,
    create_fixed_array_boolean,
    set_fixed_array_region_boolean,
    get_fixed_array_region_boolean,
    "ani_fixedarray_boolean"
);
impl_fixed_array_conversion!(
    FixedCharArray,
    u16,
    sys::ani_fixedarray_char,
    AniFixedArrayChar,
    create_fixed_array_char,
    set_fixed_array_region_char,
    get_fixed_array_region_char,
    "ani_fixedarray_char"
);
impl_fixed_array_conversion!(
    FixedByteArray,
    i8,
    sys::ani_fixedarray_byte,
    AniFixedArrayByte,
    create_fixed_array_byte,
    set_fixed_array_region_byte,
    get_fixed_array_region_byte,
    "ani_fixedarray_byte"
);
impl_fixed_array_conversion!(
    FixedShortArray,
    i16,
    sys::ani_fixedarray_short,
    AniFixedArrayShort,
    create_fixed_array_short,
    set_fixed_array_region_short,
    get_fixed_array_region_short,
    "ani_fixedarray_short"
);
impl_fixed_array_conversion!(
    FixedIntArray,
    i32,
    sys::ani_fixedarray_int,
    AniFixedArrayInt,
    create_fixed_array_int,
    set_fixed_array_region_int,
    get_fixed_array_region_int,
    "ani_fixedarray_int"
);
impl_fixed_array_conversion!(
    FixedLongArray,
    i64,
    sys::ani_fixedarray_long,
    AniFixedArrayLong,
    create_fixed_array_long,
    set_fixed_array_region_long,
    get_fixed_array_region_long,
    "ani_fixedarray_long"
);
impl_fixed_array_conversion!(
    FixedFloatArray,
    f32,
    sys::ani_fixedarray_float,
    AniFixedArrayFloat,
    create_fixed_array_float,
    set_fixed_array_region_float,
    get_fixed_array_region_float,
    "ani_fixedarray_float"
);
impl_fixed_array_conversion!(
    FixedDoubleArray,
    f64,
    sys::ani_fixedarray_double,
    AniFixedArrayDouble,
    create_fixed_array_double,
    set_fixed_array_region_double,
    get_fixed_array_region_double,
    "ani_fixedarray_double"
);
