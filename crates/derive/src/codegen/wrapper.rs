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

// ============================================================================
// Parameter Classification
// ============================================================================

/// Type of injected parameter
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InjectedParamKind {
    /// Env parameter - ANI environment
    Env,
    /// This parameter - instance object for class methods
    This,
    /// Class parameter - class object for static methods
    Class,
}

/// Represents a function parameter's classification
#[derive(Debug)]
pub enum ParamKind {
    /// Self receiver (e.g., `&self`)
    SelfReceiver,
    /// Injected parameter (Env, This, Class)
    Injected(InjectedParamKind),
    /// Regular user parameter
    Regular,
}

/// Check if a type is an Env type
fn is_env_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("Env<") || type_str == "Env" || type_str.starts_with("&Env")
}

/// Check if a type is a This type (instance object)
fn is_this_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("This<")
        || type_str.contains("This>")
        || type_str == "This"
        || type_str.starts_with("&This")
}

/// Check if a type is an AniObject (could be this)
fn is_ani_object_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("AniObject") || type_str.starts_with("&AniObject")
}

/// Check if a type is a Class type
fn is_class_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(" ", "");
    type_str.contains("AniClass") || type_str.starts_with("&AniClass")
}

/// Classify a function argument
fn classify_param(arg: &FnArg, param_name: Option<&str>) -> ParamKind {
    match arg {
        FnArg::Receiver(_) => ParamKind::SelfReceiver,
        FnArg::Typed(pat_type) => {
            let ty = &*pat_type.ty;

            // Check for Env type
            if is_env_type(ty) {
                return ParamKind::Injected(InjectedParamKind::Env);
            }

            // Check for explicit This type
            if is_this_type(ty) {
                return ParamKind::Injected(InjectedParamKind::This);
            }

            // Check for Class type
            if is_class_type(ty) {
                if let Some(name) = param_name {
                    if name == "class" || name == "_class" {
                        return ParamKind::Injected(InjectedParamKind::Class);
                    }
                }
            }

            // Check if parameter named "this" with AniObject type
            if let Some(name) = param_name {
                if name == "this" && is_ani_object_type(ty) {
                    return ParamKind::Injected(InjectedParamKind::This);
                }
            }

            ParamKind::Regular
        }
    }
}

/// Extract parameter name from FnArg
fn get_param_name(arg: &FnArg) -> Option<String> {
    if let FnArg::Typed(pat_type) = arg {
        if let Pat::Ident(pat_ident) = &*pat_type.pat {
            return Some(pat_ident.ident.to_string());
        }
    }
    None
}

// ============================================================================
// Wrapper Generation
// ============================================================================

/// Information about injected parameters
struct InjectionInfo {
    has_env: bool,
    has_this: bool,
    has_class: bool,
}

/// Analyze function parameters for injection
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

/// Generate wrapper function for ANI binding
pub fn generate_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    is_class_method: bool,
    is_static: bool,
) -> TokenStream {
    let func_name = &func.sig.ident;
    let return_type = &func.sig.output;

    // Analyze parameter injections
    let injections = analyze_injections(func);

    // Build wrapper parameters
    let wrapper_params = build_wrapper_params(func, is_class_method, is_static);

    // Collect regular user parameters (skip self and injected params)
    let regular_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| {
            let name = get_param_name(arg);
            matches!(classify_param(arg, name.as_deref()), ParamKind::Regular)
        })
        .collect();

    // Generate parameter conversions for regular params
    let conversions = generate_param_conversions(&regular_params);

    // Generate injected variable declarations
    let injected_vars = generate_injected_vars(&injections, is_class_method);

    // Generate function call arguments
    let call_args = build_call_args_with_injections(func, &injections);

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
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) #wrapper_return {
            #injected_vars
            #conversions
            #func_call
            #return_conversion
        }
    }
}

/// Generate injected variable declarations
fn generate_injected_vars(injections: &InjectionInfo, is_class_method: bool) -> TokenStream {
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

    quote! { #(#vars)* }
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

    // Add regular user parameters (skip self and injected params)
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

/// Build call arguments including injected parameters
fn build_call_args_with_injections(func: &ItemFn, injections: &InjectionInfo) -> Vec<TokenStream> {
    let mut args = Vec::new();

    for arg in &func.sig.inputs {
        let name = get_param_name(arg);
        match classify_param(arg, name.as_deref()) {
            ParamKind::SelfReceiver => {
                // Skip self receiver
            }
            ParamKind::Injected(kind) => {
                // Add injected parameter reference
                match kind {
                    InjectedParamKind::Env => {
                        if injections.has_env {
                            args.push(quote! { &__ani_env });
                        }
                    }
                    InjectedParamKind::This => {
                        if injections.has_this {
                            args.push(quote! { &__ani_this });
                        }
                    }
                    InjectedParamKind::Class => {
                        if injections.has_class {
                            args.push(quote! { &__ani_class });
                        }
                    }
                }
            }
            ParamKind::Regular => {
                // Add converted parameter
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

// ============================================================================
// Public API for other modules
// ============================================================================

/// Check if a parameter should be skipped in signature generation
pub fn should_skip_in_signature(arg: &FnArg) -> bool {
    let name = get_param_name(arg);
    !matches!(classify_param(arg, name.as_deref()), ParamKind::Regular)
}

/// Check if the function has a This parameter (instance method indicator)
pub fn has_this_injection(func: &ItemFn) -> bool {
    func.sig.inputs.iter().any(|arg| {
        let name = get_param_name(arg);
        matches!(
            classify_param(arg, name.as_deref()),
            ParamKind::Injected(InjectedParamKind::This)
        )
    })
}
