use super::either::Either;

// Option<T> <-> Either<T, Undefined>
impl<T> From<Option<T>> for Either<T, Undefined> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Either::A(val),
            None => Either::B(Undefined),
        }
    }
}

impl<T> From<Either<T, Undefined>> for Option<T> {
    fn from(either: Either<T, Undefined>) -> Self {
        match either {
            Either::A(val) => Some(val),
            Either::B(_) => None,
        }
    }
}

use super::either::{FromAniObject, ToAniObject, ValidateFromAni};
use super::traits::{FromAni, ToAni, TypeInfo};
use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::*;

/// Unit type representing undefined in ANI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Undefined;

impl TypeInfo for Undefined {
    fn type_signature() -> &'static str {
        "U"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ValidateFromAni<'env> for Undefined {
    fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
        if value.is_null() {
            return false;
        }
        let obj = unsafe { AniRef::from_raw(value) };
        env.is_undefined(&obj).unwrap_or(false)
    }
}

impl<'env> FromAniObject<'env> for Undefined {
    fn from_ani_object(_env: &Env<'env>, _value: sys::ani_object) -> Result<Self> {
        Ok(Undefined)
    }
}

impl<'env> ToAniObject<'env> for Undefined {
    fn to_ani_object(self, env: &Env<'env>) -> Result<sys::ani_object> {
        env.get_undefined_object()
    }
}

impl<'env> FromAni<'env> for Undefined {
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                Status::InvalidArgs,
                "Expected undefined, got null",
            ));
        }
        if Self::validate(env, value) {
            Ok(Undefined)
        } else {
            Err(Error::new(Status::InvalidType, "Expected undefined"))
        }
    }
}

impl<'env> ToAni<'env> for Undefined {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_ani_object(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undefined_type_info_matches_new_signature() {
        assert_eq!(<Undefined as TypeInfo>::type_signature(), "U");
        assert_eq!(<Undefined as TypeInfo>::ani_c_type(), "ani_object");
    }

    #[test]
    fn option_none_maps_to_either_undefined() {
        let either: Either<i32, Undefined> = Option::<i32>::None.into();
        assert!(matches!(either, Either::B(Undefined)));

        let option: Option<i32> = Either::<i32, Undefined>::B(Undefined).into();
        assert_eq!(option, None);
    }
}
