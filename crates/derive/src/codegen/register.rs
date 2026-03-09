//! Registration Function Generation
//!
//! Generates functions for registering native functions with ANI.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterTarget {
    Module(String),
    Namespace(String),
    Class { descriptor: String, is_static: bool },
}

pub fn generate_register_call(
    target: &RegisterTarget,
    func_name: &str,
    signature: &str,
    wrapper_expr: TokenStream,
) -> TokenStream {
    match target {
        RegisterTarget::Module(module_descriptor) => quote! {
            ::ani::module_register::queue_module_binding(
                #module_descriptor,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_expr,
            )
        },
        RegisterTarget::Namespace(namespace_descriptor) => quote! {
            ::ani::module_register::queue_namespace_binding(
                #namespace_descriptor,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_expr,
            )
        },
        RegisterTarget::Class {
            descriptor,
            is_static,
        } => quote! {
            ::ani::module_register::queue_class_binding(
                #descriptor,
                #is_static,
                concat!(#func_name, "\0"),
                concat!(#signature, "\0"),
                #wrapper_expr,
            )
        },
    }
}

pub fn generate_register_fn(
    register_name: &Ident,
    target: &RegisterTarget,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    let register_call = generate_register_call(
        target,
        func_name,
        signature,
        quote! { #wrapper_name as *const std::os::raw::c_void },
    );
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(_env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            #register_call
        }
    }
}
