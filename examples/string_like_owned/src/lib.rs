//! Owned and borrowed string-like wrapper example.

use std::borrow::Cow;
use std::ffi::{CStr, OsStr, OsString};
use std::path::{Path, PathBuf};

use ani_derive::ani;

#[ani]
pub fn append_os_segment(path: OsString, tail: String) -> OsString {
    let mut path = PathBuf::from(path);
    path.push(tail);
    path.into_os_string()
}

#[ani]
pub fn append_boxed_segment(path: Box<Path>, tail: String) -> Box<Path> {
    let mut path = PathBuf::from(path.as_ref());
    path.push(tail);
    path.into_boxed_path()
}

#[ani]
pub fn append_cow_segment(path: Cow<'static, Path>, tail: String) -> Cow<'static, Path> {
    let mut path = path.into_owned();
    path.push(tail);
    Cow::Owned(path)
}

#[ani]
pub fn append_borrowed_path_segment(path: &Path, tail: &str) -> String {
    path.join(tail).to_string_lossy().into_owned()
}

#[ani]
pub fn borrowed_os_prefix(path: &OsStr, prefix: &str) -> bool {
    path.to_string_lossy().starts_with(prefix)
}

#[ani]
pub fn borrowed_c_str_len(value: &CStr) -> i32 {
    value.to_bytes().len() as i32
}

#[ani]
pub fn borrowed_path_literal() -> &'static Path {
    Path::new("borrowed/path")
}

#[ani]
pub fn borrowed_os_str_literal() -> &'static OsStr {
    OsStr::new("borrowed-os")
}

#[ani]
pub fn borrowed_c_str_literal() -> &'static CStr {
    c"borrowed-c"
}

/// Rust `char` is represented as a one-scalar ArkTS string, preserving
/// supplementary-plane Unicode values that do not fit in UTF-16 `char`.
#[ani]
pub fn char_identity(value: char) -> char {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_string_like_paths_roundtrip_in_rust() {
        let os = append_os_segment(OsString::from("root"), "tail".to_string());
        assert_eq!(PathBuf::from(os), PathBuf::from("root").join("tail"));

        let boxed = append_boxed_segment(PathBuf::from("left").into_boxed_path(), "right".into());
        assert_eq!(boxed.as_ref(), Path::new("left").join("right").as_path());

        let cow = append_cow_segment(Cow::Owned(PathBuf::from("cow")), "leaf".into());
        assert_eq!(cow.as_ref(), Path::new("cow").join("leaf").as_path());

        assert_eq!(
            append_borrowed_path_segment(Path::new("base"), "tail"),
            Path::new("base").join("tail").to_string_lossy()
        );
        assert!(borrowed_os_prefix(OsStr::new("prefix-value"), "prefix"));
        assert_eq!(borrowed_c_str_len(c"ffi"), 3);
        assert_eq!(borrowed_path_literal(), Path::new("borrowed/path"));
        assert_eq!(borrowed_os_str_literal(), OsStr::new("borrowed-os"));
        assert_eq!(borrowed_c_str_literal().to_str().unwrap(), "borrowed-c");
        assert_eq!(char_identity('🦀'), '🦀');
    }
}
