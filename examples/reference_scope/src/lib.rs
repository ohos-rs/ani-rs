//! Reference and scope APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn use_reference_scope(env: &Env<'_>, local_ref: AniRef<'_>) -> Result<()> {
    env.ensure_enough_references(16)?;

    let scope = env.create_local_scope(8)?;
    let _ = env.is_nullish(&local_ref)?;
    scope.close()?;

    env.create_escape_local_scope(4)?;
    let escaped = env.destroy_escape_local_scope(&local_ref)?;
    env.delete_local_ref(&escaped)
}

#[ani]
pub fn compare_references(env: &Env<'_>, lhs: AniRef<'_>, rhs: AniRef<'_>) -> Result<bool> {
    Ok(env.reference_equals(&lhs, &rhs)? && env.reference_strict_equals(&lhs, &rhs)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = use_reference_scope;
        let _ = compare_references;
    }
}
