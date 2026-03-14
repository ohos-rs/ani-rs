//! Constructor overload example.

use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(Debug, Clone, PartialEq, Eq, AniClass)]
#[ani(class = "Measure")]
pub struct Measure {
    pub _kind: String,
    pub _total: i32,
}

#[ani(class = "Measure")]
impl Measure {
    #[ani(constructor)]
    pub fn from_pair(env: &Env<'_>, this: &AniObject<'_>, left: i32, right: i32) -> Result<()> {
        Measure {
            _kind: "pair".to_string(),
            _total: left + right,
        }
        .write_back_to_ani_object(env, this)
    }

    #[ani(constructor)]
    pub fn from_named_total(
        env: &Env<'_>,
        this: &AniObject<'_>,
        kind: String,
        total: i32,
    ) -> Result<()> {
        Measure {
            _kind: kind,
            _total: total,
        }
        .write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_kind(&self) -> String {
        self._kind.clone()
    }

    #[ani(getter)]
    pub fn get_total(&self) -> i32 {
        self._total
    }

    #[ani]
    pub fn describe(&self) -> String {
        format!("{}:{}", self._kind, self._total)
    }
}

#[cfg(test)]
mod tests {
    use super::Measure;

    #[test]
    fn measure_logic_matches_ctor_expectations() {
        let pair = Measure {
            _kind: "pair".to_string(),
            _total: 5,
        };
        assert_eq!(pair.get_kind(), "pair");
        assert_eq!(pair.get_total(), 5);
        assert_eq!(pair.describe(), "pair:5");

        let named = Measure {
            _kind: "named".to_string(),
            _total: 4,
        };
        assert_eq!(named.get_kind(), "named");
        assert_eq!(named.get_total(), 4);
        assert_eq!(named.describe(), "named:4");
    }
}
