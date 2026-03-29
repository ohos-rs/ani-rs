//! Generic / tuple / unit struct derive example.

use ani_derive::{ani, AniClass};

#[derive(Debug, Clone, PartialEq, Eq, AniClass)]
#[ani(class = "DeriveBox")]
pub struct DeriveBox<T> {
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, AniClass)]
#[ani(class = "DerivePair")]
pub struct DerivePair<T, U>(pub T, #[ani(property)] pub U);

#[derive(Debug, Clone, Copy, PartialEq, Eq, AniClass)]
#[ani(class = "DeriveMarker")]
pub struct DeriveMarker;

#[derive(Debug, Clone, PartialEq, Eq)]
#[ani(object = "ObjectBox")]
pub struct ObjectBox<T> {
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[ani(object = "ObjectPair")]
pub struct ObjectPair<T, U>(pub T, pub U);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[ani(object = "ObjectMarker")]
pub struct ObjectMarker;

#[ani]
pub fn make_derive_box(value: String) -> DeriveBox<String> {
    DeriveBox { value }
}

#[ani]
pub fn read_derive_box(value: DeriveBox<String>) -> String {
    value.value
}

#[ani]
pub fn make_derive_pair(left: String, right: String) -> DerivePair<String, String> {
    DerivePair(left, right)
}

#[ani]
pub fn describe_derive_pair(value: DerivePair<String, String>) -> String {
    format!("{}:{}", value.0, value.1)
}

#[ani]
pub fn marker_identity(value: DeriveMarker) -> bool {
    matches!(value, DeriveMarker)
}

#[ani]
pub fn make_object_box(value: String) -> ObjectBox<String> {
    ObjectBox { value }
}

#[ani]
pub fn describe_object_box(value: ObjectBox<String>) -> String {
    format!("box:{}", value.value)
}

#[ani]
pub fn make_object_pair(left: String, right: String) -> ObjectPair<String, String> {
    ObjectPair(left, right)
}

#[ani]
pub fn describe_object_pair(value: ObjectPair<String, String>) -> String {
    format!("{}:{}", value.0, value.1)
}

#[ani]
pub fn object_marker_identity(value: ObjectMarker) -> bool {
    matches!(value, ObjectMarker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_shapes_work_in_rust() {
        let derive_box = make_derive_box("box".to_string());
        assert_eq!(derive_box.value, "box");
        assert_eq!(read_derive_box(derive_box), "box");

        let derive_pair = make_derive_pair("ani".to_string(), "ark".to_string());
        assert_eq!(derive_pair.0, "ani");
        assert_eq!(derive_pair.1, "ark");
        assert_eq!(describe_derive_pair(derive_pair), "ani:ark");
        assert!(marker_identity(DeriveMarker));

        let object_box = make_object_box("box".to_string());
        assert_eq!(object_box.value, "box");
        assert_eq!(describe_object_box(object_box), "box:box");

        let object_pair = make_object_pair("left".to_string(), "ark".to_string());
        assert_eq!(object_pair.0, "left");
        assert_eq!(object_pair.1, "ark");
        assert_eq!(describe_object_pair(object_pair), "left:ark");
        assert!(object_marker_identity(ObjectMarker));
    }
}
