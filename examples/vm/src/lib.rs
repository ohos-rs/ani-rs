//! VM Example - Demonstrates AniVm API.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn build_vm_options_count() -> Result<i32> {
    let mut options = VmOptions::new();
    options.push("--log-level=debug")?;
    options.push("--enable-ark-aot=false")?;
    Ok(options.len() as i32)
}

#[ani]
pub fn query_vm_version(env: &Env<'_>) -> Result<u32> {
    let vm = env.get_vm()?;
    vm.with_attached(|inner_env| inner_env.get_version())
}

#[ani]
pub fn query_vm_version_with_closure(env: &Env<'_>) -> Result<u32> {
    let vm = env.get_vm()?;
    vm.with_attached(|inner_env| inner_env.get_version())
}

#[ani]
pub fn query_vm_version_with_guard(env: &Env<'_>) -> Result<u32> {
    let vm = env.get_vm()?;
    let guard = vm.attach_current_thread_scoped()?;
    guard.env().get_version()
}

#[ani]
pub fn query_vm_version_with_permanent_attach(env: &Env<'_>) -> Result<u32> {
    let vm = env.get_vm()?;
    let attached_env = vm.attach_current_thread_permanently()?;
    attached_env.get_version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_api_signatures_compile() {
        let _ = build_vm_options_count;
        let _ = query_vm_version;
        let _ = query_vm_version_with_closure;
        let _ = query_vm_version_with_guard;
        let _ = query_vm_version_with_permanent_attach;
    }
}
