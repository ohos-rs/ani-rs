//! TupleValue wrapper example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn tuple_sum(env: &Env<'_>, tuple_raw: AniTupleValue<'_>) -> Result<i32> {
    let tuple = TupleValue::from_handle(tuple_raw);
    let a = tuple.get_int(env, 0)?;
    let b = tuple.get_int(env, 1)?;
    Ok(a + b)
}

#[ani]
pub fn tuple_set_first(env: &Env<'_>, tuple_raw: AniTupleValue<'_>, value: i32) -> Result<()> {
    let tuple = TupleValue::from_handle(tuple_raw);
    tuple.set_int(env, 0, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = tuple_sum;
        let _ = tuple_set_first;
    }
}
