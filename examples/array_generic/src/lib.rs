//! Generic array APIs example.

use std::collections::{LinkedList, VecDeque};

use ani::prelude::*;
use ani_derive::ani;

#[ani(object)]
pub struct ArrayUser {
    pub id: i32,
    pub name: String,
}

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

#[ani]
pub fn join_string_array(values: Vec<String>) -> String {
    values.join("|")
}

#[ani]
pub fn make_string_array() -> Vec<String> {
    vec!["ani".to_string(), "ark".to_string(), "vm".to_string()]
}

#[ani]
pub fn join_string_vecdeque(values: VecDeque<String>) -> String {
    values.into_iter().collect::<Vec<_>>().join("|")
}

#[ani]
pub fn make_string_vecdeque() -> VecDeque<String> {
    let mut values = VecDeque::new();
    values.push_back("front".to_string());
    values.push_back("middle".to_string());
    values.push_back("back".to_string());
    values
}

#[ani]
pub fn sum_i32_linked_list(values: LinkedList<i32>) -> i32 {
    values.into_iter().sum()
}

#[ani]
pub fn make_i32_linked_list() -> LinkedList<i32> {
    let mut values = LinkedList::new();
    values.push_back(3);
    values.push_back(4);
    values.push_back(5);
    values
}

#[ani]
pub fn summarize_user_array(values: Vec<ArrayUser>) -> String {
    values
        .into_iter()
        .map(|user| format!("{}#{}", user.id, user.name))
        .collect::<Vec<_>>()
        .join("|")
}

#[ani]
pub fn make_user_array() -> Vec<ArrayUser> {
    vec![
        ArrayUser {
            id: 7,
            name: "alice".to_string(),
        },
        ArrayUser {
            id: 8,
            name: "bob".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = ArrayUser {
            id: 0,
            name: String::new(),
        };
        let _ = array_push_and_pop;
        let _ = array_set_and_get;
        let _ = join_string_array;
        let _ = make_string_array;
        let _ = join_string_vecdeque;
        let _ = make_string_vecdeque;
        let _ = sum_i32_linked_list;
        let _ = make_i32_linked_list;
        let _ = summarize_user_array;
        let _ = make_user_array;
    }
}
