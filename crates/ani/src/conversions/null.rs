use super::either::Either;

// Option<T> <-> Either<T, Null>
impl<T> From<Option<T>> for Either<T, Null> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Either::A(val),
            None => Either::B(Null),
        }
    }
}

impl<T> From<Either<T, Null>> for Option<T> {
    fn from(either: Either<T, Null>) -> Self {
        match either {
            Either::A(val) => Some(val),
            Either::B(_) => None,
        }
    }
}

use super::either::{FromAniObject, ToAniObject, ValidateFromAni};
use super::traits::TypeInfo;
use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::*;

/// Unit type representing null in ANI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Null;

impl TypeInfo for Null {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ValidateFromAni<'env> for Null {
    fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
        let obj = unsafe { AniRef::from_raw(value) };
        env.is_null(&obj).unwrap_or(false)
    }
}

impl<'env> FromAniObject<'env> for Null {
    fn from_ani_object(_env: &Env<'env>, _value: sys::ani_object) -> Result<Self> {
        Ok(Null)
    }
}

impl<'env> ToAniObject<'env> for Null {
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object> {
        env.get_null_object()
    }
}
