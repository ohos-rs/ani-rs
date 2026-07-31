//! TupleValue wrapper conversions and helpers.
//!
//! ANI exposes typed tuple item read/write APIs but does not provide tuple
//! construction APIs in `ani.h`. This wrapper focuses on ergonomic access to
//! existing tuple handles.

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniRef, AniTupleValue};

use super::traits::{FromAni, ToAni, TypeInfo};

/// Rust-side wrapper for an ANI tuple value handle.
pub struct TupleValue<'env>(AniTupleValue<'env>);

impl<'env> TupleValue<'env> {
    /// Wrap an existing tuple handle.
    #[inline]
    pub fn from_handle(handle: AniTupleValue<'env>) -> Self {
        Self(handle)
    }

    /// Borrow the underlying ANI tuple handle.
    #[inline]
    pub fn as_handle(&self) -> &AniTupleValue<'env> {
        &self.0
    }

    /// Consume and return the underlying ANI tuple handle.
    #[inline]
    pub fn into_handle(self) -> AniTupleValue<'env> {
        self.0
    }

    /// Get tuple length.
    #[inline]
    pub fn len(&self, env: &Env<'env>) -> Result<usize> {
        env.get_tuple_number_of_items(&self.0)
    }

    /// Returns true when tuple has no items.
    #[inline]
    pub fn is_empty(&self, env: &Env<'env>) -> Result<bool> {
        Ok(self.len(env)? == 0)
    }

    /// Get `bool` item by index.
    #[inline]
    pub fn get_boolean(&self, env: &Env<'env>, index: usize) -> Result<bool> {
        env.get_tuple_item_boolean(&self.0, index)
    }

    /// Get `u16` char item by index.
    #[inline]
    pub fn get_char(&self, env: &Env<'env>, index: usize) -> Result<u16> {
        env.get_tuple_item_char(&self.0, index)
    }

    /// Get `i8` item by index.
    #[inline]
    pub fn get_byte(&self, env: &Env<'env>, index: usize) -> Result<i8> {
        env.get_tuple_item_byte(&self.0, index)
    }

    /// Get `i16` item by index.
    #[inline]
    pub fn get_short(&self, env: &Env<'env>, index: usize) -> Result<i16> {
        env.get_tuple_item_short(&self.0, index)
    }

    /// Get `i32` item by index.
    #[inline]
    pub fn get_int(&self, env: &Env<'env>, index: usize) -> Result<i32> {
        env.get_tuple_item_int(&self.0, index)
    }

    /// Get `i64` item by index.
    #[inline]
    pub fn get_long(&self, env: &Env<'env>, index: usize) -> Result<i64> {
        env.get_tuple_item_long(&self.0, index)
    }

    /// Get `f32` item by index.
    #[inline]
    pub fn get_float(&self, env: &Env<'env>, index: usize) -> Result<f32> {
        env.get_tuple_item_float(&self.0, index)
    }

    /// Get `f64` item by index.
    #[inline]
    pub fn get_double(&self, env: &Env<'env>, index: usize) -> Result<f64> {
        env.get_tuple_item_double(&self.0, index)
    }

    /// Get reference item by index.
    #[inline]
    pub fn get_ref(&self, env: &Env<'env>, index: usize) -> Result<AniRef<'env>> {
        env.get_tuple_item_ref(&self.0, index)
    }

    /// Set `bool` item by index.
    #[inline]
    pub fn set_boolean(&self, env: &Env<'env>, index: usize, value: bool) -> Result<()> {
        env.set_tuple_item_boolean(&self.0, index, value)
    }

    /// Set `u16` char item by index.
    #[inline]
    pub fn set_char(&self, env: &Env<'env>, index: usize, value: u16) -> Result<()> {
        env.set_tuple_item_char(&self.0, index, value)
    }

    /// Set `i8` item by index.
    #[inline]
    pub fn set_byte(&self, env: &Env<'env>, index: usize, value: i8) -> Result<()> {
        env.set_tuple_item_byte(&self.0, index, value)
    }

    /// Set `i16` item by index.
    #[inline]
    pub fn set_short(&self, env: &Env<'env>, index: usize, value: i16) -> Result<()> {
        env.set_tuple_item_short(&self.0, index, value)
    }

    /// Set `i32` item by index.
    #[inline]
    pub fn set_int(&self, env: &Env<'env>, index: usize, value: i32) -> Result<()> {
        env.set_tuple_item_int(&self.0, index, value)
    }

    /// Set `i64` item by index.
    #[inline]
    pub fn set_long(&self, env: &Env<'env>, index: usize, value: i64) -> Result<()> {
        env.set_tuple_item_long(&self.0, index, value)
    }

    /// Set `f32` item by index.
    #[inline]
    pub fn set_float(&self, env: &Env<'env>, index: usize, value: f32) -> Result<()> {
        env.set_tuple_item_float(&self.0, index, value)
    }

    /// Set `f64` item by index.
    #[inline]
    pub fn set_double(&self, env: &Env<'env>, index: usize, value: f64) -> Result<()> {
        env.set_tuple_item_double(&self.0, index, value)
    }

    /// Set reference item by index.
    #[inline]
    pub fn set_ref(&self, env: &Env<'env>, index: usize, value: &AniRef<'_>) -> Result<()> {
        env.set_tuple_item_ref(&self.0, index, value)
    }
}

impl<'env> From<AniTupleValue<'env>> for TupleValue<'env> {
    #[inline]
    fn from(value: AniTupleValue<'env>) -> Self {
        Self(value)
    }
}

impl<'env> From<TupleValue<'env>> for AniTupleValue<'env> {
    #[inline]
    fn from(value: TupleValue<'env>) -> Self {
        value.0
    }
}

impl TypeInfo for TupleValue<'_> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_tuple_value"
    }
}

impl<'env> ToAni<'env> for TupleValue<'env> {
    type Output = sys::ani_tuple_value;

    #[inline]
    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.0.into_raw())
    }
}

impl<'env> FromAni<'env> for TupleValue<'env> {
    type Input = sys::ani_tuple_value;

    #[inline]
    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                Status::InvalidArgs,
                "TupleValue pointer is null",
            ));
        }
        Ok(Self(unsafe { AniTupleValue::from_raw(value) }))
    }
}
