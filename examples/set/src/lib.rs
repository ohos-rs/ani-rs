//! Set Example - Handling ArkTS Set<string> values.

use std::collections::HashSet;

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
    }
}
