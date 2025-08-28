/// ANI Version
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
pub struct ANIVersion {
    ver: u32,
}

impl ANIVersion {
    /// JNI Version 1.1
    pub const V1: Self = ANIVersion {
        ver: ani_sys::ANI_VERSION_1 as u32,
    };

    /// Return a version from a raw version constant like [`jni_sys::JNI_VERSION_1_2`]
    pub fn new(ver: ani_sys::ani_int) -> Self {
        Self::from(ver)
    }

    /// Get the major component of the version number
    pub fn major(&self) -> u16 {
        ((self.ver & 0x00ff0000) >> 16) as u16
    }

    /// Get the minor component of the version number
    pub fn minor(&self) -> u16 {
        (self.ver & 0xff) as u16
    }
}

impl From<jni_sys::jint> for JNIVersion {
    fn from(value: jni_sys::jint) -> Self {
        Self { ver: value as u32 }
    }
}

impl From<JNIVersion> for jni_sys::jint {
    fn from(val: JNIVersion) -> Self {
        val.ver as i32
    }
}

