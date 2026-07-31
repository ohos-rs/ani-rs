//! Safe, process-local handles for Rust values owned by ArkTS.
//!
//! Unlike [`super::NativePointer`], a [`ManagedResource`] never exposes a
//! memory address. Handles are monotonically allocated and are never reused,
//! so a stale ArkTS `long` cannot become valid for a different allocation.

use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;

use super::{FromAni, ToAni, TypeInfo};

type ErasedResource = Arc<dyn Any + Send + Sync>;

struct ResourceEntry {
    type_id: TypeId,
    type_name: &'static str,
    value: ErasedResource,
}

static NEXT_RESOURCE_HANDLE: AtomicI64 = AtomicI64::new(1);
static RESOURCE_REGISTRY: OnceLock<RwLock<HashMap<i64, ResourceEntry>>> = OnceLock::new();

fn registry() -> &'static RwLock<HashMap<i64, ResourceEntry>> {
    RESOURCE_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Returns the number of live managed resources in this native module.
pub fn live_managed_resource_count() -> Result<usize> {
    registry()
        .read()
        .map(|entries| entries.len())
        .map_err(|_| lock_error("counting resources"))
}

/// Releases every managed resource owned by this native module.
///
/// The generated `ANI_Destructor` invokes this automatically.
#[doc(hidden)]
pub fn close_all_managed_resources() -> Result<usize> {
    let removed = {
        let mut entries = registry()
            .write()
            .map_err(|_| lock_error("closing all resources"))?;
        std::mem::take(&mut *entries)
    };
    let count = removed.len();
    drop(removed);
    Ok(count)
}

fn lock_error(operation: &str) -> Error {
    Error::new(
        Status::Error,
        format!("managed resource registry lock was poisoned while {operation}"),
    )
}

fn allocate_handle() -> Result<i64> {
    NEXT_RESOURCE_HANDLE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
            next.checked_add(1).filter(|value| *value > 0)
        })
        .map_err(|_| {
            Error::new(
                Status::OutOfRef,
                "managed resource handle space is exhausted",
            )
        })
}

/// A safe, opaque handle to a Rust value stored in a process-local registry.
///
/// The handle is represented as an ArkTS `long`. Cloning this type clones only
/// the token, not the underlying value. Call [`ManagedResource::close`] from an
/// explicit ArkTS `close`/`dispose` method or a `FinalizationRegistry`
/// callback. Closing is idempotent and concurrent operations that already
/// acquired the value are allowed to finish safely.
///
/// Values must be `Send` because ANI callbacks may arrive on different
/// threads. The registry serializes mutable access with a [`Mutex`].
#[repr(transparent)]
pub struct ManagedResource<T> {
    handle: i64,
    marker: PhantomData<fn() -> T>,
}

impl<T> Clone for ManagedResource<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            marker: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for ManagedResource<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ManagedResource")
            .field("handle", &self.handle)
            .field("type", &type_name::<T>())
            .finish()
    }
}

impl<T: Send + 'static> ManagedResource<T> {
    /// Stores a value and returns its new, non-reusable handle.
    pub fn new(value: T) -> Result<Self> {
        let handle = allocate_handle()?;
        let entry = ResourceEntry {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
            value: Arc::new(Mutex::new(value)),
        };
        registry()
            .write()
            .map_err(|_| lock_error("creating a resource"))?
            .insert(handle, entry);
        Ok(Self {
            handle,
            marker: PhantomData,
        })
    }

    /// Reconstructs a typed token from an ArkTS handle.
    ///
    /// This checks only the handle representation. The resource's liveness and
    /// type are checked atomically when it is accessed or closed, which keeps
    /// repeated `close` calls idempotent.
    pub fn from_raw(handle: i64) -> Result<Self> {
        if handle <= 0 {
            return Err(Error::new(
                Status::InvalidArgs,
                format!("managed resource handle must be positive, got {handle}"),
            ));
        }
        Ok(Self {
            handle,
            marker: PhantomData,
        })
    }

    /// Returns the opaque integer passed through ANI.
    pub fn as_raw(&self) -> i64 {
        self.handle
    }

    fn acquire(&self) -> Result<Arc<Mutex<T>>> {
        let entries = registry()
            .read()
            .map_err(|_| lock_error("accessing a resource"))?;
        let entry = entries.get(&self.handle).ok_or_else(|| {
            Error::new(
                Status::NotFound,
                format!("managed resource {} is closed or unknown", self.handle),
            )
        })?;
        if entry.type_id != TypeId::of::<T>() {
            return Err(Error::new(
                Status::InvalidType,
                format!(
                    "managed resource {} contains {}, not {}",
                    self.handle,
                    entry.type_name,
                    type_name::<T>()
                ),
            ));
        }
        Arc::downcast::<Mutex<T>>(Arc::clone(&entry.value)).map_err(|_| {
            Error::new(
                Status::InvalidType,
                format!(
                    "managed resource {} could not be downcast to {}",
                    self.handle,
                    type_name::<T>()
                ),
            )
        })
    }

    /// Runs a closure with shared access to the stored value.
    pub fn with<R>(&self, operation: impl FnOnce(&T) -> R) -> Result<R> {
        let value = self.acquire()?;
        let guard = value.lock().map_err(|_| lock_error("reading a resource"))?;
        Ok(operation(&guard))
    }

    /// Runs a closure with serialized mutable access to the stored value.
    pub fn with_mut<R>(&self, operation: impl FnOnce(&mut T) -> R) -> Result<R> {
        let value = self.acquire()?;
        let mut guard = value
            .lock()
            .map_err(|_| lock_error("mutating a resource"))?;
        Ok(operation(&mut guard))
    }

    /// Reports whether this handle still refers to a value of `T`.
    pub fn is_alive(&self) -> Result<bool> {
        let entries = registry()
            .read()
            .map_err(|_| lock_error("checking a resource"))?;
        Ok(entries
            .get(&self.handle)
            .is_some_and(|entry| entry.type_id == TypeId::of::<T>()))
    }

    /// Releases the registry's ownership of the value.
    ///
    /// Returns `true` when this call closed the resource and `false` when it
    /// was already closed. A live handle with a different `T` is rejected.
    pub fn close(&self) -> Result<bool> {
        let removed = {
            let mut entries = registry()
                .write()
                .map_err(|_| lock_error("closing a resource"))?;
            let Some(entry) = entries.get(&self.handle) else {
                return Ok(false);
            };
            if entry.type_id != TypeId::of::<T>() {
                return Err(Error::new(
                    Status::InvalidType,
                    format!(
                        "managed resource {} contains {}, not {}",
                        self.handle,
                        entry.type_name,
                        type_name::<T>()
                    ),
                ));
            }
            entries.remove(&self.handle)
        };
        drop(removed);
        Ok(true)
    }
}

impl<T> TypeInfo for ManagedResource<T> {
    fn type_signature() -> &'static str {
        "J"
    }

    fn ani_c_type() -> &'static str {
        "ani_long"
    }

    fn is_primitive() -> bool {
        true
    }
}

impl<'env, T> ToAni<'env> for ManagedResource<T> {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.handle)
    }
}

impl<'env, T: Send + 'static> FromAni<'env> for ManagedResource<T> {
    type Input = sys::ani_long;

    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Self::from_raw(value)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    use super::*;

    struct DropCounter(Arc<AtomicUsize>);

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn closes_once_and_rejects_stale_handles() {
        let drops = Arc::new(AtomicUsize::new(0));
        let resource = ManagedResource::new(DropCounter(Arc::clone(&drops))).unwrap();
        let stale = resource.clone();

        assert!(resource.is_alive().unwrap());
        assert!(resource.close().unwrap());
        assert!(!resource.close().unwrap());
        assert!(!stale.is_alive().unwrap());
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        let error = stale.with(|_| ()).unwrap_err();
        assert_eq!(error.status, Status::NotFound);
    }

    #[test]
    fn rejects_a_live_handle_with_the_wrong_type() {
        let resource = ManagedResource::new(42_u32).unwrap();
        let wrong = ManagedResource::<String>::from_raw(resource.as_raw()).unwrap();

        assert_eq!(
            wrong.with(String::len).unwrap_err().status,
            Status::InvalidType
        );
        assert_eq!(wrong.close().unwrap_err().status, Status::InvalidType);
        assert_eq!(resource.with(|value| *value).unwrap(), 42);
        assert!(resource.close().unwrap());
    }

    #[test]
    fn serializes_cross_thread_mutation() {
        let resource = ManagedResource::new(0_usize).unwrap();
        let mut workers = Vec::new();
        for _ in 0..8 {
            let resource = resource.clone();
            workers.push(thread::spawn(move || {
                for _ in 0..1_000 {
                    resource.with_mut(|value| *value += 1).unwrap();
                }
            }));
        }
        for worker in workers {
            worker.join().unwrap();
        }

        assert_eq!(resource.with(|value| *value).unwrap(), 8_000);
        assert!(resource.close().unwrap());
    }

    #[test]
    fn rejects_non_positive_handles() {
        assert_eq!(
            ManagedResource::<u8>::from_raw(0).unwrap_err().status,
            Status::InvalidArgs
        );
        assert_eq!(
            ManagedResource::<u8>::from_raw(-1).unwrap_err().status,
            Status::InvalidArgs
        );
    }

    #[test]
    fn module_cleanup_releases_all_live_resources() {
        let drops = Arc::new(AtomicUsize::new(0));
        let first = ManagedResource::new(DropCounter(Arc::clone(&drops))).unwrap();
        let second = ManagedResource::new(DropCounter(Arc::clone(&drops))).unwrap();
        assert!(live_managed_resource_count().unwrap() >= 2);

        let removed = close_all_managed_resources().unwrap();
        assert!(removed >= 2);
        assert_eq!(live_managed_resource_count().unwrap(), 0);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
        assert!(!first.is_alive().unwrap());
        assert!(!second.is_alive().unwrap());
    }
}
