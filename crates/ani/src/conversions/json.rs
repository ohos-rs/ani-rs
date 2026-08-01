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

/// Runtime schema for one externally serialized Rust enum variant.
#[derive(Clone, Copy, Debug)]
pub struct StructuredEnumVariant {
    /// Variant name used by serde's external representation.
    pub rust_name: &'static str,
    /// Public discriminator value used by ArkTS.
    pub arkts_name: &'static str,
    /// Expected payload shape.
    pub shape: StructuredEnumShape,
    /// Whether ArkTS may pass this variant into Rust.
    pub input: bool,
    /// Whether Rust may return this variant to ArkTS.
    pub output: bool,
}

/// Exact runtime shape of a structured enum variant.
#[derive(Clone, Copy, Debug)]
pub enum StructuredEnumShape {
    /// No payload.
    Unit,
    /// One unnamed payload exposed under `value`.
    Newtype(StructuredEnumValueKind),
    /// Multiple unnamed payloads exposed as a tuple under `value`.
    Tuple(&'static [StructuredEnumValueKind]),
    /// Named payload fields flattened beside the discriminator.
    Struct(&'static [StructuredEnumField]),
}

/// ANI storage kind used to read a statically typed ArkTS interface field.
///
/// `Record` values box their entries, while generated variant interfaces keep
/// primitive fields unboxed. The schema therefore records the field ABI so
/// both representations can share validation without JSON stringification.
#[derive(Clone, Copy, Debug)]
pub enum StructuredEnumValueKind {
    /// ArkTS `boolean`.
    Boolean,
    /// ArkTS `byte`.
    Byte,
    /// ArkTS `char` (used by Rust `u16`).
    Char,
    /// ArkTS `short`.
    Short,
    /// ArkTS `int`.
    Int,
    /// ArkTS `long`.
    Long,
    /// ArkTS `float`.
    Float,
    /// ArkTS `double`.
    Double,
    /// Reference, including strings, containers, bigint and erased generics.
    Ref,
}

/// Directional field schema for a struct enum variant.
#[derive(Clone, Copy, Debug)]
pub struct StructuredEnumField {
    /// Field name in serde's representation.
    pub rust_name: &'static str,
    /// Field name in the ArkTS union.
    pub arkts_name: &'static str,
    /// ANI storage kind for statically typed interface objects.
    pub kind: StructuredEnumValueKind,
    /// Whether ArkTS may provide the field.
    pub input: bool,
    /// Whether Rust emits the field.
    pub output: bool,
}

fn schema_error(message: impl Into<String>) -> Error {
    Error::new(Status::InvalidType, message)
}

/// Convert serde's external enum representation into a checked ArkTS
/// discriminated-union object.
pub fn encode_structured_enum(
    value: serde_json::Value,
    discriminator: &str,
    variants: &[StructuredEnumVariant],
) -> Result<serde_json::Value> {
    let (rust_name, payload) = match value {
        serde_json::Value::String(name) => (name, None),
        serde_json::Value::Object(object) if object.len() == 1 => {
            let (name, value) = object.into_iter().next().expect("one-entry object checked");
            (name, Some(value))
        }
        _ => {
            return Err(schema_error(
                "structured enum serializer must use serde's external representation",
            ));
        }
    };
    let variant = variants
        .iter()
        .find(|variant| variant.rust_name == rust_name)
        .ok_or_else(|| schema_error(format!("unknown Rust enum variant `{rust_name}`")))?;
    if !variant.output {
        return Err(schema_error(format!(
            "enum variant `{}` is input-only",
            variant.arkts_name
        )));
    }

    let mut output = serde_json::Map::new();
    output.insert(
        discriminator.to_string(),
        serde_json::Value::String(variant.arkts_name.to_string()),
    );
    match (variant.shape, payload) {
        (StructuredEnumShape::Unit, None) => {}
        (StructuredEnumShape::Newtype(_), Some(value)) => {
            output.insert("value".to_string(), value);
        }
        (StructuredEnumShape::Tuple(kinds), Some(serde_json::Value::Array(values)))
            if values.len() == kinds.len() =>
        {
            output.insert("value".to_string(), serde_json::Value::Array(values));
        }
        (StructuredEnumShape::Struct(fields), Some(serde_json::Value::Object(mut values))) => {
            for field in fields {
                if field.output && !values.contains_key(field.rust_name) {
                    return Err(schema_error(format!(
                        "enum variant `{}` is missing field `{}`",
                        variant.arkts_name, field.rust_name
                    )));
                }
            }
            for field in fields {
                if let Some(value) = values.remove(field.rust_name)
                    && field.output
                {
                    output.insert(field.arkts_name.to_string(), value);
                }
            }
            if !values.is_empty() {
                return Err(schema_error(format!(
                    "enum variant `{}` serializer produced unknown fields",
                    variant.arkts_name
                )));
            }
        }
        _ => {
            return Err(schema_error(format!(
                "enum variant `{}` payload does not match its schema",
                variant.arkts_name
            )));
        }
    }
    Ok(serde_json::Value::Object(output))
}

/// Validate and convert an ArkTS discriminated-union object back to serde's
/// external enum representation.
pub fn decode_structured_enum(
    value: serde_json::Value,
    discriminator: &str,
    variants: &[StructuredEnumVariant],
) -> Result<serde_json::Value> {
    let serde_json::Value::Object(mut object) = value else {
        return Err(schema_error("structured enum input must be an object"));
    };
    let tag = object
        .remove(discriminator)
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or_else(|| {
            schema_error(format!(
                "structured enum input requires string discriminator `{discriminator}`"
            ))
        })?;
    let variant = variants
        .iter()
        .find(|variant| variant.arkts_name == tag)
        .ok_or_else(|| schema_error(format!("unknown enum discriminator `{tag}`")))?;
    if !variant.input {
        return Err(schema_error(format!(
            "enum variant `{}` is output-only",
            variant.arkts_name
        )));
    }

    match variant.shape {
        StructuredEnumShape::Unit => {
            if !object.is_empty() {
                return Err(schema_error(format!(
                    "unit variant `{tag}` does not accept payload fields"
                )));
            }
            Ok(serde_json::Value::String(variant.rust_name.to_string()))
        }
        StructuredEnumShape::Newtype(_) => {
            let value = object
                .remove("value")
                .ok_or_else(|| schema_error(format!("variant `{tag}` requires `value`")))?;
            if !object.is_empty() {
                return Err(schema_error(format!(
                    "variant `{tag}` contains unknown payload fields"
                )));
            }
            Ok(serde_json::json!({ variant.rust_name: value }))
        }
        StructuredEnumShape::Tuple(kinds) => {
            let value = object
                .remove("value")
                .ok_or_else(|| schema_error(format!("variant `{tag}` requires `value`")))?;
            let serde_json::Value::Array(values) = value else {
                return Err(schema_error(format!(
                    "variant `{tag}` value must be a tuple"
                )));
            };
            if values.len() != kinds.len() || !object.is_empty() {
                return Err(schema_error(format!(
                    "variant `{tag}` tuple does not match its schema"
                )));
            }
            Ok(serde_json::json!({ variant.rust_name: values }))
        }
        StructuredEnumShape::Struct(fields) => {
            for field in fields {
                if field.input && !object.contains_key(field.arkts_name) {
                    return Err(schema_error(format!(
                        "variant `{tag}` is missing field `{}`",
                        field.arkts_name
                    )));
                }
            }
            let mut translated = serde_json::Map::new();
            for (name, value) in object {
                let field = fields
                    .iter()
                    .find(|field| field.arkts_name == name)
                    .ok_or_else(|| {
                        schema_error(format!("variant `{tag}` contains unknown field `{name}`"))
                    })?;
                if !field.input {
                    return Err(schema_error(format!(
                        "variant `{tag}` field `{name}` is output-only"
                    )));
                }
                translated.insert(field.rust_name.to_string(), value);
            }
            Ok(serde_json::json!({ variant.rust_name: translated }))
        }
    }
}

/// Read a structured enum from either an ArkTS `Record` or a statically
/// declared variant interface object.
///
/// ArkTS 1.2 rejects inline object types, so generated declarations use one
/// explicit interface per variant. Interface object literals are not
/// necessarily `Record` instances; this path reads the schema's exact public
/// properties and then applies the same direction and serde validation.
///
/// # Safety
///
/// `value` must be a valid local ANI object reference for `env` and remain
/// live for the duration of this call.
pub unsafe fn decode_structured_enum_from_ani(
    env: &Env<'_>,
    value: sys::ani_object,
    discriminator: &str,
    variants: &[StructuredEnumVariant],
) -> Result<serde_json::Value> {
    let reference = unsafe { AniRef::from_raw(value as sys::ani_ref) };
    if let Ok(value) = unsafe { json_from_ref(env, AniRef::from_raw(reference.as_raw())) } {
        return decode_structured_enum(value, discriminator, variants);
    }

    let object = unsafe { AniObject::from_raw(value) };
    let tag = env
        .get_property_by_name_ref(&object, discriminator)
        .or_else(|_| env.get_field_by_name_ref(&object, discriminator))?;
    let tag = env.get_string(&unsafe { AniString::from_raw(tag.as_raw() as sys::ani_string) })?;
    let variant = variants
        .iter()
        .find(|variant| variant.arkts_name == tag)
        .ok_or_else(|| schema_error(format!("unknown enum discriminator `{tag}`")))?;
    if !variant.input {
        return Err(schema_error(format!(
            "enum variant `{}` is output-only",
            variant.arkts_name
        )));
    }

    let mut structured = serde_json::Map::new();
    structured.insert(discriminator.to_string(), serde_json::Value::String(tag));
    match variant.shape {
        StructuredEnumShape::Unit => {}
        StructuredEnumShape::Newtype(kind) => {
            structured.insert(
                "value".to_string(),
                read_structured_field(env, &object, "value", kind)?,
            );
        }
        StructuredEnumShape::Tuple(_) => {
            // A tuple field is itself an ArkTS array reference even when its
            // individual elements are primitive.
            structured.insert(
                "value".to_string(),
                read_structured_field(env, &object, "value", StructuredEnumValueKind::Ref)?,
            );
        }
        StructuredEnumShape::Struct(fields) => {
            for field in fields.iter().filter(|field| field.input) {
                structured.insert(
                    field.arkts_name.to_string(),
                    read_structured_field(env, &object, field.arkts_name, field.kind)?,
                );
            }
        }
    }
    decode_structured_enum(
        serde_json::Value::Object(structured),
        discriminator,
        variants,
    )
}

fn read_structured_field(
    env: &Env<'_>,
    object: &AniObject<'_>,
    name: &str,
    kind: StructuredEnumValueKind,
) -> Result<serde_json::Value> {
    let finite_number = |value: f64| {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .ok_or_else(|| schema_error(format!("field `{name}` contains a non-finite number")))
    };
    match kind {
        StructuredEnumValueKind::Boolean => env
            .get_property_by_name_boolean(object, name)
            .or_else(|_| env.get_field_by_name_boolean(object, name))
            .map(serde_json::Value::Bool),
        StructuredEnumValueKind::Byte => env
            .get_property_by_name_byte(object, name)
            .or_else(|_| env.get_field_by_name_byte(object, name))
            .map(|value| serde_json::Value::from(i64::from(value))),
        StructuredEnumValueKind::Char => env
            .get_property_by_name_char(object, name)
            .or_else(|_| env.get_field_by_name_char(object, name))
            .map(|value| serde_json::Value::from(u64::from(value))),
        StructuredEnumValueKind::Short => env
            .get_property_by_name_short(object, name)
            .or_else(|_| env.get_field_by_name_short(object, name))
            .map(|value| serde_json::Value::from(i64::from(value))),
        StructuredEnumValueKind::Int => env
            .get_property_by_name_int(object, name)
            .or_else(|_| env.get_field_by_name_int(object, name))
            .map(|value| serde_json::Value::from(i64::from(value))),
        StructuredEnumValueKind::Long => env
            .get_property_by_name_long(object, name)
            .or_else(|_| env.get_field_by_name_long(object, name))
            .map(serde_json::Value::from),
        StructuredEnumValueKind::Float => env
            .get_property_by_name_float(object, name)
            .or_else(|_| env.get_field_by_name_float(object, name))
            .and_then(|value| finite_number(f64::from(value))),
        StructuredEnumValueKind::Double => env
            .get_property_by_name_double(object, name)
            .or_else(|_| env.get_field_by_name_double(object, name))
            .and_then(finite_number),
        StructuredEnumValueKind::Ref => {
            let value = env
                .get_property_by_name_ref(object, name)
                .or_else(|_| env.get_field_by_name_ref(object, name))?;
            unsafe { json_from_ref(env, value) }
        }
    }
}

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

    const FIELDS: &[StructuredEnumField] = &[
        StructuredEnumField {
            rust_name: "payload",
            arkts_name: "payloadText",
            kind: StructuredEnumValueKind::Ref,
            input: true,
            output: true,
        },
        StructuredEnumField {
            rust_name: "local_only",
            arkts_name: "localOnly",
            kind: StructuredEnumValueKind::Boolean,
            input: false,
            output: false,
        },
    ];
    const VARIANTS: &[StructuredEnumVariant] = &[
        StructuredEnumVariant {
            rust_name: "RenamedField",
            arkts_name: "renamedField",
            shape: StructuredEnumShape::Struct(FIELDS),
            input: true,
            output: true,
        },
        StructuredEnumVariant {
            rust_name: "Generated",
            arkts_name: "generated",
            shape: StructuredEnumShape::Newtype(StructuredEnumValueKind::Int),
            input: false,
            output: true,
        },
    ];

    #[test]
    fn native_bridge_declares_object_abi() {
        assert_eq!(
            Json::<serde_json::Value>::type_signature(),
            "Lstd/core/Object;"
        );
        assert_eq!(serde_json::Value::ani_c_type(), "ani_object");
    }

    #[test]
    fn structured_enum_translates_discriminator_and_field_names() {
        let encoded = encode_structured_enum(
            serde_json::json!({
                "RenamedField": {"payload": "ani", "local_only": true}
            }),
            "kind",
            VARIANTS,
        )
        .unwrap();
        assert_eq!(
            encoded,
            serde_json::json!({"kind": "renamedField", "payloadText": "ani"})
        );

        let decoded = decode_structured_enum(encoded, "kind", VARIANTS).unwrap();
        assert_eq!(
            decoded,
            serde_json::json!({"RenamedField": {"payload": "ani"}})
        );
    }

    #[test]
    fn structured_enum_rejects_direction_and_schema_violations() {
        assert_eq!(
            decode_structured_enum(
                serde_json::json!({"kind": "generated", "value": 1}),
                "kind",
                VARIANTS,
            )
            .unwrap_err()
            .status,
            Status::InvalidType
        );
        assert_eq!(
            decode_structured_enum(
                serde_json::json!({"kind": "renamedField", "unknown": true}),
                "kind",
                VARIANTS,
            )
            .unwrap_err()
            .status,
            Status::InvalidType
        );
    }
}
