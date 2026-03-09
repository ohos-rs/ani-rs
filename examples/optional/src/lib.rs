//! Nullable Union Example - Handling `Option<T>` parameters
//!
//! Demonstrates how `Option<T>` is exported to ArkTS nullable unions (`T | null`).
//! This is not the same as an optional parameter (`param?: T`) and does not imply
//! that the argument may be omitted at the call site.
//!
//! Primitive `Option<T>` values use boxed ArkTS wrapper classes, for example
//! `Option<i32>` becomes `Int | null`.

use ani_derive::ani;

// ============================================================================
// Basic Optional Parameters - Using #[ani] macro and Option<T>
// ============================================================================

/// Handle optional int parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_optional_int(required: int, optional: Int | null): int;
/// ```
///
/// Mangling: X{C{std.core.Int}C{std.core.Null}}:I
/// `None` maps to `null`, and `Some(i32)` maps to boxed `Int`.
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
/// native function with_optional_double(required: double, optional: Double | null): double;
/// ```
///
/// Mangling: DX{C{std.core.Double}C{std.core.Null}}:D
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
/// native function with_optional_boolean(value: int, flag: Boolean | null): int;
/// ```
///
/// Mangling: IX{C{std.core.Boolean}C{std.core.Null}}:I
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
/// native function with_optional_string(required: string, optional: String | null): string;
/// ```
///
/// Mangling: Lstd/core/String;X{C{std.core.String}C{std.core.Null}}:Lstd/core/String;
/// Reference type nullable parameters are not boxed; `None` is passed as `null`.
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

/// Handle multiple nullable parameters
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_multiple_optional(
///     a: int,
///     b: Int | null,
///     c: Int | null,
///     d: Int | null
/// ): int;
/// ```
///
/// Mangling: IX{C{std.core.Int}C{std.core.Null}}X{C{std.core.Int}C{std.core.Null}}X{C{std.core.Int}C{std.core.Null}}:I
#[ani]
pub fn with_multiple_optional(a: i32, b: Option<i32>, c: Option<i32>, d: Option<i32>) -> i32 {
    a + b.unwrap_or(0) + c.unwrap_or(0) + d.unwrap_or(0)
}

// ============================================================================
// More Type Examples - Demonstrating generic support for other types
// ============================================================================

/// Handle nullable long (i64) parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_optional_long(required: long, optional: Long | null): long;
/// ```
#[ani]
pub fn with_optional_long(required: i64, optional: Option<i64>) -> i64 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// Handle nullable float (f32) parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_optional_float(required: float, optional: Float | null): float;
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

/// Plain required parameters example
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_default_simple(value: int, multiplier: int): int;
/// ```
///
/// Default values are an ArkTS-side concern. The generated native declaration still
/// uses required parameters unless a separate wrapper is written in ArkTS.
#[ani]
pub fn with_default_simple(value: i32, multiplier: i32) -> i32 {
    value * multiplier
}

/// Nullable parameter count
///
/// Placeholder helper used by smoke tests.
#[ani]
pub fn count_provided_args() -> i32 {
    // This function is for demonstration purposes only
    // In practice, each parameter needs to be checked for null
    0
}

// ============================================================================
// Module Initialization
// ============================================================================
