//! Host-only definitions for the two process-level ANI entry points.
//!
//! Conversion fuzz targets never create a VM, but linking the `ani` crate on a
//! non-OpenHarmony host may retain these references. Keeping the stubs inside
//! the fuzz binaries makes accidental execution fail deterministically without
//! changing the library or its OpenHarmony linkage.

use ani::sys::{ani_options, ani_size, ani_status, ani_vm};

#[unsafe(no_mangle)]
unsafe extern "C" fn ANI_CreateVM(
    _options: *const ani_options,
    _version: u32,
    _result: *mut *mut ani_vm,
) -> ani_status {
    panic!("host conversion fuzz target must not create an ANI VM")
}

#[unsafe(no_mangle)]
unsafe extern "C" fn ANI_GetCreatedVMs(
    _vms_buffer: *mut *mut ani_vm,
    _vms_buffer_length: ani_size,
    _result: *mut ani_size,
) -> ani_status {
    panic!("host conversion fuzz target must not query ANI VMs")
}
