//! ETS declaration generation example.
//!
//! Build with:
//! `ANI_ETS_OUTPUT=ets/index.ets cargo build -p ani-example-ets-declaration`
//! or without env var to emit a default file in `target/ani-ets/<pkg>.ets`.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(namespace = "AniMath.Utils")]
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[ani(class = "example.Person")]
pub fn get_name(_env: &Env<'_>, this: &AniObject<'_>) -> String {
    let _ = this;
    "ani-rs".to_string()
}

#[ani(class = "example.Person", static)]
pub fn create(_env: &Env<'_>, name: String) -> i64 {
    let _ = name;
    0
}
