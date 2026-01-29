//! # ANI-RS Core Library
//!
//! Rust binding library for ArkTS 1.2 Native Interface (ANI), inspired by napi-rs.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use ani::prelude::*;
//! use ani_derive::ani;
//!
//! // Simple function binding - automatically registered!
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
//! // That's it! No ani_module! needed.
//! // ANI_Constructor is automatically generated on first #[ani] macro usage.
//! // Module name is derived from CARGO_PKG_NAME.
//! ```
//!
//! ## Architecture
//!
//! - `bindgen_runtime`: Type conversion traits and runtime support
//! - `env`: ANI environment wrapper
//! - `types`: ANI type wrappers (AniString, AniObject, etc.)
//! - `error`: Error handling
//! - `conversions`: Type conversion system (ToAni, FromAni traits)
//! - `module_register`: Automatic registration system using `ctor`

#![warn(missing_docs)]
#![allow(clippy::upper_case_acronyms)]

// Re-export ctor for use in generated code
pub use ctor;

// Re-export sys crate
pub use ani_sys as sys;

// Core modules
pub mod bindgen_runtime;
pub mod conversions;
pub mod env;
pub mod error;
pub mod module_register;
pub mod types;

/// Prelude module - commonly used types and traits
///
/// Import everything you need with:
/// ```rust
/// use ani::prelude::*;
/// ```
pub mod prelude {
    pub use crate::env::Env;
    pub use crate::error::{BusinessError, Error, Result, Status, check_status};
    pub use crate::types::{
        AniArray, AniArrayBuffer, AniArrayDouble, AniArrayInt, AniArrayLong, AniArrayRef, AniClass,
        AniEnum, AniError, AniField, AniFnObject, AniFunction, AniMethod, AniModule, AniNamespace,
        AniObject, AniRef, AniResolver, AniStaticField, AniStaticMethod, AniString, AniType,
        AniVariable, GlobalRef, WeakRef, ani_value_boolean, ani_value_byte, ani_value_char,
        ani_value_double, ani_value_float, ani_value_int, ani_value_long, ani_value_ref,
        ani_value_short, native_function,
    };

    // Deprecated type aliases for backward compatibility
    #[allow(deprecated)]
    pub use crate::error::{JsError, JsRangeError, JsTypeError};

    // Export from conversions module
    pub use crate::conversions::{
        AniThrowable, AniValue, Boxable, Either, Either3, Either4, Either5, Either6, Either7,
        Either8, Either9, Either10, Either11, Either12, Either13, Either14, Either15, Either16,
        FromAni, FromAniDirect, FromAniObject, NativePointer, Null, ToAni, ToAniDirect,
        ToAniObject, TypeInfo, Unboxable, Undefined, ValidateFromAni,
    };

    // Keep backward compatibility by also exporting from bindgen_runtime
    pub use crate::bindgen_runtime::{
        FromAni as BrFromAni, ToAni as BrToAni, TypeInfo as BrTypeInfo,
    };

    // Promise types
    pub use crate::conversions::{Deferred, PromiseRaw};

    // Function types
    pub use crate::conversions::{FnArgs, Function, FunctionRef, ToAniArg, ToAniArgs};

    // Reference types
    pub use crate::conversions::Ref;

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
