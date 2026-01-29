//! Template 示例 - 处理模板类参数
//!
//! 演示如何处理 ArkTS 的泛型参数
//! 泛型参数在 ANI 中统一映射为 Lstd/core/Object;

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// 基本泛型函数
// ============================================================================

/// 处理泛型参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function identity<T>(value: T): T;
/// ```
///
/// Mangling: Lstd/core/Object;:Lstd/core/Object;
/// 泛型 T 统一映射为 Object
#[no_mangle]
pub extern "C" fn identity(
    _env: *mut sys::ani_env,
    _obj: sys::ani_object,
    value: sys::ani_object,
) -> sys::ani_object {
    // 直接返回传入的对象
    value
}

/// 交换两个泛型值
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function swap<T, U>(a: T, b: U): [U, T];
/// ```
///
/// 注意: 返回元组需要创建数组或元组对象
#[no_mangle]
pub extern "C" fn get_first(
    _env: *mut sys::ani_env,
    _obj: sys::ani_object,
    first: sys::ani_object,
    _second: sys::ani_object,
) -> sys::ani_object {
    first
}

#[no_mangle]
pub extern "C" fn get_second(
    _env: *mut sys::ani_env,
    _obj: sys::ani_object,
    _first: sys::ani_object,
    second: sys::ani_object,
) -> sys::ani_object {
    second
}

// ============================================================================
// Array<T> 泛型
// ============================================================================

/// 获取 Array<T> 的长度
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function arrayLength<T>(arr: Array<T>): int;
/// ```
///
/// 注意: Array<T> 的 Mangling 是 Lescompat/Array; 而不是 ani_array
#[no_mangle]
pub extern "C" fn array_length(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    arr: sys::ani_object,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return -1,
        };

        let arr_obj = AniObject::from_raw(arr);

        // 调用 Array 的 length 属性
        match env.get_property_by_name_int(&arr_obj, "length") {
            Ok(len) => len,
            Err(_) => -1,
        }
    }
}

/// 获取 Array<T> 的元素
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function arrayGet<T>(arr: Array<T>, index: int): T;
/// ```
#[no_mangle]
pub extern "C" fn array_get(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    arr: sys::ani_object,
    index: sys::ani_int,
) -> sys::ani_object {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return std::ptr::null_mut(),
        };

        let arr_obj = AniObject::from_raw(arr);

        // 调用 Array 的 $_get 方法
        // Mangling: I:Lstd/core/Object;
        match env.call_method_by_name_ref(&arr_obj, "$_get", Some("I:Lstd/core/Object;"), index) {
            Ok(elem) => elem.as_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

// ============================================================================
// 泛型容器
// ============================================================================

/// 创建泛型容器
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Container<T> {
///     private value: T;
///     constructor(v: T) { this.value = v; }
///     getValue(): T { return this.value; }
///     setValue(v: T): void { this.value = v; }
/// }
///
/// native function createIntContainer(value: int): long;
/// ```
#[ani]
pub fn create_int_container(value: i32) -> i64 {
    let container = Box::new(GenericContainer { value });
    Box::into_raw(container) as i64
}

/// 获取容器值
#[ani]
pub fn container_get_int(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let container = &*(ptr as *const GenericContainer<i32>);
        container.value
    }
}

/// 设置容器值
#[ani]
pub fn container_set_int(ptr: i64, value: i32) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let container = &mut *(ptr as *mut GenericContainer<i32>);
        container.value = value;
    }
}

/// 释放容器
#[ani]
pub fn destroy_int_container(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut GenericContainer<i32>);
        }
    }
}

struct GenericContainer<T> {
    value: T,
}

// ============================================================================
// 字符串泛型容器
// ============================================================================

#[ani]
pub fn create_string_container(value: String) -> i64 {
    let container = Box::new(GenericContainer { value });
    Box::into_raw(container) as i64
}

#[ani]
pub fn container_get_string(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let container = &*(ptr as *const GenericContainer<String>);
        container.value.clone()
    }
}

#[ani]
pub fn container_set_string(ptr: i64, value: String) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let container = &mut *(ptr as *mut GenericContainer<String>);
        container.value = value;
    }
}

#[ani]
pub fn destroy_string_container(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut GenericContainer<String>);
        }
    }
}

// ============================================================================
// 泛型对容器
// ============================================================================

/// 创建键值对容器
#[ani]
pub fn create_pair(key: String, value: i32) -> i64 {
    let pair = Box::new(Pair { key, value });
    Box::into_raw(pair) as i64
}

#[ani]
pub fn pair_get_key(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let pair = &*(ptr as *const Pair<String, i32>);
        pair.key.clone()
    }
}

#[ani]
pub fn pair_get_value(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let pair = &*(ptr as *const Pair<String, i32>);
        pair.value
    }
}

#[ani]
pub fn destroy_pair(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut Pair<String, i32>);
        }
    }
}

struct Pair<K, V> {
    key: K,
    value: V,
}

// ============================================================================
// 模块初始化
// ============================================================================
