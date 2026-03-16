//! Class static member by-name APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[cfg(test)]
const BY_NAME_HOST_CLASS: &str = "arkvm_test.ByNameHost";

#[ani]
pub fn static_field_by_name_roundtrip(env: &Env<'_>, cls: AniClass<'_>) -> Result<i32> {
    env.set_static_field_by_name_int(&cls, "COUNT", 7)?;
    env.get_static_field_by_name_int(&cls, "COUNT")
}

#[ani]
pub fn static_field_by_name_roundtrip_named(env: &Env<'_>, class_name: String) -> Result<i32> {
    let cls = env.find_class(&class_name)?;
    env.set_static_field_by_name_int(&cls, "COUNT", 7)?;
    env.get_static_field_by_name_int(&cls, "COUNT")
}

#[ani]
pub fn static_ref_by_name_roundtrip(
    env: &Env<'_>,
    cls: AniClass<'_>,
    value: AniRef<'_>,
) -> Result<bool> {
    env.set_static_field_by_name_ref(&cls, "PAYLOAD", &value)?;
    let got = env.get_static_field_by_name_ref(&cls, "PAYLOAD")?;
    env.reference_equals(&got, &value)
}

#[ani]
pub fn static_ref_by_name_roundtrip_named(
    env: &Env<'_>,
    class_name: String,
    value: AniRef<'_>,
) -> Result<bool> {
    let cls = env.find_class(&class_name)?;
    env.set_static_field_by_name_ref(&cls, "PAYLOAD", &value)?;
    let got = env.get_static_field_by_name_ref(&cls, "PAYLOAD")?;
    env.reference_equals(&got, &value)
}

#[ani]
pub fn static_method_sum_by_name_named(
    env: &Env<'_>,
    class_name: String,
    a: i32,
    b: i32,
) -> Result<i32> {
    let cls = env.find_class(&class_name)?;
    let args = [ani_value_int(a), ani_value_int(b)];
    env.call_static_method_by_name_int_with_args(&cls, "sum", Some("ii:i"), &args)
}

#[ani]
pub fn static_method_flag_by_name_named(env: &Env<'_>, class_name: String) -> Result<bool> {
    let cls = env.find_class(&class_name)?;
    env.call_static_method_by_name_boolean(&cls, "flag", Some(":z"))
}

#[ani]
pub fn static_method_label_by_name_named(
    env: &Env<'_>,
    class_name: String,
    prefix: String,
    suffix: String,
) -> Result<String> {
    let cls = env.find_class(&class_name)?;
    let prefix = env.create_string(&prefix)?;
    let suffix = env.create_string(&suffix)?;
    let args = [
        ani_value_ref(prefix.as_raw() as ani::sys::ani_ref),
        ani_value_ref(suffix.as_raw() as ani::sys::ani_ref),
    ];
    let result = env.call_static_method_by_name_ref_with_args(
        &cls,
        "label",
        Some("C{std.core.String}C{std.core.String}:C{std.core.String}"),
        &args,
    )?;
    let result = unsafe { AniString::from_raw(result.into_raw() as ani::sys::ani_string) };
    env.get_string(&result)
}

#[ani]
pub fn static_method_reset_by_name_named(
    env: &Env<'_>,
    class_name: String,
    value: i32,
) -> Result<i32> {
    let cls = env.find_class(&class_name)?;
    let args = [ani_value_int(value)];
    env.call_static_method_by_name_void_with_args(&cls, "resetTo", Some("i:"), &args)?;
    env.get_static_field_by_name_int(&cls, "COUNT")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = static_field_by_name_roundtrip;
        let _ = static_field_by_name_roundtrip_named;
        let _ = static_ref_by_name_roundtrip;
        let _ = static_ref_by_name_roundtrip_named;
        let _ = static_method_sum_by_name_named;
        let _ = static_method_flag_by_name_named;
        let _ = static_method_label_by_name_named;
        let _ = static_method_reset_by_name_named;
        assert_eq!(BY_NAME_HOST_CLASS, "arkvm_test.ByNameHost");
    }
}
