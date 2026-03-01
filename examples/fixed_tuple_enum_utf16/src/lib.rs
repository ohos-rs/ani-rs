//! FixedArray/TupleValue/EnumItem/UTF16 APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn utf16_roundtrip(env: &Env<'_>, text: String) -> Result<String> {
    let utf16 = text.encode_utf16().collect::<Vec<u16>>();
    let s = env.create_string_utf16(&utf16)?;
    let roundtrip = env.get_string_utf16(&s)?;
    Ok(String::from_utf16_lossy(&roundtrip))
}

#[ani]
pub fn enum_item_lookup_value(env: &Env<'_>, enm: AniEnum<'_>) -> Result<i32> {
    let item = env.get_enum_item_by_name(&enm, "A")?;
    env.get_enum_item_value_int(&item)
}

#[ani]
pub fn enum_item_lookup_index(env: &Env<'_>, enm: AniEnum<'_>) -> Result<i32> {
    let item = env.get_enum_item_by_name(&enm, "A")?;
    Ok(env.get_enum_item_index(&item)? as i32)
}

#[ani]
pub fn fixed_array_region(env: &Env<'_>) -> Result<Vec<i32>> {
    let arr = env.create_fixed_array_int(4)?;
    env.set_fixed_array_region_int(&arr, 0, &[1, 2, 3, 4])?;
    env.get_fixed_array_region_int(&arr, 0, 4)
}

#[ani]
pub fn tuple_roundtrip(env: &Env<'_>, tuple: AniTupleValue<'_>) -> Result<i32> {
    env.set_tuple_item_int(&tuple, 0, 42)?;
    env.get_tuple_item_int(&tuple, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = utf16_roundtrip;
        let _ = enum_item_lookup_value;
        let _ = enum_item_lookup_index;
        let _ = fixed_array_region;
        let _ = tuple_roundtrip;
    }
}
