//! Type relation APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn check_type_relation(env: &Env<'_>, from: AniType<'_>, to: AniType<'_>) -> Result<bool> {
    env.is_assignable_from(&from, &to)
}

#[ani]
pub fn get_super_class_example(env: &Env<'_>, ty: AniType<'_>) -> Result<bool> {
    let _cls = env.get_super_class(&ty)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = check_type_relation;
        let _ = get_super_class_example;
    }
}
