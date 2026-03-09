//! Class reflection API example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn resolve_getter_and_setter(env: &Env<'_>, cls: AniClass<'_>, name: String) -> Result<bool> {
    let _getter = env.find_getter(&cls, &name)?;
    let _setter = env.find_setter(&cls, &name)?;
    Ok(true)
}

#[ani]
pub fn resolve_getter_and_setter_by_name(
    env: &Env<'_>,
    class_name: String,
    name: String,
) -> Result<bool> {
    let cls = env.find_class(&class_name)?;
    let _getter = env.find_getter(&cls, &name)?;
    let _setter = env.find_setter(&cls, &name)?;
    Ok(true)
}

#[ani]
pub fn resolve_indexable_and_iterator(
    env: &Env<'_>,
    cls: AniClass<'_>,
    getter_signature: String,
    setter_signature: String,
) -> Result<bool> {
    let _getter = env.find_indexable_getter(&cls, &getter_signature)?;
    let _setter = env.find_indexable_setter(&cls, &setter_signature)?;
    let _iterator = env.find_iterator(&cls)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = resolve_getter_and_setter;
        let _ = resolve_getter_and_setter_by_name;
        let _ = resolve_indexable_and_iterator;
    }
}
