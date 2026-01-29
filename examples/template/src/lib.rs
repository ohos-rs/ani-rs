//! Template Example - Handling template class parameters
//!
//! Demonstrates how to handle ArkTS generic parameters
//! Generic parameters are uniformly mapped to Lstd/core/Object; in ANI

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// Basic Generic Functions
// ============================================================================

/// Handle generic parameter
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function identity<T>(value: T): T;
/// ```
///
/// Mangling: Lstd/core/Object;:Lstd/core/Object;
/// Generic T is uniformly mapped to Object
#[no_mangle]
pub extern "C" fn identity(
    _env: *mut sys::ani_env,
    _obj: sys::ani_object,
    value: sys::ani_object,
) -> sys::ani_object {
    // Return the passed object directly
    value
}

/// Swap two generic values
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function swap<T, U>(a: T, b: U): [U, T];
/// ```
///
/// Note: Returning tuple requires creating array or tuple object
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
// Array<T> Generic
// ============================================================================

/// Get length of Array<T>
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function arrayLength<T>(arr: Array<T>): int;
/// ```
///
/// Note: Array<T>'s mangling is Lescompat/Array; not ani_array
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

        // Call Array's length property
        match env.get_property_by_name_int(&arr_obj, "length") {
            Ok(len) => len,
            Err(_) => -1,
        }
    }
}

/// Get element of Array<T>
///
/// Corresponding ArkTS definition:
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

        // Call Array's $_get method
        // Mangling: I:Lstd/core/Object;
        match env.call_method_by_name_ref(&arr_obj, "$_get", Some("I:Lstd/core/Object;"), index) {
            Ok(elem) => elem.as_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

// ============================================================================
// Generic Container
// ============================================================================

/// Create generic container
///
/// Corresponding ArkTS definition:
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

/// Get container value
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

/// Set container value
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

/// Release container
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
// String Generic Container
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
// Generic Pair Container
// ============================================================================

/// Create key-value pair container
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
// Module Initialization
// ============================================================================
