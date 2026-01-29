//! SetField 示例 - 读写 Field
//!
//! 演示如何在 Native 层读写 ArkTS 对象的字段

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// 读取字段值
// ============================================================================

/// 获取对象的 int 字段
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Person {
///     age: int = 0;
/// }
/// native function getAge(person: Person): int;
/// ```
#[no_mangle]
pub extern "C" fn get_age(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    person: sys::ani_object,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return 0,
        };

        let person_obj = AniObject::from_raw(person);

        // 通过字段名获取值
        match env.get_field_by_name_int(&person_obj, "age") {
            Ok(age) => age,
            Err(_) => 0,
        }
    }
}

/// 获取对象的 double 字段
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Product {
///     price: double = 0.0;
/// }
/// native function getPrice(product: Product): double;
/// ```
#[no_mangle]
pub extern "C" fn get_price(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    product: sys::ani_object,
) -> sys::ani_double {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return 0.0,
        };

        let product_obj = AniObject::from_raw(product);

        // 获取 double 字段
        match env.get_property_by_name_double(&product_obj, "price") {
            Ok(price) => price,
            Err(_) => 0.0,
        }
    }
}

// ============================================================================
// 设置字段值
// ============================================================================

/// 设置对象的 int 字段
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Person {
///     age: int = 0;
/// }
/// native function setAge(person: Person, age: int): void;
/// ```
#[no_mangle]
pub extern "C" fn set_age(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    person: sys::ani_object,
    age: sys::ani_int,
) {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return,
        };

        let person_obj = AniObject::from_raw(person);

        // 设置字段值
        let _ = env.set_field_by_name_int(&person_obj, "age", age);
    }
}

/// 设置对象的 double 字段
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class Product {
///     price: double = 0.0;
/// }
/// native function setPrice(product: Product, price: double): void;
/// ```
#[no_mangle]
pub extern "C" fn set_price(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    product: sys::ani_object,
    price: sys::ani_double,
) {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return,
        };

        let product_obj = AniObject::from_raw(product);

        // 设置 double 字段
        let _ = env.set_property_by_name_double(&product_obj, "price", price);
    }
}

// ============================================================================
// 批量读写字段
// ============================================================================

/// 结构体用于存储字段值
struct PersonData {
    name: String,
    age: i32,
    height: f64,
}

/// 创建 PersonData
#[ani]
pub fn create_person_data(name: String, age: i32, height: f64) -> i64 {
    let data = Box::new(PersonData { name, age, height });
    Box::into_raw(data) as i64
}

/// 获取 name
#[ani]
pub fn person_data_get_name(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let data = &*(ptr as *const PersonData);
        data.name.clone()
    }
}

/// 设置 name
#[ani]
pub fn person_data_set_name(ptr: i64, name: String) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let data = &mut *(ptr as *mut PersonData);
        data.name = name;
    }
}

/// 获取 age
#[ani]
pub fn person_data_get_age(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let data = &*(ptr as *const PersonData);
        data.age
    }
}

/// 设置 age
#[ani]
pub fn person_data_set_age(ptr: i64, age: i32) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let data = &mut *(ptr as *mut PersonData);
        data.age = age;
    }
}

/// 获取 height
#[ani]
pub fn person_data_get_height(ptr: i64) -> f64 {
    if ptr == 0 {
        return 0.0;
    }
    unsafe {
        let data = &*(ptr as *const PersonData);
        data.height
    }
}

/// 设置 height
#[ani]
pub fn person_data_set_height(ptr: i64, height: f64) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let data = &mut *(ptr as *mut PersonData);
        data.height = height;
    }
}

/// 释放 PersonData
#[ani]
pub fn destroy_person_data(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut PersonData);
        }
    }
}

// ============================================================================
// 使用 Field 句柄（高性能版本）
// ============================================================================

/// 通过 Field 句柄获取值（比通过名称更快）
///
/// 这需要先通过 Class_FindField 获取 Field 句柄
#[no_mangle]
pub extern "C" fn get_field_by_handle(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    target: sys::ani_object,
    field: sys::ani_field,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return 0,
        };

        let target_obj = AniObject::from_raw(target);
        let field_handle = AniField::from_raw(field);

        match env.get_field_int(&target_obj, &field_handle) {
            Ok(value) => value,
            Err(_) => 0,
        }
    }
}

/// 通过 Field 句柄设置值
#[no_mangle]
pub extern "C" fn set_field_by_handle(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    target: sys::ani_object,
    field: sys::ani_field,
    value: sys::ani_int,
) {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return,
        };

        let target_obj = AniObject::from_raw(target);
        let field_handle = AniField::from_raw(field);

        let _ = env.set_field_int(&target_obj, &field_handle, value);
    }
}

// ============================================================================
// 模块初始化
// ============================================================================
