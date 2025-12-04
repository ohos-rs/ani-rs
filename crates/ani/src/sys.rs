#![allow(non_camel_case_types)]

pub use ani_sys::*;

// Type aliases for JNI compatibility mapping
pub type jboolean = ani_boolean;
pub type jbyte = ani_byte;
pub type jchar = ani_char;
pub type jshort = ani_short;
pub type jint = ani_int;
pub type jlong = ani_long;
pub type jfloat = ani_float;
pub type jdouble = ani_double;
pub type jsize = ani_size;

// Reference types
pub type jobject = ani_ref;
pub type jclass = ani_class;
pub type jstring = ani_string;
pub type jarray = ani_array;
pub type jbooleanArray = ani_array_boolean;
pub type jbyteArray = ani_array_byte;
pub type jcharArray = ani_array_char;
pub type jshortArray = ani_array_short;
pub type jintArray = ani_array_int;
pub type jlongArray = ani_array_long;
pub type jfloatArray = ani_array_float;
pub type jdoubleArray = ani_array_double;
pub type jobjectArray = ani_array_ref;
pub type jthrowable = ani_error;
pub type jweak = ani_wref;

// ID types
pub type jmethodID = ani_method;
pub type jstaticMethodID = ani_static_method;
pub type jfieldID = ani_field;
pub type jstaticFieldID = ani_static_field;

// Environment and VM types
pub type JNIEnv = ani_env;
pub type JavaVM = ani_vm;
pub type JNINativeInterface_ = __ani_interaction_api;
pub type JNIInvokeInterface_ = __ani_vm_api;

// Value union - map JNI field names to ANI equivalents
// JNI: z=boolean, b=byte, c=char, s=short, i=int, j=long, f=float, d=double, l=object
// ANI: z=boolean, b=byte, c=char, s=short, i=int, l=long, f=float, d=double, r=ref
#[repr(C)]
#[derive(Copy, Clone)]
pub union jvalue {
    pub z: jboolean,  // maps to ANI z (boolean)
    pub b: jbyte,     // maps to ANI b (byte)  
    pub c: jchar,     // maps to ANI c (char)
    pub s: jshort,    // maps to ANI s (short)
    pub i: jint,      // maps to ANI i (int)
    pub j: jlong,     // maps to ANI l (long) - note: JNI uses 'j', ANI uses 'l'
    pub f: jfloat,    // maps to ANI f (float)
    pub d: jdouble,   // maps to ANI d (double)
    pub l: jobject,   // maps to ANI r (ref) - note: JNI uses 'l', ANI uses 'r'
}

// JNI constants
pub const JNI_OK: jint = ani_status_ANI_OK as jint;
pub const JNI_ERR: jint = ani_status_ANI_ERROR as jint;
pub const JNI_EDETACHED: jint = -2;
pub const JNI_EVERSION: jint = ani_status_ANI_INVALID_VERSION as jint;
pub const JNI_ENOMEM: jint = ani_status_ANI_OUT_OF_MEMORY as jint;
pub const JNI_EEXIST: jint = ani_status_ANI_ALREADY_BINDED as jint;
pub const JNI_EINVAL: jint = ani_status_ANI_INVALID_ARGS as jint;

pub const JNI_TRUE: jboolean = ANI_TRUE as jboolean;
pub const JNI_FALSE: jboolean = ANI_FALSE as jboolean;

// JNI array operation modes (not directly mapped in ANI)
pub const JNI_COMMIT: i32 = 1;
pub const JNI_ABORT: i32 = 2;

// Native method registration struct
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JNINativeMethod {
    pub name: *const std::os::raw::c_char,
    pub signature: *const std::os::raw::c_char,
    pub fnPtr: *mut std::os::raw::c_void,
}
