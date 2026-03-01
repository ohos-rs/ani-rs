//! Module/namespace member lookup APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn find_module_members(env: &Env<'_>, module: AniModule<'_>) -> Result<bool> {
    let _function = env.find_module_function(&module, "sum", "II:I")?;
    let _variable = env.find_module_variable(&module, "counter")?;
    Ok(true)
}

#[ani]
pub fn find_namespace_members(env: &Env<'_>, namespace: AniNamespace<'_>) -> Result<bool> {
    let _function = env.find_namespace_function(&namespace, "mul", "II:I")?;
    let _variable = env.find_namespace_variable(&namespace, "state")?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = find_module_members;
        let _ = find_namespace_members;
    }
}
