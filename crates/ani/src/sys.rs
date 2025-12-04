#![allow(non_camel_case_types)]
#![allow(missing_docs)]

pub use ani_sys::*;

// Type aliases - ANI native types
pub type aboolean = ani_boolean;
pub type abyte = ani_byte;
pub type achar = ani_char;
pub type ashort = ani_short;
pub type aint = ani_int;
pub type along = ani_long;
pub type afloat = ani_float;
pub type adouble = ani_double;
pub type asize = ani_size;

// Reference types
pub type aobject = ani_ref;
pub type aclass = ani_class;
pub type astring = ani_string;
pub type aarray = ani_array;
pub type abooleanArray = ani_array_boolean;
pub type abyteArray = ani_array_byte;
pub type acharArray = ani_array_char;
pub type ashortArray = ani_array_short;
pub type aintArray = ani_array_int;
pub type alongArray = ani_array_long;
pub type afloatArray = ani_array_float;
pub type adoubleArray = ani_array_double;
pub type aobjectArray = ani_array_ref;
pub type athrowable = ani_error;
pub type aweak = ani_wref;

// ID types
pub type amethodID = ani_method;
pub type astaticMethodID = ani_static_method;
pub type afieldID = ani_field;
pub type astaticFieldID = ani_static_field;

// Environment and VM types
pub type ANIEnvRaw = ani_env;
pub type AniVMRaw = ani_vm;
pub type ANINativeInterface_ = __ani_interaction_api;
pub type ANIInvokeInterface_ = __ani_vm_api;

// Value union
#[repr(C)]
#[derive(Copy, Clone)]
pub union avalue {
    pub z: aboolean,  // boolean
    pub b: abyte,     // byte  
    pub c: achar,     // char
    pub s: ashort,    // short
    pub i: aint,      // int
    pub j: along,     // long
    pub f: afloat,    // float
    pub d: adouble,   // double
    pub l: aobject,   // object reference
}

// ANI constants
pub const ANI_OK: aint = ani_status_ANI_OK as aint;
pub const ANI_ERR: aint = ani_status_ANI_ERROR as aint;
pub const ANI_EDETACHED: aint = -2;
pub const ANI_EVERSION: aint = ani_status_ANI_INVALID_VERSION as aint;
pub const ANI_ENOMEM: aint = ani_status_ANI_OUT_OF_MEMORY as aint;
pub const ANI_EEXIST: aint = ani_status_ANI_ALREADY_BINDED as aint;
pub const ANI_EINVAL: aint = ani_status_ANI_INVALID_ARGS as aint;

pub const ANI_TRUE_VAL: aboolean = ANI_TRUE as aboolean;
pub const ANI_FALSE_VAL: aboolean = ANI_FALSE as aboolean;

// Array operation modes
pub const ANI_COMMIT: i32 = 1;
pub const ANI_ABORT: i32 = 2;

// Native method registration struct
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ANINativeMethod {
    pub name: *const std::os::raw::c_char,
    pub signature: *const std::os::raw::c_char,
    pub fnPtr: *mut std::os::raw::c_void,
}
