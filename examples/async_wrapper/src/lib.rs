//! Async Wrapper Example - Wrapping synchronous interfaces for async Promise operations
//!
//! Demonstrates how to use ANI Promise API to implement asynchronous operations in Rust.
//!
//! ## Corresponding ArkTS Declarations
//!
//! ```typescript
//! // Synchronous functions
//! native function asyncSquare(n: int): int;
//! native function asyncFetch(url: string): string;
//! native function asyncComputeStart(input: int): int;
//! native function asyncCheckStatus(taskId: int): boolean;
//! native function asyncGetResult(taskId: int): int;
//! native function batchCompute(count: int): long;
//!
//! // Promise functions - using #[ani] macro + Env injection
//! native function promiseResolveInt(value: int): Promise<Int>;
//! native function promiseResolveString(value: string): Promise<string>;
//! native function promiseReject(message: string): Promise<void>;
//! native function promiseDelayed(delayMs: int, value: string): Promise<string>;
//! native function promiseMaybeSucceed(shouldSucceed: boolean, value: int): Promise<Int>;
//! ```
//!
//! ## Parameter Injection System
//!
//! The `#[ani]` macro supports automatic parameter injection, similar to napi-rs:
//!
//! - `env: &Env<'_>` - ANI environment, automatically injected, not part of ArkTS signature
//! - `this: &AniObject<'_>` - Instance object for class methods
//! - `class: &AniClass<'_>` - Class object for static methods
//!
//! These parameters are automatically handled by the macro and will NOT appear
//! in the ArkTS function signature.

use ani::conversions::PromiseRaw;
use ani::prelude::*;
use ani::sys;
use ani_derive::ani;
use std::thread;
use std::time::Duration;

// ============================================================================
// Synchronous Operations - Simulating expensive computations
// ============================================================================

/// Simulates an expensive computation task
fn expensive_computation(input: i32) -> i32 {
    thread::sleep(Duration::from_millis(100));
    input * input
}

/// Simulates a network request
fn fetch_data(url: &str) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("Response from: {}", url)
}

// ============================================================================
// Synchronous Functions (using #[ani] macro)
// ============================================================================

/// Computes the square of a number
#[ani]
pub fn async_square(n: i32) -> i32 {
    expensive_computation(n)
}

/// Fetches data from a URL
#[ani]
pub fn async_fetch(url: String) -> String {
    fetch_data(&url)
}

/// Starts an async computation and returns a task ID
#[ani]
pub fn async_compute_start(input: i32) -> i32 {
    input.abs() % 1000
}

/// Checks the status of an async task
#[ani]
pub fn async_check_status(task_id: i32) -> bool {
    task_id > 0
}

/// Gets the result of an async task
#[ani]
pub fn async_get_result(task_id: i32) -> i32 {
    task_id * task_id
}

/// Batch computation - returns the sum of all results
#[ani]
pub fn batch_compute(count: i32) -> i64 {
    (0..count).map(|i| expensive_computation(i) as i64).sum()
}

// ============================================================================
// Promise Functions (using #[ani] macro + Env injection)
// ============================================================================
//
// You can now use the #[ani] macro to define Promise functions!
// Simply add `env: &Env<'_>` as the first parameter, and the macro will
// automatically inject it. This parameter will NOT appear in the ArkTS signature.

/// Creates a Promise and immediately resolves it with an int value
///
/// Corresponding ArkTS:
/// ```typescript
/// native function promiseResolveInt(value: int): Promise<Int>;
/// ```
///
/// Note: The `env` parameter is automatically injected by the macro.
/// ArkTS callers only need to pass `value`.
#[ani]
pub fn promise_resolve_int(env: &Env<'_>, value: i32) -> sys::ani_object {
    match PromiseRaw::resolve_int(env, value) {
        Ok(p) => p.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a Promise and immediately resolves it with a string value
///
/// Corresponding ArkTS:
/// ```typescript
/// native function promiseResolveString(value: string): Promise<string>;
/// ```
#[ani]
pub fn promise_resolve_string(env: &Env<'_>, value: String) -> sys::ani_object {
    match PromiseRaw::resolve_string(env, &value) {
        Ok(p) => p.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a Promise and immediately rejects it
///
/// Corresponding ArkTS:
/// ```typescript
/// native function promiseReject(message: string): Promise<void>;
/// ```
#[ani]
pub fn promise_reject(env: &Env<'_>, message: String) -> sys::ani_object {
    match PromiseRaw::reject(env, &message) {
        Ok(p) => p.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a Promise that resolves after a delay
///
/// Corresponding ArkTS:
/// ```typescript
/// native function promiseDelayed(delayMs: int, value: string): Promise<string>;
/// ```
#[ani]
pub fn promise_delayed(env: &Env<'_>, delay_ms: i32, value: String) -> sys::ani_object {
    let result = (|| -> Result<PromiseRaw<'_>> {
        let (deferred, promise) = PromiseRaw::deferred(env)?;

        // Simulate delay
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms as u64));
        }

        // Resolve with string value
        deferred.resolve_string(env, &value)?;

        Ok(promise)
    })();

    match result {
        Ok(p) => p.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Creates a Promise that may succeed or fail based on input
///
/// Corresponding ArkTS:
/// ```typescript
/// native function promiseMaybeSucceed(shouldSucceed: boolean, value: int): Promise<Int>;
/// ```
#[ani]
pub fn promise_maybe_succeed(env: &Env<'_>, should_succeed: bool, value: i32) -> sys::ani_object {
    let result = (|| -> Result<PromiseRaw<'_>> {
        let (deferred, promise) = PromiseRaw::deferred(env)?;

        if should_succeed {
            deferred.resolve_int(env, value)?;
        } else {
            deferred.reject(env, "Operation failed as requested")?;
        }

        Ok(promise)
    })();

    match result {
        Ok(p) => p.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// ============================================================================
// Class Method Examples - This Injection
// ============================================================================
//
// For class methods, you can use `this: &AniObject<'_>` parameter to receive
// the instance object. The macro will automatically inject it.

/// Class instance method example - gets instance information
///
/// Corresponding ArkTS:
/// ```typescript
/// class MyClass {
///     native getInfo(): string;
/// }
/// ```
#[ani(class = "example.MyClass")]
pub fn get_info(env: &Env<'_>, this: &AniObject<'_>) -> String {
    // You can use `this` to access instance properties
    let _ = (env, this); // Just demonstrating parameter injection
    "Instance info from Rust".to_string()
}

/// Static method example
///
/// Corresponding ArkTS:
/// ```typescript
/// class MyClass {
///     static native create(name: string): MyClass;
/// }
/// ```
#[ani(class = "example.MyClass", static)]
pub fn create(env: &Env<'_>, name: String) -> sys::ani_object {
    // Use env to create a new instance
    let _ = (env, name); // Just demonstrating parameter injection
    std::ptr::null_mut() // Should return the created object in real implementation
}
