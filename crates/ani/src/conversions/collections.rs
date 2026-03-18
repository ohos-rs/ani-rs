//! Collection Type Conversion
//!
//! Implements conversion between Rust collection types and ANI types
//! - HashMap<K, V> <-> Record<K, V>
//! - HashSet<T> <-> Set<T>
//! - BTreeMap<K, V> <-> Map<K, V>
//! - Tuple types

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

use crate::env::Env;
use crate::error::{Error, Result, check_status};
use crate::sys;
use crate::types::*;

use super::boxed::{Boxable, Unboxable};
use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// HashMap<String, V> - Record<String, V>
// ============================================================================

impl<V: TypeInfo> TypeInfo for HashMap<String, V> {
    fn type_signature() -> &'static str {
        "Lstd/core/Record;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

/// Shared ref conversion used by Record/Map/Set container elements.
pub trait RecordValue<'env>: Sized {
    /// Convert Rust value into ANI ref used by ArkTS container entries.
    fn to_record_ref(self, env: &Env<'env>) -> Result<AniRef<'env>>;

    /// Convert ANI ref value from ArkTS container entries into Rust value.
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

fn find_record_indexable_getter<'env>(env: &Env<'env>, class: &AniClass<'_>) -> Result<AniMethod> {
    let raw = env.as_raw();
    let mut method: sys::ani_method = std::ptr::null_mut();
    let status = unsafe {
        let api = &*(*raw);
        (api.Class_FindIndexableGetter.unwrap())(raw, class.as_raw(), std::ptr::null(), &mut method)
    };
    check_status(status)?;
    Ok(unsafe { AniMethod::from_raw(method) })
}

fn find_record_indexable_setter<'env>(env: &Env<'env>, class: &AniClass<'_>) -> Result<AniMethod> {
    let raw = env.as_raw();
    let mut method: sys::ani_method = std::ptr::null_mut();
    let status = unsafe {
        let api = &*(*raw);
        (api.Class_FindIndexableSetter.unwrap())(raw, class.as_raw(), std::ptr::null(), &mut method)
    };
    check_status(status)?;
    Ok(unsafe { AniMethod::from_raw(method) })
}

fn find_method_no_signature<'env>(
    env: &Env<'env>,
    class: &AniClass<'_>,
    name: &str,
) -> Result<AniMethod> {
    let c_name = std::ffi::CString::new(name)
        .map_err(|_| Error::new(crate::error::Status::Error, "Invalid method name"))?;
    let raw = env.as_raw();
    let mut method: sys::ani_method = std::ptr::null_mut();
    let status = unsafe {
        let api = &*(*raw);
        (api.Class_FindMethod.unwrap())(
            raw,
            class.as_raw(),
            c_name.as_ptr(),
            std::ptr::null(),
            &mut method,
        )
    };
    check_status(status)?;
    Ok(unsafe { AniMethod::from_raw(method) })
}

impl<'env, V> ToAni<'env> for HashMap<String, V>
where
    V: RecordValue<'env>,
{
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let record_class = env.find_class("std.core.Record")?;
        let ctor = env.find_constructor(&record_class, ":")?;
        let setter = find_record_indexable_setter(env, &record_class)?;
        let record = env.new_object(&record_class, &ctor, &[])?;

        for (key, value) in self {
            let ani_key = env.create_string(&key)?;
            let key_ref = unsafe { AniRef::from_raw(ani_key.as_raw() as sys::ani_ref) };
            let value_ref = value.to_record_ref(env)?;
            let args = [
                ani_value_ref(key_ref.as_raw()),
                ani_value_ref(value_ref.as_raw()),
            ];
            env.call_method_void(&record, &setter, &args)?;
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

        let record = unsafe { AniObject::from_raw(value) };
        let record_class = env.find_class("std.core.Record")?;
        let getter = find_record_indexable_getter(env, &record_class)?;
        let keys_method = find_method_no_signature(env, &record_class, "keys")?;
        let keys_iter_ref = env.call_ref_method(&record, &keys_method, &[])?;
        let keys_iter = unsafe { AniObject::from_raw(keys_iter_ref.as_raw() as sys::ani_object) };
        let keys_iter_type = env.get_object_type(&keys_iter)?;
        let keys_iter_class =
            unsafe { AniClass::from_raw(keys_iter_type.as_raw() as sys::ani_class) };
        let next_method = find_method_no_signature(env, &keys_iter_class, "next")?;
        let mut out = HashMap::new();

        loop {
            let next_ref = env.call_ref_method(&keys_iter, &next_method, &[])?;
            let next = unsafe { AniObject::from_raw(next_ref.as_raw() as sys::ani_object) };
            if env.get_property_by_name_boolean(&next, "done")? {
                break;
            }

            let key_ref = env.get_property_by_name_ref(&next, "value")?;
            let key = String::from_record_ref(env, &key_ref)?;
            let args = [ani_value_ref(key_ref.as_raw())];
            let value_ref = env.call_ref_method(&record, &getter, &args)?;
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
        "Lstd/core/Set;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T> ToAni<'env> for HashSet<T>
where
    T: RecordValue<'env> + Eq + Hash,
{
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let set_class = env.find_class("std.core.Set")?;
        let ctor = env.find_constructor(&set_class, "i:")?;
        let ctor_args = [ani_value_int(0)];
        let set = env.new_object(&set_class, &ctor, &ctor_args)?;

        let add_method = find_method_no_signature(env, &set_class, "add")?;

        for item in self {
            let item_ref = item.to_record_ref(env)?;
            let args = [ani_value_ref(item_ref.as_raw())];
            let _ = env.call_ref_method(&set, &add_method, &args)?;
        }

        Ok(set)
    }
}

impl<'env, T> FromAni<'env> for HashSet<T>
where
    T: RecordValue<'env> + Eq + Hash,
{
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                "Null pointer: set",
            ));
        }

        let set = unsafe { AniObject::from_raw(value) };
        let set_class = env.find_class("std.core.Set")?;
        let values_method = find_method_no_signature(env, &set_class, "values")?;
        let values_iter_ref = env.call_ref_method(&set, &values_method, &[])?;
        let values_iter =
            unsafe { AniObject::from_raw(values_iter_ref.as_raw() as sys::ani_object) };
        let values_iter_type = env.get_object_type(&values_iter)?;
        let values_iter_class =
            unsafe { AniClass::from_raw(values_iter_type.as_raw() as sys::ani_class) };
        let next_method = find_method_no_signature(env, &values_iter_class, "next")?;
        let mut out = HashSet::new();

        loop {
            let next_ref = env.call_ref_method(&values_iter, &next_method, &[])?;
            let next = unsafe { AniObject::from_raw(next_ref.as_raw() as sys::ani_object) };
            if env.get_property_by_name_boolean(&next, "done")? {
                break;
            }

            let value_ref = env.get_property_by_name_ref(&next, "value")?;
            let item = T::from_record_ref(env, &value_ref)?;
            out.insert(item);
        }

        Ok(out)
    }
}

// ============================================================================
impl<T: TypeInfo> TypeInfo for BTreeSet<T> {
    fn type_signature() -> &'static str {
        "Lstd/core/Set;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T> ToAni<'env> for BTreeSet<T>
where
    T: RecordValue<'env> + Ord,
{
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let set_class = env.find_class("std.core.Set")?;
        let ctor = env.find_constructor(&set_class, "i:")?;
        let ctor_args = [ani_value_int(0)];
        let set = env.new_object(&set_class, &ctor, &ctor_args)?;

        let add_method = find_method_no_signature(env, &set_class, "add")?;

        for item in self {
            let item_ref = item.to_record_ref(env)?;
            let args = [ani_value_ref(item_ref.as_raw())];
            let _ = env.call_ref_method(&set, &add_method, &args)?;
        }

        Ok(set)
    }
}

impl<'env, T> FromAni<'env> for BTreeSet<T>
where
    T: RecordValue<'env> + Ord,
{
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                "Null pointer: set",
            ));
        }

        let set = unsafe { AniObject::from_raw(value) };
        let set_class = env.find_class("std.core.Set")?;
        let values_method = find_method_no_signature(env, &set_class, "values")?;
        let values_iter_ref = env.call_ref_method(&set, &values_method, &[])?;
        let values_iter =
            unsafe { AniObject::from_raw(values_iter_ref.as_raw() as sys::ani_object) };
        let values_iter_type = env.get_object_type(&values_iter)?;
        let values_iter_class =
            unsafe { AniClass::from_raw(values_iter_type.as_raw() as sys::ani_class) };
        let next_method = find_method_no_signature(env, &values_iter_class, "next")?;
        let mut out = BTreeSet::new();

        loop {
            let next_ref = env.call_ref_method(&values_iter, &next_method, &[])?;
            let next = unsafe { AniObject::from_raw(next_ref.as_raw() as sys::ani_object) };
            if env.get_property_by_name_boolean(&next, "done")? {
                break;
            }

            let value_ref = env.get_property_by_name_ref(&next, "value")?;
            let item = T::from_record_ref(env, &value_ref)?;
            out.insert(item);
        }

        Ok(out)
    }
}

// BTreeMap<K, V> - Map<K, V>
// ============================================================================

impl<K: TypeInfo, V: TypeInfo> TypeInfo for BTreeMap<K, V> {
    fn type_signature() -> &'static str {
        "Lstd/core/Map;"
    }
    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, K, V> ToAni<'env> for BTreeMap<K, V>
where
    K: RecordValue<'env> + Ord,
    V: RecordValue<'env>,
{
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let map_class = env.find_class("std.core.Map")?;
        let ctor = env.find_constructor(&map_class, "i:")?;
        let ctor_args = [ani_value_int(0)];
        let map = env.new_object(&map_class, &ctor, &ctor_args)?;
        let set_method = find_method_no_signature(env, &map_class, "set")?;

        for (key, value) in self {
            let key_ref = key.to_record_ref(env)?;
            let value_ref = value.to_record_ref(env)?;
            let args = [
                ani_value_ref(key_ref.as_raw()),
                ani_value_ref(value_ref.as_raw()),
            ];
            let _ = env.call_ref_method(&map, &set_method, &args)?;
        }

        Ok(map)
    }
}

impl<'env, K, V> FromAni<'env> for BTreeMap<K, V>
where
    K: RecordValue<'env> + Ord,
    V: RecordValue<'env>,
{
    type Input = sys::ani_object;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                crate::error::Status::InvalidArgs,
                "Null pointer: map",
            ));
        }

        let map = unsafe { AniObject::from_raw(value) };
        let map_class = env.find_class("std.core.Map")?;
        let keys_method = find_method_no_signature(env, &map_class, "keys")?;
        let values_method = find_method_no_signature(env, &map_class, "values")?;
        let keys_iter_ref = env.call_ref_method(&map, &keys_method, &[])?;
        let keys_iter = unsafe { AniObject::from_raw(keys_iter_ref.as_raw() as sys::ani_object) };
        let values_iter_ref = env.call_ref_method(&map, &values_method, &[])?;
        let values_iter =
            unsafe { AniObject::from_raw(values_iter_ref.as_raw() as sys::ani_object) };
        let keys_iter_type = env.get_object_type(&keys_iter)?;
        let keys_iter_class =
            unsafe { AniClass::from_raw(keys_iter_type.as_raw() as sys::ani_class) };
        let next_method = find_method_no_signature(env, &keys_iter_class, "next")?;
        let mut out = BTreeMap::new();

        loop {
            let next_key_ref = env.call_ref_method(&keys_iter, &next_method, &[])?;
            let next_key = unsafe { AniObject::from_raw(next_key_ref.as_raw() as sys::ani_object) };
            let next_value_ref = env.call_ref_method(&values_iter, &next_method, &[])?;
            let next_value =
                unsafe { AniObject::from_raw(next_value_ref.as_raw() as sys::ani_object) };
            let key_done = env.get_property_by_name_boolean(&next_key, "done")?;
            let value_done = env.get_property_by_name_boolean(&next_value, "done")?;
            if key_done || value_done {
                if key_done == value_done {
                    break;
                }
                return Err(Error::new(
                    crate::error::Status::InvalidType,
                    "Map keys()/values() iterator length mismatch",
                ));
            }

            let key_ref = env.get_property_by_name_ref(&next_key, "value")?;
            let value_ref = env.get_property_by_name_ref(&next_value, "value")?;
            let key = K::from_record_ref(env, &key_ref)?;
            let mapped_value = V::from_record_ref(env, &value_ref)?;
            out.insert(key, mapped_value);
        }

        Ok(out)
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
    type Output = sys::ani_fixedarray_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        vec![self.0, self.1].to_ani(env)
    }
}

impl<'env> FromAni<'env> for (i32, i32) {
    type Input = sys::ani_fixedarray_int;

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
    type Output = sys::ani_fixedarray_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        vec![self.0, self.1, self.2].to_ani(env)
    }
}

impl<'env> ToAni<'env> for (f64, f64) {
    type Output = sys::ani_fixedarray_double;

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
            "Lstd/core/Record;"
        );
    }

    #[test]
    fn test_hashset_type_signature() {
        assert_eq!(<HashSet<String>>::type_signature(), "Lstd/core/Set;");
    }

    #[test]
    fn test_btreeset_type_signature() {
        assert_eq!(<BTreeSet<String>>::type_signature(), "Lstd/core/Set;");
    }

    #[test]
    fn test_btreemap_type_signature() {
        assert_eq!(<BTreeMap<String, i32>>::type_signature(), "Lstd/core/Map;");
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
