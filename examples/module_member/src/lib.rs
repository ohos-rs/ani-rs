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

#[ani]
pub fn find_current_module_members(env: &Env<'_>) -> Result<bool> {
    let module = env.find_module(&current_test_module_name())?;
    let _function = env.find_module_function(&module, "sum", "ii:i")?;
    let _variable = env.find_module_variable(&module, "counter")?;
    Ok(true)
}

#[ani]
pub fn find_current_namespace_members(env: &Env<'_>, namespace_name: String) -> Result<bool> {
    let descriptor = format!("{}.{}", current_test_module_name(), namespace_name);
    let namespace = env.find_namespace(&descriptor)?;
    let _function = env.find_namespace_function(&namespace, "mul", "ii:i")?;
    let _variable = env.find_namespace_variable(&namespace, "state")?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = find_current_module_members;
        let _ = find_current_namespace_members;
    }
}
