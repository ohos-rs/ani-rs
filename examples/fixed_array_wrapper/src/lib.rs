//! FixedArray conversion wrappers example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn sum_fixed_int(values: FixedIntArray) -> i32 {
    values.as_ref().iter().copied().sum()
}

#[ani]
pub fn negate_fixed_bool(values: FixedBooleanArray) -> FixedBooleanArray {
    let result = values
        .as_ref()
        .iter()
        .copied()
        .map(|v| !v)
        .collect::<Vec<bool>>();
    result.into()
}

#[ani]
pub fn roundtrip_fixed_int(values: FixedIntArray) -> FixedIntArray {
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_usage_compile() {
        let _ = sum_fixed_int(FixedIntArray::from([1, 2, 3]));
        let _ = negate_fixed_bool(FixedBooleanArray::from([true, false]));
        let _ = roundtrip_fixed_int(FixedIntArray::from([3, 4, 5]));
    }
}
