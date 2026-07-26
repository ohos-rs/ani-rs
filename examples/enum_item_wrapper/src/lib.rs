//! EnumItem wrapper example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn enum_item_name(env: &Env<'_>, enum_descriptor: String, name: String) -> Result<String> {
    let enm = env.find_enum(&enum_descriptor)?;
    let item = EnumItem::from_name(env, &enm, &name)?;
    item.name(env)
}

#[ani]
pub fn enum_item_value_int(env: &Env<'_>, enum_descriptor: String, name: String) -> Result<i32> {
    let enm = env.find_enum(&enum_descriptor)?;
    let item = EnumItem::from_name(env, &enm, &name)?;
    item.int_value(env)
}

#[ani]
pub fn enum_item_index_by_name(
    env: &Env<'_>,
    enum_descriptor: String,
    name: String,
) -> Result<i32> {
    let enm = env.find_enum(&enum_descriptor)?;
    let item = EnumItem::from_name(env, &enm, &name)?;
    Ok(item.index(env)? as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = enum_item_name;
        let _ = enum_item_value_int;
        let _ = enum_item_index_by_name;
    }
}
