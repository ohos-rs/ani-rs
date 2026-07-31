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
//! - `bigint` - Arbitrary-precision ArkTS bigint conversion
//! - `optional` - Optional type conversions
//! - `either` - Union/Either type conversions
//! - `null` / `undefined` - Null and Undefined type conversions
//! - `promise` - Promise and Deferred types
//! - `resource` - Managed native resource handles
//! - `result` - Result type conversions
//! - `collections` - Collection type conversions
//! - `fixed_array` - Fixed array wrapper conversions
//! - `tuple_value` - Tuple value wrappers
//! - `enum_item` - Enum item wrappers
//! - `any_value` - Dynamic Any wrapper
//! - `traits` - Core conversion traits

mod any_value;
mod array;
mod arraybuffer;
mod bigint;
mod boxed;
mod collections;
mod either;
mod enum_item;
mod fixed_array;
mod function;
#[cfg(feature = "serde-json")]
mod json;
mod null;
mod object;
mod optional;
mod primitives;
mod promise;
mod reference;
mod resource;
mod result;
mod stream;
mod string;
mod task;
mod traits;
mod tuple_value;
mod typed_array;
mod undefined;

// Re-export all public items
pub use any_value::*;
pub use arraybuffer::*;
pub use bigint::*;
pub use boxed::*;
pub use collections::*;
pub use either::*;
pub use enum_item::*;
pub use fixed_array::*;
pub use function::*;
#[cfg(feature = "serde-json")]
pub use json::*;
pub use null::*;
pub use object::*;
pub use optional::*;
pub use promise::*;
pub use reference::*;
pub use resource::*;
pub use result::*;
pub use stream::*;
pub use string::*;
pub use task::*;
pub use traits::*;
pub use tuple_value::*;
pub use typed_array::*;
pub use undefined::*;
