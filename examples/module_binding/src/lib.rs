//! `#[ani(module = "...")]` binding example.
//!
//! This example intentionally uses a module descriptor that does not match the
//! crate/generated ETS module name so ArkVM smoke covers explicit override
//! binding instead of the trivial "same name" path.

use ani_derive::ani;

/// Bind a native function to an explicit module descriptor.
///
/// Note: the module descriptor must be resolvable by ArkVM `FindModule`.
#[ani(module = "explicit_module_binding")]
pub fn module_add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(module = "explicit_module_binding")]
pub fn module_greet(name: String) -> String {
    format!("hello {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = module_add;
        let _ = module_greet;
    }
}
