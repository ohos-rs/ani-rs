//! Class Binding Example - Demonstrates how to bind Rust structs to ArkTS classes
//!
//! This example shows class methods, constructors, getter/setter bindings

use ani_derive::ani;
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex, OnceLock,
};

// ============================================================================
// Calculator Class - Demonstrates class method binding
// ============================================================================

/// Calculator class
///
/// Corresponding ArkTS definition:
/// ```typescript
/// class Calculator {
///     native static create(): Calculator;
///     native add(a: int, b: int): int;
///     native subtract(a: int, b: int): int;
///     native multiply(a: double, b: double): double;
///     native divide(a: double, b: double): double;
/// }
/// ```
pub struct Calculator {
    _value: f64,
}

/// Create calculator (static method)
#[ani(class = "Calculator", static)]
pub fn create() -> i64 {
    let calc = Box::new(Calculator { _value: 0.0 });
    Box::into_raw(calc) as i64
}

/// Addition (instance method)
#[ani(class = "Calculator")]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtraction (instance method)
#[ani(class = "Calculator")]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplication (instance method)
#[ani(class = "Calculator")]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Division (instance method)
#[ani(class = "Calculator")]
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        0.0
    } else {
        a / b
    }
}

// ============================================================================
// Person Class - Demonstrates stateful class
// ============================================================================

static PERSON_AGE: AtomicI32 = AtomicI32::new(0);
static PERSON_NAME: OnceLock<Mutex<String>> = OnceLock::new();

fn person_name_store() -> &'static Mutex<String> {
    PERSON_NAME.get_or_init(|| Mutex::new(String::new()))
}

/// Create Person (constructor)
#[ani(class = "Person", constructor)]
pub fn person_new(name: String, age: i32) {
    PERSON_AGE.store(age, Ordering::SeqCst);
    if let Ok(mut slot) = person_name_store().lock() {
        *slot = name;
    }
}

/// Get name
#[ani(class = "Person", getter = "name")]
pub fn person_get_name() -> String {
    person_name_store()
        .lock()
        .map(|s| s.clone())
        .unwrap_or_default()
}

/// Get age
#[ani(class = "Person", getter = "age")]
pub fn person_get_age() -> i32 {
    PERSON_AGE.load(Ordering::SeqCst)
}

/// Set age
#[ani(class = "Person", setter = "age")]
pub fn person_set_age(age: i32) {
    PERSON_AGE.store(age, Ordering::SeqCst);
}

/// Greeting
#[ani(class = "Person")]
pub fn greet() -> String {
    let name = person_get_name();
    let age = person_get_age();
    format!("Hello, I'm {} and I'm {} years old!", name, age)
}

/// Release Person
#[ani(class = "Person", name = "destroy")]
pub fn person_destroy() {
    PERSON_AGE.store(0, Ordering::SeqCst);
    if let Ok(mut slot) = person_name_store().lock() {
        slot.clear();
    }
}

// ============================================================================
// No ani_module! macro needed!
// ANI_Constructor is automatically generated on first #[ani] macro expansion
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let calc_token = create();
        assert_ne!(calc_token, 0);

        assert_eq!(add(2, 3), 5);
        assert_eq!(subtract(5, 3), 2);
        assert!((multiply(2.0, 3.0) - 6.0).abs() < f64::EPSILON);
        assert!((divide(6.0, 2.0) - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_person() {
        person_new("Alice".to_string(), 30);

        assert_eq!(person_get_name(), "Alice");
        assert_eq!(person_get_age(), 30);

        person_set_age(31);
        assert_eq!(person_get_age(), 31);

        assert_eq!(greet(), "Hello, I'm Alice and I'm 31 years old!");

        // Cleanup
        person_destroy();
    }
}
