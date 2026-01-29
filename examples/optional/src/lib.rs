//! Optional Parameter Example - Handling optional parameters
//!
//! Demonstrates how to handle ArkTS optional parameters (?)
//! Uses generic Option<T> implementation, all types implementing ToAni/FromAni support Option
//!
//! Note: Optional primitive types are automatically boxed, e.g., int? -> Lstd/core/Int;

use ani_derive::ani;

// ============================================================================
// Basic Optional Parameters - Using #[ani] macro and Option<T>
// ============================================================================

/// Handle optional int parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalInt(required: int, optional?: int): int;
/// ```
///
/// Mangling: ILstd/core/Int;:I
/// Option<i32> automatically handles boxed Int, None represents null
#[ani]
pub fn with_optional_int(required: i32, optional: Option<i32>) -> i32 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// Handle optional double parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalDouble(required: double, optional?: double): double;
/// ```
///
/// Mangling: DLstd/core/Double;:D
#[ani]
pub fn with_optional_double(required: f64, optional: Option<f64>) -> f64 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// Handle optional boolean parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalBoolean(value: int, flag?: boolean): int;
/// ```
///
/// Mangling: ILstd/core/Boolean;:I
#[ani]
pub fn with_optional_boolean(value: i32, flag: Option<bool>) -> i32 {
    match flag {
        Some(true) => value * 2,
        Some(false) | None => value,
    }
}

// ============================================================================
// Optional String Parameters (reference types don't need boxing)
// ============================================================================

/// Handle optional string parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalString(required: string, optional?: string): string;
/// ```
///
/// Mangling: Lstd/core/String;Lstd/core/String;:Lstd/core/String;
/// Reference type optional parameters are not boxed, null is passed directly
#[ani]
pub fn with_optional_string(required: String, optional: Option<String>) -> String {
    match optional {
        Some(opt_value) => format!("{} {}", required, opt_value),
        None => required,
    }
}

// ============================================================================
// Multiple Optional Parameters
// ============================================================================

/// Handle multiple optional parameters
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withMultipleOptional(
///     a: int,
///     b?: int,
///     c?: int,
///     d?: int
/// ): int;
/// ```
///
/// Mangling: ILstd/core/Int;Lstd/core/Int;Lstd/core/Int;:I
#[ani]
pub fn with_multiple_optional(a: i32, b: Option<i32>, c: Option<i32>, d: Option<i32>) -> i32 {
    a + b.unwrap_or(0) + c.unwrap_or(0) + d.unwrap_or(0)
}

// ============================================================================
// More Type Examples - Demonstrating generic support for other types
// ============================================================================

/// Handle optional long (i64) parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalLong(required: long, optional?: long): long;
/// ```
#[ani]
pub fn with_optional_long(required: i64, optional: Option<i64>) -> i64 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// Handle optional float (f32) parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withOptionalFloat(required: float, optional?: float): float;
/// ```
#[ani]
pub fn with_optional_float(required: f32, optional: Option<f32>) -> f32 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

// ============================================================================
// Simplified Version (using macro wrapper)
// ============================================================================

/// Simplified version using macro - parameter with default value
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function withDefault(value: int, multiplier: int = 1): int;
/// ```
///
/// Note: Parameters with default values are handled the same as optional parameters
#[ani]
pub fn with_default_simple(value: i32, multiplier: i32) -> i32 {
    value * multiplier
}

/// Optional parameter count
///
/// Count how many non-null optional parameters
#[ani]
pub fn count_provided_args() -> i32 {
    // This function is for demonstration purposes only
    // In practice, each parameter needs to be checked for null
    0
}

// ============================================================================
// Module Initialization
// ============================================================================
