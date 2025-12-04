use crate::sys::ani_method;

/// Wrapper around [`ani_method`] that implements `Send` + `Sync` since method IDs
/// are valid across threads (not tied to an `ANIEnv`).
///
/// There is no lifetime associated with these since they aren't garbage
/// collected like objects and their lifetime is not implicitly connected with
/// the scope in which they are queried.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take an [`ani_method`].
///
/// # Safety
///
/// According to the ANI spec method IDs may be invalidated when the
/// corresponding class is unloaded.
///
/// Since this constraint can't be encoded as a Rust lifetime, and to avoid the
/// excessive cost of having every Method ID be associated with a global
/// reference to the corresponding class then it is the developers
/// responsibility to ensure they hold some class reference for the lifetime of
/// cached method IDs.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct AStaticMethodID {
    internal: ani_method,
}

// Method IDs are valid across threads (not tied to an ANIEnv)
unsafe impl Send for AStaticMethodID {}
unsafe impl Sync for AStaticMethodID {}

impl AStaticMethodID {
    /// Creates a [`AStaticMethodID`] that wraps the given `raw` [`ani_method`]
    ///
    /// # Safety
    ///
    /// Expects a valid, non-`null` ID
    pub const unsafe fn from_raw(raw: ani_method) -> Self {
        Self { internal: raw }
    }

    /// Unwrap to the internal ani type.
    pub const fn into_raw(self) -> ani_method {
        self.internal
    }
}

impl AsRef<AStaticMethodID> for AStaticMethodID {
    fn as_ref(&self) -> &AStaticMethodID {
        self
    }
}

impl AsMut<AStaticMethodID> for AStaticMethodID {
    fn as_mut(&mut self) -> &mut AStaticMethodID {
        self
    }
}


