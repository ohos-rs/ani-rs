//! # ANI-RS Core Library
//!
//! Rust binding library for ArkTS 1.2 Native Interface (ANI), inspired by napi-rs.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! // Simple function binding
//! #[ani]
//! fn add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//!
//! // Class method binding
//! #[ani(class = "Calculator")]
//! fn multiply(this: i64, a: f64, b: f64) -> f64 {
//!     a * b
//! }
//!
//! // Module registration
//! ani_module! {
//!     name: "my_module",
//!     lib_name: "libmy_module",
//!     functions: [add, multiply],
//! }
//! ```
//!
//! ## Architecture
//!
//! - `bindgen_runtime`: Type conversion traits and runtime support
//! - `env`: ANI environment wrapper
//! - `types`: ANI type wrappers (AniString, AniObject, etc.)
//! - `error`: Error handling
//! - `conversions`: Type conversion system (ToAni, FromAni traits)

#![warn(missing_docs)]
#![allow(clippy::upper_case_acronyms)]

// 重新导出 sys crate
pub use ani_sys as sys;

// 核心模块
pub mod bindgen_runtime;
pub mod conversions;
pub mod env;
pub mod error;
pub mod types;

// re-export
pub use ani_derive::{AniClass, ani, ani_module};

/// Prelude module - commonly used types and traits
///
/// Import everything you need with:
/// ```rust
/// use ani::prelude::*;
/// ```
pub mod prelude {
    pub use crate::env::Env;
    pub use crate::error::{Error, Result};
    pub use crate::types::*;

    // Export from conversions module
    pub use crate::conversions::{
        AniThrowable, AniValue, Boxable, FromAni, FromAniDirect, NativePointer, ToAni, ToAniDirect,
        TypeInfo, Unboxable,
    };

    // Keep backward compatibility by also exporting from bindgen_runtime
    pub use crate::bindgen_runtime::{
        FromAni as BrFromAni, ToAni as BrToAni, TypeInfo as BrTypeInfo,
    };

    pub use ani_derive::{AniClass, ani, ani_module};

    pub use crate::sys::{ANI_VERSION_1, ani_status_ANI_OK as ANI_OK};
}

/// ANI version info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AniVersion(pub u32);

impl AniVersion {
    /// ANI Version 1
    pub const V1: AniVersion = AniVersion(sys::ANI_VERSION_1);
}

impl From<u32> for AniVersion {
    fn from(v: u32) -> Self {
        AniVersion(v)
    }
}

impl From<AniVersion> for u32 {
    fn from(v: AniVersion) -> Self {
        v.0
    }
}
