//! Generic array APIs example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn array_push_and_pop(env: &Env<'_>, value: AniRef<'_>) -> Result<bool> {
    let array = env.create_array(0, None)?;
    env.push_array_element(&array, &value)?;
    let popped = env.pop_array_element(&array)?;
    env.reference_equals(&popped, &value)
}

#[ani]
pub fn array_set_and_get(env: &Env<'_>, value: AniRef<'_>) -> Result<bool> {
    let array = env.create_array(1, Some(&value))?;
    env.set_array_element(&array, 0, &value)?;
    let got = env.get_array_element(&array, 0)?;
    env.reference_equals(&got, &value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = array_push_and_pop;
        let _ = array_set_and_get;
    }
}
