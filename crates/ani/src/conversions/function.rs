//! Function type support for ANI
//!
//! This module provides Function types for handling ArkTS callbacks in Rust:
//!
//! - [`Function`] - Scoped function type for immediate use within current scope
//! - [`FunctionRef`] - Global reference function type for cross-scope/storage use
//! - [`FnArgs`] - Wrapper for multiple function arguments (optional, tuples work directly)
//! - [`ToAniArgs`] - Trait for converting Rust arguments to ANI function call arguments
//! - [`ToAniArg`] - Trait for converting a single Rust value to ani_ref
//!
//! # Design
//!
//! The design follows ANI/JNI function calling conventions:
//! - Uses `FunctionalObject_Call` API for invoking callbacks
//! - Generates proper ANI signatures for function types
//! - Supports type-safe argument passing via `ToAniArgs` trait
//! - Primitive types are automatically boxed when passed as function arguments
//!
//! # Function vs FunctionRef
//!
//! - Use `Function` when you only need the callback within the current function scope
//! - Use `FunctionRef` when you need to store the callback for later use (cross-scope)
//!
//! # Type Signatures
//!
//! Function arguments are passed as boxed types to ANI:
//! - `i32` -> `Lstd/core/Int;` (boxed)
//! - `i64` -> `Lstd/core/Long;` (boxed)
//! - `f64` -> `Lstd/core/Double;` (boxed)
//! - `bool` -> `Lstd/core/Boolean;` (boxed)
//! - `String` -> `Lstd/core/String;`
//!
//! # Examples
//!
//! ## Single argument callback (scoped)
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! #[ani]
//! pub fn call_callback(env: &Env, callback: Function<(i32,), i32>) -> Result<i32> {
//!     callback.call(env, (42,))
//! }
//! ```
//!
//! ## Multiple arguments
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! #[ani]
//! pub fn call_with_args(env: &Env, callback: Function<(i32, i32), i32>) -> Result<i32> {
//!     callback.call(env, (1, 2))
//! }
//! ```
//!
//! ## Callback stored for later use (FunctionRef)
//!
//! ```rust,ignore
//! use ani::prelude::*;
//!
//! // FunctionRef can be stored in a struct or static
//! static STORED_CALLBACK: Mutex<Option<FunctionRef<(i32,), i32>>> = Mutex::new(None);
//!
//! #[ani]
//! pub fn register_callback(env: &Env, callback: FunctionRef<(i32,), i32>) -> Result<()> {
//!     *STORED_CALLBACK.lock().unwrap() = Some(callback);
//!     Ok(())
//! }
//!
//! #[ani]
//! pub fn invoke_stored_callback(env: &Env, value: i32) -> Result<i32> {
//!     let callback = STORED_CALLBACK.lock().unwrap();
//!     if let Some(ref cb) = *callback {
//!         cb.call(env, (value,))
//!     } else {
//!         Err(Error::new(Status::GenericFailure, "No callback registered"))
//!     }
//! }
//! ```

use std::marker::PhantomData;

use crate::ani_call_ret;
use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::GlobalRef;

use super::{FromAni, TypeInfo};

// ============================================================================
// ToAniArgs Trait
// ============================================================================

/// Trait for converting Rust arguments to ANI function call arguments
///
/// This trait is implemented for tuples of types that implement `ToAni`,
/// allowing type-safe argument passing to ANI functions.
pub trait ToAniArgs {
    /// Convert arguments to a vector of ani_ref for function calls
    fn to_ani_args<'env>(self, env: &Env<'env>) -> Result<Vec<sys::ani_ref>>;

    /// Get the number of arguments
    fn args_count() -> usize;

    /// Get the ANI signature for the arguments part (e.g., "II" for two ints)
    fn args_signature() -> String;
}

// ============================================================================
// FnArgs Wrapper
// ============================================================================

/// Wrapper for multiple function arguments
///
/// Use `FnArgs` when passing multiple arguments to a `Function`:
///
/// ```rust,ignore
/// // For Function<FnArgs<(i32, i32)>, i32>
/// callback.call((1, 2).into())
///
/// // Or explicitly:
/// callback.call(FnArgs((1, 2)))
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FnArgs<T>(pub T);

impl<T> From<T> for FnArgs<T> {
    #[inline]
    fn from(t: T) -> Self {
        FnArgs(t)
    }
}

// ============================================================================
// ToAniArgs Implementations for Tuples
// ============================================================================

// Empty tuple - no arguments
impl ToAniArgs for () {
    fn to_ani_args<'env>(self, _env: &Env<'env>) -> Result<Vec<sys::ani_ref>> {
        Ok(vec![])
    }

    fn args_count() -> usize {
        0
    }

    fn args_signature() -> String {
        String::new()
    }
}

// FnArgs wrapper delegates to inner type
impl<T: ToAniArgs> ToAniArgs for FnArgs<T> {
    fn to_ani_args<'env>(self, env: &Env<'env>) -> Result<Vec<sys::ani_ref>> {
        self.0.to_ani_args(env)
    }

    fn args_count() -> usize {
        T::args_count()
    }

    fn args_signature() -> String {
        T::args_signature()
    }
}

// ============================================================================
// ToAniArg Trait - Single Argument Conversion
// ============================================================================

/// Trait for converting a single Rust value to ani_ref for function calls
///
/// This trait handles the conversion of both reference types (which are already
/// ani_ref compatible) and primitive types (which need to be boxed).
///
/// # Note
///
/// Currently only reference types (String, objects, etc.) are supported directly.
/// Primitive types (i32, f64, bool, etc.) need to be wrapped in their boxed
/// counterparts (Int, Double, Boolean) if you want to pass them to functions.
pub trait ToAniArg {
    /// Convert to ani_ref for function call
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref>;

    /// Get the type signature for this argument
    fn arg_signature() -> &'static str;
}

// Implement ToAniArg for String
impl ToAniArg for String {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        let s = env.create_string(self)?;
        Ok(s.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/String;"
    }
}

impl ToAniArg for &str {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        let s = env.create_string(self)?;
        Ok(s.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/String;"
    }
}

// Implement ToAniArg for boxed primitive types
// These use the boxed type signatures

impl ToAniArg for i32 {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        // Box the int value
        let boxed = create_boxed_int(env, *self)?;
        Ok(boxed.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Int;"
    }
}

impl ToAniArg for i64 {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        let boxed = create_boxed_long(env, *self)?;
        Ok(boxed.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Long;"
    }
}

impl ToAniArg for f64 {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        let boxed = create_boxed_double(env, *self)?;
        Ok(boxed.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Double;"
    }
}

impl ToAniArg for bool {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        let boxed = create_boxed_boolean(env, *self)?;
        Ok(boxed.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Boolean;"
    }
}

impl<'a> ToAniArg for crate::types::AniRef<'a> {
    fn to_ani_arg<'env>(&self, _env: &Env<'env>) -> Result<sys::ani_ref> {
        Ok(self.as_raw())
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Object;"
    }
}

impl<'a> ToAniArg for crate::types::AniObject<'a> {
    fn to_ani_arg<'env>(&self, _env: &Env<'env>) -> Result<sys::ani_ref> {
        Ok(self.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Object;"
    }
}

impl<'a> ToAniArg for crate::types::AniString<'a> {
    fn to_ani_arg<'env>(&self, _env: &Env<'env>) -> Result<sys::ani_ref> {
        Ok(self.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/String;"
    }
}

impl<'a> ToAniArg for crate::types::AniClass<'a> {
    fn to_ani_arg<'env>(&self, _env: &Env<'env>) -> Result<sys::ani_ref> {
        Ok(self.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Class;"
    }
}

// ============================================================================
// Boxing Helpers
// ============================================================================

use crate::types::{AniObject, ani_value_boolean, ani_value_double, ani_value_int, ani_value_long};

/// Create a boxed Int value
fn create_boxed_int<'a>(env: &Env<'a>, value: i32) -> Result<AniObject<'a>> {
    let int_class = env.find_class("Lstd/core/Int;")?;
    let ctor = env.find_method(&int_class, "<ctor>", "I:V")?;
    let args = [ani_value_int(value)];
    env.new_object(&int_class, &ctor, &args)
}

/// Create a boxed Long value
fn create_boxed_long<'a>(env: &Env<'a>, value: i64) -> Result<AniObject<'a>> {
    let long_class = env.find_class("Lstd/core/Long;")?;
    let ctor = env.find_method(&long_class, "<ctor>", "J:V")?;
    let args = [ani_value_long(value)];
    env.new_object(&long_class, &ctor, &args)
}

/// Create a boxed Double value
fn create_boxed_double<'a>(env: &Env<'a>, value: f64) -> Result<AniObject<'a>> {
    let double_class = env.find_class("Lstd/core/Double;")?;
    let ctor = env.find_method(&double_class, "<ctor>", "D:V")?;
    let args = [ani_value_double(value)];
    env.new_object(&double_class, &ctor, &args)
}

/// Create a boxed Boolean value
fn create_boxed_boolean<'a>(env: &Env<'a>, value: bool) -> Result<AniObject<'a>> {
    let boolean_class = env.find_class("Lstd/core/Boolean;")?;
    let ctor = env.find_method(&boolean_class, "<ctor>", "Z:V")?;
    let args = [ani_value_boolean(value)];
    env.new_object(&boolean_class, &ctor, &args)
}

// ============================================================================
// ToAniArgs Implementations for Tuples (1-26 elements)
// ============================================================================

/// Macro to implement ToAniArgs for tuples.
/// Uses a cleaner recursive approach that only requires listing types once.
macro_rules! impl_to_ani_args_for_tuples {
    // Internal implementation macro for a single tuple
    (@impl $($idx:tt: $T:ident),+) => {
        impl<$($T: ToAniArg),+> ToAniArgs for ($($T,)+) {
            fn to_ani_args<'env>(self, env: &Env<'env>) -> Result<Vec<sys::ani_ref>> {
                Ok(vec![$(self.$idx.to_ani_arg(env)?),+])
            }

            fn args_count() -> usize {
                impl_to_ani_args_for_tuples!(@count $($T),+)
            }

            fn args_signature() -> String {
                let mut sig = String::new();
                $(sig.push_str($T::arg_signature());)+
                sig
            }
        }
    };

    // Count helper
    (@count $($T:ident),+) => {
        <[()]>::len(&[$(impl_to_ani_args_for_tuples!(@unit $T)),+])
    };
    (@unit $_:ident) => { () };

    // Recursive expansion: accumulate indices and types, implement, then continue
    // Base case: last type processed
    (@expand [$($acc_idx:tt: $acc_T:ident),*] $idx:tt: $T:ident) => {
        impl_to_ani_args_for_tuples!(@impl $($acc_idx: $acc_T,)* $idx: $T);
    };
    // Recursive case: accumulate and continue
    (@expand [$($acc_idx:tt: $acc_T:ident),*] $idx:tt: $T:ident, $($rest:tt)+) => {
        impl_to_ani_args_for_tuples!(@impl $($acc_idx: $acc_T,)* $idx: $T);
        impl_to_ani_args_for_tuples!(@expand [$($acc_idx: $acc_T,)* $idx: $T] $($rest)+);
    };

    // Entry point: start expansion with empty accumulator
    ($($rest:tt)+) => {
        impl_to_ani_args_for_tuples!(@expand [] $($rest)+);
    };
}

// Generate ToAniArgs for tuples of 1-26 elements
// Only the type names and indices need to be listed once
impl_to_ani_args_for_tuples!(
    0: A,  1: B,  2: C,  3: D,  4: E,  5: F,  6: G,  7: H,
    8: I,  9: J, 10: K, 11: L, 12: M, 13: N, 14: O, 15: P,
   16: Q, 17: R, 18: S, 19: T, 20: U, 21: V, 22: W, 23: X,
   24: Y, 25: Z
);

// ============================================================================
// Function Type
// ============================================================================

/// A type-safe wrapper for ArkTS function objects (scoped)
///
/// `Function<Args, Return>` represents an ArkTS function that can be called from Rust.
/// It provides type-safe argument passing and return value handling.
///
/// # Type Parameters
///
/// - `Args`: The argument type(s). Use `()` for no args, `(A,)` for one arg,
///           or `(A, B, ...)` tuple for multiple args.
/// - `Return`: The return type. Use `()` for void functions.
///
/// # Lifetime
///
/// The `'scope` lifetime ensures the function cannot outlive the current ANI call scope.
/// If you need to store the function for later use, use [`FunctionRef`] instead.
///
/// # Examples
///
/// ```rust,ignore
/// // No arguments, no return
/// fn call_void(env: &Env, callback: Function<(), ()>) -> Result<()> {
///     callback.call(env, ())
/// }
///
/// // Single argument
/// fn call_single(env: &Env, callback: Function<(i32,), String>) -> Result<String> {
///     callback.call(env, (42,))
/// }
///
/// // Multiple arguments
/// fn call_multi(env: &Env, callback: Function<(i32, String), bool>) -> Result<bool> {
///     callback.call(env, (42, "hello".to_string()))
/// }
/// ```
pub struct Function<'scope, Args, Return> {
    value: sys::ani_fn_object,
    _args: PhantomData<Args>,
    _return: PhantomData<Return>,
    _scope: PhantomData<&'scope ()>,
}

impl<'scope, Args, Return> Function<'scope, Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a>,
{
    /// Create a new Function from raw pointer
    ///
    /// # Safety
    ///
    /// The caller must ensure `value` is a valid function object pointer
    #[inline]
    pub unsafe fn from_raw(value: sys::ani_fn_object) -> Self {
        Self {
            value,
            _args: PhantomData,
            _return: PhantomData,
            _scope: PhantomData,
        }
    }

    /// Get the raw function object pointer
    #[inline]
    pub fn as_raw(&self) -> sys::ani_fn_object {
        self.value
    }

    /// Call the function with the given arguments
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `args` - The arguments to pass to the function
    ///
    /// # Returns
    ///
    /// The return value of the function, or an error if the call failed.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // No arguments
    /// callback.call(&env, ())?;
    ///
    /// // Single argument (note the tuple syntax)
    /// let result = callback.call(&env, (42,))?;
    ///
    /// // Multiple arguments
    /// let result = callback.call(&env, (1, 2))?;
    /// ```
    pub fn call(&self, env: &Env<'_>, args: Args) -> Result<Return> {
        call_function_impl(env, self.value, args)
    }
}

/// Internal helper to call a function object
fn call_function_impl<Args, Return>(
    env: &Env<'_>,
    fn_object: sys::ani_fn_object,
    args: Args,
) -> Result<Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a>,
{
    let args_vec = args.to_ani_args(env)?;
    let result = ani_call_ret!(
        env,
        FunctionalObject_Call,
        sys::ani_ref,
        std::ptr::null_mut(),
        fn_object,
        args_vec.len() as sys::ani_size,
        args_vec.as_ptr() as *mut sys::ani_ref
    )
    .map_err(|_| Error::new(Status::GenericFailure, "Function call failed"))?;

    Return::from_ani(env, unsafe { std::mem::transmute_copy(&result) })
}

// ============================================================================
// FunctionRef - Cross-Scope Function Reference
// ============================================================================

/// A global reference to a function that can outlive the original scope
///
/// `FunctionRef` holds a global reference to an ArkTS function, allowing it
/// to be stored and used across different ANI call scopes.
///
/// Use `FunctionRef` as a parameter type when you need to store the callback
/// for later use. Use [`Function`] when you only need the callback within
/// the current function scope.
///
/// # Thread Safety
///
/// `FunctionRef` is `Send + Sync` and can be transferred between threads.
/// However, calling the function requires access to a valid `Env` for the
/// current thread.
///
/// # Examples
///
/// ```rust,ignore
/// use ani::prelude::*;
/// use std::sync::Mutex;
///
/// // Store callback for later use
/// static CALLBACK: Mutex<Option<FunctionRef<(i32,), i32>>> = Mutex::new(None);
///
/// #[ani]
/// pub fn register_callback(env: &Env, callback: FunctionRef<(i32,), i32>) -> Result<()> {
///     *CALLBACK.lock().unwrap() = Some(callback);
///     Ok(())
/// }
///
/// #[ani]
/// pub fn invoke_callback(env: &Env, value: i32) -> Result<i32> {
///     let guard = CALLBACK.lock().unwrap();
///     if let Some(ref cb) = *guard {
///         cb.call(env, (value,))
///     } else {
///         Err(Error::new(Status::GenericFailure, "No callback"))
///     }
/// }
/// ```
pub struct FunctionRef<Args, Return> {
    inner: GlobalRef,
    _args: PhantomData<Args>,
    _return: PhantomData<Return>,
}

// FunctionRef can be sent across threads
unsafe impl<Args, Return> Send for FunctionRef<Args, Return> {}
unsafe impl<Args, Return> Sync for FunctionRef<Args, Return> {}

impl<Args, Return> FunctionRef<Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a>,
{
    /// Call the function with the given arguments
    ///
    /// # Arguments
    ///
    /// * `env` - The ANI environment
    /// * `args` - The arguments to pass to the function
    ///
    /// # Returns
    ///
    /// The return value of the function, or an error if the call failed.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Call stored callback
    /// let result = callback_ref.call(&env, (42,))?;
    /// ```
    pub fn call(&self, env: &Env<'_>, args: Args) -> Result<Return> {
        call_function_impl(env, self.inner.as_raw() as sys::ani_fn_object, args)
    }

    /// Borrow the function back as a scoped Function
    ///
    /// This is useful when you need to pass the function to an API that
    /// expects a scoped `Function` type.
    ///
    /// # Arguments
    ///
    /// * `_env` - The current ANI environment (used for lifetime binding)
    ///
    /// # Returns
    ///
    /// A `Function` that can be used in the current scope.
    pub fn borrow_back<'scope>(&self, _env: &Env<'scope>) -> Function<'scope, Args, Return> {
        Function {
            value: self.inner.as_raw() as sys::ani_fn_object,
            _args: PhantomData,
            _return: PhantomData,
            _scope: PhantomData,
        }
    }

    /// Get the raw global reference
    #[inline]
    pub fn as_raw(&self) -> sys::ani_ref {
        self.inner.as_raw()
    }
}

// ============================================================================
// TypeInfo Implementation
// ============================================================================

impl<Args, Return> TypeInfo for Function<'_, Args, Return> {
    /// Get the ANI type signature for Function
    ///
    /// Function types are represented as `Lstd/core/Function;` in ANI
    fn type_signature() -> &'static str {
        "Lstd/core/Function;"
    }

    fn ani_c_type() -> &'static str {
        "ani_fn_object"
    }
}

impl<Args, Return> TypeInfo for FunctionRef<Args, Return> {
    fn type_signature() -> &'static str {
        "Lstd/core/Function;"
    }

    fn ani_c_type() -> &'static str {
        "ani_fn_object"
    }
}

// TypeInfo for FnArgs - represents the wrapped tuple's type info
impl<T> TypeInfo for FnArgs<T> {
    fn type_signature() -> &'static str {
        // FnArgs is just a wrapper, the actual signature is determined by the tuple
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

// ============================================================================
// FromAni Implementation
// ============================================================================

impl<'env, Args, Return> FromAni<'env> for Function<'env, Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a>,
{
    type Input = sys::ani_fn_object;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Function value is null"));
        }

        Ok(Function {
            value,
            _args: PhantomData,
            _return: PhantomData,
            _scope: PhantomData,
        })
    }
}

impl<'env, Args, Return> FromAni<'env> for FunctionRef<Args, Return>
where
    Args: ToAniArgs,
    Return: for<'a> FromAni<'a>,
{
    type Input = sys::ani_fn_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Function value is null"));
        }

        // Create a global reference to the function object
        use crate::types::AniRef;
        let ani_ref = unsafe { AniRef::from_raw(value as sys::ani_ref) };
        let global_ref = env.create_global_ref(&ani_ref)?;

        Ok(FunctionRef {
            inner: global_ref,
            _args: PhantomData,
            _return: PhantomData,
        })
    }
}

// ============================================================================
// Helper Functions for Signature Generation
// ============================================================================

/// Generate the full function signature for a Function type
///
/// This generates signatures like:
/// - `()` for `Function<(), ()>` (no args, void return)
/// - `(I)I` for `Function<i32, i32>` (int arg, int return)
/// - `(II)Lstd/core/String;` for `Function<FnArgs<(i32, i32)>, String>`
pub fn generate_function_signature<Args: ToAniArgs, Return: TypeInfo>() -> String {
    format!("({}){}", Args::args_signature(), Return::type_signature())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_signature() {
        assert_eq!(<()>::args_signature(), "");
        // Note: primitives are boxed, so signatures use boxed types
        assert_eq!(<(i32,)>::args_signature(), "Lstd/core/Int;");
        assert_eq!(
            <(i32, i64)>::args_signature(),
            "Lstd/core/Int;Lstd/core/Long;"
        );
        assert_eq!(
            <(i32, i64, f64)>::args_signature(),
            "Lstd/core/Int;Lstd/core/Long;Lstd/core/Double;"
        );
        assert_eq!(
            <(bool, String)>::args_signature(),
            "Lstd/core/Boolean;Lstd/core/String;"
        );
    }

    #[test]
    fn test_args_count() {
        assert_eq!(<()>::args_count(), 0);
        assert_eq!(<(i32,)>::args_count(), 1);
        assert_eq!(<(i32, i64)>::args_count(), 2);
        assert_eq!(<(i32, i64, f64)>::args_count(), 3);
    }

    #[test]
    fn test_fn_args_wrapper() {
        let args: FnArgs<(i32, i32)> = (1, 2).into();
        assert_eq!(args.0, (1, 2));
    }

    #[test]
    fn test_function_type_info() {
        assert_eq!(<Function<(), ()>>::type_signature(), "Lstd/core/Function;");
        assert_eq!(<Function<(i32,), String>>::ani_c_type(), "ani_fn_object");
    }

    #[test]
    fn test_generate_function_signature() {
        assert_eq!(generate_function_signature::<(), ()>(), "()V");
        // Note: primitives are boxed for function arguments
        assert_eq!(
            generate_function_signature::<(i32,), i32>(),
            "(Lstd/core/Int;)I"
        );
        assert_eq!(
            generate_function_signature::<(i32, i64), String>(),
            "(Lstd/core/Int;Lstd/core/Long;)Lstd/core/String;"
        );
    }

    #[test]
    fn test_tuple_args_up_to_26() {
        // Test that tuples with many elements compile and work correctly

        // 9 elements
        assert_eq!(
            <(i32, i32, i32, i32, i32, i32, i32, i32, i32)>::args_count(),
            9
        );

        // 10 elements
        assert_eq!(
            <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)>::args_count(),
            10
        );

        // 16 elements
        assert_eq!(
            <(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )>::args_count(),
            16
        );

        // 20 elements
        assert_eq!(
            <(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )>::args_count(),
            20
        );

        // 26 elements (maximum)
        assert_eq!(
            <(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )>::args_count(),
            26
        );
    }

    #[test]
    fn test_tuple_signature_26_elements() {
        // Test signature generation for 26 element tuple
        let sig = <(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        )>::args_signature();
        // Each i32 generates "Lstd/core/Int;" signature
        let expected = "Lstd/core/Int;".repeat(26);
        assert_eq!(sig, expected);
    }

    #[test]
    fn test_mixed_type_tuple() {
        // Test with mixed types
        let sig = <(i32, String, bool, f64)>::args_signature();
        assert_eq!(
            sig,
            "Lstd/core/Int;Lstd/core/String;Lstd/core/Boolean;Lstd/core/Double;"
        );
    }
}
