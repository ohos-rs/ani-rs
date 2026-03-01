//! ANI VM Wrapper
//!
//! Provides a safe wrapper for `ani_vm`, similar to `JavaVM` in jni-rs.

use std::ffi::{CString, c_void};
use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::env::Env;
use crate::error::{Error, Result, Status, check_status};
use crate::sys;

/// VM creation/attach options.
///
/// This type owns all option strings so the underlying C pointers remain valid
/// while the options are used in VM calls.
#[derive(Default, Debug)]
pub struct VmOptions {
    options: Vec<CString>,
    raw_options: Vec<sys::ani_option>,
}

impl VmOptions {
    /// Creates an empty options set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns number of options.
    pub fn len(&self) -> usize {
        self.raw_options.len()
    }

    /// Returns whether no options are set.
    pub fn is_empty(&self) -> bool {
        self.raw_options.is_empty()
    }

    /// Adds a string option with `extra = null`.
    pub fn push(&mut self, option: &str) -> Result<&mut Self> {
        self.push_with_extra(option, ptr::null_mut())
    }

    /// Adds a string option with an extra pointer.
    pub fn push_with_extra(&mut self, option: &str, extra: *mut c_void) -> Result<&mut Self> {
        let option = CString::new(option)
            .map_err(|_| Error::new(Status::InvalidArgs, "Option contains interior null"))?;
        let option_ptr = option.as_ptr();
        self.options.push(option);
        self.raw_options.push(sys::ani_option {
            option: option_ptr,
            extra,
        });
        Ok(self)
    }

    #[inline]
    fn as_ani_options(&self) -> sys::ani_options {
        sys::ani_options {
            nr_options: self.raw_options.len(),
            options: if self.raw_options.is_empty() {
                ptr::null()
            } else {
                self.raw_options.as_ptr()
            },
        }
    }
}

/// ANI VM handle.
///
/// This is the VM-level entry point, analogous to `JavaVM` in jni-rs.
///
/// `AniVm` is `Send + Sync`, while `Env` remains thread-affine.
#[repr(transparent)]
pub struct AniVm {
    raw: *mut sys::ani_vm,
}

impl std::fmt::Debug for AniVm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AniVm").field(&self.raw).finish()
    }
}

impl AniVm {
    /// Creates a VM using default version (`ANI_VERSION_1`) and no options.
    pub fn create() -> Result<Self> {
        Self::create_with_version(sys::ANI_VERSION_1)
    }

    /// Creates a VM with an explicit version and no options.
    pub fn create_with_version(version: u32) -> Result<Self> {
        let mut raw: *mut sys::ani_vm = ptr::null_mut();
        let status = unsafe { sys::ANI_CreateVM(ptr::null(), version, &mut raw) };
        check_status(status)?;
        unsafe { Self::from_raw(raw) }
    }

    /// Creates a VM with explicit options and version.
    pub fn create_with_options(options: &VmOptions, version: u32) -> Result<Self> {
        let mut raw: *mut sys::ani_vm = ptr::null_mut();
        let raw_options = options.as_ani_options();
        let status = unsafe { sys::ANI_CreateVM(&raw_options, version, &mut raw) };
        check_status(status)?;
        unsafe { Self::from_raw(raw) }
    }

    /// Returns all currently created VMs.
    pub fn get_created_vms() -> Result<Vec<Self>> {
        let mut count: sys::ani_size = 0;
        let status = unsafe { sys::ANI_GetCreatedVMs(ptr::null_mut(), 0, &mut count) };
        check_status(status)?;

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut raw_vms: Vec<*mut sys::ani_vm> = vec![ptr::null_mut(); count];
        let mut actual_count = count;
        let status = unsafe {
            sys::ANI_GetCreatedVMs(raw_vms.as_mut_ptr(), raw_vms.len(), &mut actual_count)
        };
        check_status(status)?;

        let actual_count = actual_count.min(raw_vms.len());
        raw_vms.truncate(actual_count);

        let mut vms = Vec::with_capacity(actual_count);
        for raw in raw_vms {
            if !raw.is_null() {
                vms.push(unsafe { Self::from_raw_unchecked(raw) });
            }
        }
        Ok(vms)
    }

    /// Creates a VM handle from a raw pointer.
    ///
    /// # Safety
    ///
    /// Caller must ensure pointer is valid.
    pub unsafe fn from_raw(raw: *mut sys::ani_vm) -> Result<Self> {
        if raw.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Null pointer: ani_vm"));
        }
        Ok(Self { raw })
    }

    /// Creates a VM handle from a raw pointer without null check.
    ///
    /// # Safety
    ///
    /// Caller must ensure pointer is non-null and valid.
    pub unsafe fn from_raw_unchecked(raw: *mut sys::ani_vm) -> Self {
        Self { raw }
    }

    /// Returns the raw `ani_vm*`.
    pub fn as_raw(&self) -> *mut sys::ani_vm {
        self.raw
    }

    /// Returns whether current thread is already attached to this VM.
    pub fn is_current_thread_attached(&self) -> bool {
        self.get_env().is_ok()
    }

    /// Gets `Env` for current thread using default version (`ANI_VERSION_1`).
    ///
    /// Returns error when current thread is not attached.
    pub fn get_env<'vm>(&'vm self) -> Result<Env<'vm>> {
        self.get_env_with_version(sys::ANI_VERSION_1)
    }

    /// Gets `Env` for current thread using a specific version.
    ///
    /// Returns error when current thread is not attached.
    pub fn get_env_with_version<'vm>(&'vm self, version: u32) -> Result<Env<'vm>> {
        let mut env: *mut sys::ani_env = ptr::null_mut();
        let status = unsafe {
            let api = &*(*self.raw);
            (api.GetEnv.unwrap())(self.raw, version, &mut env)
        };
        check_status(status)?;
        unsafe { Env::from_raw(env) }
    }

    /// Attaches current thread and returns `Env` using default version (`ANI_VERSION_1`).
    pub fn attach_current_thread<'vm>(&'vm self) -> Result<Env<'vm>> {
        self.attach_current_thread_with_version(sys::ANI_VERSION_1)
    }

    /// Attaches current thread permanently and returns `Env`.
    ///
    /// Permanent attachment means thread detachment must be handled manually via
    /// [`detach_current_thread`](Self::detach_current_thread).
    pub fn attach_current_thread_permanently<'vm>(&'vm self) -> Result<Env<'vm>> {
        self.attach_current_thread()
    }

    /// Attaches current thread permanently with explicit version.
    ///
    /// Permanent attachment means thread detachment must be handled manually via
    /// [`detach_current_thread`](Self::detach_current_thread).
    pub fn attach_current_thread_permanently_with_version<'vm>(
        &'vm self,
        version: u32,
    ) -> Result<Env<'vm>> {
        self.attach_current_thread_with_version(version)
    }

    /// Attaches current thread permanently with options and explicit version.
    ///
    /// Permanent attachment means thread detachment must be handled manually via
    /// [`detach_current_thread`](Self::detach_current_thread).
    pub fn attach_current_thread_permanently_with_options<'vm>(
        &'vm self,
        options: &VmOptions,
        version: u32,
    ) -> Result<Env<'vm>> {
        self.attach_current_thread_with_options(options, version)
    }

    /// Attaches current thread and returns an [`AttachGuard`].
    ///
    /// The guard automatically detaches the thread when dropped.
    pub fn attach_current_thread_scoped<'vm>(&'vm self) -> Result<AttachGuard<'vm>> {
        let env = self.attach_current_thread()?;
        Ok(AttachGuard {
            vm: self,
            env: Some(env),
            detach_on_drop: true,
        })
    }

    /// Attaches current thread with options and returns an [`AttachGuard`].
    ///
    /// The guard automatically detaches the thread when dropped.
    pub fn attach_current_thread_scoped_with_options<'vm>(
        &'vm self,
        options: &VmOptions,
        version: u32,
    ) -> Result<AttachGuard<'vm>> {
        let env = self.attach_current_thread_with_options(options, version)?;
        Ok(AttachGuard {
            vm: self,
            env: Some(env),
            detach_on_drop: true,
        })
    }

    /// Executes closure with an attached environment.
    ///
    /// If the current thread is already attached, closure uses existing env
    /// and no detach is performed.
    /// If detached, this method attaches the thread for the duration of closure
    /// and detaches automatically.
    pub fn with_attached<'vm, R>(&'vm self, f: impl FnOnce(&Env<'vm>) -> Result<R>) -> Result<R> {
        match self.get_env() {
            Ok(env) => f(&env),
            Err(_) => {
                let guard = self.attach_current_thread_scoped()?;
                f(guard.env())
            }
        }
    }

    /// Executes closure with an attached environment using explicit options/version.
    ///
    /// If already attached for this version, options are ignored and existing env is used.
    /// If detached, thread is attached for closure duration and detached automatically.
    pub fn with_attached_with_options<'vm, R>(
        &'vm self,
        options: &VmOptions,
        version: u32,
        f: impl FnOnce(&Env<'vm>) -> Result<R>,
    ) -> Result<R> {
        match self.get_env_with_version(version) {
            Ok(env) => f(&env),
            Err(_) => {
                let guard = self.attach_current_thread_scoped_with_options(options, version)?;
                f(guard.env())
            }
        }
    }

    /// Attaches current thread and returns `Env` with explicit version.
    pub fn attach_current_thread_with_version<'vm>(&'vm self, version: u32) -> Result<Env<'vm>> {
        let mut env: *mut sys::ani_env = ptr::null_mut();
        let status = unsafe {
            let api = &*(*self.raw);
            (api.AttachCurrentThread.unwrap())(self.raw, ptr::null(), version, &mut env)
        };
        check_status(status)?;
        unsafe { Env::from_raw(env) }
    }

    /// Attaches current thread with options and explicit version.
    pub fn attach_current_thread_with_options<'vm>(
        &'vm self,
        options: &VmOptions,
        version: u32,
    ) -> Result<Env<'vm>> {
        let mut env: *mut sys::ani_env = ptr::null_mut();
        let raw_options = options.as_ani_options();
        let status = unsafe {
            let api = &*(*self.raw);
            (api.AttachCurrentThread.unwrap())(self.raw, &raw_options, version, &mut env)
        };
        check_status(status)?;
        unsafe { Env::from_raw(env) }
    }

    /// Detaches current thread from VM.
    pub fn detach_current_thread(&self) -> Result<()> {
        let status = unsafe {
            let api = &*(*self.raw);
            (api.DetachCurrentThread.unwrap())(self.raw)
        };
        check_status(status)
    }

    /// Destroys the VM.
    ///
    /// # Safety
    ///
    /// VM destruction is a global operation and must only be done when all
    /// attached threads and VM resources are in a safe state.
    pub unsafe fn destroy(self) -> Result<()> {
        let status = unsafe {
            let api = &*(*self.raw);
            (api.DestroyVM.unwrap())(self.raw)
        };
        check_status(status)
    }
}

unsafe impl Send for AniVm {}
unsafe impl Sync for AniVm {}

/// Scoped thread attachment guard.
///
/// When dropped, this guard detaches the current thread from VM if it attached
/// the thread.
pub struct AttachGuard<'vm> {
    vm: &'vm AniVm,
    env: Option<Env<'vm>>,
    detach_on_drop: bool,
}

impl<'vm> AttachGuard<'vm> {
    /// Returns inner environment.
    pub fn env(&self) -> &Env<'vm> {
        self.env
            .as_ref()
            .expect("AttachGuard internal env must exist")
    }

    /// Consumes guard and returns environment without detaching on drop.
    ///
    /// After calling this method, caller is responsible for detachment.
    pub fn into_env(mut self) -> Env<'vm> {
        self.detach_on_drop = false;
        self.env
            .take()
            .expect("AttachGuard internal env must exist")
    }

    /// Explicitly detaches current thread now.
    pub fn detach(mut self) -> Result<()> {
        self.detach_on_drop = false;
        self.vm.detach_current_thread()
    }
}

impl<'vm> Deref for AttachGuard<'vm> {
    type Target = Env<'vm>;

    fn deref(&self) -> &Self::Target {
        self.env()
    }
}

impl<'vm> DerefMut for AttachGuard<'vm> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.env
            .as_mut()
            .expect("AttachGuard internal env must exist")
    }
}

impl Drop for AttachGuard<'_> {
    fn drop(&mut self) {
        if self.detach_on_drop {
            let _ = self.vm.detach_current_thread();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_options_push_and_len() {
        let mut opts = VmOptions::new();
        assert!(opts.is_empty());
        opts.push("--foo").expect("push option");
        opts.push_with_extra("--bar", ptr::null_mut())
            .expect("push option with extra");
        assert_eq!(opts.len(), 2);
        assert!(!opts.is_empty());
    }

    #[test]
    fn from_raw_null_is_error() {
        let result = unsafe { AniVm::from_raw(ptr::null_mut()) };
        assert!(result.is_err());
    }

    #[test]
    fn attach_guard_into_env_disables_auto_detach() {
        let vm = unsafe { AniVm::from_raw_unchecked(ptr::NonNull::dangling().as_ptr()) };
        let env = unsafe { Env::from_raw_unchecked(ptr::NonNull::dangling().as_ptr()) };
        let guard = AttachGuard {
            vm: &vm,
            env: Some(env),
            detach_on_drop: true,
        };
        let _env = guard.into_env();
    }
}
