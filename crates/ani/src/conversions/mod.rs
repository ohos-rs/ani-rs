//! Conversions between Rust and ArkTS Types
//!
//! This module contains all type conversion implementations, grouped by type category into separate files.
//!
//! ## Module Organization
//!
//! - `primitives` - Primitive type conversions (int, long, double, etc.)
//! - `string` - String type conversions
//! - `array` - Array type conversions
//! - `object` - Object type conversions
//! - `boxed` - Boxed type conversions (Int, Long, Double, etc.)
//! - `optional` - Optional type conversions
//! - `either` - Union/Either type conversions
//! - `null` / `undefined` - Null and Undefined type conversions
//! - `promise` - Promise and Deferred types
//! - `result` - Result type conversions
//! - `collections` - Collection type conversions
//! - `traits` - Core conversion traits

mod array;
mod boxed;
mod collections;
mod either;
mod null;
mod object;
mod optional;
mod primitives;
mod promise;
mod result;
mod string;
mod traits;
mod undefined;

// Re-export all public items
pub use boxed::*;
pub use collections::*;
pub use either::*;
pub use null::*;
pub use object::*;
pub use optional::*;
pub use promise::*;
pub use result::*;
pub use string::*;
pub use traits::*;
pub use undefined::*;
