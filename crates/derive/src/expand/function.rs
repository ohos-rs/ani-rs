//! Function Expansion
//!
//! Expands `#[ani]` macro for functions.

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn};

use crate::codegen::{
    generate_class_register, generate_module_register, generate_namespace_register,
    generate_wrapper,
};
use crate::parser::{BindgenAttrs, InitAttrs};
use crate::types::{
    class_to_descriptor, generate_fn_signature, module_to_descriptor, namespace_to_descriptor,
};

/// Expand `#[ani]` for functions
pub fn expand_function(attrs: BindgenAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    if attrs.skip {
        return quote! { #prepare #func };
    }

    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();

    // Determine ArkTS function name (camelCase)
    let ets_name = attrs
        .name
        .clone()
        .unwrap_or_else(|| func_name.to_string().to_case(Case::Camel));

    // Check if function has self parameter (instance method)
    let has_self = func
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

    let is_static = attrs.is_static || !has_self;
    let is_class_method = attrs.class.is_some();

    // Generate signature
    let signature = attrs
        .signature
        .clone()
        .unwrap_or_else(|| generate_fn_signature(&func.sig, has_self || is_class_method));

    // Generate wrapper function name
    let wrapper_name = format_ident!("__ani_native_{}", func_name);
    let register_name = format_ident!("__ani_register_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_{}", func_name);

    // Generate wrapper function
    let wrapper = generate_wrapper(&func, &wrapper_name, is_class_method, is_static);

    // Generate registration function based on target
    let register_fn =
        generate_register_fn(&attrs, &register_name, &ets_name, &signature, &wrapper_name);

    // Generate ctor auto-registration function
    let ctor_fn = quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[::ani::ctor::ctor(crate_path = ::ani::ctor)]
        fn #ctor_register_name() {
            ::ani::module_register::register_module_export(
                #func_name_str,
                #register_name
            );
        }
    };

    quote! {
        #prepare
        #func
        #wrapper
        #register_fn
        #ctor_fn
    }
}

/// Generate appropriate registration function based on attributes
fn generate_register_fn(
    attrs: &BindgenAttrs,
    register_name: &proc_macro2::Ident,
    ets_name: &str,
    signature: &str,
    wrapper_name: &proc_macro2::Ident,
) -> TokenStream {
    if let Some(ref class) = attrs.class {
        let descriptor = class_to_descriptor(class);
        generate_class_register(
            register_name,
            &descriptor,
            ets_name,
            signature,
            wrapper_name,
        )
    } else if let Some(ref ns) = attrs.namespace {
        let descriptor = namespace_to_descriptor(ns);
        generate_namespace_register(
            register_name,
            &descriptor,
            ets_name,
            signature,
            wrapper_name,
        )
    } else {
        let descriptor = attrs
            .module
            .as_ref()
            .map(|m| module_to_descriptor(m))
            .unwrap_or_default();
        generate_module_register(
            register_name,
            &descriptor,
            ets_name,
            signature,
            wrapper_name,
        )
    }
}

/// Expand `#[ani(init)]` for initialization functions
pub fn expand_init(_attrs: InitAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    let func_name = &func.sig.ident;
    let init_fn_name = format_ident!("__ani_init_{}", func_name);

    quote! {
        #prepare
        #func

        #[doc(hidden)]
        pub fn #init_fn_name() {
            #func_name();
        }
    }
}
