//! SetField Example - Reading and Writing Fields
//!
//! Demonstrates how to read and write ArkTS object fields in Native layer

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// Reading Field Values
// ============================================================================

/// Get object's int field
///
/// Corresponding ArkTS definition:
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

        // Get value by field name
        match env.get_field_by_name_int(&person_obj, "age") {
            Ok(age) => age,
            Err(_) => 0,
        }
    }
}

/// Get object's double field
///
/// Corresponding ArkTS definition:
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

        // Get double field
        match env.get_property_by_name_double(&product_obj, "price") {
            Ok(price) => price,
            Err(_) => 0.0,
        }
    }
}

// ============================================================================
// Setting Field Values
// ============================================================================

/// Set object's int field
///
/// Corresponding ArkTS definition:
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

        // Set field value
        let _ = env.set_field_by_name_int(&person_obj, "age", age);
    }
}

/// Set object's double field
///
/// Corresponding ArkTS definition:
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

        // Set double field
        let _ = env.set_property_by_name_double(&product_obj, "price", price);
    }
}

// ============================================================================
// Batch Reading and Writing Fields
// ============================================================================

/// Struct for storing field values
struct PersonData {
    name: String,
    age: i32,
    height: f64,
}

/// Create PersonData
#[ani]
pub fn create_person_data(name: String, age: i32, height: f64) -> i64 {
    let data = Box::new(PersonData { name, age, height });
    Box::into_raw(data) as i64
}

/// Get name
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

/// Set name
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

/// Get age
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

/// Set age
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

/// Get height
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

/// Set height
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

/// Release PersonData
#[ani]
pub fn destroy_person_data(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut PersonData);
        }
    }
}

// ============================================================================
// Using Field Handle (High-Performance Version)
// ============================================================================

/// Get value by Field handle (faster than by name)
///
/// This requires first obtaining Field handle through Class_FindField
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

/// Set value by Field handle
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
// Module Initialization
// ============================================================================
