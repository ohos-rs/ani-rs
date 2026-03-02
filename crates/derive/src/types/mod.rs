//! Types Module - Type Conversion and Signature Generation
//!
//! Handles:
//! - ANI type signatures generation
//! - Rust to ANI type mapping
//! - Parameter and return value conversion code generation
//!
//! The type system is built around the `AniType` enum which provides
//! a structured representation of Rust types for ANI FFI code generation.

pub mod ani_type;
mod conversion;
mod ets;
mod signature;

pub use conversion::*;
pub use ets::*;
pub use signature::*;
