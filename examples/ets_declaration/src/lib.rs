//! ETS declaration generation example.
//!
//! This example intentionally mixes module-level functions, nested namespace
//! functions, and namespaced class exports in a single dynamic library.

use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(Debug, Clone, PartialEq, Eq, AniClass)]
#[ani(class = "example.Person")]
pub struct Person {
    pub _name: String,
    pub _score: i32,
}

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(namespace = "AniMath.Utils")]
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[ani(namespace = "AniMath.Utils")]
pub fn sum3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[ani(class = "example.Person")]
impl Person {
    #[ani(constructor)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(env: &Env<'_>, this: &AniObject<'_>, name: String, score: i32) -> Result<()> {
        Person {
            _name: name,
            _score: score,
        }
        .write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_name(&self) -> String {
        self._name.clone()
    }

    #[ani(getter)]
    pub fn get_score(&self) -> i32 {
        self._score
    }

    #[ani(setter)]
    pub fn set_score(&mut self, score: i32) {
        self._score = score;
    }

    #[ani]
    pub fn label(&self) -> String {
        format!("{}#{}", self._name, self._score)
    }

    #[ani(static)]
    pub fn species() -> String {
        "human".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{add, sqrt, sum3, Person};

    #[test]
    fn mixed_export_logic_matches_expected_values() {
        assert_eq!(add(3, 4), 7);
        assert!((sqrt(9.0) - 3.0).abs() < f64::EPSILON);
        assert_eq!(sum3(1, 2, 3), 6);

        let mut person = Person {
            _name: "ani-rs".to_string(),
            _score: 7,
        };
        assert_eq!(person.get_name(), "ani-rs");
        assert_eq!(person.get_score(), 7);
        person.set_score(9);
        assert_eq!(person.get_score(), 9);
        assert_eq!(person.label(), "ani-rs#9");
        assert_eq!(Person::species(), "human");
    }
}
