//! Wrap Native Ptr 示例 - 在 ETS 对象中保存 Native 对象指针
//!
//! 演示如何在 ArkTS 对象中存储和管理 Rust 对象指针
//! 这是实现类绑定的核心技术

use ani::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================================
// 基本指针包装
// ============================================================================

/// Native 资源结构体
pub struct NativeResource {
    id: i32,
    name: String,
    data: Vec<u8>,
}

impl NativeResource {
    fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            data: Vec::new(),
        }
    }
}

/// 创建 Native 资源并返回指针
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class NativeWrapper {
///     private nativePtr: long = 0;
///     
///     constructor(id: int, name: string) {
///         this.nativePtr = NativeWrapper.createNative(id, name);
///     }
///     
///     native static createNative(id: int, name: string): long;
/// }
/// ```
#[ani]
pub fn create_native_resource(id: i32, name: String) -> i64 {
    let resource = Box::new(NativeResource::new(id, name));
    Box::into_raw(resource) as i64
}

/// 获取资源 ID
#[ani]
pub fn get_resource_id(ptr: i64) -> i32 {
    if ptr == 0 {
        return -1;
    }
    unsafe {
        let resource = &*(ptr as *const NativeResource);
        resource.id
    }
}

/// 获取资源名称
#[ani]
pub fn get_resource_name(ptr: i64) -> String {
    if ptr == 0 {
        return String::new();
    }
    unsafe {
        let resource = &*(ptr as *const NativeResource);
        resource.name.clone()
    }
}

/// 设置资源名称
#[ani]
pub fn set_resource_name(ptr: i64, name: String) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let resource = &mut *(ptr as *mut NativeResource);
        resource.name = name;
    }
}

/// 添加数据到资源
#[ani]
pub fn resource_add_data(ptr: i64, byte: i32) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let resource = &mut *(ptr as *mut NativeResource);
        resource.data.push(byte as u8);
    }
}

/// 获取资源数据大小
#[ani]
pub fn resource_data_size(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let resource = &*(ptr as *const NativeResource);
        resource.data.len() as i32
    }
}

/// 释放 Native 资源
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// class NativeWrapper {
///     dispose(): void {
///         if (this.nativePtr != 0) {
///             NativeWrapper.destroyNative(this.nativePtr);
///             this.nativePtr = 0;
///         }
///     }
///     
///     native static destroyNative(ptr: long): void;
/// }
/// ```
#[ani]
pub fn destroy_native_resource(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut NativeResource);
        }
    }
}

// ============================================================================
// 复杂对象包装
// ============================================================================

/// 数据库连接模拟
#[allow(dead_code)]
pub struct DatabaseConnection {
    connection_string: String,
    is_connected: bool,
    query_count: i32,
}

impl DatabaseConnection {
    fn new(conn_str: String) -> Self {
        Self {
            connection_string: conn_str,
            is_connected: false,
            query_count: 0,
        }
    }

    fn connect(&mut self) -> bool {
        self.is_connected = true;
        true
    }

    fn disconnect(&mut self) {
        self.is_connected = false;
    }

    fn execute_query(&mut self, _query: &str) -> i32 {
        if !self.is_connected {
            return -1;
        }
        self.query_count += 1;
        self.query_count
    }
}

/// 创建数据库连接
#[ani]
pub fn create_db_connection(connection_string: String) -> i64 {
    let conn = Box::new(DatabaseConnection::new(connection_string));
    Box::into_raw(conn) as i64
}

/// 连接数据库
#[ani]
pub fn db_connect(ptr: i64) -> bool {
    if ptr == 0 {
        return false;
    }
    unsafe {
        let conn = &mut *(ptr as *mut DatabaseConnection);
        conn.connect()
    }
}

/// 断开数据库连接
#[ani]
pub fn db_disconnect(ptr: i64) {
    if ptr == 0 {
        return;
    }
    unsafe {
        let conn = &mut *(ptr as *mut DatabaseConnection);
        conn.disconnect();
    }
}

/// 检查是否已连接
#[ani]
pub fn db_is_connected(ptr: i64) -> bool {
    if ptr == 0 {
        return false;
    }
    unsafe {
        let conn = &*(ptr as *const DatabaseConnection);
        conn.is_connected
    }
}

/// 执行查询
#[ani]
pub fn db_execute_query(ptr: i64, query: String) -> i32 {
    if ptr == 0 {
        return -1;
    }
    unsafe {
        let conn = &mut *(ptr as *mut DatabaseConnection);
        conn.execute_query(&query)
    }
}

/// 获取查询计数
#[ani]
pub fn db_get_query_count(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        let conn = &*(ptr as *const DatabaseConnection);
        conn.query_count
    }
}

/// 释放数据库连接
#[ani]
pub fn destroy_db_connection(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let mut conn = Box::from_raw(ptr as *mut DatabaseConnection);
            conn.disconnect();
            // Box 会自动释放
        }
    }
}

// ============================================================================
// 带引用计数的对象管理
// ============================================================================

/// 全局资源管理器
static RESOURCE_MANAGER: Mutex<Option<HashMap<i64, i32>>> = Mutex::new(None);

fn get_manager() -> std::sync::MutexGuard<'static, Option<HashMap<i64, i32>>> {
    let mut guard = RESOURCE_MANAGER.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

/// 增加引用计数
#[ani]
pub fn add_ref(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let mut manager = get_manager();
    if let Some(ref mut map) = *manager {
        let count = map.entry(ptr).or_insert(0);
        *count += 1;
        *count
    } else {
        0
    }
}

/// 减少引用计数，返回剩余计数
#[ani]
pub fn release_ref(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let mut manager = get_manager();
    if let Some(ref mut map) = *manager {
        if let Some(count) = map.get_mut(&ptr) {
            *count -= 1;
            let remaining = *count;
            if remaining <= 0 {
                map.remove(&ptr);
            }
            return remaining;
        }
    }
    0
}

/// 获取当前引用计数
#[ani]
pub fn get_ref_count(ptr: i64) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let manager = get_manager();
    if let Some(ref map) = *manager {
        *map.get(&ptr).unwrap_or(&0)
    } else {
        0
    }
}

// ============================================================================
// 类型安全的指针包装
// ============================================================================

/// 使用类型标记的安全指针
#[ani]
pub fn create_typed_ptr(type_id: i32, value: i64) -> i64 {
    // 将类型 ID 编码到高 32 位，值编码到低 32 位
    ((type_id as i64) << 32) | (value & 0xFFFFFFFF)
}

/// 获取类型 ID
#[ani]
pub fn get_ptr_type_id(encoded: i64) -> i32 {
    ((encoded >> 32) & 0xFFFFFFFF) as i32
}

/// 获取指针值
#[ani]
pub fn get_ptr_value(encoded: i64) -> i64 {
    encoded & 0xFFFFFFFF
}

/// 验证指针类型
#[ani]
pub fn validate_ptr_type(encoded: i64, expected_type: i32) -> bool {
    get_ptr_type_id(encoded) == expected_type
}
