//! Module/namespace member lookup APIs example.

use ani::prelude::*;
use ani_derive::ani;

fn current_test_module_name() -> String {
    std::env::var("ANI_TEST_MODULE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| env!("CARGO_PKG_NAME").replace('-', "_"))
}

fn current_namespace_descriptor(namespace_name: &str) -> String {
    format!("{}.{}", current_test_module_name(), namespace_name)
}

fn string_from_ref(env: &Env<'_>, value: AniRef<'_>) -> Result<String> {
    let string = unsafe { AniString::from_raw(value.into_raw() as ani::sys::ani_string) };
    env.get_string(&string)
}

#[ani]
pub fn find_current_module_members(env: &Env<'_>) -> Result<bool> {
    let module = env.find_module(&current_test_module_name())?;
    let _function = env.find_module_function(&module, "sum", "ii:i")?;
    let _ref_function = env.find_module_function(
        &module,
        "join",
        "C{std.core.String}C{std.core.String}:C{std.core.String}",
    )?;
    let _variable = env.find_module_variable(&module, "counter")?;
    let _ref_variable = env.find_module_variable(&module, "label")?;
    Ok(true)
}

#[ani]
pub fn call_current_module_sum(env: &Env<'_>, a: i32, b: i32) -> Result<i32> {
    let module = env.find_module(&current_test_module_name())?;
    let function = env.find_module_function(&module, "sum", "ii:i")?;
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_function_int(&function, &args)
}

#[ani]
pub fn call_current_module_join(env: &Env<'_>, left: String, right: String) -> Result<String> {
    let module = env.find_module(&current_test_module_name())?;
    let function = env.find_module_function(
        &module,
        "join",
        "C{std.core.String}C{std.core.String}:C{std.core.String}",
    )?;
    let left = env.create_string(&left)?;
    let right = env.create_string(&right)?;
    let args = [
        ani_value_ref(left.as_raw() as ani::sys::ani_ref),
        ani_value_ref(right.as_raw() as ani::sys::ani_ref),
    ];
    let result = env.call_function_ref(&function, &args)?;
    string_from_ref(env, result)
}

#[ani]
pub fn roundtrip_current_module_counter(env: &Env<'_>, value: i32) -> Result<i32> {
    let module = env.find_module(&current_test_module_name())?;
    let variable = env.find_module_variable(&module, "counter")?;
    env.set_variable_int(&variable, value)?;
    env.get_variable_int(&variable)
}

#[ani]
pub fn roundtrip_current_module_label(env: &Env<'_>, value: String) -> Result<String> {
    let module = env.find_module(&current_test_module_name())?;
    let variable = env.find_module_variable(&module, "label")?;
    let value = env.create_string(&value)?;
    let value_ref = unsafe { AniRef::from_raw(value.as_raw() as ani::sys::ani_ref) };
    env.set_variable_ref(&variable, &value_ref)?;
    let result = env.get_variable_ref(&variable)?;
    string_from_ref(env, result)
}

#[ani]
pub fn find_current_namespace_members(env: &Env<'_>, namespace_name: String) -> Result<bool> {
    let descriptor = current_namespace_descriptor(&namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let _function = env.find_namespace_function(&namespace, "mul", "ii:i")?;
    let _ref_function =
        env.find_namespace_function(&namespace, "tag", "C{std.core.String}:C{std.core.String}")?;
    let _variable = env.find_namespace_variable(&namespace, "state")?;
    let _ref_variable = env.find_namespace_variable(&namespace, "note")?;
    Ok(true)
}

#[ani]
pub fn call_current_namespace_mul(
    env: &Env<'_>,
    namespace_name: String,
    a: i32,
    b: i32,
) -> Result<i32> {
    let descriptor = current_namespace_descriptor(&namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let function = env.find_namespace_function(&namespace, "mul", "ii:i")?;
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_function_int(&function, &args)
}

#[ani]
pub fn call_current_namespace_tag(
    env: &Env<'_>,
    namespace_name: String,
    value: String,
) -> Result<String> {
    let descriptor = current_namespace_descriptor(&namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let function =
        env.find_namespace_function(&namespace, "tag", "C{std.core.String}:C{std.core.String}")?;
    let value = env.create_string(&value)?;
    let args = [ani_value_ref(value.as_raw() as ani::sys::ani_ref)];
    let result = env.call_function_ref(&function, &args)?;
    string_from_ref(env, result)
}

#[ani]
pub fn roundtrip_current_namespace_state(
    env: &Env<'_>,
    namespace_name: String,
    value: i32,
) -> Result<i32> {
    let descriptor = current_namespace_descriptor(&namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let variable = env.find_namespace_variable(&namespace, "state")?;
    env.set_variable_int(&variable, value)?;
    env.get_variable_int(&variable)
}

#[ani]
pub fn roundtrip_current_namespace_note(
    env: &Env<'_>,
    namespace_name: String,
    value: String,
) -> Result<String> {
    let descriptor = current_namespace_descriptor(&namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let variable = env.find_namespace_variable(&namespace, "note")?;
    let value = env.create_string(&value)?;
    let value_ref = unsafe { AniRef::from_raw(value.as_raw() as ani::sys::ani_ref) };
    env.set_variable_ref(&variable, &value_ref)?;
    let result = env.get_variable_ref(&variable)?;
    string_from_ref(env, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = find_current_module_members;
        let _ = call_current_module_sum;
        let _ = call_current_module_join;
        let _ = roundtrip_current_module_counter;
        let _ = roundtrip_current_module_label;
        let _ = find_current_namespace_members;
        let _ = call_current_namespace_mul;
        let _ = call_current_namespace_tag;
        let _ = roundtrip_current_namespace_state;
        let _ = roundtrip_current_namespace_note;
    }
}
