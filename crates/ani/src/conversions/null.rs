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
use super::traits::{FromAni, ToAni, TypeInfo};
use crate::env::Env;
use crate::error::Error;
use crate::error::{Result, Status};
use crate::sys;
use crate::types::*;

/// Unit type representing null in ANI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Null;

impl TypeInfo for Null {
    fn type_signature() -> &'static str {
        "C{std.core.Null}"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ValidateFromAni<'env> for Null {
    fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
        if value.is_null() {
            return true;
        }
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

impl<'env> FromAni<'env> for Null {
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if Self::validate(env, value) {
            Ok(Null)
        } else {
            Err(Error::new(Status::InvalidType, "Expected null"))
        }
    }
}

impl<'env> ToAni<'env> for Null {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_ani_object(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_type_info_matches_new_signature() {
        assert_eq!(<Null as TypeInfo>::type_signature(), "C{std.core.Null}");
        assert_eq!(<Null as TypeInfo>::ani_c_type(), "ani_object");
    }

    #[test]
    fn option_none_maps_to_either_null() {
        let either: Either<i32, Null> = Option::<i32>::None.into();
        assert!(matches!(either, Either::B(Null)));

        let option: Option<i32> = Either::<i32, Null>::B(Null).into();
        assert_eq!(option, None);
    }
}
