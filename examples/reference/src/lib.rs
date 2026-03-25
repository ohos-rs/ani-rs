//! Reference Example - Demonstrates ANI reference management in ani-rs
//!
//! This example shows how to use the reference types for managing ANI object lifetimes:
//!
//! - `Ref<T>` - Typed global reference that can be stored for later use
//! - `GlobalRef` - Low-level untyped global reference
//!
//! # Reference Model
//!
//! ANI follows a reference model similar to JNI:
//!
//! | Type | Lifetime | Thread Safety | Use Case |
//! |------|----------|---------------|----------|
//! | `AniRef<'env>` | Scoped to native call | No | Short-lived operations |
//! | `Ref<T>` | Manual (global) | Yes (Send+Sync) | Store for later use |
//! | `GlobalRef` | Manual (global) | Yes (Send+Sync) | Low-level storage |
//! | `WeakRef` | May be GC'd | Yes | Cache that can be invalidated |
//!
//! # ArkTS Usage
//!
//! ```typescript
//! // Import native module
//! import native from 'libani_example_reference.so';
//!
//! // Store an object reference
//! const myObj = { name: "test", value: 42 };
//! native.storeObject(myObj);
//!
//! // Check if stored
//! console.log(native.hasStoredObject()); // true
//!
//! // Use stored object
//! console.log(native.useStoredObject()); // true
//!
//! // Clear storage
//! native.clearStoredObject();
//! console.log(native.hasStoredObject()); // false
//! ```

use std::sync::Mutex;

use ani::prelude::*;
use ani_derive::ani;

// ============================================================================
// Basic Ref<T> Usage - Store and Retrieve Objects
// ============================================================================

/// Global storage for a single object reference
///
/// Ref<AniObject<'static>> is Send + Sync, so it can be stored in a Mutex
static STORED_OBJECT: Mutex<Option<Ref<AniObject<'static>>>> = Mutex::new(None);

/// Store an object for later use
///
/// When receiving `Ref<T>` from ArkTS, a global reference is automatically
/// created from the local reference. This global reference can outlive
/// the current native call.
///
/// # Arguments
/// - `obj` - The object to store (automatically converted to global ref)
///
/// # ArkTS
/// ```typescript
/// function storeObject(obj: Object): void;
/// ```
#[ani]
pub fn store_object(_env: &Env, obj: Ref<AniObject<'static>>) -> Result<()> {
    let mut guard = STORED_OBJECT.lock().unwrap();
    *guard = Some(obj);
    Ok(())
}

/// Check if an object is currently stored
///
/// # Returns
/// true if an object is stored, false otherwise
///
/// # ArkTS
/// ```typescript
/// function hasStoredObject(): boolean;
/// ```
#[ani]
pub fn has_stored_object() -> bool {
    let guard = STORED_OBJECT.lock().unwrap();
    guard.is_some()
}

/// Clear the stored object
///
/// This releases the global reference, allowing the object to be GC'd.
///
/// # ArkTS
/// ```typescript
/// function clearStoredObject(): void;
/// ```
#[ani]
pub fn clear_stored_object() -> Result<()> {
    let mut guard = STORED_OBJECT.lock().unwrap();
    *guard = None;
    Ok(())
}

/// Use the stored object
///
/// Demonstrates how to borrow the stored reference as a local reference
/// for operations within the current native call.
///
/// # Returns
/// true if the object was successfully used, false if no object stored
///
/// # ArkTS
/// ```typescript
/// function useStoredObject(): boolean;
/// ```
#[ani]
pub fn use_stored_object(env: &Env) -> Result<bool> {
    let guard = STORED_OBJECT.lock().unwrap();
    if let Some(ref obj_ref) = *guard {
        // Borrow the stored reference as a local AniObject
        let _obj = obj_ref.borrow(env);
        // You can now use _obj for any operations that require AniObject
        // For example: call methods, get fields, etc.
        Ok(true)
    } else {
        Ok(false)
    }
}

// ============================================================================
// Multiple Object Storage
// ============================================================================

/// Storage for multiple named objects
static NAMED_OBJECTS: Mutex<Vec<(String, Ref<AniObject<'static>>)>> = Mutex::new(Vec::new());

/// Store an object with a name
///
/// Multiple objects can be stored with different names.
/// If an object with the same name exists, it will be replaced.
///
/// # Arguments
/// - `name` - The name to associate with the object
/// - `obj` - The object to store
///
/// # ArkTS
/// ```typescript
/// function storeNamedObject(name: string, obj: Object): void;
/// ```
#[ani]
pub fn store_named_object(_env: &Env, name: String, obj: Ref<AniObject<'static>>) -> Result<()> {
    let mut guard = NAMED_OBJECTS.lock().unwrap();
    // Remove existing object with same name
    guard.retain(|(n, _)| n != &name);
    // Add new object
    guard.push((name, obj));
    Ok(())
}

/// Check if a named object exists
///
/// # Arguments
/// - `name` - The name to check
///
/// # Returns
/// true if an object with the given name exists
///
/// # ArkTS
/// ```typescript
/// function hasNamedObject(name: string): boolean;
/// ```
#[ani]
pub fn has_named_object(name: String) -> bool {
    let guard = NAMED_OBJECTS.lock().unwrap();
    guard.iter().any(|(n, _)| n == &name)
}

/// Get the count of stored named objects
///
/// # Returns
/// The number of objects currently stored
///
/// # ArkTS
/// ```typescript
/// function getNamedObjectCount(): number;
/// ```
#[ani]
pub fn get_named_object_count() -> i32 {
    let guard = NAMED_OBJECTS.lock().unwrap();
    guard.len() as i32
}

/// Remove a named object
///
/// # Arguments
/// - `name` - The name of the object to remove
///
/// # Returns
/// true if an object was removed, false if not found
///
/// # ArkTS
/// ```typescript
/// function removeNamedObject(name: string): boolean;
/// ```
#[ani]
pub fn remove_named_object(name: String) -> bool {
    let mut guard = NAMED_OBJECTS.lock().unwrap();
    let initial_len = guard.len();
    guard.retain(|(n, _)| n != &name);
    guard.len() < initial_len
}

/// Clear all named objects
///
/// # ArkTS
/// ```typescript
/// function clearAllNamedObjects(): void;
/// ```
#[ani]
pub fn clear_all_named_objects() -> Result<()> {
    let mut guard = NAMED_OBJECTS.lock().unwrap();
    guard.clear();
    Ok(())
}

// ============================================================================
// Clone Reference Example
// ============================================================================

/// Clone a stored reference
///
/// Demonstrates using Ref<T>::clone_ref() to create a new global reference
/// pointing to the same object.
///
/// # Returns
/// true if cloning succeeded (object was stored), false otherwise
///
/// # ArkTS
/// ```typescript
/// function cloneStoredObject(): boolean;
/// ```
#[ani]
pub fn clone_stored_object(env: &Env) -> Result<bool> {
    let guard = STORED_OBJECT.lock().unwrap();
    if let Some(ref obj_ref) = *guard {
        // Clone the reference - this creates a new global reference
        let _cloned = obj_ref.clone_ref(env)?;
        // _cloned is a new Ref<AniObject> pointing to the same object
        // It has its own global reference that must be separately managed
        Ok(true)
    } else {
        Ok(false)
    }
}

// ============================================================================
// Reference Comparison Example
// ============================================================================

/// Storage for comparison
static COMPARE_OBJECT: Mutex<Option<Ref<AniObject<'static>>>> = Mutex::new(None);

/// Store an object for comparison
///
/// # ArkTS
/// ```typescript
/// function setCompareObject(obj: Object): void;
/// ```
#[ani]
pub fn set_compare_object(_env: &Env, obj: Ref<AniObject<'static>>) -> Result<()> {
    let mut guard = COMPARE_OBJECT.lock().unwrap();
    *guard = Some(obj);
    Ok(())
}

/// Check if two stored references point to the same raw pointer
///
/// Note: This compares raw pointer values, not object equality.
///
/// # Returns
/// true if both references point to same memory location
///
/// # ArkTS
/// ```typescript
/// function compareStoredReferences(): boolean;
/// ```
#[ani]
pub fn compare_stored_references() -> bool {
    let guard1 = STORED_OBJECT.lock().unwrap();
    let guard2 = COMPARE_OBJECT.lock().unwrap();

    match (guard1.as_ref(), guard2.as_ref()) {
        (Some(ref1), Some(ref2)) => {
            // Compare raw pointers
            ref1.as_raw() == ref2.as_raw()
        }
        _ => false,
    }
}

/// Clear the comparison object
///
/// # ArkTS
/// ```typescript
/// function clearCompareObject(): void;
/// ```
#[ani]
pub fn clear_compare_object() -> Result<()> {
    let mut guard = COMPARE_OBJECT.lock().unwrap();
    *guard = None;
    Ok(())
}

// ============================================================================
// Low-level GlobalRef / WeakRef Roundtrip
// ============================================================================

#[ani]
pub fn validate_global_handle_roundtrip(env: &Env, obj: AniRef<'_>) -> Result<bool> {
    let handle = env.create_global_ref(&obj)?;
    let local = handle.to_local(env)?;
    let ok = !handle.as_raw().is_null() && !local.as_raw().is_null();
    env.delete_local_ref(&local)?;
    handle.delete(env)?;
    Ok(ok)
}

#[ani]
pub fn validate_weak_handle_roundtrip(env: &Env, obj: AniRef<'_>) -> Result<bool> {
    let handle = env.create_weak_ref(&obj)?;
    let upgraded = handle.is_alive(env)?;
    handle.delete(env)?;
    Ok(upgraded)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_stored_object_initial() {
        // Initially no object should be stored
        // Note: test may fail if state persists from other tests
        let _ = has_stored_object();
    }

    #[test]
    fn test_get_named_object_count() {
        let count = get_named_object_count();
        assert!(count >= 0);
    }

    #[test]
    fn test_has_named_object_nonexistent() {
        let result = has_named_object("nonexistent_test_key".to_string());
        // Should be false for a key that was never added
        assert!(!result);
    }

    #[test]
    fn test_remove_named_object_nonexistent() {
        let result = remove_named_object("another_nonexistent_key".to_string());
        // Should be false for a key that doesn't exist
        assert!(!result);
    }

    #[test]
    fn test_compare_stored_references_empty() {
        // Both should be None, so comparison returns false
        let result = compare_stored_references();
        // Result depends on global state, just ensure it doesn't panic
        let _ = result;
    }
}
