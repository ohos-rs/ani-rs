//! Basic Example - Demonstrates simple usage of ani-rs
//!
//! This example shows how to use the `#[ani]` macro to create ANI bindings, similar to napi-rs.
//!
//! `#[ani]` is a unified macro that can be used for:
//! - Module-level function binding (auto-registration, no manual listing needed!)
//! - Class method binding (instance methods and static methods)
//! - Namespace function binding
//! - Initialization function marking
//!
//! Uses the `ctor` crate to implement napi-rs-like auto-registration mechanism,
//! all functions marked with `#[ani]` are automatically registered to the global registry when the library loads.

use ani_derive::ani;

// ============================================================================
// Basic Math Functions - Module Level (Auto-registered!)
// ============================================================================

/// Add two numbers
#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtract two numbers
#[ani]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiply two numbers
#[ani]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divide two numbers (returns 0 when b is 0)
#[ani]
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

// ============================================================================
// String Operations
// ============================================================================

/// Greeting function
#[ani]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// String length
#[ani]
pub fn string_length(s: String) -> i32 {
    s.len() as i32
}

// ============================================================================
// Advanced Math Functions
// ============================================================================

/// Calculate factorial
#[ani]
pub fn factorial(n: i32) -> i64 {
    if n <= 1 {
        1
    } else {
        (1..=n as i64).product()
    }
}

/// Check if a number is prime
#[ani]
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Calculate greatest common divisor
#[ani]
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Calculate Fibonacci number
#[ani]
pub fn fibonacci(n: i32) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0i64;
    let mut b = 1i64;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

// ============================================================================
// No ani_module! macro needed!
// ANI_Constructor is automatically generated on first #[ani] macro expansion
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World".to_string()), "Hello, World!");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
