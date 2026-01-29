//! Record 示例 - 处理简单 Record
//!
//! 演示如何处理 ArkTS 的 Record<K, V> 类型
//! Record 类似于 HashMap/Dictionary

use ani::prelude::*;
use ani_derive::ani;
use std::collections::HashMap;

// ============================================================================
// Record 基本操作
// ============================================================================

/// 创建一个简单的 Record
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function createRecord(): Record<string, int>;
/// ```
///
/// 返回指向 HashMap 的指针
#[ani]
pub fn create_record() -> i64 {
    let map: HashMap<String, i32> = HashMap::new();
    let boxed = Box::new(map);
    Box::into_raw(boxed) as i64
}

/// 向 Record 中设置值
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordSet(record: long, key: string, value: int): void;
/// ```
#[ani]
pub fn record_set(record: i64, key: String, value: i32) {
    if record == 0 {
        return;
    }
    unsafe {
        let map = &mut *(record as *mut HashMap<String, i32>);
        map.insert(key, value);
    }
}

/// 从 Record 中获取值
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordGet(record: long, key: string): int;
/// ```
#[ani]
pub fn record_get(record: i64, key: String) -> i32 {
    if record == 0 {
        return 0;
    }
    unsafe {
        let map = &*(record as *const HashMap<String, i32>);
        *map.get(&key).unwrap_or(&0)
    }
}

/// 检查 Record 是否包含 key
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordHas(record: long, key: string): boolean;
/// ```
#[ani]
pub fn record_has(record: i64, key: String) -> bool {
    if record == 0 {
        return false;
    }
    unsafe {
        let map = &*(record as *const HashMap<String, i32>);
        map.contains_key(&key)
    }
}

/// 从 Record 中删除值
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordDelete(record: long, key: string): boolean;
/// ```
#[ani]
pub fn record_delete(record: i64, key: String) -> bool {
    if record == 0 {
        return false;
    }
    unsafe {
        let map = &mut *(record as *mut HashMap<String, i32>);
        map.remove(&key).is_some()
    }
}

/// 获取 Record 的大小
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordSize(record: long): int;
/// ```
#[ani]
pub fn record_size(record: i64) -> i32 {
    if record == 0 {
        return 0;
    }
    unsafe {
        let map = &*(record as *const HashMap<String, i32>);
        map.len() as i32
    }
}

/// 清空 Record
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function recordClear(record: long): void;
/// ```
#[ani]
pub fn record_clear(record: i64) {
    if record == 0 {
        return;
    }
    unsafe {
        let map = &mut *(record as *mut HashMap<String, i32>);
        map.clear();
    }
}

/// 释放 Record
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function destroyRecord(record: long): void;
/// ```
#[ani]
pub fn destroy_record(record: i64) {
    if record != 0 {
        unsafe {
            let _ = Box::from_raw(record as *mut HashMap<String, i32>);
        }
    }
}

// ============================================================================
// Record<string, string> 操作
// ============================================================================

/// 创建字符串 Record
#[ani]
pub fn create_string_record() -> i64 {
    let map: HashMap<String, String> = HashMap::new();
    let boxed = Box::new(map);
    Box::into_raw(boxed) as i64
}

/// 设置字符串值
#[ani]
pub fn string_record_set(record: i64, key: String, value: String) {
    if record == 0 {
        return;
    }
    unsafe {
        let map = &mut *(record as *mut HashMap<String, String>);
        map.insert(key, value);
    }
}

/// 获取字符串值
#[ani]
pub fn string_record_get(record: i64, key: String) -> String {
    if record == 0 {
        return String::new();
    }
    unsafe {
        let map = &*(record as *const HashMap<String, String>);
        map.get(&key).cloned().unwrap_or_default()
    }
}

/// 释放字符串 Record
#[ani]
pub fn destroy_string_record(record: i64) {
    if record != 0 {
        unsafe {
            let _ = Box::from_raw(record as *mut HashMap<String, String>);
        }
    }
}

// ============================================================================
// Record 转换为 JSON 字符串
// ============================================================================

/// 将 int Record 转换为 JSON 字符串
#[ani]
pub fn record_to_json(record: i64) -> String {
    if record == 0 {
        return "{}".to_string();
    }
    unsafe {
        let map = &*(record as *const HashMap<String, i32>);
        let mut parts: Vec<String> = map
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", k, v))
            .collect();
        parts.sort(); // 确保顺序一致
        format!("{{{}}}", parts.join(","))
    }
}

/// 将字符串 Record 转换为 JSON 字符串
#[ani]
pub fn string_record_to_json(record: i64) -> String {
    if record == 0 {
        return "{}".to_string();
    }
    unsafe {
        let map = &*(record as *const HashMap<String, String>);
        let mut parts: Vec<String> = map
            .iter()
            .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
            .collect();
        parts.sort();
        format!("{{{}}}", parts.join(","))
    }
}

// ============================================================================
// 批量操作
// ============================================================================

/// 从 JSON 字符串解析 Record（简化版）
#[ani]
pub fn record_from_json(json: String) -> i64 {
    let mut map: HashMap<String, i32> = HashMap::new();

    // 简单解析 - 实际应使用 JSON 库
    let content = json.trim_start_matches('{').trim_end_matches('}');
    for pair in content.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().trim_matches('"');
            if let Ok(value) = parts[1].trim().parse::<i32>() {
                map.insert(key.to_string(), value);
            }
        }
    }

    let boxed = Box::new(map);
    Box::into_raw(boxed) as i64
}

// ============================================================================
// 模块初始化
// ============================================================================
