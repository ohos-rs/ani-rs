//! Nullable Union Example - Handling `Option<T>` parameters
//!
//! Demonstrates how `Option<T>` is exported to ArkTS nullish unions (`T | null | undefined`).
//! This is not the same as an optional parameter (`param?: T`) and does not imply
//! that the argument may be omitted at the call site.
//!
//! Primitive `Option<T>` values keep ArkTS primitive unions at the ETS boundary, for example
//! `Option<i32>` becomes `int | null | undefined`.

use ani_derive::ani;

// ============================================================================
// Basic Optional Parameters - Using #[ani] macro and Option<T>
// ============================================================================

/// Handle optional int parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function with_optional_int(required: int, optional: int | null | undefined): int;
/// ```
///
/// Mangling: X{C{std.core.Int}C{std.core.Null}U}:I
/// `None` maps to `undefined`, and `Some(i32)` maps to `int` at the ETS boundary.
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
/// native function with_optional_double(required: double, optional: double | null | undefined): double;
/// ```
///
/// Mangling: DX{C{std.core.Double}C{std.core.Null}U}:D
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
/// native function with_optional_boolean(value: int, flag: boolean | null | undefined): int;
/// ```
///
/// Mangling: IX{C{std.core.Boolean}C{std.core.Null}U}:I
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
/// native function with_optional_string(required: string, optional: string | null | undefined): string;
/// ```
///
/// Mangling: Lstd/core/String;X{C{std.core.String}C{std.core.Null}U}:Lstd/core/String;
/// Reference type nullish parameters are not boxed; `None` is passed as `undefined`.
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
///     b: int | null | undefined,
///     c: int | null | undefined,
///     d: int | null | undefined
/// ): int;
/// ```
///
/// Mangling: IX{C{std.core.Int}C{std.core.Null}U}X{C{std.core.Int}C{std.core.Null}U}X{C{std.core.Int}C{std.core.Null}U}:I
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
/// native function with_optional_long(required: long, optional: long | null | undefined): long;
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
/// native function with_optional_float(required: float, optional: float | null | undefined): float;
/// ```
#[ani]
pub fn with_optional_float(required: f32, optional: Option<f32>) -> f32 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// Return optional int to exercise Option<T> return conversion.
#[ani]
pub fn make_optional_int(use_value: bool) -> Option<i32> {
    if use_value {
        Some(7)
    } else {
        None
    }
}

/// Return optional string to exercise reference-type nullish returns.
#[ani]
pub fn make_optional_string(use_value: bool) -> Option<String> {
    if use_value {
        Some("ok".to_string())
    } else {
        None
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
