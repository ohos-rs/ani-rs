//! Null and Undefined example.
//!
//! The current ArkVM ANI runtime can bind standalone `null` / `undefined`
//! exports, but module-level union exports involving `undefined` still fail
//! module precheck in the OpenHarmony runtime we validate against.

use ani::conversions::{Null, Undefined};
use ani_derive::ani;

#[ani]
pub fn accept_undefined(_value: Undefined) -> bool {
    true
}

#[ani]
pub fn accept_null(_value: Null) -> bool {
    true
}

#[ani]
pub fn make_undefined() -> Undefined {
    Undefined
}

#[ani]
pub fn make_null() -> Null {
    Null
}
