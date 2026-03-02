//! Async Wrapper Example - Wrapping synchronous interfaces for Promise operations.

use ani::conversions::PromiseRaw;
use ani::prelude::*;
use ani_derive::ani;
use std::thread;
use std::time::Duration;

fn expensive_computation(input: i32) -> i32 {
    thread::sleep(Duration::from_millis(100));
    input * input
}

fn fetch_data(url: &str) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("Response from: {}", url)
}

#[ani]
pub fn async_square(n: i32) -> i32 {
    expensive_computation(n)
}

#[ani]
pub fn async_fetch(url: String) -> String {
    fetch_data(&url)
}

#[ani]
pub fn async_compute_start(input: i32) -> i32 {
    input.abs() % 1000
}

#[ani]
pub fn async_check_status(task_id: i32) -> bool {
    task_id > 0
}

#[ani]
pub fn async_get_result(task_id: i32) -> i32 {
    task_id * task_id
}

#[ani]
pub fn batch_compute(count: i32) -> i64 {
    (0..count).map(|i| expensive_computation(i) as i64).sum()
}

#[ani]
pub fn promise_resolve_int(env: &Env<'_>, value: i32) -> Result<i64> {
    let promise = PromiseRaw::resolve_int(env, value)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_resolve_string(env: &Env<'_>, value: String) -> Result<i64> {
    let promise = PromiseRaw::resolve_string(env, &value)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_reject(env: &Env<'_>, message: String) -> Result<i64> {
    let promise = PromiseRaw::reject(env, &message)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_delayed(env: &Env<'_>, delay_ms: i32, value: String) -> Result<i64> {
    let (deferred, promise) = PromiseRaw::deferred(env)?;

    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms as u64));
    }

    deferred.resolve_string(env, &value)?;
    Ok(promise.into_object().as_raw() as i64)
}

#[ani]
pub fn promise_maybe_succeed(env: &Env<'_>, should_succeed: bool, value: i32) -> Result<i64> {
    let (deferred, promise) = PromiseRaw::deferred(env)?;

    if should_succeed {
        deferred.resolve_int(env, value)?;
    } else {
        deferred.reject(env, "Operation failed as requested")?;
    }

    Ok(promise.into_object().as_raw() as i64)
}

#[ani(class = "example.MyClass")]
pub fn get_info(_env: &Env<'_>, this: &AniObject<'_>) -> String {
    let _ = this;
    "Instance info from Rust".to_string()
}

#[ani(class = "example.MyClass", static)]
pub fn create(env: &Env<'_>, _name: String) -> Result<i64> {
    let raw = env.get_undefined_object()?;
    Ok(raw as i64)
}
