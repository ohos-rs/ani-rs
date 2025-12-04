use std::marker::PhantomData;

use crate::objects::AObject;

/// Wrapper for AObjects that implement `java/util/Map`. Provides methods to get
/// and set entries andடி iterate over key/value pairs.
///
/// Looks up the class and method ids on creation rather than for every method
/// call.
#[allow(dead_code)]
pub struct AMap<'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    internal: AObject<'local>,
    class: AObject<'other_local_1>,
    lifetime: PhantomData<&'obj_ref ()>,
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> ::std::ops::Deref
    for AMap<'local, 'other_local_1, 'obj_ref>
{
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref>
    From<AMap<'local, 'other_local_1, 'obj_ref>> for AObject<'local>
{
    fn from(other: AMap<'local, 'other_local_1, 'obj_ref>) -> AObject<'local> {
        other.internal
    }
}


