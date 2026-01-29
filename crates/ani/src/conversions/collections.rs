//! 集合类型转换
//!
//! 实现 Rust 集合类型和 ANI 类型之间的转换
//! - HashMap<K, V> <-> Record<K, V>
//! - HashSet<T> <-> Set<T>
//! - BTreeMap<K, V> <-> Map<K, V>
//! - Tuple types

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;
use crate::types::*;

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

// HashMap<String, String> 的转换
impl<'env> ToAni<'env> for HashMap<String, String> {
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        // 创建 Record 对象
        let record_class = env.find_class("Lescompat/Record;")?;
        let ctor = env.find_constructor(&record_class, ":V")?;
        let record = env.new_object(&record_class, &ctor, &[])?;

        // 获取 set 方法
        let set_method =
            env.find_method(&record_class, "set", "Lstd/core/String;Lstd/core/Object;:V")?;

        // 设置每个键值对
        for (key, value) in self {
            let ani_key = env.create_string(&key)?;
            let ani_value = env.create_string(&value)?;

            let args = [
                ani_value_ref(ani_key.as_raw() as sys::ani_ref),
                ani_value_ref(ani_value.as_raw() as sys::ani_ref),
            ];

            env.call_method_void(&record, &set_method, &args)?;
        }

        Ok(record)
    }
}

// HashMap<String, i32> 的转换
impl<'env> ToAni<'env> for HashMap<String, i32> {
    type Output = AniObject<'env>;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let record_class = env.find_class("Lescompat/Record;")?;
        let ctor = env.find_constructor(&record_class, ":V")?;
        let record = env.new_object(&record_class, &ctor, &[])?;

        let set_method =
            env.find_method(&record_class, "set", "Lstd/core/String;Lstd/core/Object;:V")?;

        for (key, value) in self {
            let ani_key = env.create_string(&key)?;
            let boxed_value = super::boxed::Boxable::box_value(value, env)?;

            let args = [
                ani_value_ref(ani_key.as_raw() as sys::ani_ref),
                ani_value_ref(boxed_value.as_raw() as sys::ani_ref),
            ];

            env.call_method_void(&record, &set_method, &args)?;
        }

        Ok(record)
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
// 元组类型
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

// 元组转换为数组
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
// JSON 值类型（用于动态数据）
// ============================================================================

/// 动态 ANI 值，类似于 JSON
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
    /// 从 bool 创建
    pub fn from_bool(v: bool) -> Self {
        AniValue::Bool(v)
    }

    /// 从 i32 创建
    pub fn from_int(v: i32) -> Self {
        AniValue::Int(v)
    }

    /// 从 i64 创建
    pub fn from_long(v: i64) -> Self {
        AniValue::Long(v)
    }

    /// 从 f64 创建
    pub fn from_double(v: f64) -> Self {
        AniValue::Double(v)
    }

    /// 从 String 创建
    pub fn from_string(v: String) -> Self {
        AniValue::String(v)
    }

    /// 检查是否为 null
    pub fn is_null(&self) -> bool {
        matches!(self, AniValue::Null)
    }

    /// 尝试获取 bool 值
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AniValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// 尝试获取 i32 值
    pub fn as_int(&self) -> Option<i32> {
        match self {
            AniValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// 尝试获取 i64 值
    pub fn as_long(&self) -> Option<i64> {
        match self {
            AniValue::Long(v) => Some(*v),
            AniValue::Int(v) => Some(*v as i64),
            _ => None,
        }
    }

    /// 尝试获取 f64 值
    pub fn as_double(&self) -> Option<f64> {
        match self {
            AniValue::Double(v) => Some(*v),
            AniValue::Int(v) => Some(*v as f64),
            AniValue::Long(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// 尝试获取字符串值
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AniValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// 尝试获取数组
    pub fn as_array(&self) -> Option<&Vec<AniValue>> {
        match self {
            AniValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// 尝试获取对象
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
