use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
    thread::{current, Thread},
};

use log::{debug, error};

use crate::{errors::*, sys, anienv::ANIEnv, ANIVersion};

#[cfg(feature = "invocation")]
use std::os::raw::c_void;

/// The ANI VM singleton.
///
/// ANI does not support creating more than one VM per-process.
static ANI_VM_SINGLETON: once_cell::sync::OnceCell<AniVM> = once_cell::sync::OnceCell::new();

/// The ANI VM, providing Invocation API support.
///
/// An existing AniVM can be obtained either via [`AniVM::singleton`], or [`ANIEnv::get_ani_vm`]
/// in an already attached thread.
///
/// ## Attaching Native Threads
///
/// A native thread must «attach» itself to be able to call methods outside of a native method.
/// This library provides two modes of attachment:
/// * A scoped attachment with [`attach_current_thread`][act].
///   The thread will automatically detach itself once the returned guard is dropped.
/// * A permanent attachment with [`attach_current_thread_permanently`][actp]
///   The thread will automatically detach itself before it terminates.
///
/// [act]: struct.AniVM.html#method.attach_current_thread
/// [actp]: struct.AniVM.html#method.attach_current_thread_permanently
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AniVM(*mut sys::ani_vm);

unsafe impl Send for AniVM {}
unsafe impl Sync for AniVM {}


impl AniVM {
    /// Get a [`AniVM`] for the global ANI VM
    ///
    /// If no [`AniVM`] has been initialized, this will return [`Error::UninitializedAniVM`].
    pub fn singleton() -> Result<Self> {
        ANI_VM_SINGLETON
            .get()
            .cloned()
            .ok_or(Error::UninitializedAniVM)
    }

    /// Create an AniVM from a raw pointer.
    ///
    /// # Safety
    ///
    /// Expects a valid, non-null ani_vm pointer.
    ///
    /// Only does a `null` check.
    pub unsafe fn from_raw(ptr: *mut sys::ani_vm) -> Self {
        assert!(!ptr.is_null());
        ANI_VM_SINGLETON.get_or_init(|| AniVM(ptr)).clone()
    }

    /// Returns underlying `sys::ani_vm` interface.
    pub fn get_raw(&self) -> *mut sys::ani_vm {
        self.0
    }

    pub(crate) fn from_env(env: &ANIEnv) -> Self {
        // Don't use `.get_or_init()` here because it would deadlock if calling `AniVM::from_raw`
        // which also uses `.get_or_init()`
        if let Some(vm) = ANI_VM_SINGLETON.get() {
            vm.clone()
        } else {
            let mut raw: *mut sys::ani_vm = ptr::null_mut();
            unsafe {
                let status = ani_call_unchecked!(env, GetVM, &mut raw);
                ani_status_to_result(status)
                    .expect("Spurious failure to get AniVM from ANIEnv");
                AniVM::from_raw(raw)
            }
        }
    }

    /// Attaches the current thread to the VM. Calling this for a thread that is already attached
    /// is a no-op.
    ///
    /// The thread will detach itself automatically when it exits.
    pub fn attach_current_thread_permanently(&'_ self) -> Result<ANIEnv<'_>> {
        unsafe {
            match self.get_env(ANIVersion::V1) {
                Ok(env) => Ok(env),
                Err(_) => self.attach_current_thread_impl(),
            }
        }
    }

    /// Attaches the current thread to the ANI VM. The returned [`AttachGuard`]
    /// can be dereferenced to a [`ANIEnv`] and automatically detaches the
    /// thread when dropped.
    ///
    /// Calling this in a thread that is already attached is a no-op.
    pub fn attach_current_thread(&'_ self) -> Result<AttachGuard<'_>> {
        unsafe {
            match self.get_env(ANIVersion::V1) {
                Ok(env) => Ok(AttachGuard::new_nested(env)),
                Err(_) => {
                    let env = self.attach_current_thread_impl()?;
                    Ok(AttachGuard::new(env))
                }
            }
        }
    }

    /// Explicitly detaches the current thread from the VM.
    ///
    /// _**Note**: This operation is _rarely_ appropriate to use, because the
    /// attachment methods ensure that the thread is automatically detached._
    ///
    /// Detaching a non-attached thread is a no-op.
    ///
    /// # Safety
    ///
    /// __Any existing `ANIEnv`s and `AttachGuard`s created in the calling thread
    /// will be invalidated after this method completes. It is the__ caller's __responsibility
    /// to ensure that no calls are subsequently performed on these objects.__
    pub unsafe fn detach_current_thread(&self) {
        InternalAttachGuard::clear_tls();
    }

    /// Returns the current number of threads attached to the VM.
    pub fn threads_attached(&self) -> usize {
        ATTACHED_THREADS.load(Ordering::SeqCst)
    }

    /// Get the `ANIEnv` associated with the current thread, or
    /// `Error::AniCall` if the current thread is not attached to the VM.
    ///
    /// # Safety
    ///
    /// You must not use this API to materialize a [`ANIEnv`] if there is
    /// already another [`ANIEnv`] or local reference in scope.
    pub unsafe fn get_env(&'_ self, version: ANIVersion) -> Result<ANIEnv<'_>> {
        let mut ptr: *mut sys::ani_env = ptr::null_mut();
        unsafe {
            let status = ani_vm_call_unchecked!(self, GetEnv, version.raw(), &mut ptr);
            ani_status_to_result(status)?;
            Ok(ANIEnv::from_raw_unchecked(ptr))
        }
    }

    /// Creates `InternalAttachGuard` and attaches current thread.
    unsafe fn attach_current_thread_impl(&'_ self) -> Result<ANIEnv<'_>> {
        let guard = InternalAttachGuard::new(self.clone());
        let env_ptr = unsafe { guard.attach_current_thread()? };

        InternalAttachGuard::fill_tls(guard);

        unsafe { ANIEnv::from_raw(env_ptr) }
    }

    /// Unloads the AniVM and frees all its associated resources
    ///
    /// # Safety
    ///
    /// After `destroy()` returns then the `AniVM` will be in an undefined
    /// state and must be dropped to avoid undefined behaviour.
    pub unsafe fn destroy(&self) -> Result<()> {
        unsafe {
            let status = ani_vm_call_unchecked!(self, DestroyVM);
            ani_status_to_result(status)
        }
    }
}

thread_local! {
    static THREAD_ATTACH_GUARD: RefCell<Option<InternalAttachGuard>> = const { RefCell::new(None) }
}

static ATTACHED_THREADS: AtomicUsize = AtomicUsize::new(0);

/// A RAII implementation of scoped guard which detaches the current thread
/// when dropped. The attached `ANIEnv` can be accessed through this guard
/// via its `Deref` implementation.
pub struct AttachGuard<'local> {
    env: ANIEnv<'local>,
    should_detach: bool,
}

impl<'local> AttachGuard<'local> {
    /// AttachGuard created with this method will detach current thread on drop
    fn new(env: ANIEnv<'local>) -> Self {
        Self {
            env,
            should_detach: true,
        }
    }

    /// AttachGuard created with this method will not detach current thread on drop, which is
    /// the case for nested attaches.
    fn new_nested(env: ANIEnv<'local>) -> Self {
        Self {
            env,
            should_detach: false,
        }
    }
}

impl<'local> Deref for AttachGuard<'local> {
    type Target = ANIEnv<'local>;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl DerefMut for AttachGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}

impl Drop for AttachGuard<'_> {
    fn drop(&mut self) {
        if self.should_detach {
            InternalAttachGuard::clear_tls();
        }
    }
}

#[derive(Debug)]
struct InternalAttachGuard {
    ani_vm: AniVM,
    thread: Thread,
}

impl InternalAttachGuard {
    fn new(ani_vm: AniVM) -> Self {
        Self {
            ani_vm,
            thread: current(),
        }
    }

    /// Stores guard in thread local storage.
    fn fill_tls(guard: InternalAttachGuard) {
        THREAD_ATTACH_GUARD.with(move |f| {
            *f.borrow_mut() = Some(guard);
        });
    }

    /// Clears thread local storage, dropping the InternalAttachGuard and causing detach of
    /// the current thread.
    fn clear_tls() {
        THREAD_ATTACH_GUARD.with(move |f| {
            *f.borrow_mut() = None;
        });
    }

    unsafe fn attach_current_thread(&self) -> Result<*mut sys::ani_env> {
        let mut env_ptr: *mut sys::ani_env = ptr::null_mut();
        let status = ani_vm_call_unchecked!(
            self.ani_vm,
            AttachCurrentThread,
            ptr::null(),
            sys::ANI_VERSION_1,
            &mut env_ptr
        );
        ani_status_to_result(status)?;

        ATTACHED_THREADS.fetch_add(1, Ordering::SeqCst);

        debug!(
            "Attached thread {} ({:?}). {} threads attached",
            self.thread.name().unwrap_or_default(),
            self.thread.id(),
            ATTACHED_THREADS.load(Ordering::SeqCst)
        );

        Ok(env_ptr)
    }

    fn detach(&mut self) -> Result<()> {
        unsafe {
            let status = ani_vm_call_unchecked!(self.ani_vm, DetachCurrentThread);
            ani_status_to_result(status)?;
        }
        ATTACHED_THREADS.fetch_sub(1, Ordering::SeqCst);
        debug!(
            "Detached thread {} ({:?}). {} threads remain attached",
            self.thread.name().unwrap_or_default(),
            self.thread.id(),
            ATTACHED_THREADS.load(Ordering::SeqCst)
        );

        Ok(())
    }
}

impl Drop for InternalAttachGuard {
    fn drop(&mut self) {
        if let Err(e) = self.detach() {
            error!(
                "Error detaching current thread: {:#?}\nThread {} id={:?}",
                e,
                self.thread.name().unwrap_or_default(),
                self.thread.id(),
            );
        }
    }
}

