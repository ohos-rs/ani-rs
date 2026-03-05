//! Registration Function Generation
//!
//! Generates functions for registering native functions with ANI.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Generate class method registration function
pub fn generate_class_register(
    register_name: &Ident,
    class_descriptor: &str,
    is_static: bool,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(_env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            ::ani::module_register::queue_class_binding(
                #class_descriptor,
                #is_static,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_name as *const std::os::raw::c_void,
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
        pub unsafe extern "C" fn #register_name(_env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            ::ani::module_register::queue_namespace_binding(
                #namespace_descriptor,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_name as *const std::os::raw::c_void,
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
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(_env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            ::ani::module_register::queue_module_binding(
                #module_descriptor,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_name as *const std::os::raw::c_void,
            )
        }
    }
}
