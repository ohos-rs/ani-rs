//! Class Binding Example - Demonstrates how to bind Rust structs to ArkTS classes
//!
//! This example shows class methods, constructors, getter/setter bindings

use ani_derive::ani;

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
pub fn add(_this: i64, a: i32, b: i32) -> i32 {
    a + b
}

/// Subtraction (instance method)
#[ani(class = "Calculator")]
pub fn subtract(_this: i64, a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplication (instance method)
#[ani(class = "Calculator")]
pub fn multiply(_this: i64, a: f64, b: f64) -> f64 {
    a * b
}

/// Division (instance method)
#[ani(class = "Calculator")]
pub fn divide(_this: i64, a: f64, b: f64) -> f64 {
    if b == 0.0 {
        0.0
    } else {
        a / b
    }
}

// ============================================================================
// Person Class - Demonstrates stateful class
// ============================================================================

/// Person class
pub struct Person {
    name: String,
    age: i32,
}

/// Create Person (constructor)
#[ani(class = "Person", constructor)]
pub fn person_new(name: String, age: i32) -> i64 {
    let person = Box::new(Person { name, age });
    Box::into_raw(person) as i64
}

/// Get name
#[ani(class = "Person", name = "getName")]
pub fn person_get_name(this: i64) -> String {
    unsafe {
        let person = &*(this as *const Person);
        person.name.clone()
    }
}

/// Get age
#[ani(class = "Person", name = "getAge")]
pub fn person_get_age(this: i64) -> i32 {
    unsafe {
        let person = &*(this as *const Person);
        person.age
    }
}

/// Set age
#[ani(class = "Person", name = "setAge")]
pub fn person_set_age(this: i64, age: i32) {
    unsafe {
        let person = &mut *(this as *mut Person);
        person.age = age;
    }
}

/// Greeting
#[ani(class = "Person")]
pub fn greet(this: i64) -> String {
    unsafe {
        let person = &*(this as *const Person);
        format!(
            "Hello, I'm {} and I'm {} years old!",
            person.name, person.age
        )
    }
}

/// Release Person
#[ani(class = "Person", name = "destroy")]
pub fn person_destroy(this: i64) {
    unsafe {
        let _ = Box::from_raw(this as *mut Person);
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
        let calc_ptr = create();
        assert_ne!(calc_ptr, 0);

        assert_eq!(add(calc_ptr, 2, 3), 5);
        assert_eq!(subtract(calc_ptr, 5, 3), 2);
        assert!((multiply(calc_ptr, 2.0, 3.0) - 6.0).abs() < f64::EPSILON);
        assert!((divide(calc_ptr, 6.0, 2.0) - 3.0).abs() < f64::EPSILON);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(calc_ptr as *mut Calculator);
        }
    }

    #[test]
    fn test_person() {
        let person_ptr = person_new("Alice".to_string(), 30);
        assert_ne!(person_ptr, 0);

        assert_eq!(person_get_name(person_ptr), "Alice");
        assert_eq!(person_get_age(person_ptr), 30);

        person_set_age(person_ptr, 31);
        assert_eq!(person_get_age(person_ptr), 31);

        assert_eq!(greet(person_ptr), "Hello, I'm Alice and I'm 31 years old!");

        // Cleanup
        person_destroy(person_ptr);
    }
}
