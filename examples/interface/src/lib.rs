//! Interface 示例 - 创建 Interface 实例
//!
//! 演示如何处理 ArkTS Interface 类型
//! 注意: native 函数不能直接声明在 interface 中

use ani::prelude::*;

// ============================================================================
// Interface 实现的 Native 包装
// ============================================================================

/// 创建实现 Comparable 接口的对象
///
/// 对应的 ArkTS 定义:
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
    // 存储值并返回指针
    let boxed = Box::new(ComparableImpl { value });
    Box::into_raw(boxed) as i64
}

/// 比较两个 Comparable 对象
///
/// 对应的 ArkTS 定义:
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

/// 释放 Comparable 对象
///
/// 对应的 ArkTS 定义:
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
// 内部实现结构
// ============================================================================

struct ComparableImpl {
    value: i32,
}

// ============================================================================
// Serializable 接口示例
// ============================================================================

/// 创建可序列化对象
///
/// 对应的 ArkTS 定义:
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

/// 序列化对象
///
/// 对应的 ArkTS 定义:
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

/// 反序列化对象
///
/// 对应的 ArkTS 定义:
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
        // 简单解析 - 实际应使用 JSON 库
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

/// 获取序列化数据
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

/// 释放可序列化对象
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
// Iterable 接口示例
// ============================================================================

/// 创建可迭代对象
///
/// 对应的 ArkTS 定义:
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

/// 检查是否还有下一个元素
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

/// 获取下一个元素
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

/// 重置迭代器
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

/// 释放迭代器
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
// 模块初始化
// ============================================================================
