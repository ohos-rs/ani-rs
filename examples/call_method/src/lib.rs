//! 调用方法示例 - 在 Native 调用 ArkTS 方法
//!
//! 演示如何从 Rust 调用 ArkTS 对象的方法

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// 调用对象方法
// ============================================================================

/// 调用对象的 int 方法
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Calculator {
///     add(a: int, b: int): int { return a + b; }
/// }
/// native function callAdd(calc: Calculator, a: int, b: int): int;
/// ```
#[ani]
pub fn call_add(_calc_ptr: i64, a: i32, b: i32) -> i32 {
    // 这里演示直接计算，实际应该调用对象方法
    // 真实调用需要使用 env.call_method_by_name_int()
    a + b
}

/// 调用对象的字符串方法
///
/// 对应的 ArkTS 定义:
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
// 使用低级 API 调用方法
// ============================================================================

/// 通过 env 调用对象方法的示例
///
/// 这个函数展示了如何使用 Env API 调用 ArkTS 对象的方法
///
/// 对应的 ArkTS 定义:
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

        // 获取方法名
        let method_name_str = match env.get_string(&method_str) {
            Ok(s) => s,
            Err(_) => return -2,
        };

        // 调用方法（无参数，返回 int）
        match env.call_method_by_name_int(&target_obj, &method_name_str, Some(":I")) {
            Ok(result) => result,
            Err(_) => -3,
        }
    }
}

/// 调用对象的 getter 方法
///
/// 对应的 ArkTS 定义:
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

        // 获取 int 属性值
        match env.get_property_by_name_int(&target_obj, "value") {
            Ok(result) => result,
            Err(_) => -2,
        }
    }
}

// ============================================================================
// 调用静态方法
// ============================================================================

/// 调用类的静态方法
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class MathUtils {
///     static square(n: int): int { return n * n; }
/// }
/// native function callStaticSquare(n: int): int;
/// ```
#[ani]
pub fn call_static_square(n: i32) -> i32 {
    // 简化版本 - 直接计算
    // 实际需要通过 Class_FindMethod 和 Class_CallStaticMethod
    n * n
}

/// 调用类的静态方法获取单例
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Singleton {
///     static getInstance(): Singleton;
/// }
/// native function getSingleton(): long;
/// ```
#[ani]
pub fn get_singleton() -> i64 {
    // 返回模拟的单例指针
    0x12345678
}

// ============================================================================
// 调用带多参数的方法
// ============================================================================

/// 调用带多个参数的方法
///
/// 对应的 ArkTS 定义:
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
// 调用带返回对象的方法
// ============================================================================

/// 调用返回字符串的方法
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function callGetString(obj: Object): string;
/// ```
#[ani]
pub fn call_get_string(obj_ptr: i64) -> String {
    format!("String from object at {}", obj_ptr)
}

/// 调用返回数字的方法
#[ani]
pub fn call_get_number(obj_ptr: i64) -> f64 {
    obj_ptr as f64 * 1.5
}

// ============================================================================
// 模块初始化
// ============================================================================
