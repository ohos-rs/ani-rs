//! Class static field/method APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn static_field_roundtrip_int(
    env: &Env<'_>,
    cls: AniClass<'_>,
    field: AniStaticField,
    value: i32,
) -> Result<i32> {
    env.set_static_field_int(&cls, &field, value)?;
    env.get_static_field_int(&cls, &field)
}

#[ani]
pub fn call_static_int(
    env: &Env<'_>,
    cls: AniClass<'_>,
    method: AniStaticMethod,
) -> Result<i32> {
    let args = [ani_value_int(2), ani_value_int(3)];
    env.call_static_method_int(&cls, &method, &args)
}

#[ani]
pub fn lookup_static_field(env: &Env<'_>, cls: AniClass<'_>) -> Result<bool> {
    let _field = env.find_static_field(&cls, "COUNT")?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = static_field_roundtrip_int;
        let _ = call_static_int;
        let _ = lookup_static_field;
    }
}
