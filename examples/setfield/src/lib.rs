//! SetField Example - Reading and Writing Fields.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn get_age(env: &Env<'_>, person: AniObject<'_>) -> i32 {
    env.get_field_by_name_int(&person, "age").unwrap_or(0)
}

#[ani]
pub fn get_price(env: &Env<'_>, product: AniObject<'_>) -> f64 {
    env.get_property_by_name_double(&product, "price")
        .unwrap_or(0.0)
}

#[ani]
pub fn set_age(env: &Env<'_>, person: AniObject<'_>, age: i32) {
    let _ = env.set_field_by_name_int(&person, "age", age);
}

#[ani]
pub fn set_price(env: &Env<'_>, product: AniObject<'_>, price: f64) {
    let _ = env.set_property_by_name_double(&product, "price", price);
}

struct PersonData {
    name: String,
    age: i32,
    height: f64,
}

#[ani]
pub fn create_person_data(name: String, age: i32, height: f64) -> i64 {
    let data = Box::new(PersonData { name, age, height });
    Box::into_raw(data) as i64
}

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

#[ani]
pub fn destroy_person_data(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut PersonData);
        }
    }
}

#[ani]
pub fn get_field_by_handle(env: &Env<'_>, target: AniObject<'_>, field: AniField) -> i32 {
    env.get_field_int(&target, &field).unwrap_or(0)
}

#[ani]
pub fn set_field_by_handle(env: &Env<'_>, target: AniObject<'_>, field: AniField, value: i32) {
    let _ = env.set_field_int(&target, &field, value);
}
