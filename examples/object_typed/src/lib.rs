//! Object typed APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn field_roundtrip_boolean(
    env: &Env<'_>,
    obj: AniObject<'_>,
    field: AniField,
    value: bool,
) -> Result<bool> {
    env.set_field_boolean(&obj, &field, value)?;
    env.get_field_boolean(&obj, &field)
}

#[ani]
pub fn field_by_name_roundtrip_long(env: &Env<'_>, obj: AniObject<'_>, value: i64) -> Result<i64> {
    env.set_field_by_name_long(&obj, "counter", value)?;
    env.get_field_by_name_long(&obj, "counter")
}

#[ani]
pub fn property_roundtrip_float(env: &Env<'_>, obj: AniObject<'_>, value: f32) -> Result<f32> {
    env.set_property_by_name_float(&obj, "ratio", value)?;
    env.get_property_by_name_float(&obj, "ratio")
}

#[ani]
pub fn call_typed_methods(env: &Env<'_>, obj: AniObject<'_>, method: AniMethod) -> Result<f32> {
    let args = [ani_value_int(1)];
    let ch = env.call_char_method(&obj, &method, &args)? as f32;
    let byte = env.call_byte_method(&obj, &method, &args)? as f32;
    let short = env.call_short_method(&obj, &method, &args)? as f32;
    let float = env.call_float_method(&obj, &method, &args)?;
    Ok(ch + byte + short + float)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = field_roundtrip_boolean;
        let _ = field_by_name_roundtrip_long;
        let _ = property_roundtrip_float;
        let _ = call_typed_methods;
    }
}
