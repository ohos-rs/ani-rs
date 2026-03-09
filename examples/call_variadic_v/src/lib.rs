//! Variadic-call fallback example (`_A` + `FnArgs`).

use ani::prelude::*;
use ani_derive::ani;

fn current_test_module_name() -> String {
    std::env::var("ANI_TEST_MODULE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| env!("CARGO_PKG_NAME").replace('-', "_"))
}

#[ani]
pub fn call_function_with_value_array_by_name(
    env: &Env<'_>,
    function_name: String,
    a: i32,
    b: i32,
) -> Result<i32> {
    let module = env.find_module(&current_test_module_name())?;
    let function = env.find_module_function(&module, &function_name, "ii:i")?;
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_function_int(&function, args.as_slice())
}

#[ani]
pub fn call_any_with_fn_args(env: &Env<'_>, func: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&func);
    let left = env.create_string("left")?;
    let right = env.create_string("right")?;
    let result = any.call(env, (left, right))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = call_function_with_value_array_by_name;
        let _ = call_any_with_fn_args;
    }
}
