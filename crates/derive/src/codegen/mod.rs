//! Codegen Module - Code Generation
//!
//! Generates ANI binding code including:
//! - Function wrappers
//! - Registration functions

mod register;
mod wrapper;

pub use register::*;
pub use wrapper::*;
