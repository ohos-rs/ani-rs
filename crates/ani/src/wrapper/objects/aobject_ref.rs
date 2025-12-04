use crate::sys::aobject;

#[cfg(doc)]
use crate::objects::GlobalRef;

use super::AObject;

/// A trait for types that represents an ANI reference (could be local, global or
/// weak global as well as wrapper types like [`AutoLocal`] and [`GlobalRef`])
///
///
/// This makes it possible for APIs like [`ANIEnv::new_global_ref`] to be given
/// a non-static local reference type like [`AString<'local>`] (or an
/// [`AutoLocal`] wrapper) and return a [`GlobalRef`] that is instead
/// parameterized by [`AString<'static>`].
pub trait AObjectRef: Sized {
    /// The generic associated [`Self::Kind`] type corresponds to the underlying
    /// class type (such as [`AObject`] or [`AString`]), parameterized by the
    /// lifetime that indicates whether the type holds a global reference
    /// (`'static`) or a local reference that's tied to an ANI stack frame.
    type Kind<'local>: AObjectRef + Default + Into<AObject<'local>> + AsRef<AObject<'local>>;

    /// The associated `GlobalKind` type should be equivalent to
    /// `Kind<'static>`, with the additional bound that ensures the type is
    /// `Send + Sync`
    type GlobalKind: AObjectRef
        + Default
        + Into<AObject<'static>>
        + AsRef<AObject<'static>>
        + Send
        + Sync;

    /// Returns the underlying, raw [`crate::sys::aobject`] reference.
    fn as_raw(&self) -> aobject;

    /// Returns `true` if this is a `null` object reference
    fn is_null(&self) -> bool {
        self.as_raw().is_null()
    }

    /// Returns `null` reference based on [`Self::Kind`]
    fn null<'any>() -> Self::Kind<'any> {
        Self::Kind::default()
    }

    /// Returns a new reference type based on [`Self::Kind`] for the given `local_ref` that is
    /// tied to the ANI stack frame for the given lifetime.
    ///
    /// # Safety
    ///
    /// The given lifetime must associated with an AttachGuard or an ANIEnv and represent an
    /// ANI stack frame.
    ///
    /// There must not be no other wrapper for the given `local_ref` reference (unless it is
    /// `null`)
    ///
    /// You are responsible to knowing that `Self::Kind` is a suitable wrapper type for the
    /// given `local_ref` reference. E.g. because the `local_ref` came from an `into_raw`
    /// call from the same type.
    ///
    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env>;

    /// Returns a (`'static`) reference type based on [`Self::GlobalKind`] for the given `global_ref`.
    ///
    /// # Safety
    ///
    /// There must not be no other wrapper for the given `global_ref` reference (unless it is
    /// `null`)
    ///
    /// You are responsible to knowing that `Self::GlobalKind` is a suitable wrapper type for the
    /// given `global_ref` reference. E.g. because the `global_ref` came from an `into_raw`
    /// call from the same type.
    ///
    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind;
}

impl<T> AObjectRef for &T
where
    T: AObjectRef,
{
    type Kind<'local> = T::Kind<'local>;
    type GlobalKind = T::GlobalKind;

    fn as_raw(&self) -> aobject {
        (*self).as_raw()
    }

    unsafe fn from_local_raw<'env>(local_ref: aobject) -> Self::Kind<'env> {
        T::from_local_raw(local_ref)
    }

    unsafe fn from_global_raw(global_ref: aobject) -> Self::GlobalKind {
        T::from_global_raw(global_ref)
    }
}

// Removed JObjectRef type alias - use AObjectRef trait directly

