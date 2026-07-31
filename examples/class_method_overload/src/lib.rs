//! Class method overload example.
//!
//! Mirrors ANI bind_ops coverage for repeated native methods on the same class.

use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(Debug, Clone, AniClass)]
#[ani(class = "MathBox")]
pub struct MathBox {
    pub _base: i32,
}

#[ani(class = "MathBox")]
impl MathBox {
    #[ani(constructor)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(env: &Env<'_>, this: &AniObject<'_>, base: i32) -> Result<()> {
        MathBox { _base: base }.write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_base(&self) -> i32 {
        self._base
    }

    #[ani(name = "mix")]
    pub fn mix2(&self, left: i32, right: i32) -> i32 {
        self._base + left + right
    }

    #[ani(name = "mix")]
    pub fn mix3(&self, left: i32, right: i32, extra: i32) -> i32 {
        self._base + left + right + extra
    }

    #[ani(static, name = "tag")]
    pub fn tag1(value: String) -> String {
        format!("[{value}]")
    }

    #[ani(static, name = "tag")]
    pub fn tag2(value: String, suffix: String) -> String {
        format!("[{value}:{suffix}]")
    }
}

#[cfg(test)]
mod tests {
    use super::MathBox;

    #[test]
    fn class_overload_logic_matches_expected_values() {
        let model = MathBox { _base: 5 };
        assert_eq!(model.get_base(), 5);
        assert_eq!(model.mix2(2, 3), 10);
        assert_eq!(model.mix3(2, 3, 4), 14);
        assert_eq!(MathBox::tag1("ark".to_string()), "[ark]");
        assert_eq!(
            MathBox::tag2("ark".to_string(), "ts".to_string()),
            "[ark:ts]"
        );
    }
}
