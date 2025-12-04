use crate::sys::ani_field;

/// Wrapper around [`ani_field`] that implements `Send` + `Sync` since field IDs
/// are valid across threads (not tied to an `ANIEnv`).
///
/// There is no lifetime associated with these since they aren't garbage
/// collected like objects and their lifetime is not implicitly connected with
/// the scope in which they are queried.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take an [`ani_field`].
///
/// # Safety
///
/// According to the ANI spec field IDs may be invalidated when the
/// corresponding class is unloaded.
///
/// Since this constraint can't be encoded as a Rust lifetime, and to avoid the
/// excessive cost of having every Field ID be associated with a global
/// reference to the corresponding class then it is the developers
/// responsibility to ensure they hold some class reference for the lifetime of
/// cached field IDs.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AFieldID {
    internal: ani_field,
}

// Field IDs are valid across threads (not tied to an ANIEnv)
unsafe impl Send for AFieldID {}
unsafe impl Sync for AFieldID {}

impl AFieldID {
    /// Creates a [`AFieldID`] that wraps the given `raw` [`ani_field`]
    ///
    /// # Safety
    ///
    /// Expects a valid, non-`null` ID
    pub const unsafe fn from_raw(raw: ani_field) -> Self {
        Self { internal: raw }
    }

    /// Unwrap to the internal ani type.
    pub const fn into_raw(self) -> ani_field {
        self.internal
    }
}

impl AsRef<AFieldID> for AFieldID {
    fn as_ref(&self) -> &AFieldID {
        self
    }
}

impl AsMut<AFieldID> for AFieldID {
    fn as_mut(&mut self) -> &mut AFieldID {
        self
    }
}


