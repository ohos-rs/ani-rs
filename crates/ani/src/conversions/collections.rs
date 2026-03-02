//! Collection Type Conversion
//!
//! Implements conversion between Rust collection types and ANI types
//! - HashMap<K, V> <-> Record<K, V>
//! - HashSet<T> <-> Set<T>
//! - BTreeMap<K, V> <-> Map<K, V>
//! - Tuple types

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;
use crate::types::*;

use super::boxed::{Boxable, Unboxable};
use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// HashMap<String, V> - Record<String, V>
// ============================================================================

impl<V: TypeInfo> TypeInfo for HashMap<String, V> {
    fn type_signature() -> &'static str {
        "Lescompat/Record;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

/// Value conversion for `Record<string, V>` mapping.
pub trait RecordValue<'env>: Sized {
    /// Convert Rust value into ANI ref used by Record values.
    fn to_record_ref(self, env: &Env<'env>) -> Result<AniRef<'env>>;

    /// Convert ANI ref value from Record into Rust value.
    fn from_record_ref(env: &Env<'env>, value: &AniRef<'env>) -> Result<Self>;
}

impl<'env> RecordValue<'env> for String {
    fn to_record_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = env.create_string(&self)?;
        Ok(unsafe { AniRef::from_raw(value.into_raw() as sys::ani_ref) })
    }

    fn from_record_ref(env: &Env<'env>, value: &AniRef<'env>) -> Result<Self> {
        let ani_string = unsafe { AniString::from_raw(value.as_raw() as sys::ani_string) };
        env.get_string(&ani_string)
    }
}

macro_rules! impl_record_value_boxed {
    ($ty:ty) => {
        impl<'env> RecordValue<'env> for $ty {
            fn to_record_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
                let boxed = <$ty as Boxable<'env>>::box_value(self, env)?;
                Ok(unsafe { AniRef::from_raw(boxed.into_raw() as sys::ani_ref) })
            }

            fn from_record_ref(env: &Env<'env>, value: &AniRef<'env>) -> Result<Self> {
                let obj = unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) };
                <$ty as Unboxable<'env>>::unbox(env, &obj)
            }
        }
    };
}

impl_record_value_boxed!(bool);
impl_record_value_boxed!(i8);
impl_record_value_boxed!(i16);
impl_record_value_boxed!(u16);
impl_record_value_boxed!(i32);
impl_record_value_boxed!(i64);
impl_record_value_boxed!(f32);
impl_record_value_boxed!(f64);

impl<'env> RecordValue<'env> for AniRef<'env> {
    fn to_record_ref(self, _env: &Env<'env>) -> Result<AniRef<'env>> {
        Ok(self)
    }

    fn from_record_ref(_env: &Env<'env>, value: &AniRef<'env>) -> Result<Self> {
        Ok(unsafe { AniRef::from_raw(value.as_raw()) })
    }
}

impl<'env> RecordValue<'env> for AniObject<'env> {
    fn to_record_ref(self, _env: &Env<'env>) -> Result<AniRef<'env>> {
        Ok(unsafe { AniRef::from_raw(self.into_raw() as sys::ani_ref) })
    }

    fn from_record_ref(_env: &Env<'env>, value: &AniRef<'env>) -> Result<Self> {
        Ok(unsafe { AniObject::from_raw(value.as_raw() as sys::ani_object) })
    }
}

impl<'env, V> ToAni<'env> for HashMap<String, V>
where
    V: RecordValue<'env>,
{
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let record_class = env.find_class("Lescompat/Record;")?;
        let ctor = env.find_constructor(&record_class, ":V")?;
        let record = env.new_object(&record_class, &ctor, &[])?;
        let record_ref = unsafe { AniRef::from_raw(record.as_raw() as sys::ani_ref) };

        for (key, value) in self {
            let ani_key = env.create_string(&key)?;
            let key_ref = unsafe { AniRef::from_raw(ani_key.as_raw() as sys::ani_ref) };
            let value_ref = value.to_record_ref(env)?;
            env.any_set_by_value(&record_ref, &key_ref, &value_ref)?;
        }

        Ok(record)
    }
}

impl<'env, V> FromAni<'env> for HashMap<String, V>
where
    V: RecordValue<'env>,
{
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                "Null pointer: record",
            ));
        }

        let record_ref = unsafe { AniRef::from_raw(value as sys::ani_ref) };
        let keys_ref = env.any_call_method(&record_ref, "keys", &[])?;

        // Record.keys() should return an array of keys.
        let keys_array = unsafe { AniArray::from_raw(keys_ref.as_raw() as sys::ani_array) };
        let len = env.get_array_length(&keys_array)?;
        let mut out = HashMap::with_capacity(len);

        for i in 0..len {
            let key_ref = env.get_array_element(&keys_array, i)?;
            let key = String::from_record_ref(env, &key_ref)?;
            let value_ref = env.any_get_by_value(&record_ref, &key_ref)?;
            let value = V::from_record_ref(env, &value_ref)?;
            out.insert(key, value);
        }

        Ok(out)
    }
}

// ============================================================================
// HashSet<T> - Set<T>
// ============================================================================

impl<T: TypeInfo> TypeInfo for HashSet<T> {
    fn type_signature() -> &'static str {
        "Lescompat/Set;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ToAni<'env> for HashSet<String> {
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let set_class = env.find_class("Lescompat/Set;")?;
        let ctor = env.find_constructor(&set_class, ":V")?;
        let set = env.new_object(&set_class, &ctor, &[])?;

        let add_method = env.find_method(&set_class, "add", "Lstd/core/Object;:V")?;

        for item in self {
            let ani_item = env.create_string(&item)?;
            let args = [ani_value_ref(ani_item.as_raw() as sys::ani_ref)];
            env.call_method_void(&set, &add_method, &args)?;
        }

        Ok(set)
    }
}

// ============================================================================
// BTreeMap<K, V> - Map<K, V>
// ============================================================================

impl<K: TypeInfo, V: TypeInfo> TypeInfo for BTreeMap<K, V> {
    fn type_signature() -> &'static str {
        "Lescompat/Map;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

// ============================================================================
// Tuple Types
// ============================================================================

impl TypeInfo for (i32, i32) {
    fn type_signature() -> &'static str {
        "Lescompat/Tuple2;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl TypeInfo for (i32, i32, i32) {
    fn type_signature() -> &'static str {
        "Lescompat/Tuple3;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl TypeInfo for (String, String) {
    fn type_signature() -> &'static str {
        "Lescompat/Tuple2;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

// Tuple to array conversion
impl<'env> ToAni<'env> for (i32, i32) {
    type Output = sys::ani_array_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        vec![self.0, self.1].to_ani(env)
    }
}

impl<'env> FromAni<'env> for (i32, i32) {
    type Input = sys::ani_array_int;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let vec: Vec<i32> = Vec::from_ani(env, value)?;
        if vec.len() != 2 {
            return Err(Error::new(
                crate::error::Status::InvalidType,
                "Expected tuple of 2 elements",
            ));
        }
        Ok((vec[0], vec[1]))
    }
}

impl<'env> ToAni<'env> for (i32, i32, i32) {
    type Output = sys::ani_array_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        vec![self.0, self.1, self.2].to_ani(env)
    }
}

impl<'env> ToAni<'env> for (f64, f64) {
    type Output = sys::ani_array_double;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        vec![self.0, self.1].to_ani(env)
    }
}

// ============================================================================
// JSON Value Type (for dynamic data)
// ============================================================================

/// Dynamic ANI value, similar to JSON
#[derive(Debug, Clone)]
pub enum AniValue {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// Integer value (32-bit)
    Int(i32),
    /// Long integer value (64-bit)
    Long(i64),
    /// Double precision floating point value
    Double(f64),
    /// String value
    String(String),
    /// Array of ANI values
    Array(Vec<AniValue>),
    /// Object with string keys and ANI values
    Object(HashMap<String, AniValue>),
}

impl TypeInfo for AniValue {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl AniValue {
    /// Create from bool
    pub fn from_bool(v: bool) -> Self {
        AniValue::Bool(v)
    }

    /// Create from i32
    pub fn from_int(v: i32) -> Self {
        AniValue::Int(v)
    }

    /// Create from i64
    pub fn from_long(v: i64) -> Self {
        AniValue::Long(v)
    }

    /// Create from f64
    pub fn from_double(v: f64) -> Self {
        AniValue::Double(v)
    }

    /// Create from String
    pub fn from_string(v: String) -> Self {
        AniValue::String(v)
    }

    /// Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, AniValue::Null)
    }

    /// Try to get bool value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AniValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Try to get i32 value
    pub fn as_int(&self) -> Option<i32> {
        match self {
            AniValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Try to get i64 value
    pub fn as_long(&self) -> Option<i64> {
        match self {
            AniValue::Long(v) => Some(*v),
            AniValue::Int(v) => Some(*v as i64),
            _ => None,
        }
    }

    /// Try to get f64 value
    pub fn as_double(&self) -> Option<f64> {
        match self {
            AniValue::Double(v) => Some(*v),
            AniValue::Int(v) => Some(*v as f64),
            AniValue::Long(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Try to get string value
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AniValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Try to get array
    pub fn as_array(&self) -> Option<&Vec<AniValue>> {
        match self {
            AniValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Try to get object
    pub fn as_object(&self) -> Option<&HashMap<String, AniValue>> {
        match self {
            AniValue::Object(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_type_signature() {
        assert_eq!(
            <HashMap<String, String>>::type_signature(),
            "Lescompat/Record;"
        );
    }

    #[test]
    fn test_hashset_type_signature() {
        assert_eq!(<HashSet<String>>::type_signature(), "Lescompat/Set;");
    }

    #[test]
    fn test_ani_value() {
        let val = AniValue::from_int(42);
        assert_eq!(val.as_int(), Some(42));
        assert_eq!(val.as_long(), Some(42));
        assert!(val.as_str().is_none());

        let null = AniValue::Null;
        assert!(null.is_null());
    }
}
