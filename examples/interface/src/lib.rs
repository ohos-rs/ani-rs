//! Interface Example - Creating Interface instances
//!
//! Demonstrates how to handle ArkTS Interface types
//! Note: native functions cannot be declared directly in interfaces

use ani_derive::ani;

// ============================================================================
// Native Wrapper for Interface Implementation
// ============================================================================

/// Create object implementing Comparable interface
///
/// Corresponding ArkTS definition:
/// ```typescript
/// interface Comparable {
///     compareTo(other: Object): int;
/// }
///
/// class IntWrapper implements Comparable {
///     private value: int;
///     constructor(v: int) { this.value = v; }
///     compareTo(other: Object): int { ... }
/// }
///
/// native function createComparable(value: int): long;
/// ```
#[ani]
pub fn create_comparable(value: i32) -> i64 {
    // Store value and return pointer
    let boxed = Box::new(ComparableImpl { value });
    Box::into_raw(boxed) as i64
}

/// Compare two Comparable objects
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function compareValues(a: long, b: long): int;
/// ```
#[ani]
pub fn compare_values(a: i64, b: i64) -> i32 {
    unsafe {
        let obj_a = &*(a as *const ComparableImpl);
        let obj_b = &*(b as *const ComparableImpl);

        match obj_a.value.cmp(&obj_b.value) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

/// Release Comparable object
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function destroyComparable(ptr: long): void;
/// ```
#[ani]
pub fn destroy_comparable(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut ComparableImpl);
        }
    }
}

// ============================================================================
// Internal Implementation Struct
// ============================================================================

struct ComparableImpl {
    value: i32,
}

// ============================================================================
// Serializable Interface Example
// ============================================================================

/// Create serializable object
///
/// Corresponding ArkTS definition:
/// ```typescript
/// interface Serializable {
///     serialize(): string;
///     deserialize(data: string): void;
/// }
///
/// native function createSerializable(data: string): long;
/// ```
#[ani]
pub fn create_serializable(data: String) -> i64 {
    let boxed = Box::new(SerializableImpl { data });
    Box::into_raw(boxed) as i64
}

/// Serialize object
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function serialize(ptr: long): string;
/// ```
#[ani]
pub fn serialize(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let obj = &*(ptr as *const SerializableImpl);
        format!("{{\"data\":\"{}\"}}", obj.data)
    }
}

/// Deserialize object
///
/// Corresponding ArkTS definition:
/// ```typescript
/// native function deserialize(ptr: long, json: string): void;
/// ```
#[ani]
pub fn deserialize(ptr: i64, json: String) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let obj = &mut *(ptr as *mut SerializableImpl);
        // Simple parsing - should use JSON library in practice
        if json.contains("data") {
            if let Some(start) = json.find("\"data\":\"") {
                let rest = &json[start + 8..];
                if let Some(end) = rest.find("\"") {
                    obj.data = rest[..end].to_string();
                }
            }
        }
    }
}

/// Get serialized data
#[ani]
pub fn get_data(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let obj = &*(ptr as *const SerializableImpl);
        obj.data.clone()
    }
}

/// Release serializable object
#[ani]
pub fn destroy_serializable(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut SerializableImpl);
        }
    }
}

struct SerializableImpl {
    data: String,
}

// ============================================================================
// Iterable Interface Example
// ============================================================================

/// Create iterable object
///
/// Corresponding ArkTS definition:
/// ```typescript
/// interface Iterable<T> {
///     hasNext(): boolean;
///     next(): T;
/// }
///
/// native function createIntIterator(start: int, end: int): long;
/// ```
#[ani]
pub fn create_int_iterator(start: i32, end: i32) -> i64 {
    let boxed = Box::new(IntIterator {
        current: start,
        end,
    });
    Box::into_raw(boxed) as i64
}

/// Check if there's a next element
#[ani]
pub fn iterator_has_next(ptr: i64) -> bool {
    if ptr == 0 {
        return false;
    }
    unsafe {
        let iter = &*(ptr as *const IntIterator);
        iter.current < iter.end
    }
}

/// Get next element
#[ani]
pub fn iterator_next(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let iter = &mut *(ptr as *mut IntIterator);
        if iter.current < iter.end {
            let result = iter.current;
            iter.current += 1;
            result
        } else {
            0
        }
    }
}

/// Reset iterator
#[ani]
pub fn iterator_reset(ptr: i64, start: i32) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let iter = &mut *(ptr as *mut IntIterator);
        iter.current = start;
    }
}

/// Release iterator
#[ani]
pub fn destroy_iterator(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut IntIterator);
        }
    }
}

struct IntIterator {
    current: i32,
    end: i32,
}

// ============================================================================
// Module Initialization
// ============================================================================
