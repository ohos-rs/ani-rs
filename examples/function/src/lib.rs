//! Function Example - Demonstrates callback function handling in ani-rs
//!
//! This example shows how to use `Function` and `FunctionRef` types to handle
//! ArkTS callback functions in Rust.
//!
//! # Types
//!
//! - `Function<Args, Return>` - Scoped callback, use within current function scope
//! - `FunctionRef<Args, Return>` - Global reference callback, can be stored for later use
//!
//! # Argument Syntax
//!
//! - No arguments: `Function<(), Return>`
//! - Single argument: `Function<(A,), Return>` (note the trailing comma)
//! - Multiple arguments: `Function<(A, B, C), Return>`
//!
//! # ArkTS Usage
//!
//! ```typescript
//! // Import native module
//! import native from 'libani_example_function.so';
//!
//! // Basic callback
//! const result = native.callWithCallback((x: number) => x * 2, 21);
//! console.log(result); // 42
//!
//! // Multiple arguments callback
//! const sum = native.callWithMultiArgs((a: number, b: number) => a + b, 10, 20);
//! console.log(sum); // 30
//!
//! // Store callback for later use
//! native.registerCallback((value: number) => {
//!     console.log(`Callback received: ${value}`);
//!     return value * 10;
//! });
//!
//! // Invoke stored callback
//! const result2 = native.invokeStoredCallback(5);
//! console.log(result2); // 50
//! ```

use std::sync::Mutex;

use ani::prelude::*;
use ani_derive::ani;

// ============================================================================
// Basic Callback - Function (Scoped)
// ============================================================================

/// Call a callback function with a single argument
///
/// # Arguments
/// - `callback` - A function that takes an i32 and returns an i32
/// - `value` - The value to pass to the callback
///
/// # Returns
/// The result from the callback function
///
/// # ArkTS
/// ```typescript
/// function callWithCallback(callback: (x: Int) => Int, value: number): number;
/// ```
#[ani]
pub fn call_with_callback(env: &Env, callback: Function<(i32,), i32>, value: i32) -> Result<i32> {
    callback.call(env, (value,))
}

/// Call a callback function with no arguments
///
/// # ArkTS
/// ```typescript
/// function callNoArgsCallback(callback: () => string): string;
/// ```
#[ani]
pub fn call_no_args_callback(env: &Env, callback: Function<(), String>) -> Result<String> {
    callback.call(env, ())
}

/// Call a callback function with multiple arguments
///
/// # ArkTS
/// ```typescript
/// function callWithMultiArgs(
///     callback: (a: Int, b: Int) => Int,
///     a: number,
///     b: number
/// ): number;
/// ```
#[ani]
pub fn call_with_multi_args(
    env: &Env,
    callback: Function<(i32, i32), i32>,
    a: i32,
    b: i32,
) -> Result<i32> {
    callback.call(env, (a, b))
}

/// Call a callback that returns void
///
/// # ArkTS
/// ```typescript
/// function callVoidCallback(callback: (message: string) => void, message: string): void;
/// ```
#[ani]
pub fn call_void_callback(
    env: &Env,
    callback: Function<(String,), ()>,
    message: String,
) -> Result<()> {
    callback.call(env, (message,))
}

/// Call a callback with boolean return
///
/// # ArkTS
/// ```typescript
/// function callBoolCallback(callback: (x: Int) => Boolean, value: number): boolean;
/// ```
#[ani]
pub fn call_bool_callback(env: &Env, callback: Function<(i32,), bool>, value: i32) -> Result<bool> {
    callback.call(env, (value,))
}

/// Call a callback with double return
///
/// # ArkTS
/// ```typescript
/// function callDoubleCallback(callback: (x: Double) => Double, value: number): number;
/// ```
#[ani]
pub fn call_double_callback(env: &Env, callback: Function<(f64,), f64>, value: f64) -> Result<f64> {
    callback.call(env, (value,))
}

/// Call a callback with string argument and return
///
/// # ArkTS
/// ```typescript
/// function callStringCallback(callback: (s: string) => string, input: string): string;
/// ```
#[ani]
pub fn call_string_callback(
    env: &Env,
    callback: Function<(String,), String>,
    input: String,
) -> Result<String> {
    callback.call(env, (input,))
}

/// Return the same scoped callback back to ArkTS.
///
/// This exercises `Function<'_, ...>` as a native return value.
#[ani]
pub fn echo_scoped_string_callback(
    callback: Function<'_, (String,), String>,
) -> Function<'_, (String,), String> {
    callback
}

// ============================================================================
// Stored Callback - FunctionRef (Global Reference)
// ============================================================================

/// Global storage for a callback function
///
/// FunctionRef is Send + Sync, so it can be safely stored in a Mutex
static STORED_CALLBACK: Mutex<Option<FunctionRef<(i32,), i32>>> = Mutex::new(None);

/// Register a callback for later invocation
///
/// The callback is stored as a global reference and can be invoked later
/// from any ANI call.
///
/// # Arguments
/// - `callback` - The callback function to store
///
/// # ArkTS
/// ```typescript
/// function registerCallback(callback: (value: Int) => Int): void;
/// ```
#[ani]
pub fn register_callback(_env: &Env, callback: FunctionRef<(i32,), i32>) -> Result<()> {
    let mut guard = STORED_CALLBACK.lock().unwrap();
    *guard = Some(callback);
    Ok(())
}

/// Invoke the stored callback
///
/// # Arguments
/// - `value` - The value to pass to the callback
///
/// # Returns
/// The result from the callback, or an error if no callback is registered
///
/// # ArkTS
/// ```typescript
/// function invokeStoredCallback(value: number): number;
/// ```
#[ani]
pub fn invoke_stored_callback(env: &Env, value: i32) -> Result<i32> {
    let guard = STORED_CALLBACK.lock().unwrap();
    if let Some(ref callback) = *guard {
        callback.call(env, (value,))
    } else {
        Err(Error::new(
            Status::GenericFailure,
            "No callback registered. Call registerCallback first.",
        ))
    }
}

/// Clear the stored callback
///
/// # ArkTS
/// ```typescript
/// function clearCallback(): void;
/// ```
#[ani]
pub fn clear_callback() -> Result<()> {
    let mut guard = STORED_CALLBACK.lock().unwrap();
    *guard = None;
    Ok(())
}

/// Check if a callback is registered
///
/// # ArkTS
/// ```typescript
/// function hasCallback(): boolean;
/// ```
#[ani]
pub fn has_callback() -> bool {
    let guard = STORED_CALLBACK.lock().unwrap();
    guard.is_some()
}

// ============================================================================
// Advanced: Multiple Stored Callbacks (using name as key)
// ============================================================================

/// Storage for string transformer callback
static STRING_TRANSFORMER: Mutex<Option<FunctionRef<(String,), String>>> = Mutex::new(None);

/// Register a string transformer callback
///
/// # Arguments
/// - `callback` - A function that transforms strings
///
/// # ArkTS
/// ```typescript
/// function registerStringTransformer(callback: (input: string) => string): void;
/// ```
#[ani]
pub fn register_string_transformer(
    _env: &Env,
    callback: FunctionRef<(String,), String>,
) -> Result<()> {
    let mut guard = STRING_TRANSFORMER.lock().unwrap();
    *guard = Some(callback);
    Ok(())
}

/// Return the same stored-style callback back to ArkTS.
///
/// This exercises `FunctionRef<...>` as a native return value.
#[ani]
pub fn echo_function_ref(
    callback: FunctionRef<(String,), String>,
) -> FunctionRef<(String,), String> {
    callback
}

/// Transform a string using the registered transformer
///
/// # Arguments
/// - `input` - The string to transform
///
/// # Returns
/// The transformed string
///
/// # ArkTS
/// ```typescript
/// function transformString(input: string): string;
/// ```
#[ani]
pub fn transform_string(env: &Env, input: String) -> Result<String> {
    let guard = STRING_TRANSFORMER.lock().unwrap();
    if let Some(ref callback) = *guard {
        callback.call(env, (input,))
    } else {
        // If no transformer registered, return input unchanged
        Ok(input)
    }
}

/// Check if a string transformer is registered
///
/// # ArkTS
/// ```typescript
/// function hasStringTransformer(): boolean;
/// ```
#[ani]
pub fn has_string_transformer() -> bool {
    let guard = STRING_TRANSFORMER.lock().unwrap();
    guard.is_some()
}

// ============================================================================
// Callback Chaining Example
// ============================================================================

/// Apply two callbacks in sequence
///
/// # Arguments
/// - `first` - First callback to apply
/// - `second` - Second callback to apply
/// - `value` - Initial value
///
/// # Returns
/// Result of applying first then second callback
///
/// # ArkTS
/// ```typescript
/// function chainCallbacks(
///     first: (x: Int) => Int,
///     second: (x: Int) => Int,
///     value: number
/// ): number;
/// ```
#[ani]
pub fn chain_callbacks(
    env: &Env,
    first: Function<(i32,), i32>,
    second: Function<(i32,), i32>,
    value: i32,
) -> Result<i32> {
    let intermediate = first.call(env, (value,))?;
    second.call(env, (intermediate,))
}

/// Apply a callback multiple times
///
/// # Arguments
/// - `callback` - The callback to apply
/// - `value` - Initial value
/// - `times` - Number of times to apply the callback
///
/// # Returns
/// Result after applying callback n times
///
/// # ArkTS
/// ```typescript
/// function repeatCallback(callback: (x: Int) => Int, value: number, times: number): number;
/// ```
#[ani]
pub fn repeat_callback(
    env: &Env,
    callback: Function<(i32,), i32>,
    value: i32,
    times: i32,
) -> Result<i32> {
    let mut result = value;
    for _ in 0..times {
        result = callback.call(env, (result,))?;
    }
    Ok(result)
}

// ============================================================================
// Conditional Callback Example
// ============================================================================

/// Call callback only if condition is true
///
/// # Arguments
/// - `callback` - The callback to potentially call
/// - `value` - Value to pass to callback
/// - `condition` - Whether to call the callback
///
/// # Returns
/// Callback result if condition is true, otherwise returns value unchanged
///
/// # ArkTS
/// ```typescript
/// function callIf(callback: (x: Int) => Int, value: number, condition: boolean): number;
/// ```
#[ani]
pub fn call_if(
    env: &Env,
    callback: Function<(i32,), i32>,
    value: i32,
    condition: bool,
) -> Result<i32> {
    if condition {
        callback.call(env, (value,))
    } else {
        Ok(value)
    }
}

// ============================================================================
// FnArgs Examples - Explicit Multiple Arguments Wrapper
// ============================================================================

/// Calculate using a callback with three arguments (using FnArgs)
///
/// FnArgs is a wrapper type for multiple arguments. It can be used
/// for explicit argument wrapping, especially useful with many arguments.
///
/// # Arguments
/// - `callback` - A function that takes three i32 arguments
/// - `a`, `b`, `c` - The values to pass to the callback
///
/// # ArkTS
/// ```typescript
/// function callWithThreeArgs(
///     callback: (a: Int, b: Int, c: Int) => Int,
///     a: number,
///     b: number,
///     c: number
/// ): number;
/// ```
#[ani]
pub fn call_with_three_args(
    env: &Env,
    callback: Function<FnArgs<(i32, i32, i32)>, i32>,
    a: i32,
    b: i32,
    c: i32,
) -> Result<i32> {
    // Using FnArgs wrapper explicitly
    callback.call(env, FnArgs((a, b, c)))
}

/// Calculate using a callback with four arguments (using FnArgs)
///
/// # Arguments
/// - `callback` - A function that takes four i32 arguments
/// - `a`, `b`, `c`, `d` - The values to pass to the callback
///
/// # ArkTS
/// ```typescript
/// function callWithFourArgs(
///     callback: (a: Int, b: Int, c: Int, d: Int) => Int,
///     a: number,
///     b: number,
///     c: number,
///     d: number
/// ): number;
/// ```
#[ani]
pub fn call_with_four_args(
    env: &Env,
    callback: Function<FnArgs<(i32, i32, i32, i32)>, i32>,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) -> Result<i32> {
    // Using .into() to convert tuple to FnArgs
    callback.call(env, (a, b, c, d).into())
}

/// Calculate weighted average using a callback with mixed types (using FnArgs)
///
/// This demonstrates FnArgs with different argument types.
///
/// # Arguments
/// - `callback` - A function that takes string, i32, and f64 arguments
/// - `name` - A string argument
/// - `count` - An integer argument
/// - `weight` - A float argument
///
/// # ArkTS
/// ```typescript
/// function callMixedArgs(
///     callback: (name: string, count: Int, weight: Double) => string,
///     name: string,
///     count: number,
///     weight: number
/// ): string;
/// ```
#[ani]
pub fn call_mixed_args(
    env: &Env,
    callback: Function<FnArgs<(String, i32, f64)>, String>,
    name: String,
    count: i32,
    weight: f64,
) -> Result<String> {
    callback.call(env, FnArgs((name, count, weight)))
}

/// Store and invoke a callback with multiple arguments using FunctionRef
///
/// This demonstrates storing a FunctionRef with FnArgs for later use.
type MultiArgCallback = FunctionRef<FnArgs<(i32, i32, i32)>, i32>;
static MULTI_ARG_CALLBACK: Mutex<Option<MultiArgCallback>> = Mutex::new(None);

/// Register a callback with three arguments
///
/// # ArkTS
/// ```typescript
/// function registerMultiArgCallback(callback: (a: Int, b: Int, c: Int) => Int): void;
/// ```
#[ani]
pub fn register_multi_arg_callback(
    _env: &Env,
    callback: FunctionRef<FnArgs<(i32, i32, i32)>, i32>,
) -> Result<()> {
    let mut guard = MULTI_ARG_CALLBACK.lock().unwrap();
    *guard = Some(callback);
    Ok(())
}

/// Invoke the stored multi-argument callback
///
/// # ArkTS
/// ```typescript
/// function invokeMultiArgCallback(a: number, b: number, c: number): number;
/// ```
#[ani]
pub fn invoke_multi_arg_callback(env: &Env, a: i32, b: i32, c: i32) -> Result<i32> {
    let guard = MULTI_ARG_CALLBACK.lock().unwrap();
    if let Some(ref callback) = *guard {
        callback.call(env, FnArgs((a, b, c)))
    } else {
        Err(Error::new(
            Status::GenericFailure,
            "No multi-arg callback registered",
        ))
    }
}

/// Reduce multiple values using a binary callback
///
/// Demonstrates using a two-argument callback (FnArgs<(i32, i32)>)
/// to reduce a series of values.
///
/// # Arguments
/// - `callback` - Binary operation (takes two i32, returns i32)
/// - `initial` - Initial value for the reduction
/// - `v1`, `v2`, `v3` - Values to reduce
///
/// # ArkTS
/// ```typescript
/// function reduceValues(
///     callback: (acc: Int, val: Int) => Int,
///     initial: number,
///     v1: number,
///     v2: number,
///     v3: number
/// ): number;
/// ```
#[ani]
pub fn reduce_values(
    env: &Env,
    callback: Function<FnArgs<(i32, i32)>, i32>,
    initial: i32,
    v1: i32,
    v2: i32,
    v3: i32,
) -> Result<i32> {
    let mut acc = initial;
    // Apply callback to each value
    acc = callback.call(env, FnArgs((acc, v1)))?;
    acc = callback.call(env, FnArgs((acc, v2)))?;
    acc = callback.call(env, FnArgs((acc, v3)))?;
    Ok(acc)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_callback_initial() {
        // Test that has_callback function works
        let _ = has_callback();
    }

    #[test]
    fn test_has_string_transformer_initial() {
        // Test that has_string_transformer function works
        let _ = has_string_transformer();
    }
}
