//! Init lifecycle example for `#[ani(init)]`.
//!
//! This validates:
//! - `#[ani(init, before_bindings)]` callback registration
//! - `#[ani(init)]` callback registration (after bindings)
//! - `env: &Env<'_>` injection for init callback
//! - `ani::error::Result<()>` and `std::result::Result<(), ani::error::Error>` return handling for init callbacks

use ani::prelude::*;
use ani_derive::ani;
use std::sync::atomic::{AtomicU8, Ordering};

static BEFORE_BINDINGS_INIT: AtomicU8 = AtomicU8::new(0);
static AFTER_BINDINGS_INIT: AtomicU8 = AtomicU8::new(0);
static STD_RESULT_INIT: AtomicU8 = AtomicU8::new(0);

#[ani(init, before_bindings)]
fn init_before_bindings(env: &Env<'_>) -> Result<()> {
    let _ = env;
    BEFORE_BINDINGS_INIT.store(1, Ordering::SeqCst);
    Ok(())
}

#[ani(init)]
fn init_after_bindings() {
    AFTER_BINDINGS_INIT.store(1, Ordering::SeqCst);
}

#[ani(init)]
fn init_std_result() -> std::result::Result<(), ani::error::Error> {
    STD_RESULT_INIT.store(1, Ordering::SeqCst);
    Ok(())
}

#[ani]
pub fn init_state() -> i32 {
    let before = BEFORE_BINDINGS_INIT.load(Ordering::SeqCst) as i32;
    let after = AFTER_BINDINGS_INIT.load(Ordering::SeqCst) as i32;
    let std_result = STD_RESULT_INIT.load(Ordering::SeqCst) as i32;
    before * 100 + std_result * 10 + after
}

#[ani]
pub fn reset_init_state() {
    BEFORE_BINDINGS_INIT.store(0, Ordering::SeqCst);
    AFTER_BINDINGS_INIT.store(0, Ordering::SeqCst);
    STD_RESULT_INIT.store(0, Ordering::SeqCst);
}
