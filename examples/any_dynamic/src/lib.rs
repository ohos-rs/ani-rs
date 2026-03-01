//! Any dynamic APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn dynamic_get_set(env: &Env<'_>, obj: AniRef<'_>, value: AniRef<'_>) -> Result<bool> {
    env.any_set_property(&obj, "value", &value)?;
    let got = env.any_get_property(&obj, "value")?;
    env.reference_equals(&got, &value)
}

#[ani]
pub fn dynamic_call(env: &Env<'_>, func: AniRef<'_>, arg: AniRef<'_>) -> Result<bool> {
    let result = env.any_call(&func, std::slice::from_ref(&arg))?;
    Ok(!env.is_nullish(&result)?)
}

#[ani]
pub fn dynamic_construct(env: &Env<'_>, ctor: AniRef<'_>, arg0: AniRef<'_>) -> Result<bool> {
    let args = [arg0];
    let result = env.any_new(&ctor, &args)?;
    Ok(!env.is_nullish(&result)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = dynamic_get_set;
        let _ = dynamic_call;
        let _ = dynamic_construct;
    }
}
