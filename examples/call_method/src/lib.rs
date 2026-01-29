//! Call Method Example - Calling ArkTS methods from Native
//!
//! Demonstrates how to call ArkTS object methods from Rust

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// Calling Object Methods
// ============================================================================

/// Call object's int method
///
/// Corresponding ArkTS definition:
/// ```typescript
/// class Calculator {
///     add(a: int, b: int): int { return a + b; }
/// }
/// native function callAdd(calc: Calculator, a: int, b: int): int;
/// ```
#[ani]
pub fn call_add(_calc_ptr: i64, a: i32, b: i32) -> i32 {
    // Here we demonstrate direct calculation, actual implementation should call object method
    // Real call needs to use env.call_method_by_name_int()
    a + b
}

/// Call object's string method
///
/// Corresponding ArkTS definition:
/// ```typescript
/// class Greeter {
///     greet(name: string): string { return "Hello, " + name; }
/// }
/// native function callGreet(greeter: Greeter, name: string): string;
/// ```
#[ani]
pub fn call_greet(greeter_ptr: i64, name: String) -> String {
    format!("Hello, {}! (from native, greeter: {})", name, greeter_ptr)
}

// ============================================================================
// Using Low-Level API to Call Methods
// ============================================================================

/// Example of calling object method through env
///
/// This function demonstrates how to use Env API to call ArkTS object methods
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function invokeMethod(obj: Object, methodName: string): int;
/// ```
#[no_mangle]
pub extern "C" fn invoke_object_method_int(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    target: sys::ani_object,
    method_name: sys::ani_string,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return -1,
        };

        let target_obj = AniObject::from_raw(target);
        let method_str = AniString::from_raw(method_name);

        // Get method name
        let method_name_str = match env.get_string(&method_str) {
            Ok(s) => s,
            Err(_) => return -2,
        };

        // Call method (no arguments, returns int)
        match env.call_method_by_name_int(&target_obj, &method_name_str, Some(":I")) {
            Ok(result) => result,
            Err(_) => -3,
        }
    }
}

/// Call object's getter method
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function getProperty(obj: Object, propName: string): int;
/// ```
#[no_mangle]
pub extern "C" fn get_property_int(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    target: sys::ani_object,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return -1,
        };

        let target_obj = AniObject::from_raw(target);

        // Get int property value
        match env.get_property_by_name_int(&target_obj, "value") {
            Ok(result) => result,
            Err(_) => -2,
        }
    }
}

// ============================================================================
// Calling Static Methods
// ============================================================================

/// Call class's static method
///
/// Corresponding ArkTS definition:
/// ```typescript
/// class MathUtils {
///     static square(n: int): int { return n * n; }
/// }
/// native function callStaticSquare(n: int): int;
/// ```
#[ani]
pub fn call_static_square(n: i32) -> i32 {
    // Simplified version - direct calculation
    // Actual implementation needs Class_FindMethod and Class_CallStaticMethod
    n * n
}

/// Call class's static method to get singleton
///
/// Corresponding ArkTS definition:
/// ```typescript
/// class Singleton {
///     static getInstance(): Singleton;
/// }
/// native function getSingleton(): long;
/// ```
#[ani]
pub fn get_singleton() -> i64 {
    // Return simulated singleton pointer
    0x12345678
}

// ============================================================================
// Calling Methods with Multiple Arguments
// ============================================================================

/// Call method with multiple arguments
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function callWithMultipleArgs(
///     obj: Object,
///     intVal: int,
///     doubleVal: double,
///     strVal: string
/// ): string;
/// ```
#[ani]
pub fn call_with_multiple_args(
    obj_ptr: i64,
    int_val: i32,
    double_val: f64,
    str_val: String,
) -> String {
    format!(
        "Called with: obj={}, int={}, double={:.2}, str={}",
        obj_ptr, int_val, double_val, str_val
    )
}

// ============================================================================
// Calling Methods that Return Objects
// ============================================================================

/// Call method returning string
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function callGetString(obj: Object): string;
/// ```
#[ani]
pub fn call_get_string(obj_ptr: i64) -> String {
    format!("String from object at {}", obj_ptr)
}

/// Call method returning number
#[ani]
pub fn call_get_number(obj_ptr: i64) -> f64 {
    obj_ptr as f64 * 1.5
}

// ============================================================================
// Module Initialization
// ============================================================================
