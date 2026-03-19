//! AnyValue wrapper example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn dynamic_call_with_fn_args(env: &Env<'_>, func: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&func);
    let arg_a = env.create_string("left")?;
    let arg_b = env.create_string("right")?;
    let result = any.call(env, (arg_a, arg_b))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[ani]
pub fn dynamic_method_call(env: &Env<'_>, obj: AniRef<'_>) -> Result<bool> {
    let any = AnyValue::from_borrowed_ref(&obj);
    let method = any.get_property(env, "next")?;
    let arg = env.create_string("step")?;
    let result = method.call(env, (arg,))?;
    Ok(!env.is_nullish(result.as_ref())?)
}

#[ani]
pub fn dynamic_set_property(env: &Env<'_>, obj: AniRef<'_>) -> Result<()> {
    let any = AnyValue::from_borrowed_ref(&obj);
    any.set_property_arg(env, "count", 42_i32)
}

#[ani]
pub fn dynamic_identity(value: AnyValue<'_>) -> AnyValue<'_> {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = dynamic_call_with_fn_args;
        let _ = dynamic_method_call;
        let _ = dynamic_set_property;
        let _ = dynamic_identity;
    }
}
