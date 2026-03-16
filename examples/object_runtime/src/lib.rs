//! Runtime object operations example.
//!
//! Covers ANI object construction, type relation checks, and by-name/by-handle
//! object method calls with real argument arrays.

use ani::prelude::*;
use ani_derive::ani;

const RUNTIME_BOX_CLASS: &str = "arkvm_test.RuntimeBox";
const RUNTIME_BOX_CTOR_SIG: &str = "iC{std.core.String}:";
const RUNTIME_BOX_DESCRIBE_SIG: &str = ":C{std.core.String}";

fn ref_to_string(env: &Env<'_>, value: AniRef<'_>) -> Result<String> {
    let string = unsafe { AniString::from_raw(value.into_raw() as ani::sys::ani_string) };
    env.get_string(&string)
}

#[ani]
pub fn create_runtime_box(env: &Env<'_>, value: i32, label: String) -> Result<String> {
    let cls = env.find_class(RUNTIME_BOX_CLASS)?;
    let ctor = env.find_constructor(&cls, RUNTIME_BOX_CTOR_SIG)?;
    let label = ani::conversions::ToAni::to_ani(label, env)?;
    let args = [
        ani_value_int(value),
        ani_value_ref(label.as_raw() as ani::sys::ani_ref),
    ];
    let obj = env.new_object(&cls, &ctor, &args)?;
    let actual_value = env.get_field_by_name_int(&obj, "value")?;
    let actual_label = ref_to_string(env, env.get_field_by_name_ref(&obj, "label")?)?;
    Ok(format!("{}:{}", actual_label, actual_value))
}

#[ani]
pub fn sum_by_name(env: &Env<'_>, obj: AniObject<'_>, left: i32, right: i32) -> Result<i32> {
    let args = [ani_value_int(left), ani_value_int(right)];
    env.call_method_by_name_int_with_args(&obj, "sumNumbers", None, &args)
}

#[ani]
pub fn compare_by_name(env: &Env<'_>, obj: AniObject<'_>, left: i32, right: i32) -> Result<String> {
    let args = [ani_value_int(left), ani_value_int(right)];
    let result = env.call_method_by_name_ref_with_args(&obj, "compareNumbers", None, &args)?;
    ref_to_string(env, result)
}

#[ani]
pub fn describe_by_name_zero(env: &Env<'_>, obj: AniObject<'_>) -> Result<String> {
    let result = env.call_method_by_name_ref(&obj, "describe", Some(RUNTIME_BOX_DESCRIBE_SIG))?;
    ref_to_string(env, result)
}

#[ani]
pub fn is_positive_by_name(env: &Env<'_>, obj: AniObject<'_>) -> Result<bool> {
    env.call_method_by_name_boolean(&obj, "isPositive", None)
}

#[ani]
pub fn clear_label_by_name(env: &Env<'_>, obj: AniObject<'_>) -> Result<String> {
    env.call_method_by_name_void(&obj, "clearLabel", Some(":"))?;
    let label = env.get_field_by_name_ref(&obj, "label")?;
    ref_to_string(env, label)
}

#[ani]
pub fn describe_by_handle(env: &Env<'_>, obj: AniObject<'_>) -> Result<String> {
    let cls = env.find_class(RUNTIME_BOX_CLASS)?;
    let method = env.find_method(&cls, "describe", RUNTIME_BOX_DESCRIBE_SIG)?;
    let result = env.call_ref_method(&obj, &method, &[])?;
    ref_to_string(env, result)
}

#[ani]
pub fn is_runtime_box_instance(env: &Env<'_>, obj: AniObject<'_>) -> Result<bool> {
    let cls = env.find_class(RUNTIME_BOX_CLASS)?;
    env.object_instance_of(&obj, &cls)
}

#[ani]
pub fn runtime_box_assignable_to_base(env: &Env<'_>, obj: AniObject<'_>) -> Result<bool> {
    let obj_ty = env.get_object_type(&obj)?;
    let base_cls = env.find_class(RUNTIME_BOX_CLASS)?;
    let base_ty: AniType<'_> = base_cls.into();
    env.is_assignable_from(&obj_ty, &base_ty)
}

#[ani]
pub fn runtime_box_has_super(env: &Env<'_>, obj: AniObject<'_>) -> Result<bool> {
    let obj_ty = env.get_object_type(&obj)?;
    let super_cls = env.get_super_class(&obj_ty)?;
    env.object_instance_of(&obj, &super_cls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = create_runtime_box;
        let _ = sum_by_name;
        let _ = compare_by_name;
        let _ = describe_by_name_zero;
        let _ = is_positive_by_name;
        let _ = clear_label_by_name;
        let _ = describe_by_handle;
        let _ = is_runtime_box_instance;
        let _ = runtime_box_assignable_to_base;
        let _ = runtime_box_has_super;
        assert_eq!(RUNTIME_BOX_CLASS, "arkvm_test.RuntimeBox");
        assert_eq!(RUNTIME_BOX_CTOR_SIG, "iC{std.core.String}:");
        assert_eq!(RUNTIME_BOX_DESCRIBE_SIG, ":C{std.core.String}");
    }
}
