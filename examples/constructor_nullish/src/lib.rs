//! Constructor nullish bridge example.

use ani_derive::ani;
use std::sync::{Mutex, OnceLock};

static DISPLAY_NAME: OnceLock<Mutex<String>> = OnceLock::new();

fn display_name_store() -> &'static Mutex<String> {
    DISPLAY_NAME.get_or_init(|| Mutex::new(String::new()))
}

pub struct Person;

#[ani(class = "Person", constructor)]
pub fn person_new(name: Option<String>) {
    let value = name.unwrap_or_else(|| "anonymous".to_string());
    if let Ok(mut slot) = display_name_store().lock() {
        *slot = value;
    }
}

#[ani(class = "Person", getter = "name")]
pub fn person_get_name() -> String {
    display_name_store()
        .lock()
        .map(|slot| slot.clone())
        .unwrap_or_default()
}

#[ani(class = "Person", name = "rename")]
pub fn person_rename(name: Option<String>) {
    let value = name.unwrap_or_else(|| "anonymous".to_string());
    if let Ok(mut slot) = display_name_store().lock() {
        *slot = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nullish_constructor_logic_works() {
        person_new(None);
        assert_eq!(person_get_name(), "anonymous");

        person_new(Some("ark".to_string()));
        assert_eq!(person_get_name(), "ark");

        person_rename(None);
        assert_eq!(person_get_name(), "anonymous");
    }
}
