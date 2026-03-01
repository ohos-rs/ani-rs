//! Variadic-call fallback example (`_A` + `FnArgs`).

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn call_function_with_value_array(
    env: &Env<'_>,
    function: AniFunction,
    a: i32,
    b: i32,
) -> Result<i32> {
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_function_int(&function, args.as_slice())
}

#[ani]
pub fn call_any_with_fn_args(env: &Env<'_>, func: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&func);
    let result = any.call(env, FnArgs((1_i32, 2_i32)))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = call_function_with_value_array;
        let _ = call_any_with_fn_args;
    }
}
