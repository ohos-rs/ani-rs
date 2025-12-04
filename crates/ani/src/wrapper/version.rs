use crate::sys;

/// ANI Version
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
pub struct ANIVersion {
    ver: u32,
}

impl ANIVersion {
    /// ANI Version 1
    pub const V1: Self = ANIVersion {
        ver: sys::ANI_VERSION_1,
    };

    /// Return a version from a raw version constant
    pub fn new(ver: u32) -> Self {
        Self::from(ver)
    }

    /// Get the raw version value
    pub fn raw(&self) -> u32 {
        self.ver
    }
}

impl From<u32> for ANIVersion {
    fn from(value: u32) -> Self {
        Self { ver: value }
    }
}

impl From<ANIVersion> for u32 {
    fn from(val: ANIVersion) -> Self {
        val.ver
    }
}
