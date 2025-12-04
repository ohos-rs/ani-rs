use crate::objects::JObject;

use std::marker::PhantomData;

/// Wrapper for objects that implement List interface.
///
/// Note: This is a placeholder for ANI compatibility. Full implementation
/// requires additional ANI method call support.
pub struct JList<'local, 'other_local_1: 'obj_ref, 'obj_ref> {
    internal: &'obj_ref JObject<'other_local_1>,
    _lifetime: PhantomData<&'local ()>,
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> AsRef<JList<'local, 'other_local_1, 'obj_ref>>
    for JList<'local, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JList<'local, 'other_local_1, 'obj_ref> {
        self
    }
}

impl<'other_local_1: 'obj_ref, 'obj_ref> AsRef<JObject<'other_local_1>>
    for JList<'_, 'other_local_1, 'obj_ref>
{
    fn as_ref(&self) -> &JObject<'other_local_1> {
        self.internal
    }
}

impl<'local, 'other_local_1: 'obj_ref, 'obj_ref> JList<'local, 'other_local_1, 'obj_ref> {
    /// Create a list wrapper from an object reference.
    pub fn new(obj: &'obj_ref JObject<'other_local_1>) -> Self {
        Self {
            internal: obj,
            _lifetime: PhantomData,
        }
    }

    /// Returns the internal object reference.
    pub fn as_obj(&self) -> &JObject<'other_local_1> {
        self.internal
    }
}
