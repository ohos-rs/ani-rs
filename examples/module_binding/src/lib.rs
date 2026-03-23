//! `#[ani(module = "...")]` binding example.

use ani_derive::ani;

/// Bind a native function to an explicit module descriptor.
///
/// Note: the module descriptor must be resolvable by ArkVM `FindModule`.
#[ani(module = "arkvm_test")]
pub fn module_add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(module = "arkvm_test")]
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
