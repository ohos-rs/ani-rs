//! BigInt Example - Parsing BigInt parameters
//!
//! Demonstrates how to handle ArkTS BigInt type
//! BigInt corresponds to Lescompat/BigInt; in ANI

use ani_derive::ani;

// ============================================================================
// BigInt Basic Operations
// ============================================================================

/// Create BigInt value from i64 (returns long representation)
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function createBigInt(value: long): bigint;
/// ```
#[ani]
pub fn create_big_int(value: i64) -> i64 {
    value
}

/// Convert BigInt to long
///
/// Corresponding ArkTS definition:
/// ```typescript  
/// native function bigIntToLong(value: bigint): long;
/// ```
///
/// Note: BigInt's mangling is Lescompat/BigInt;
/// When received in native layer, it needs to be handled as an object
#[ani]
pub fn big_int_to_long(value: i64) -> i64 {
    value
}

// ============================================================================
// BigInt Arithmetic Operations
// ============================================================================

/// BigInt addition
#[ani]
pub fn big_int_add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}

/// BigInt subtraction
#[ani]
pub fn big_int_subtract(a: i64, b: i64) -> i64 {
    a.wrapping_sub(b)
}

/// BigInt multiplication
#[ani]
pub fn big_int_multiply(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b)
}

/// BigInt division
#[ani]
pub fn big_int_divide(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// BigInt modulo
#[ani]
pub fn big_int_modulo(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a % b
    }
}

// ============================================================================
// BigInt Bitwise Operations
// ============================================================================

/// BigInt bitwise AND
#[ani]
pub fn big_int_and(a: i64, b: i64) -> i64 {
    a & b
}

/// BigInt bitwise OR
#[ani]
pub fn big_int_or(a: i64, b: i64) -> i64 {
    a | b
}

/// BigInt bitwise XOR
#[ani]
pub fn big_int_xor(a: i64, b: i64) -> i64 {
    a ^ b
}

/// BigInt left shift
#[ani]
pub fn big_int_shl(a: i64, bits: i32) -> i64 {
    a << bits
}

/// BigInt right shift
#[ani]
pub fn big_int_shr(a: i64, bits: i32) -> i64 {
    a >> bits
}

// ============================================================================
// BigInt Comparison
// ============================================================================

/// Compare two BigInt values
/// Returns: -1 if a < b, 0 if a == b, 1 if a > b
#[ani]
pub fn big_int_compare(a: i64, b: i64) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Check if BigInt is zero
#[ani]
pub fn big_int_is_zero(value: i64) -> bool {
    value == 0
}

/// Check if BigInt is negative
#[ani]
pub fn big_int_is_negative(value: i64) -> bool {
    value < 0
}

// ============================================================================
// BigInt Utility Functions
// ============================================================================

/// Get bit length of BigInt
#[ani]
pub fn big_int_bit_length(value: i64) -> i32 {
    if value == 0 {
        0
    } else {
        64 - value.abs().leading_zeros() as i32
    }
}

/// Get absolute value
#[ani]
pub fn big_int_abs(value: i64) -> i64 {
    value.abs()
}

/// Negate value
#[ani]
pub fn big_int_negate(value: i64) -> i64 {
    -value
}

/// Calculate power (a^b mod 2^64)
#[ani]
pub fn big_int_pow(base: i64, exp: i32) -> i64 {
    if exp < 0 {
        return 0;
    }
    let mut result: i64 = 1;
    let mut b = base;
    let mut e = exp as u32;
    while e > 0 {
        if e & 1 == 1 {
            result = result.wrapping_mul(b);
        }
        b = b.wrapping_mul(b);
        e >>= 1;
    }
    result
}

// ============================================================================
// Module Initialization
// ============================================================================
