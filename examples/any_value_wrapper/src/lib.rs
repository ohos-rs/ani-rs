//! AnyValue wrapper example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn dynamic_call_with_fn_args(env: &Env<'_>, func: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&func);
    let result = any.call(env, FnArgs((1_i32, 2_i32)))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[ani]
pub fn dynamic_method_call(env: &Env<'_>, obj: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&obj);
    let result = any.call_method(env, "next", (1_i32,))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[ani]
pub fn dynamic_set_property(env: &Env<'_>, obj: AniRef<'_>) -> Result<()> {
    let any = AnyValue::from_borrowed_ref(&obj);
    any.set_property_arg(env, "count", 42_i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = dynamic_call_with_fn_args;
        let _ = dynamic_method_call;
        let _ = dynamic_set_property;
    }
}
