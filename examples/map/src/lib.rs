//! Map Example - Handling ArkTS Map<string, int> values.

use std::collections::BTreeMap;

use ani_derive::ani;

#[ani]
pub fn make_score_map() -> BTreeMap<String, i32> {
    let mut values = BTreeMap::new();
    values.insert("ani".to_string(), 1);
    values.insert("arkts".to_string(), 2);
    values.insert("ets".to_string(), 3);
    values
}

#[ani]
pub fn make_empty_score_map() -> BTreeMap<String, i32> {
    BTreeMap::new()
}

#[ani]
pub fn sum_score_map(values: BTreeMap<String, i32>) -> i32 {
    values.values().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_map_contents_are_stable() {
        let values = make_score_map();
        assert_eq!(values.get("ani"), Some(&1));
        assert_eq!(values.get("arkts"), Some(&2));
        assert_eq!(values.get("ets"), Some(&3));
        assert_eq!(sum_score_map(values), 6);
        assert!(make_empty_score_map().is_empty());
    }
}
