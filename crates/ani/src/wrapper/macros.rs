/// Directly calls an ANIEnv FFI function, nothing else
///
/// # Safety
///
/// When calling any function you must know that it's valid
/// for the current ANI version.
macro_rules! ani_call_unchecked {
    ( $anienv:expr, $name:tt $(, $args:expr )*) => {{
        // Safety: we know that the ANIEnv pointer can't be null, since that's
        // checked in `from_raw()`
        let env: *mut crate::sys::ani_env = $anienv.get_raw();
        let interface: *const crate::sys::__ani_interaction_api = *env;
        ((*interface).$name.unwrap())(env $(, $args)*)
    }};
}

/// Calls an ANIEnv function, then checks for status
///
/// Returns `Err` if the status indicates an error.
macro_rules! ani_call_check {
    ( $anienv:expr, $name:tt $(, $args:expr )* ) => ({
        let status = ani_call_unchecked!($anienv, $name $(, $args)*);
        if status != crate::sys::ani_status_ANI_OK {
            Err(crate::errors::Error::AniCall(crate::errors::ani_status_to_error(status)))
        } else {
            Ok(())
        }
    })
}

/// Calls an ANIEnv function with a result output parameter, then checks for status
///
/// Returns `Err` if the status indicates an error, otherwise returns the result.
macro_rules! ani_call_result {
    ( $anienv:expr, $name:tt, $result_type:ty $(, $args:expr )* ) => ({
        let mut result: $result_type = std::mem::zeroed();
        let status = ani_call_unchecked!($anienv, $name $(, $args)*, &mut result);
        if status != crate::sys::ani_status_ANI_OK {
            Err(crate::errors::Error::AniCall(crate::errors::ani_status_to_error(status)))
        } else {
            Ok(result)
        }
    })
}

/// Maps a pointer to either Ok(ptr) or Err(Error::NullPtr)
///
/// This makes it reasonably ergonomic to use `?` to early-exit with an `Err` in
/// case of `null` pointer arguments.
///
/// Unlike earlier macros this avoids using `return`, since that can result in
/// surprising control flow if the caller doesn't realize that a macro might
/// explicitly return from the current function.
macro_rules! null_check {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            Err($crate::errors::Error::NullPtr($ctx))
        } else {
            Ok($obj)
        }
    };
}

/// Directly calls an ANI VM function, nothing else
macro_rules! ani_vm_call_unchecked {
    ( $vm:expr, $name:tt $(, $args:expr )*) => {{
        // Safety: we know that the pointer can't be null, since that's
        // checked in `from_raw()`
        let vm: *mut crate::sys::ani_vm = $vm.get_raw();
        ((*(*vm)).$name.unwrap())(vm $(, $args)*)
    }};
}

/// Calls an ANI VM function, then checks for status
macro_rules! ani_vm_call_check {
    ( $vm:expr, $name:tt $(, $args:expr )* ) => ({
        let status = ani_vm_call_unchecked!($vm, $name $(, $args)*);
        if status != crate::sys::ani_status_ANI_OK {
            Err(crate::errors::Error::AniCall(crate::errors::ani_status_to_error(status)))
        } else {
            Ok(())
        }
    })
}

/// Calls an ANI VM function with a result output parameter, then checks for status
macro_rules! ani_vm_call_result {
    ( $vm:expr, $name:tt, $result_type:ty $(, $args:expr )* ) => ({
        let mut result: $result_type = std::mem::zeroed();
        let status = ani_vm_call_unchecked!($vm, $name $(, $args)*, &mut result);
        if status != crate::sys::ani_status_ANI_OK {
            Err(crate::errors::Error::AniCall(crate::errors::ani_status_to_error(status)))
        } else {
            Ok(result)
        }
    })
}
