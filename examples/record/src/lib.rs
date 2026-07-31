//! Record Example - Handling simple Record
//!
//! Demonstrates how to handle the ArkTS `Record<K, V>` type.
//! Record is similar to HashMap/Dictionary

use ani_derive::ani;
use std::collections::HashMap;

// ============================================================================
// Record Basic Operations
// ============================================================================

/// Create a simple Record
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function createRecord(): Record<string, int>;
/// ```
///
/// Returns pointer to HashMap
#[ani]
pub fn create_record() -> i64 {
    let map: HashMap<String, i32> = HashMap::new();
    let boxed = Box::new(map);
    Box::into_raw(boxed) as i64
}

/// Build a Record directly via Rust <-> ANI collection conversion.
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function createRecordDirect(): Record<string, int>;
/// ```
#[ani]
pub fn create_record_direct() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("answer".to_string(), 42);
    map.insert("size".to_string(), 2);
    map
}

/// Consume a Record from ArkTS as `HashMap<String, i32>`.
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function recordSum(entry: Record<string, int>): int;
/// ```
#[ani]
pub fn record_sum(entry: HashMap<String, i32>) -> i32 {
    entry.values().copied().sum()
}

/// Set value in Record
///
/// Corresponding ArkTS definition:
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

/// Get value from Record
///
/// Corresponding ArkTS definition:
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

/// Check if Record contains key
///
/// Corresponding ArkTS definition:
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

/// Delete value from Record
///
/// Corresponding ArkTS definition:
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

/// Get Record size
///
/// Corresponding ArkTS definition:
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

/// Clear Record
///
/// Corresponding ArkTS definition:
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

/// Release Record
///
/// Corresponding ArkTS definition:
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
// Record<string, string> Operations
// ============================================================================

/// Create string Record
#[ani]
pub fn create_string_record() -> i64 {
    let map: HashMap<String, String> = HashMap::new();
    let boxed = Box::new(map);
    Box::into_raw(boxed) as i64
}

/// Set string value
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

/// Get string value
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

/// Release string Record
#[ani]
pub fn destroy_string_record(record: i64) {
    if record != 0 {
        unsafe {
            let _ = Box::from_raw(record as *mut HashMap<String, String>);
        }
    }
}

// ============================================================================
// Record to JSON String Conversion
// ============================================================================

/// Convert int Record to JSON string
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
        parts.sort(); // Ensure consistent ordering
        format!("{{{}}}", parts.join(","))
    }
}

/// Convert string Record to JSON string
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
// Batch Operations
// ============================================================================

/// Parse Record from JSON string (simplified)
#[ani]
pub fn record_from_json(json: String) -> i64 {
    let mut map: HashMap<String, i32> = HashMap::new();

    // Simple parsing - should use JSON library in practice
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
// Module Initialization
// ============================================================================
