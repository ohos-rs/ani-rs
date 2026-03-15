//! Object field/property access example.
//!
//! Focuses on by-name and by-handle reads/writes for primitive and reference
//! values, matching the core ANI object_ops coverage model.

use ani::prelude::*;
use ani_derive::ani;

const ACCESS_TARGET_CLASS: &str = "arkvm_test.AccessTarget";

#[ani]
pub fn field_by_name_int_roundtrip(env: &Env<'_>, obj: AniObject<'_>, value: i32) -> Result<i32> {
    env.set_field_by_name_int(&obj, "counter", value)?;
    env.get_field_by_name_int(&obj, "counter")
}

#[ani]
pub fn field_by_handle_int_roundtrip(env: &Env<'_>, obj: AniObject<'_>, value: i32) -> Result<i32> {
    let cls = env.find_class(ACCESS_TARGET_CLASS)?;
    let field = env.find_field(&cls, "counter")?;
    env.set_field_int(&obj, &field, value)?;
    env.get_field_int(&obj, &field)
}

#[ani]
pub fn field_ref_roundtrip(
    env: &Env<'_>,
    obj: AniObject<'_>,
    value: AniString<'_>,
) -> Result<bool> {
    let expected: AniRef<'_> = value.into();
    env.set_field_by_name_ref(&obj, "label", &expected)?;
    let got = env.get_field_by_name_ref(&obj, "label")?;
    env.reference_strict_equals(&got, &expected)
}

#[ani]
pub fn property_by_name_double_roundtrip(
    env: &Env<'_>,
    obj: AniObject<'_>,
    value: f64,
) -> Result<f64> {
    env.set_property_by_name_double(&obj, "ratio", value)?;
    env.get_property_by_name_double(&obj, "ratio")
}

#[ani]
pub fn property_ref_roundtrip(
    env: &Env<'_>,
    obj: AniObject<'_>,
    value: AniString<'_>,
) -> Result<bool> {
    let expected: AniRef<'_> = value.into();
    env.set_property_by_name_ref(&obj, "alias", &expected)?;
    let got = env.get_property_by_name_ref(&obj, "alias")?;
    env.reference_strict_equals(&got, &expected)
}

#[cfg(test)]
mod tests {
    use super::ACCESS_TARGET_CLASS;
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = field_by_name_int_roundtrip;
        let _ = field_by_handle_int_roundtrip;
        let _ = field_ref_roundtrip;
        let _ = property_by_name_double_roundtrip;
        let _ = property_ref_roundtrip;
        assert_eq!(ACCESS_TARGET_CLASS, "arkvm_test.AccessTarget");
    }
}
