//! Set Example - Handling ArkTS Set<string> values.

use std::collections::{BTreeSet, HashSet};

use ani_derive::ani;

#[ani]
pub fn make_word_set() -> HashSet<String> {
    let mut values = HashSet::new();
    values.insert("ani".to_string());
    values.insert("arkts".to_string());
    values.insert("ets".to_string());
    values
}

#[ani]
pub fn make_empty_word_set() -> HashSet<String> {
    HashSet::new()
}

#[ani]
pub fn count_word_set(values: HashSet<String>) -> i32 {
    values.len() as i32
}

#[ani]
pub fn make_sorted_word_set() -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    values.insert("ani".to_string());
    values.insert("arkts".to_string());
    values.insert("ets".to_string());
    values
}

#[ani]
pub fn count_sorted_word_set(values: BTreeSet<String>) -> i32 {
    values.len() as i32
}

#[ani]
pub fn has_word(values: HashSet<String>, word: String) -> bool {
    values.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_set_contents_are_stable() {
        let values = make_word_set();
        assert!(values.contains("ani"));
        assert!(values.contains("arkts"));
        assert!(values.contains("ets"));
        assert!(!values.contains("missing"));
        assert_eq!(count_word_set(values.clone()), 3);
        assert!(has_word(values, "ani".to_string()));
        assert!(make_empty_word_set().is_empty());

        let sorted = make_sorted_word_set();
        assert_eq!(count_sorted_word_set(sorted.clone()), 3);
        assert!(sorted.contains("ani"));
        assert!(sorted.contains("arkts"));
        assert!(sorted.contains("ets"));
        assert!(!sorted.contains("missing"));
    }
}
