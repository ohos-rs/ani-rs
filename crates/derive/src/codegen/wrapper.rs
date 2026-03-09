//! Wrapper Function Generation
//!
//! Generates the `extern "C"` wrapper functions that bridge Rust and ANI.
//!
//! ## Parameter Injection System
//!
//! Following napi-rs patterns, the macro supports automatic parameter injection:
//!
//! - `Env` / `&Env<'_>` - ANI environment, automatically injected
//! - `This` / `&This<T>` / `this: AniObject` - Instance object for class methods
//! - `Class` / `&AniClass` - Class object for static methods
//!
//! These injected parameters are NOT part of the ArkTS function signature.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, ReturnType, Type};

use crate::types::{generate_param_conversions, generate_return_conversion, rust_type_to_ani_type};

/// Type of injected parameter
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InjectedParamKind {
    Env,
    This,
    Class,
}

/// Represents a function parameter's classification
#[derive(Debug)]
pub enum ParamKind {
    SelfReceiver,
    Injected(InjectedParamKind),
    Regular,
}

fn is_env_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("Env<") || type_str == "Env" || type_str.starts_with("&Env")
}

fn is_this_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("This<")
        || type_str.contains("This>")
        || type_str == "This"
        || type_str.starts_with("&This")
}

fn is_ani_object_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("AniObject") || type_str.starts_with("&AniObject")
}

fn is_class_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("AniClass") || type_str.starts_with("&AniClass")
}

fn classify_param(arg: &FnArg, param_name: Option<&str>) -> ParamKind {
    match arg {
        FnArg::Receiver(_) => ParamKind::SelfReceiver,
        FnArg::Typed(pat_type) => {
            let ty = &*pat_type.ty;

            if is_env_type(ty) {
                return ParamKind::Injected(InjectedParamKind::Env);
            }
            if is_this_type(ty) {
                return ParamKind::Injected(InjectedParamKind::This);
            }
            if is_class_type(ty) {
                if let Some(name) = param_name {
                    if name == "class" || name == "_class" {
                        return ParamKind::Injected(InjectedParamKind::Class);
                    }
                }
            }
            if let Some(name) = param_name {
                if name == "this" && is_ani_object_type(ty) {
                    return ParamKind::Injected(InjectedParamKind::This);
                }
            }

            ParamKind::Regular
        }
    }
}

fn get_param_name(arg: &FnArg) -> Option<String> {
    if let FnArg::Typed(pat_type) = arg {
        if let Pat::Ident(pat_ident) = &*pat_type.pat {
            return Some(pat_ident.ident.to_string());
        }
    }
    None
}

fn injected_binding_ident(ident: &Ident) -> Ident {
    format_ident!("__ani_injected_{}", ident)
}

struct InjectionInfo {
    has_env: bool,
    has_this: bool,
    has_class: bool,
}

fn analyze_injections(func: &ItemFn) -> InjectionInfo {
    let mut info = InjectionInfo {
        has_env: false,
        has_this: false,
        has_class: false,
    };

    for arg in &func.sig.inputs {
        let name = get_param_name(arg);
        match classify_param(arg, name.as_deref()) {
            ParamKind::Injected(InjectedParamKind::Env) => info.has_env = true,
            ParamKind::Injected(InjectedParamKind::This) => info.has_this = true,
            ParamKind::Injected(InjectedParamKind::Class) => info.has_class = true,
            _ => {}
        }
    }

    info
}

pub fn generate_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    is_class_method: bool,
    is_static: bool,
) -> TokenStream {
    let func_name = &func.sig.ident;
    generate_wrapper_with_target(
        func,
        wrapper_name,
        is_class_method,
        is_static,
        quote! { #func_name },
    )
}

pub fn generate_wrapper_with_target(
    func: &ItemFn,
    wrapper_name: &Ident,
    is_class_method: bool,
    is_static: bool,
    call_target: TokenStream,
) -> TokenStream {
    let return_type = &func.sig.output;
    let injections = analyze_injections(func);
    let wrapper_params = build_wrapper_params(func, is_class_method, is_static);

    let regular_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| {
            let name = get_param_name(arg);
            matches!(classify_param(arg, name.as_deref()), ParamKind::Regular)
        })
        .collect();

    let param_error_return = build_param_error_return(return_type);
    let conversions = generate_param_conversions(&regular_params, &param_error_return);
    let injected_vars = generate_injected_vars(func, &injections, is_class_method);
    let call_args = build_call_args_with_injections(func, &injections);
    let func_call = quote! {
        let result = #call_target(#(#call_args),*);
    };
    let return_conversion = generate_return_conversion(return_type);
    let wrapper_return = build_wrapper_return(return_type);

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) #wrapper_return {
            #injected_vars
            #conversions
            #func_call
            #return_conversion
        }
    }
}

fn generate_injected_vars(
    func: &ItemFn,
    injections: &InjectionInfo,
    is_class_method: bool,
) -> TokenStream {
    let mut vars = Vec::new();

    if injections.has_env {
        vars.push(quote! {
            let __ani_env = ani::env::Env::from_raw_unchecked(env);
        });
    }

    if injections.has_this && is_class_method {
        vars.push(quote! {
            let __ani_this = ani::types::AniObject::from_raw(this);
        });
    }

    if injections.has_class && is_class_method {
        vars.push(quote! {
            let __ani_class = ani::types::AniClass::from_raw(_class);
        });
    }

    for arg in &func.sig.inputs {
        let name = get_param_name(arg);
        if let FnArg::Typed(pat_type) = arg {
            let Pat::Ident(pat_ident) = &*pat_type.pat else {
                continue;
            };
            let ident = &pat_ident.ident;
            let binding_ident = injected_binding_ident(ident);
            match classify_param(arg, name.as_deref()) {
                ParamKind::Injected(InjectedParamKind::Env) => {
                    let ty = &pat_type.ty;
                    let ty_str = quote!(#ty).to_string().replace(" ", "");
                    if ty_str.starts_with('&') {
                        vars.push(quote! {
                            let #binding_ident = &__ani_env;
                        });
                    } else {
                        vars.push(quote! {
                            let #binding_ident = __ani_env;
                        });
                    }
                }
                ParamKind::Injected(InjectedParamKind::This) if is_class_method => {
                    let ty = &pat_type.ty;
                    let ty_str = quote!(#ty).to_string().replace(" ", "");
                    if ty_str.starts_with('&') {
                        vars.push(quote! {
                            let #binding_ident = &__ani_this;
                        });
                    } else {
                        vars.push(quote! {
                            let #binding_ident = __ani_this;
                        });
                    }
                }
                ParamKind::Injected(InjectedParamKind::Class) if is_class_method => {
                    let ty = &pat_type.ty;
                    let ty_str = quote!(#ty).to_string().replace(" ", "");
                    if ty_str.starts_with('&') {
                        vars.push(quote! {
                            let #binding_ident = &__ani_class;
                        });
                    } else {
                        vars.push(quote! {
                            let #binding_ident = __ani_class;
                        });
                    }
                }
                _ => {}
            }
        }
    }

    quote! { #(#vars)* }
}

fn build_wrapper_params(func: &ItemFn, is_class_method: bool, is_static: bool) -> Vec<TokenStream> {
    let mut params = vec![quote! { env: *mut ani::sys::ani_env }];

    if is_class_method {
        if is_static {
            params.push(quote! { _class: ani::sys::ani_class });
        } else {
            params.push(quote! { this: ani::sys::ani_object });
        }
    }

    for (i, param) in func
        .sig
        .inputs
        .iter()
        .filter(|arg| {
            let name = get_param_name(arg);
            matches!(classify_param(arg, name.as_deref()), ParamKind::Regular)
        })
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

fn build_call_args_with_injections(func: &ItemFn, injections: &InjectionInfo) -> Vec<TokenStream> {
    let mut args = Vec::new();

    for arg in &func.sig.inputs {
        let name = get_param_name(arg);
        match classify_param(arg, name.as_deref()) {
            ParamKind::SelfReceiver => {}
            ParamKind::Injected(kind) => match kind {
                InjectedParamKind::Env => {
                    if injections.has_env {
                        if let FnArg::Typed(pat_type) = arg {
                            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                                let ident = injected_binding_ident(&pat_ident.ident);
                                args.push(quote! { #ident });
                            }
                        }
                    }
                }
                InjectedParamKind::This => {
                    if injections.has_this {
                        if let FnArg::Typed(pat_type) = arg {
                            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                                let ident = injected_binding_ident(&pat_ident.ident);
                                args.push(quote! { #ident });
                            }
                        }
                    }
                }
                InjectedParamKind::Class => {
                    if injections.has_class {
                        if let FnArg::Typed(pat_type) = arg {
                            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                                let ident = injected_binding_ident(&pat_ident.ident);
                                args.push(quote! { #ident });
                            }
                        }
                    }
                }
            },
            ParamKind::Regular => {
                if let FnArg::Typed(pat_type) = arg {
                    let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        format_ident!("{}_converted", pat_ident.ident)
                    } else {
                        format_ident!("arg_converted")
                    };
                    args.push(quote! { #param_name });
                }
            }
        }
    }

    args
}

fn build_wrapper_return(return_type: &ReturnType) -> TokenStream {
    match return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            let ani_type = rust_type_to_ani_type(ty);
            quote! { -> #ani_type }
        }
    }
}

fn build_param_error_return(return_type: &ReturnType) -> TokenStream {
    match return_type {
        ReturnType::Default => quote! { return; },
        ReturnType::Type(_, _) => quote! { return Default::default(); },
    }
}

pub fn should_skip_in_signature(arg: &FnArg) -> bool {
    let name = get_param_name(arg);
    !matches!(classify_param(arg, name.as_deref()), ParamKind::Regular)
}

pub fn has_this_injection(func: &ItemFn) -> bool {
    func.sig.inputs.iter().any(|arg| {
        let name = get_param_name(arg);
        matches!(
            classify_param(arg, name.as_deref()),
            ParamKind::Injected(InjectedParamKind::This)
        )
    })
}
