//! Wrapper Function Generation
//!
//! Generates the `extern "C"` wrapper functions that bridge Rust and ANI.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, ReturnType};

use crate::types::{generate_param_conversions, generate_return_conversion, rust_type_to_ani_type};

/// Generate wrapper function for ANI binding
pub fn generate_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    is_class_method: bool,
    is_static: bool,
) -> TokenStream {
    let func_name = &func.sig.ident;
    let return_type = &func.sig.output;

    // Build wrapper parameters
    let wrapper_params = build_wrapper_params(func, is_class_method, is_static);

    // Collect user parameters (skip self)
    let user_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| !matches!(arg, FnArg::Receiver(_)))
        .collect();

    // Generate parameter conversions
    let conversions = generate_param_conversions(&user_params);

    // Generate call arguments
    let call_args = build_call_args(&user_params);

    // Generate function call
    let func_call = quote! {
        let result = #func_name(#(#call_args),*);
    };

    // Generate return value conversion
    let return_conversion = generate_return_conversion(return_type);

    // Generate return type
    let wrapper_return = build_wrapper_return(return_type);

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) #wrapper_return {
            #conversions
            #func_call
            #return_conversion
        }
    }
}

/// Build wrapper function parameters
fn build_wrapper_params(func: &ItemFn, is_class_method: bool, is_static: bool) -> Vec<TokenStream> {
    let mut params = vec![quote! { env: *mut ani::sys::ani_env }];

    // Class methods need extra this/class parameter
    if is_class_method {
        if is_static {
            params.push(quote! { _class: ani::sys::ani_class });
        } else {
            params.push(quote! { this: ani::sys::ani_object });
        }
    }

    // Add user parameters
    for (i, param) in func
        .sig
        .inputs
        .iter()
        .filter(|arg| !matches!(arg, FnArg::Receiver(_)))
        .enumerate()
    {
        if let FnArg::Typed(pat_type) = param {
            let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                pat_ident.ident.clone()
            } else {
                format_ident!("arg{}", i)
            };

            let ani_type = rust_type_to_ani_type(&pat_type.ty);
            params.push(quote! { #param_name: #ani_type });
        }
    }

    params
}

/// Build call arguments for invoking the original function
fn build_call_args(params: &[&FnArg]) -> Vec<TokenStream> {
    params
        .iter()
        .enumerate()
        .map(|(i, param)| {
            if let FnArg::Typed(pat_type) = param {
                let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    format_ident!("{}_converted", pat_ident.ident)
                } else {
                    format_ident!("arg{}_converted", i)
                };
                quote! { #param_name }
            } else {
                quote! {}
            }
        })
        .collect()
}

/// Build wrapper return type
fn build_wrapper_return(return_type: &ReturnType) -> TokenStream {
    match return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            let ani_type = rust_type_to_ani_type(ty);
            quote! { -> #ani_type }
        }
    }
}
