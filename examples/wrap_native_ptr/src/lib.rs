//! Wrap Native Ptr Example - Storing Native object pointers in ETS objects
//!
//! Demonstrates how to store and manage Rust object pointers in ArkTS objects
//! This is the core technique for implementing class bindings

use ani::conversions::NativePointer;
use ani_derive::ani;
use std::collections::HashMap;
use std::sync::Mutex;

// ============================================================================
// Basic Pointer Wrapping
// ============================================================================

/// Native resource struct
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

/// Create Native resource and return pointer
///
/// Corresponding ArkTS definition:
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

/// Get resource ID
#[ani]
pub fn create_native_resource_handle(id: i32, name: String) -> NativePointer<NativeResource> {
    NativePointer::from_box(Box::new(NativeResource::new(id, name)))
}

#[ani]
pub fn get_native_resource_handle_id(ptr: NativePointer<NativeResource>) -> i32 {
    unsafe { ptr.as_ref().id }
}

#[ani]
pub fn destroy_native_resource_handle(ptr: NativePointer<NativeResource>) {
    unsafe {
        let _ = ptr.into_box();
    }
}

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

/// Get resource name
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

/// Set resource name
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

/// Add data to resource
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

/// Get resource data size
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

/// Release Native resource
///
/// Corresponding ArkTS definition:
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
// Complex Object Wrapping
// ============================================================================

/// Database connection simulation
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

/// Create database connection
#[ani]
pub fn create_db_connection(connection_string: String) -> i64 {
    let conn = Box::new(DatabaseConnection::new(connection_string));
    Box::into_raw(conn) as i64
}

/// Connect to database
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

/// Disconnect from database
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

/// Check if connected
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

/// Execute query
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

/// Get query count
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

/// Release database connection
#[ani]
pub fn destroy_db_connection(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let mut conn = Box::from_raw(ptr as *mut DatabaseConnection);
            conn.disconnect();
            // Box is automatically released
        }
    }
}

// ============================================================================
// Reference Counted Object Management
// ============================================================================

/// Global resource manager
static RESOURCE_MANAGER: Mutex<Option<HashMap<i64, i32>>> = Mutex::new(None);

fn get_manager() -> std::sync::MutexGuard<'static, Option<HashMap<i64, i32>>> {
    let mut guard = RESOURCE_MANAGER.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    guard
}

/// Add reference count
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

/// Decrease reference count, returns remaining count
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

/// Get current reference count
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
// Type-Safe Pointer Wrapping
// ============================================================================

/// Create type-tagged safe pointer
#[ani]
pub fn create_typed_ptr(type_id: i32, value: i64) -> i64 {
    // Encode type ID in high 32 bits, value in low 32 bits
    ((type_id as i64) << 32) | (value & 0xFFFFFFFF)
}

/// Get type ID
#[ani]
pub fn get_ptr_type_id(encoded: i64) -> i32 {
    ((encoded >> 32) & 0xFFFFFFFF) as i32
}

/// Get pointer value
#[ani]
pub fn get_ptr_value(encoded: i64) -> i64 {
    encoded & 0xFFFFFFFF
}

/// Validate pointer type
#[ani]
pub fn validate_ptr_type(encoded: i64, expected_type: i32) -> bool {
    get_ptr_type_id(encoded) == expected_type
}
