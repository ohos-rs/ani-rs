//! Union Example - Using Either type to handle union types
//!
//! Demonstrates how to elegantly handle ArkTS union types using the `Either` type
//! Union types are uniformly mapped to Lstd/core/Object; in ANI
//!
//! Either type automatically handles:
//! - Type checking (instanceof)
//! - Boxing/unboxing
//! - Type-safe pattern matching

use ani::conversions::{Either, Either3, Either4};
use ani_derive::ani;

// ============================================================================
// Using #[ani] macro + Either type to handle union types
// ============================================================================

/// Handle string | int union type using Either
///
/// Corresponding ArkTS definition:
/// ```typescript
/// type StringOrInt = string | int;
/// native function handleStringOrIntEither(value: StringOrInt): string;
/// ```
#[ani]
pub fn handle_string_or_int_either(value: Either<String, i32>) -> String {
    match value {
        Either::A(s) => format!("String: {}", s),
        Either::B(i) => format!("Int: {}", i),
    }
}

/// Handle three-type union using Either3
///
/// Corresponding ArkTS definition:
/// ```typescript
/// type ThreeTypes = string | int | boolean;
/// native function handleThreeTypes(value: ThreeTypes): string;
/// ```
#[ani]
pub fn handle_three_types(value: Either3<String, i32, bool>) -> String {
    match value {
        Either3::A(s) => format!("String: {}", s),
        Either3::B(i) => format!("Int: {}", i),
        Either3::C(b) => format!("Boolean: {}", b),
    }
}

/// Handle four-type union using Either4
///
/// Corresponding ArkTS definition:
/// ```typescript
/// type FourTypes = string | int | boolean | double;
/// native function handleFourTypes(value: FourTypes): string;
/// ```
#[ani]
pub fn handle_four_types(value: Either4<String, i32, bool, f64>) -> String {
    match value {
        Either4::A(s) => format!("String: {}", s),
        Either4::B(i) => format!("Int: {}", i),
        Either4::C(b) => format!("Boolean: {}", b),
        Either4::D(d) => format!("Double: {}", d),
    }
}

/// Return union type - using Either
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function returnEither(useString: boolean): string | int;
/// ```
#[ani]
pub fn return_either(use_string: bool) -> Either<String, i32> {
    if use_string {
        Either::A("Hello from Either!".to_string())
    } else {
        Either::B(42)
    }
}

// ============================================================================
// Helper Function Examples
// ============================================================================

/// Handle simple numeric or string value
/// Returns type identifier code: 0=unknown, 1=int, 2=string
#[ani]
pub fn get_type_code(value_type: i32) -> i32 {
    match value_type {
        1 => 1, // int
        2 => 2, // string
        _ => 0, // unknown
    }
}

/// Create different values based on type
#[ani]
pub fn create_by_type(type_code: i32, int_val: i32, str_val: String) -> String {
    match type_code {
        1 => format!("Int: {}", int_val),
        2 => format!("String: {}", str_val),
        _ => "Unknown type".to_string(),
    }
}
