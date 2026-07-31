//! Optional serde JSON bridge transported as an ArkTS string.

use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::types::AniString;

use super::{FromAni, ToAni, TypeInfo};

/// A strongly typed serde value encoded as JSON at the ANI boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct Json<T> {
    /// Decoded Rust value.
    pub value: T,
    marker: PhantomData<T>,
}

impl<T> Json<T> {
    /// Wraps a serializable value.
    pub fn new(value: T) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }

    /// Consumes the wrapper.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> TypeInfo for Json<T> {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }

    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env, T: Serialize> ToAni<'env> for Json<T> {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let json = serde_json::to_string(&self.value).map_err(|error| {
            Error::new(
                Status::InvalidArgs,
                format!("failed to serialize JSON value: {error}"),
            )
        })?;
        env.create_string(&json)
    }
}

impl<'env, T: DeserializeOwned> FromAni<'env> for Json<T> {
    type Input = AniString<'env>;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let json = env.get_string(&value)?;
        serde_json::from_str(&json).map(Self::new).map_err(|error| {
            Error::new(
                Status::InvalidArgs,
                format!("failed to deserialize JSON value: {error}"),
            )
        })
    }
}

impl TypeInfo for serde_json::Value {
    fn type_signature() -> &'static str {
        "Lstd/core/String;"
    }

    fn ani_c_type() -> &'static str {
        "ani_string"
    }
}

impl<'env> ToAni<'env> for serde_json::Value {
    type Output = AniString<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        Json::new(self).to_ani(env)
    }
}

impl<'env> FromAni<'env> for serde_json::Value {
    type Input = AniString<'env>;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe { Json::<Self>::from_ani(env, value) }.map(Json::into_inner)
    }
}
