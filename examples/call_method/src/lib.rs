//! Call Method Example - Calling ArkTS methods from Native.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn call_add(_calc_ptr: i64, a: i32, b: i32) -> i32 {
    a + b
}

#[ani]
pub fn call_greet(greeter_ptr: i64, name: String) -> String {
    format!("Hello, {}! (from native, greeter: {})", name, greeter_ptr)
}

#[ani]
pub fn invoke_object_method_int(
    env: &Env<'_>,
    target: AniObject<'_>,
    method_name: String,
) -> Result<i32> {
    env.call_method_by_name_int(&target, &method_name, Some(":i"))
}

#[ani]
pub fn get_property_int(env: &Env<'_>, target: AniObject<'_>) -> Result<i32> {
    env.get_property_by_name_int(&target, "value")
}

#[ani]
pub fn call_static_square(n: i32) -> i32 {
    n * n
}

#[ani]
pub fn get_singleton() -> i64 {
    0x1234_5678
}

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

#[ani]
pub fn call_get_string(obj_ptr: i64) -> String {
    format!("String from object at {}", obj_ptr)
}

#[ani]
pub fn call_get_number(obj_ptr: i64) -> f64 {
    obj_ptr as f64 * 1.5
}
