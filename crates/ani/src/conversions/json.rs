//! Serde bridge backed by native ArkTS values (`Record`, `Array`, and boxed primitives).

use std::collections::HashMap;
use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniObject, AniRef, AniString};

use super::{Boxable, FromAni, ToAni, TypeInfo, Unboxable};

/// A strongly typed serde value represented by native ArkTS structured values.
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
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T: Serialize> ToAni<'env> for Json<T> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let value = serde_json::to_value(&self.value).map_err(|error| {
            Error::new(
                Status::InvalidArgs,
                format!("failed to serialize structured value: {error}"),
            )
        })?;
        json_to_ref(env, value).map(|value| value.into_raw() as sys::ani_object)
    }
}

impl<'env, T: DeserializeOwned> FromAni<'env> for Json<T> {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let value = unsafe { json_from_ref(env, AniRef::from_raw(value as sys::ani_ref)) }?;
        serde_json::from_value(value)
            .map(Self::new)
            .map_err(|error| {
                Error::new(
                    Status::InvalidArgs,
                    format!("failed to deserialize structured value: {error}"),
                )
            })
    }
}

impl TypeInfo for serde_json::Value {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ToAni<'env> for serde_json::Value {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        json_to_ref(env, self).map(|value| value.into_raw() as sys::ani_object)
    }
}

impl<'env> FromAni<'env> for serde_json::Value {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe { json_from_ref(env, AniRef::from_raw(value as sys::ani_ref)) }
    }
}

fn object_ref<'env>(object: AniObject<'env>) -> AniRef<'env> {
    unsafe { AniRef::from_raw(object.into_raw() as sys::ani_ref) }
}

fn json_to_ref<'env>(env: &Env<'env>, value: serde_json::Value) -> Result<AniRef<'env>> {
    match value {
        serde_json::Value::Null => {
            Ok(unsafe { AniRef::from_raw(env.get_null_object()? as sys::ani_ref) })
        }
        serde_json::Value::Bool(value) => value.box_value(env).map(object_ref),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                if let Ok(value) = i32::try_from(value) {
                    value.box_value(env).map(object_ref)
                } else {
                    value.box_value(env).map(object_ref)
                }
            } else if let Some(value) = value.as_u64() {
                let value = i64::try_from(value).map_err(|_| {
                    Error::new(
                        Status::OutOfRange,
                        "JSON unsigned integer exceeds ArkTS long range",
                    )
                })?;
                value.box_value(env).map(object_ref)
            } else {
                value
                    .as_f64()
                    .ok_or_else(|| Error::new(Status::InvalidType, "invalid JSON number"))?
                    .box_value(env)
                    .map(object_ref)
            }
        }
        serde_json::Value::String(value) => env
            .create_string(&value)
            .map(|value| unsafe { AniRef::from_raw(value.into_raw() as sys::ani_ref) }),
        serde_json::Value::Array(values) => {
            let values = values
                .into_iter()
                .map(|value| json_to_ref(env, value))
                .collect::<Result<Vec<_>>>()?;
            values
                .to_ani(env)
                .map(|value| unsafe { AniRef::from_raw(value as sys::ani_ref) })
        }
        serde_json::Value::Object(values) => {
            let values = values
                .into_iter()
                .map(|(key, value)| json_to_ref(env, value).map(|value| (key, value)))
                .collect::<Result<HashMap<_, _>>>()?;
            values.to_ani(env).map(object_ref)
        }
    }
}

fn instance_of(env: &Env<'_>, object: &AniObject<'_>, class: &str) -> Result<bool> {
    env.find_class(class)
        .and_then(|class| env.object_instance_of(object, &class))
}

unsafe fn json_from_ref<'env>(env: &Env<'env>, value: AniRef<'env>) -> Result<serde_json::Value> {
    if env.is_nullish(&value)? {
        return Ok(serde_json::Value::Null);
    }
    let object = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
    if instance_of(env, &object, "std.core.String")? {
        let value = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        return env.get_string(&value).map(serde_json::Value::String);
    }
    if instance_of(env, &object, "std.core.Boolean")? {
        return bool::unbox(env, &object).map(serde_json::Value::Bool);
    }
    if instance_of(env, &object, "std.core.Int")? {
        return i32::unbox(env, &object).map(|value| serde_json::Value::from(value as i64));
    }
    if instance_of(env, &object, "std.core.Long")? {
        return i64::unbox(env, &object).map(serde_json::Value::from);
    }
    if instance_of(env, &object, "std.core.Double")? {
        let value = f64::unbox(env, &object)?;
        return serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .ok_or_else(|| Error::new(Status::InvalidType, "non-finite ArkTS number"));
    }
    if instance_of(env, &object, "std.core.Array")? {
        let values =
            unsafe { Vec::<AniRef<'env>>::from_ani(env, value.as_raw() as sys::ani_array) }?;
        let values = values
            .into_iter()
            .map(|value| unsafe { json_from_ref(env, value) })
            .collect::<Result<Vec<_>>>()?;
        return Ok(serde_json::Value::Array(values));
    }
    if instance_of(env, &object, "std.core.Record")? {
        let values = unsafe { HashMap::<String, AniRef<'env>>::from_ani(env, object.as_raw()) }?;
        let values = values
            .into_iter()
            .map(|(key, value)| unsafe { json_from_ref(env, value) }.map(|value| (key, value)))
            .collect::<Result<serde_json::Map<_, _>>>()?;
        return Ok(serde_json::Value::Object(values));
    }
    Err(Error::new(
        Status::InvalidType,
        "structured value must be null, String, boxed number/boolean, Array, or Record",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_bridge_declares_object_abi() {
        assert_eq!(
            Json::<serde_json::Value>::type_signature(),
            "Lstd/core/Object;"
        );
        assert_eq!(serde_json::Value::ani_c_type(), "ani_object");
    }
}
