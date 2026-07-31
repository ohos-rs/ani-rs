//! Real ArkTS `bigint` conversion without narrowing values to `i64`.

use ani::conversions::BigInt;
use ani::error::Result;
use ani_derive::ani;

/// Returns the same arbitrary-precision value through the ANI object ABI.
#[ani]
pub fn big_int_identity(value: BigInt) -> BigInt {
    value
}

/// Returns the full canonical decimal representation.
#[ani]
pub fn big_int_to_decimal(value: BigInt) -> String {
    value.to_string()
}

/// Creates an arbitrary-precision ArkTS bigint from a decimal string.
#[ani]
pub fn big_int_from_decimal(value: String) -> Result<BigInt> {
    BigInt::from_decimal(value)
}

/// Converts only when the value fits exactly in `i64`.
#[ani]
pub fn big_int_to_i64_lossless(value: BigInt) -> Result<i64> {
    value.to_i64()
}

/// Reports whether the arbitrary-precision value is negative.
#[ani]
pub fn big_int_is_negative(value: BigInt) -> bool {
    value.as_decimal().starts_with('-')
}

/// Returns the number of decimal digits, excluding the sign.
#[ani]
pub fn big_int_decimal_digits(value: BigInt) -> i32 {
    value
        .as_decimal()
        .strip_prefix('-')
        .unwrap_or_else(|| value.as_decimal())
        .len() as i32
}

/// Lossless built-in unsigned and 128-bit integer mappings use ArkTS bigint.
#[ani]
pub fn u64_identity(value: u64) -> u64 {
    value
}

#[ani]
pub fn i128_identity(value: i128) -> i128 {
    value
}

#[ani]
pub fn u128_identity(value: u128) -> u128 {
    value
}
