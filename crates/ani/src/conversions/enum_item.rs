//! Enum item wrapper conversions and helpers.

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniEnum, AniEnumItem};

use super::traits::{FromAni, ToAni, TypeInfo};

/// Enum item value for Rust-side handling.
#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
    /// Integer enum value.
    Int(i32),
    /// String enum value.
    String(String),
}

/// Rust-side wrapper for an ANI enum item handle.
pub struct EnumItem<'env>(AniEnumItem<'env>);

impl<'env> EnumItem<'env> {
    /// Wrap an existing enum item handle.
    #[inline]
    pub fn from_handle(handle: AniEnumItem<'env>) -> Self {
        Self(handle)
    }

    /// Find item by enum name.
    #[inline]
    pub fn from_name(env: &Env<'env>, enm: &AniEnum<'_>, name: &str) -> Result<Self> {
        Ok(Self(env.get_enum_item_by_name(enm, name)?))
    }

    /// Find item by enum index.
    #[inline]
    pub fn from_index(env: &Env<'env>, enm: &AniEnum<'_>, index: usize) -> Result<Self> {
        Ok(Self(env.get_enum_item_by_index(enm, index)?))
    }

    /// Borrow the underlying ANI enum item handle.
    #[inline]
    pub fn as_handle(&self) -> &AniEnumItem<'env> {
        &self.0
    }

    /// Consume and return the underlying ANI enum item handle.
    #[inline]
    pub fn into_handle(self) -> AniEnumItem<'env> {
        self.0
    }

    /// Get enum name of this item.
    #[inline]
    pub fn name(&self, env: &Env<'env>) -> Result<String> {
        let s = env.get_enum_item_name(&self.0)?;
        env.get_string(&s)
    }

    /// Get enum index of this item.
    #[inline]
    pub fn index(&self, env: &Env<'env>) -> Result<usize> {
        env.get_enum_item_index(&self.0)
    }

    /// Get enum type containing this item.
    #[inline]
    pub fn enum_type(&self, env: &Env<'env>) -> Result<AniEnum<'env>> {
        env.get_enum_of_item(&self.0)
    }

    /// Get integer value of this item.
    #[inline]
    pub fn int_value(&self, env: &Env<'env>) -> Result<i32> {
        env.get_enum_item_value_int(&self.0)
    }

    /// Get string value of this item.
    #[inline]
    pub fn string_value(&self, env: &Env<'env>) -> Result<String> {
        let s = env.get_enum_item_value_string(&self.0)?;
        env.get_string(&s)
    }

    /// Get value as `EnumValue`.
    ///
    /// The function first tries integer value, then string value.
    #[inline]
    pub fn value(&self, env: &Env<'env>) -> Result<EnumValue> {
        if let Ok(v) = self.int_value(env) {
            return Ok(EnumValue::Int(v));
        }
        if let Ok(v) = self.string_value(env) {
            return Ok(EnumValue::String(v));
        }
        Err(Error::new(
            Status::InvalidType,
            "Unsupported enum item value type",
        ))
    }
}

impl<'env> From<AniEnumItem<'env>> for EnumItem<'env> {
    #[inline]
    fn from(value: AniEnumItem<'env>) -> Self {
        Self(value)
    }
}

impl<'env> From<EnumItem<'env>> for AniEnumItem<'env> {
    #[inline]
    fn from(value: EnumItem<'env>) -> Self {
        value.0
    }
}

impl TypeInfo for EnumItem<'_> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_enum_item"
    }
}

impl<'env> ToAni<'env> for EnumItem<'env> {
    type Output = sys::ani_enum_item;

    #[inline]
    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.0.into_raw())
    }
}

impl<'env> FromAni<'env> for EnumItem<'env> {
    type Input = sys::ani_enum_item;

    #[inline]
    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "EnumItem pointer is null"));
        }
        Ok(Self(unsafe { AniEnumItem::from_raw(value) }))
    }
}
