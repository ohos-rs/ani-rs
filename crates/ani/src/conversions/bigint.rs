//! Arbitrary-precision ArkTS `bigint` values.

use std::fmt;
use std::str::FromStr;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniClass, AniObject, AniString, ani_value_ref};

use super::{FromAni, ToAni, ToAniArg, TypeInfo};

/// An owned arbitrary-precision integer exchanged with ArkTS `bigint`.
///
/// The Rust representation is a canonical decimal string. This avoids the
/// truncation inherent in mapping ArkTS BigInt to `i64` and does not force a
/// particular big-number arithmetic crate on applications.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BigInt {
    decimal: String,
}

impl BigInt {
    /// Parses and canonicalizes a base-10 integer.
    pub fn from_decimal(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim();
        let (negative, digits) = match value.as_bytes().first() {
            Some(b'-') => (true, &value[1..]),
            Some(b'+') => (false, &value[1..]),
            _ => (false, value),
        };
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::new(
                Status::InvalidArgs,
                format!("invalid decimal BigInt: {value:?}"),
            ));
        }
        let digits = digits.trim_start_matches('0');
        let decimal = if digits.is_empty() {
            "0".to_string()
        } else if negative {
            format!("-{digits}")
        } else {
            digits.to_string()
        };
        Ok(Self { decimal })
    }

    /// Returns the canonical base-10 representation.
    pub fn as_decimal(&self) -> &str {
        &self.decimal
    }

    /// Converts to `i64`, returning `OutOfRange` instead of truncating.
    pub fn to_i64(&self) -> Result<i64> {
        self.decimal.parse().map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("BigInt {} does not fit in i64", self.decimal),
            )
        })
    }

    /// Converts to `u64`, returning `OutOfRange` instead of truncating.
    pub fn to_u64(&self) -> Result<u64> {
        self.parse_integer("u64")
    }

    /// Converts to `i128`, returning `OutOfRange` instead of truncating.
    pub fn to_i128(&self) -> Result<i128> {
        self.parse_integer("i128")
    }

    /// Converts to `u128`, returning `OutOfRange` instead of truncating.
    pub fn to_u128(&self) -> Result<u128> {
        self.parse_integer("u128")
    }

    fn parse_integer<T>(&self, target: &'static str) -> Result<T>
    where
        T: FromStr,
    {
        self.decimal.parse().map_err(|_| {
            Error::new(
                Status::OutOfRange,
                format!("BigInt {} does not fit in {target}", self.decimal),
            )
        })
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.decimal)
    }
}

impl FromStr for BigInt {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::from_decimal(value)
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        Self {
            decimal: value.to_string(),
        }
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        Self {
            decimal: value.to_string(),
        }
    }
}

impl From<i128> for BigInt {
    fn from(value: i128) -> Self {
        Self {
            decimal: value.to_string(),
        }
    }
}

impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        Self {
            decimal: value.to_string(),
        }
    }
}

macro_rules! impl_rust_bigint_conversion {
    ($ty:ty, $method:ident) => {
        impl TypeInfo for $ty {
            fn type_signature() -> &'static str {
                "Lstd/core/BigInt;"
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        impl<'env> ToAni<'env> for $ty {
            type Output = sys::ani_object;

            fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
                BigInt::from(self).to_ani(env)
            }
        }

        impl<'env> FromAni<'env> for $ty {
            type Input = sys::ani_object;

            unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
                unsafe { BigInt::from_ani(env, value) }?.$method()
            }
        }

        impl ToAniArg for $ty {
            fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
                BigInt::from(*self).to_ani_arg(env)
            }

            fn arg_signature() -> &'static str {
                "Lstd/core/BigInt;"
            }
        }
    };
}

impl_rust_bigint_conversion!(u64, to_u64);
impl_rust_bigint_conversion!(i128, to_i128);
impl_rust_bigint_conversion!(u128, to_u128);

impl TypeInfo for BigInt {
    fn type_signature() -> &'static str {
        "Lstd/core/BigInt;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

fn find_bigint_class<'env>(env: &Env<'env>) -> Result<AniClass<'env>> {
    env.find_class("std.core.BigInt")
        .or_else(|_| env.find_class("escompat.BigInt"))
        .or_else(|_| env.find_class("Lescompat/BigInt;"))
}

impl<'env> ToAni<'env> for BigInt {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let class = find_bigint_class(env)?;
        let constructor = env
            .find_constructor(&class, "C{std.core.String}:")
            .or_else(|_| env.find_constructor(&class, "Lstd/core/String;:V"))?;
        let decimal = env.create_string(&self.decimal)?;
        let args = [ani_value_ref(decimal.as_raw() as sys::ani_ref)];
        Ok(env.new_object(&class, &constructor, &args)?.into_raw())
    }
}

impl<'env> FromAni<'env> for BigInt {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "BigInt value is null"));
        }
        let object = unsafe { AniObject::from_raw(value) };
        let class = find_bigint_class(env)?;
        if !env.object_instance_of(&object, &class)? {
            return Err(Error::new(
                Status::InvalidType,
                "value is not an ArkTS bigint",
            ));
        }
        let method = env
            .find_method(&class, "toString", ":C{std.core.String}")
            .or_else(|_| env.find_method(&class, "toString", ":Lstd/core/String;"))?;
        let value = env.call_ref_method(&object, &method, &[])?;
        let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        Self::from_decimal(env.get_string(&value)?)
    }
}

impl ToAniArg for BigInt {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        self.clone().to_ani(env).map(|value| value as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/BigInt;"
    }
}

impl From<BigInt> for String {
    fn from(value: BigInt) -> Self {
        value.decimal
    }
}

impl TryFrom<&str> for BigInt {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_decimal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_decimal_values() {
        assert_eq!(BigInt::from_decimal(" +00042 ").unwrap().as_decimal(), "42");
        assert_eq!(BigInt::from_decimal("-00042").unwrap().as_decimal(), "-42");
        assert_eq!(BigInt::from_decimal("-000").unwrap().as_decimal(), "0");
    }

    #[test]
    fn rejects_non_decimal_values() {
        for invalid in ["", "-", "12.0", "0xff", "1_000"] {
            assert_eq!(
                BigInt::from_decimal(invalid).unwrap_err().status,
                Status::InvalidArgs
            );
        }
    }

    #[test]
    fn i64_conversion_is_lossless() {
        assert_eq!(BigInt::from(i64::MIN).to_i64().unwrap(), i64::MIN);
        assert_eq!(
            BigInt::from_decimal("9223372036854775808")
                .unwrap()
                .to_i64()
                .unwrap_err()
                .status,
            Status::OutOfRange
        );
    }

    #[test]
    fn rust_integer_boundaries_are_lossless() {
        assert_eq!(BigInt::from(u64::MAX).to_u64().unwrap(), u64::MAX);
        assert_eq!(BigInt::from(i128::MIN).to_i128().unwrap(), i128::MIN);
        assert_eq!(BigInt::from(i128::MAX).to_i128().unwrap(), i128::MAX);
        assert_eq!(BigInt::from(u128::MAX).to_u128().unwrap(), u128::MAX);
        assert_eq!(
            BigInt::from_decimal("-1")
                .unwrap()
                .to_u128()
                .unwrap_err()
                .status,
            Status::OutOfRange
        );
    }
}
