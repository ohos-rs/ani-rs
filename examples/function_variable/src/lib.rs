//! Function call and variable access APIs example.

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
pub fn call_module_function_int_example(env: &Env<'_>, function_name: String) -> Result<i32> {
    let module = env.find_module(&current_test_module_name())?;
    let function = env.find_module_function(&module, &function_name, "ii:i")?;
    let args = [ani_value_int(7), ani_value_int(9)];
    env.call_function_int(&function, &args)
}

#[ani]
pub fn call_module_function_void_example(env: &Env<'_>, function_name: String) -> Result<()> {
    let module = env.find_module(&current_test_module_name())?;
    let function = env.find_module_function(&module, &function_name, ":")?;
    env.call_function_void(&function, &[])
}

#[ani]
pub fn module_variable_roundtrip_int(
    env: &Env<'_>,
    variable_name: String,
    value: i32,
) -> Result<i32> {
    let module = env.find_module(&current_test_module_name())?;
    let variable = env.find_module_variable(&module, &variable_name)?;
    env.set_variable_int(&variable, value)?;
    env.get_variable_int(&variable)
}

#[ani]
pub fn module_variable_roundtrip_ref(
    env: &Env<'_>,
    variable_name: String,
    value: AniRef<'_>,
) -> Result<bool> {
    let module = env.find_module(&current_test_module_name())?;
    let variable = env.find_module_variable(&module, &variable_name)?;
    env.set_variable_ref(&variable, &value)?;
    let got = env.get_variable_ref(&variable)?;
    env.reference_equals(&got, &value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = call_module_function_int_example;
        let _ = call_module_function_void_example;
        let _ = module_variable_roundtrip_int;
        let _ = module_variable_roundtrip_ref;
    }
}
