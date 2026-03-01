//! Weak reference APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn weak_ref_roundtrip(env: &Env<'_>, value: AniRef<'_>) -> Result<bool> {
    let weak = env.create_weak_ref(&value)?;
    let upgraded = env.upgrade_weak_ref(&weak)?;
    env.delete_weak_ref(weak)?;
    Ok(upgraded.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = weak_ref_roundtrip;
    }
}
