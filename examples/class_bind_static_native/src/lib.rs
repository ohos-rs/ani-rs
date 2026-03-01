//! Class static native binding example.

use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn bind_static_natives(env: &Env<'_>, cls: AniClass<'_>) -> Result<()> {
    let methods = [native_function("noop\0", "V:V\0", std::ptr::null())];
    env.bind_class_static_native_methods(&cls, &methods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = bind_static_natives;
    }
}
