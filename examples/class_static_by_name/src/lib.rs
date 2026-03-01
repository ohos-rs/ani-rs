//! Class static field by-name APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn static_field_by_name_roundtrip(env: &Env<'_>, cls: AniClass<'_>) -> Result<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = static_field_by_name_roundtrip;
        let _ = static_ref_by_name_roundtrip;
    }
}
