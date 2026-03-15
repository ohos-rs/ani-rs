//! Overloaded module and namespace export example.
//!
//! This mirrors OpenHarmony ANI bind_ops coverage for same-name native
//! functions exported at module scope and within namespaces.

use ani_derive::ani;

#[ani(name = "sum")]
pub fn sum2(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(name = "sum")]
pub fn sum3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[ani(name = "concat")]
pub fn concat2(left: String, right: String) -> String {
    format!("{left}{right}")
}

#[ani(name = "concat")]
pub fn concat3(a: String, b: String, c: String) -> String {
    format!("{a}{b}{c}")
}

#[ani(namespace = "ops", name = "sum")]
pub fn ops_sum2(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(namespace = "ops", name = "sum")]
pub fn ops_sum3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[ani(namespace = "ops", name = "concat")]
pub fn ops_concat2(left: String, right: String) -> String {
    format!("{left}{right}")
}

#[ani(namespace = "A", name = "recursiveFunction")]
pub fn recursive_function(value: i32) -> i32 {
    if value <= 0 {
        0
    } else {
        value + recursive_function(value - 1)
    }
}

#[ani(namespace = "A.B", name = "sumB")]
pub fn nested_sum_b(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overloaded_exports_match_expected_values() {
        assert_eq!(sum2(8, 16), 24);
        assert_eq!(sum3(8, 16, 6), 30);
        assert_eq!(concat2("abc".to_string(), "def".to_string()), "abcdef");
        assert_eq!(
            concat3("abc".to_string(), "def".to_string(), "ghi".to_string()),
            "abcdefghi"
        );
        assert_eq!(ops_sum2(8, 16), 24);
        assert_eq!(ops_sum3(8, 16, 6), 30);
        assert_eq!(ops_concat2("abc".to_string(), "def".to_string()), "abcdef");
        assert_eq!(recursive_function(5), 15);
        assert_eq!(nested_sum_b(8, 16), 24);
    }
}
