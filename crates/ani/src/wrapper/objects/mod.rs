// wrappers around ani pointer types that add lifetimes and other functionality.
mod avalue;
pub use self::avalue::*;

mod amethodid;
pub use self::amethodid::*;

mod astaticmethodid;
pub use self::astaticmethodid::*;

mod afieldid;
pub use self::afieldid::*;

mod astaticfieldid;
pub use self::astaticfieldid::*;

mod aobject_ref;
pub use self::aobject_ref::*;

mod aobject;
pub use self::aobject::*;

mod athrowable;
pub use self::athrowable::*;

mod aclass;
pub use self::aclass::*;

mod astring;
pub use self::astring::*;

mod amap;
pub use self::amap::*;

mod alist;
pub use self::alist::*;

mod abytebuffer;
pub use self::abytebuffer::*;

// For storing a reference to a java object
mod global_ref;
pub use self::global_ref::*;

mod weak_ref;
pub use self::weak_ref::*;

// For automatic local ref deletion
mod auto_local;
pub use self::auto_local::*;

mod release_mode;
pub use self::release_mode::*;

/// Object Array types
mod aobject_array;
pub use self::aobject_array::*;

/// Primitive Array types
mod aprimitive_array;
pub use self::aprimitive_array::*;

// For automatic pointer-based generic array release
mod auto_elements;
pub use self::auto_elements::*;

// For automatic pointer-based primitive array release
mod auto_elements_critical;
pub use self::auto_elements_critical::*;
