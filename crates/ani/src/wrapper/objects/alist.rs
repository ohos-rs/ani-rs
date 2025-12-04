use std::marker::PhantomData;

use crate::objects::AObject;

/// Wrapper for AObjects that implement `java/util/List`. Provides methods to get
/// and set entries and iterate over values.
#[allow(dead_code)]
pub struct AList<'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    internal: AObject<'local>,
    class: AObject<'other_local_1>,
    lifetime: PhantomData<&'obj_ref ()>,
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> ::std::ops::Deref
    for AList<'local, 'other_local_1, 'obj_ref>
{
    type Target = AObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref>
    From<AList<'local, 'other_local_1, 'obj_ref>> for AObject<'local>
{
    fn from(other: AList<'local, 'other_local_1, 'obj_ref>) -> AObject<'local> {
        other.internal
    }
}


