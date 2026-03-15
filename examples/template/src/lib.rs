//! Template Example - Handling generic class parameters.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn identity(value: AniObject<'_>) -> i64 {
    value.as_raw() as i64
}

#[ani]
pub fn get_first(first: AniObject<'_>, _second: AniObject<'_>) -> i64 {
    first.as_raw() as i64
}

#[ani]
pub fn get_second(_first: AniObject<'_>, second: AniObject<'_>) -> i64 {
    second.as_raw() as i64
}

#[ani]
pub fn array_length(env: &Env<'_>, arr: AniObject<'_>) -> i32 {
    env.get_property_by_name_int(&arr, "length").unwrap_or(-1)
}

#[ani]
pub fn array_get(env: &Env<'_>, arr: AniObject<'_>, index: i32) -> Result<bool> {
    let args = [ani_value_int(index)];
    let elem =
        env.call_method_by_name_ref_with_args(&arr, "$_get", Some("I:Lstd/core/Object;"), &args)?;
    Ok(!env.is_nullish(&elem)?)
}

#[ani]
pub fn create_int_container(value: i32) -> i64 {
    let container = Box::new(GenericContainer { value });
    Box::into_raw(container) as i64
}

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
