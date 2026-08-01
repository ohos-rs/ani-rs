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
//! // Set ANI_MODULE_DESCRIPTOR for a fully-qualified Stage/HAP descriptor.
//! ```
//!
//! ## Architecture
//!
//! - `bindgen_runtime`: Type conversion traits and runtime support
//! - `env`: ANI environment wrapper
//! - `vm`: ANI VM wrapper
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
#[cfg(feature = "serde-json")]
pub use serde;
#[cfg(feature = "serde-json")]
pub use serde_json;

// Core modules
pub mod async_runtime;
pub mod bindgen_runtime;
pub mod conversions;
#[macro_use]
pub mod env;
pub mod error;
pub mod module_register;
pub mod platform;
pub mod runtime;
pub mod scheduler;
pub mod tokio;
pub mod types;
pub mod vm;

/// Prelude module - commonly used types and traits
///
/// Import everything you need with:
/// ```rust
/// use ani::prelude::*;
/// ```
pub mod prelude {
    pub use crate::async_runtime::{
        AsyncRuntime, AsyncRuntimeGuard, AsyncRuntimeMetrics, AsyncRuntimeRejection,
        RuntimeBlockingTask, RuntimeCancelReason, RuntimeTask, RuntimeTaskHandle,
        activate_async_runtime, register_async_runtime, register_cancellation_error_factory,
        runtime_cancellation_error, shutdown_runtime_domain,
        spawn_future_result_factory_with_handle, try_register_async_runtime,
    };
    pub use crate::env::{Env, LocalScopeGuard};
    pub use crate::error::{
        AniErrorPayload, AniErrorValue, BusinessError, DynAniError, Error, PreservedArktsError,
        Result, Status, check_status,
    };
    pub use crate::runtime::{RuntimeMetrics, assert_no_runtime_leaks, runtime_metrics};
    pub use crate::scheduler::{RuntimeKernel, SchedulerMetrics, runtime_kernel, shutdown_runtime};
    pub use crate::types::{
        AniArray, AniArrayBuffer, AniArrayDouble, AniArrayInt, AniArrayLong, AniArrayRef, AniClass,
        AniEnum, AniEnumItem, AniError, AniField, AniFixedArray, AniFixedArrayBoolean,
        AniFixedArrayByte, AniFixedArrayChar, AniFixedArrayDouble, AniFixedArrayFloat,
        AniFixedArrayInt, AniFixedArrayLong, AniFixedArrayRef, AniFixedArrayShort, AniFnObject,
        AniFunction, AniMethod, AniModule, AniNamespace, AniObject, AniRef, AniResolver,
        AniStaticField, AniStaticMethod, AniString, AniTupleValue, AniType, AniVariable, GlobalRef,
        WeakRef, ani_value_boolean, ani_value_byte, ani_value_char, ani_value_double,
        ani_value_float, ani_value_int, ani_value_long, ani_value_ref, ani_value_short,
        native_function,
    };
    pub use crate::vm::{AniVm, AttachGuard, VmOptions};

    // Deprecated type aliases for backward compatibility
    #[allow(deprecated)]
    pub use crate::error::{JsError, JsRangeError, JsTypeError};

    // Export all conversion types/traits/helpers.
    pub use crate::conversions::*;

    // Keep backward compatibility by also exporting from bindgen_runtime
    pub use crate::bindgen_runtime::{
        FromAni as BrFromAni, ToAni as BrToAni, TypeInfo as BrTypeInfo,
    };

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
