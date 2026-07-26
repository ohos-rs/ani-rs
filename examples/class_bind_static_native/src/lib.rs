//! Class static native binding example.

use ani::prelude::*;
use ani_derive::ani;

unsafe extern "C" fn native_answer(
    _env: *mut ani::sys::ani_env,
    _cls: ani::sys::ani_class,
) -> ani::sys::ani_int {
    43
}

#[ani]
pub fn bind_static_natives(env: &Env<'_>, cls: AniClass<'_>) -> Result<()> {
    let methods = [native_function(
        "answer\0",
        ":i\0",
        native_answer as *const std::ffi::c_void,
    )];
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
