//! Function call and variable access APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn call_function_int_example(env: &Env<'_>, function: AniFunction) -> Result<i32> {
    let args = [ani_value_int(7), ani_value_int(9)];
    env.call_function_int(&function, &args)
}

#[ani]
pub fn call_function_void_example(env: &Env<'_>, function: AniFunction) -> Result<()> {
    env.call_function_void(&function, &[])
}

#[ani]
pub fn variable_roundtrip_int(env: &Env<'_>, variable: AniVariable, value: i32) -> Result<i32> {
    env.set_variable_int(&variable, value)?;
    env.get_variable_int(&variable)
}

#[ani]
pub fn variable_roundtrip_ref(
    env: &Env<'_>,
    variable: AniVariable,
    value: AniRef<'_>,
) -> Result<bool> {
    env.set_variable_ref(&variable, &value)?;
    let got = env.get_variable_ref(&variable)?;
    env.reference_equals(&got, &value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = call_function_int_example;
        let _ = call_function_void_example;
        let _ = variable_roundtrip_int;
        let _ = variable_roundtrip_ref;
    }
}
