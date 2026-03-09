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
pub fn static_field_roundtrip_int_by_name(
    env: &Env<'_>,
    class_name: String,
    field_name: String,
    value: i32,
) -> Result<i32> {
    let cls = env.find_class(&class_name)?;
    let field = env.find_static_field(&cls, &field_name)?;
    env.set_static_field_int(&cls, &field, value)?;
    env.get_static_field_int(&cls, &field)
}

#[ani]
pub fn call_static_int(env: &Env<'_>, cls: AniClass<'_>, method: AniStaticMethod) -> Result<i32> {
    let args = [ani_value_int(2), ani_value_int(3)];
    env.call_static_method_int(&cls, &method, &args)
}

#[ani]
pub fn call_static_int_by_name(
    env: &Env<'_>,
    class_name: String,
    method_name: String,
    a: i32,
    b: i32,
) -> Result<i32> {
    let cls = env.find_class(&class_name)?;
    let method = env.find_static_method(&cls, &method_name, "ii:i")?;
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_static_method_int(&cls, &method, &args)
}

#[ani]
pub fn lookup_static_field(env: &Env<'_>, cls: AniClass<'_>) -> Result<bool> {
    let _field = env.find_static_field(&cls, "COUNT")?;
    Ok(true)
}

#[ani]
pub fn lookup_static_field_by_name(
    env: &Env<'_>,
    class_name: String,
    field_name: String,
) -> Result<bool> {
    let cls = env.find_class(&class_name)?;
    let _field = env.find_static_field(&cls, &field_name)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = static_field_roundtrip_int;
        let _ = static_field_roundtrip_int_by_name;
        let _ = call_static_int;
        let _ = call_static_int_by_name;
        let _ = lookup_static_field;
        let _ = lookup_static_field_by_name;
    }
}
