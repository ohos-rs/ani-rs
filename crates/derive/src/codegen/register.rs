//! Registration Function Generation
//!
//! Generates functions for registering native functions with ANI.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Generate class method registration function
pub fn generate_class_register(
    register_name: &Ident,
    class_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            let mut cls: ani::sys::ani_class = std::ptr::null_mut();
            let class_name = concat!(#class_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindClass.unwrap())(
                env,
                class_name.as_ptr() as *const std::os::raw::c_char,
                &mut cls
            );

            if status != ani::sys::ani_status_ANI_OK {
                return status;
            }

            let methods = [
                ani::sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            (api.Class_BindNativeMethods.unwrap())(
                env,
                cls,
                methods.as_ptr(),
                methods.len()
            )
        }
    }
}

/// Generate namespace function registration function
pub fn generate_namespace_register(
    register_name: &Ident,
    namespace_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            let mut ns: ani::sys::ani_namespace = std::ptr::null_mut();
            let ns_name = concat!(#namespace_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindNamespace.unwrap())(
                env,
                ns_name.as_ptr() as *const std::os::raw::c_char,
                &mut ns
            );

            if status != ani::sys::ani_status_ANI_OK {
                return status;
            }

            let functions = [
                ani::sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            (api.Namespace_BindNativeFunctions.unwrap())(
                env,
                ns,
                functions.as_ptr(),
                functions.len()
            )
        }
    }
}

/// Generate module function registration function
pub fn generate_module_register(
    register_name: &Ident,
    module_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    let find_module = if module_descriptor.is_empty() {
        quote! {
            let mut module: ani::sys::ani_module = std::ptr::null_mut();
        }
    } else {
        quote! {
            let mut module: ani::sys::ani_module = std::ptr::null_mut();
            let module_name = concat!(#module_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindModule.unwrap())(
                env,
                module_name.as_ptr() as *const std::os::raw::c_char,
                &mut module
            );

            if status != ani::sys::ani_status_ANI_OK {
                return status;
            }
        }
    };

    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            #find_module

            let functions = [
                ani::sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            let api = &*(*env);
            (api.Module_BindNativeFunctions.unwrap())(
                env,
                module,
                functions.as_ptr(),
                functions.len()
            )
        }
    }
}
