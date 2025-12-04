#![warn(missing_docs)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! # Safe ANI Bindings in Rust
//!
//! This crate provides a (mostly) safe way to implement native methods
//! using the ANI (Application Native Interface).

/// `ani-sys` re-exports
pub mod sys;

mod wrapper {
    mod version;
    pub use self::version::*;

    #[macro_use]
    mod macros;

    /// Errors.
    pub mod errors;

    /// Descriptors for classes and method IDs.
    pub mod descriptors;

    /// Parser for type signatures.
    pub mod signature;

    /// Wrappers for object pointers returned from the VM.
    pub mod objects;

    /// Handling of strings, including UTF-8 and UTF-16 conversions.
    pub mod strings;

    /// Actual communication with the VM.
    pub mod anienv;
    pub use self::anienv::*;

    /// ANI VM interface.
    mod java_vm;
    pub use self::java_vm::*;

    /// Optional thread attachment manager.
    mod executor;
    pub use self::executor::*;
}

pub use wrapper::*;

// Re-export ANI types at crate root for convenience
pub use wrapper::anienv::ANIEnv;
pub use wrapper::ANIVersion;
pub use wrapper::AniVM;
pub use wrapper::AttachGuard;

// JNI compatibility aliases
pub use wrapper::anienv::ANIEnv as JNIEnv;
pub use wrapper::AniVM as JavaVM;
pub use wrapper::ANIVersion as JNIVersion;
