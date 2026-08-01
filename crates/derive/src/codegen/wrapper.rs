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

use crate::types::{
    ani_type::{AniType, FunctionType, RuntimeHandleType},
    generate_param_conversions, generate_param_conversions_with_custom_error,
    generate_return_conversion, rust_type_to_ani_type,
};

/// Type of injected parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectedParamKind {
    Env,
    This,
    Class,
}

/// Represents a function parameter's classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    SelfReceiver,
    Injected(InjectedParamKind),
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapperBindingKind {
    Global,
    ClassInstance,
    ClassStatic,
}

impl WrapperBindingKind {
    pub(crate) fn is_class(self) -> bool {
        !matches!(self, Self::Global)
    }

    pub(crate) fn is_static(self) -> bool {
        matches!(self, Self::ClassStatic)
    }
}

struct WrapperParam<'a> {
    arg: &'a FnArg,
    kind: ParamKind,
    ident: Option<&'a Ident>,
}

fn is_env_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(' ', "");
    type_str.contains("Env<") || type_str == "Env" || type_str.starts_with("&Env")
}

fn is_this_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(' ', "");
    type_str.contains("This<")
        || type_str.contains("This>")
        || type_str == "This"
        || type_str.starts_with("&This")
}

fn is_ani_object_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(' ', "");
    type_str.contains("AniObject") || type_str.starts_with("&AniObject")
}

fn is_class_type(ty: &Type) -> bool {
    let type_str = quote!(#ty).to_string().replace(' ', "");
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
            if is_class_type(ty) && matches!(param_name, Some("class") | Some("_class")) {
                return ParamKind::Injected(InjectedParamKind::Class);
            }
            if matches!(param_name, Some("this")) && is_ani_object_type(ty) {
                return ParamKind::Injected(InjectedParamKind::This);
            }

            ParamKind::Regular
        }
    }
}

fn get_param_name(arg: &FnArg) -> Option<String> {
    if let FnArg::Typed(pat_type) = arg
        && let Pat::Ident(pat_ident) = &*pat_type.pat
    {
        return Some(pat_ident.ident.to_string());
    }
    None
}

fn classify_sig_param(arg: &FnArg) -> ParamKind {
    let name = get_param_name(arg);
    classify_param(arg, name.as_deref())
}

fn injected_binding_ident(ident: &Ident) -> Ident {
    format_ident!("__ani_injected_{}", ident)
}

fn analyze_wrapper_params(func: &ItemFn) -> Vec<WrapperParam<'_>> {
    func.sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => WrapperParam {
                arg,
                kind: ParamKind::SelfReceiver,
                ident: None,
            },
            FnArg::Typed(pat_type) => {
                let ident = match &*pat_type.pat {
                    Pat::Ident(pat_ident) => Some(&pat_ident.ident),
                    _ => None,
                };
                let name = ident.map(|ident| ident.to_string());
                WrapperParam {
                    arg,
                    kind: classify_param(arg, name.as_deref()),
                    ident,
                }
            }
        })
        .collect()
}

fn has_injected_param(params: &[WrapperParam<'_>], kind: InjectedParamKind) -> bool {
    params
        .iter()
        .any(|param| matches!(param.kind, ParamKind::Injected(param_kind) if param_kind == kind))
}

fn ty_is_ref(ty: &Type) -> bool {
    quote!(#ty).to_string().replace(' ', "").starts_with('&')
}

fn async_ref_container_name(ident: &Ident) -> Ident {
    format_ident!("__ani_async_ref_container_{}", ident)
}

fn async_param_ident(arg: &FnArg, index: usize) -> Option<Ident> {
    match arg {
        FnArg::Typed(pat_type) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => Some(pat_ident.ident.clone()),
            _ => Some(format_ident!("arg{}", index)),
        },
        FnArg::Receiver(_) => None,
    }
}

fn supports_async_ref_container(ty: &Type) -> bool {
    matches!(
        AniType::from_syn_type(ty),
        AniType::Function(FunctionType::Function { .. })
            | AniType::AniObject
            | AniType::AnyValue
            | AniType::TupleValue
            | AniType::EnumItem
            | AniType::ArrayBuffer
            | AniType::ArrayHandle(_)
            | AniType::FixedArray(_)
            | AniType::RuntimeHandle(
                RuntimeHandleType::Ref
                    | RuntimeHandleType::Class
                    | RuntimeHandleType::Type
                    | RuntimeHandleType::Module
                    | RuntimeHandleType::Namespace
                    | RuntimeHandleType::String
                    | RuntimeHandleType::Enum
                    | RuntimeHandleType::Error
                    | RuntimeHandleType::FunctionObject
            )
    )
}

pub(crate) fn generate_async_ref_container_captures(
    params: &[&FnArg],
    env_ident: &Ident,
    on_error: &TokenStream,
) -> TokenStream {
    let mut captures = Vec::new();

    for (index, arg) in params.iter().enumerate() {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };
        if !supports_async_ref_container(&pat_type.ty) {
            continue;
        }

        let Some(param_ident) = async_param_ident(arg, index) else {
            continue;
        };
        let converted_ident = format_ident!("{}_converted", param_ident);
        let container_ident = async_ref_container_name(&param_ident);

        captures.push(quote! {
            let #container_ident = match ani::conversions::RefContainer::new(&#env_ident, &#converted_ident) {
                Ok(value) => value,
                Err(e) => { #on_error }
            };
        });
    }

    quote! { #(#captures)* }
}

pub(crate) fn generate_async_ref_container_restores(
    params: &[&FnArg],
    env_ident: &Ident,
) -> TokenStream {
    let mut restores = Vec::new();

    for (index, arg) in params.iter().enumerate() {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };
        if !supports_async_ref_container(&pat_type.ty) {
            continue;
        }

        let Some(param_ident) = async_param_ident(arg, index) else {
            continue;
        };
        let converted_ident = format_ident!("{}_converted", param_ident);
        let container_ident = async_ref_container_name(&param_ident);
        let ty = &pat_type.ty;

        restores.push(quote! {
            let #converted_ident: #ty = #container_ident
                .to_local(&#env_ident)
                .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        });
    }

    quote! { #(#restores)* }
}

pub fn generate_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let func_name = &func.sig.ident;
    generate_wrapper_with_target(func, wrapper_name, binding_kind, quote! { #func_name })
}

pub fn generate_wrapper_with_target(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
    call_target: TokenStream,
) -> TokenStream {
    let return_type = &func.sig.output;
    let params = analyze_wrapper_params(func);
    let wrapper_params = build_wrapper_params(&params, binding_kind);
    let regular_params = params
        .iter()
        .filter(|param| param.kind == ParamKind::Regular)
        .map(|param| param.arg)
        .collect::<Vec<_>>();

    let param_error_return = build_param_error_return(return_type);
    let panic_error_return = param_error_return.clone();
    let conversions = generate_param_conversions(&regular_params, &param_error_return);
    let injected_vars = generate_injected_vars(&params, binding_kind);
    let call_args = build_call_args(&params);
    let func_call = quote! {
        let result = match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
            #call_target(#(#call_args),*)
        })) {
            Ok(result) => result,
            Err(panic) => {
                let message = {
                    if let Some(string) = panic.downcast_ref::<String>() {
                        string.clone()
                    } else if let Some(string) = panic.downcast_ref::<&str>() {
                        (*string).to_string()
                    } else {
                        format!("panic from Rust code: {:?}", panic)
                    }
                };
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let _ = ani::conversions::throw_error(&env_wrapper, &message);
                #panic_error_return
            }
        };
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

pub fn generate_async_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let func_name = &func.sig.ident;
    generate_async_wrapper_with_target(func, wrapper_name, binding_kind, quote! { #func_name })
}

pub fn generate_async_wrapper_with_target(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
    call_target: TokenStream,
) -> TokenStream {
    let params = analyze_wrapper_params(func);
    let wrapper_params = build_wrapper_params(&params, binding_kind);
    let regular_params = params
        .iter()
        .filter(|param| param.kind == ParamKind::Regular)
        .map(|param| param.arg)
        .collect::<Vec<_>>();

    let conversion_error = quote! {
        return {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::PromiseRaw::<()>::reject_with_error(&env_wrapper, e) {
                Ok(promise) => promise.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        };
    };
    let conversions =
        generate_param_conversions_with_custom_error(&regular_params, &conversion_error);
    let async_param_captures = generate_async_ref_container_captures(
        &regular_params,
        &format_ident!("__ani_env"),
        &conversion_error,
    );
    let call_args = build_call_args(&params);
    let async_setup = generate_async_promise_setup(&params, binding_kind);
    let async_injected = generate_async_promise_injected_vars(&params, binding_kind);
    let async_param_restores =
        generate_async_ref_container_restores(&regular_params, &format_ident!("__ani_env"));

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) -> ani::sys::ani_object {
            #async_setup
            #conversions
            #async_param_captures

            match ani::async_runtime::spawn_future_result_factory(&__ani_env, move || async move {
                #async_injected
                #async_param_restores
                #call_target(#(#call_args),*)
                    .await
                    .map_err(|e| -> ani::error::DynAniError { Box::new(e) })
            }) {
                Ok(promise) => promise.into_raw(),
                Err(e) => {
                    match ani::conversions::PromiseRaw::<()>::reject_with_error(&__ani_env, e) {
                        Ok(promise) => promise.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                }
            }
        }
    }
}

pub fn generate_async_blocking_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let func_name = &func.sig.ident;
    generate_async_blocking_wrapper_with_target(
        func,
        wrapper_name,
        binding_kind,
        quote! { #func_name },
    )
}

pub fn generate_async_blocking_wrapper_with_target(
    func: &ItemFn,
    wrapper_name: &Ident,
    binding_kind: WrapperBindingKind,
    call_target: TokenStream,
) -> TokenStream {
    let return_type = &func.sig.output;
    let params = analyze_wrapper_params(func);
    let wrapper_params = build_wrapper_params(&params, binding_kind);
    let regular_params = params
        .iter()
        .filter(|param| param.kind == ParamKind::Regular)
        .map(|param| param.arg)
        .collect::<Vec<_>>();

    let param_error_return = build_param_error_return(return_type);
    let conversions = generate_param_conversions(&regular_params, &param_error_return);
    let call_args = build_call_args(&params);
    let async_setup = generate_async_blocking_setup(&params, binding_kind, &param_error_return);
    let async_injected = generate_async_blocking_injected_vars(&params, binding_kind);
    let return_conversion = generate_return_conversion(return_type);
    let wrapper_return = build_wrapper_return(return_type);

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) #wrapper_return {
            #async_setup
            #conversions
            let __ani_future = async move {
                #async_injected
                #call_target(#(#call_args),*)
                    .await
                    .map_err(|e| -> ani::error::DynAniError { Box::new(e) })
            };
            let result = match ani::tokio::block_on_future_result(__ani_future) {
                Ok(result) => result,
                Err(e) => {
                    unsafe { ani::error::throw_error_payload(__ani_env_outer.as_raw(), &e) };
                    #param_error_return
                }
            };
            #return_conversion
        }
    }
}

fn generate_async_promise_setup(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let reject_return = promise_reject_return();
    let mut setup = vec![quote! {
        let __ani_env = ani::env::Env::from_raw_unchecked(env);
    }];
    setup.push(quote! {
        let __ani_vm = match __ani_env.get_vm() {
            Ok(vm) => vm,
            Err(e) => {
                #reject_return
            }
        };
    });

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::This) {
        setup.push(quote! {
            let __ani_this_container = {
                let __ani_this_ref = unsafe { ani::types::AniRef::from_raw(this as ani::sys::ani_ref) };
                match ani::conversions::RefContainer::new(&__ani_env, &__ani_this_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        #reject_return
                    }
                }
            };
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::Class) {
        setup.push(quote! {
            let __ani_class_container = {
                let __ani_class_ref: ani::types::AniRef<'_> =
                    unsafe { ani::types::AniClass::from_raw(_class) }.into();
                match ani::conversions::RefContainer::new(&__ani_env, &__ani_class_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        #reject_return
                    }
                }
            };
        });
    }

    quote! { #(#setup)* }
}

fn generate_async_blocking_setup(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
    on_error_return: &TokenStream,
) -> TokenStream {
    let mut setup = vec![quote! {
        let __ani_env_outer = ani::env::Env::from_raw_unchecked(env);
    }];
    let on_error_return = quote! {
        unsafe { ani::error::throw_error_payload(__ani_env_outer.as_raw(), &e) };
        #on_error_return
    };

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::This) {
        setup.push(quote! {
            let __ani_this_container = {
                let __ani_this_ref =
                    unsafe { ani::types::AniRef::from_raw(this as ani::sys::ani_ref) };
                match ani::conversions::RefContainer::new(&__ani_env_outer, &__ani_this_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        #on_error_return
                    }
                }
            };
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::Class) {
        setup.push(quote! {
            let __ani_class_container = {
                let __ani_class_ref: ani::types::AniRef<'_> =
                    unsafe { ani::types::AniClass::from_raw(_class) }.into();
                match ani::conversions::RefContainer::new(&__ani_env_outer, &__ani_class_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        #on_error_return
                    }
                }
            };
        });
    }

    quote! { #(#setup)* }
}

fn generate_async_promise_injected_vars(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let mut vars = vec![quote! {
        let __ani_attach = __ani_vm.attach_current_thread_scoped()
            .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        let __ani_env = __ani_attach.env();
        let env = __ani_env.as_raw();
    }];

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::This) {
        vars.push(quote! {
            let __ani_this: ani::types::AniObject<'_> = __ani_this_container
                .to_local(&__ani_env)
                .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::Class) {
        vars.push(quote! {
            let __ani_class: ani::types::AniClass<'_> = __ani_class_container
                .to_local(&__ani_env)
                .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        });
    }

    for param in params {
        let FnArg::Typed(pat_type) = param.arg else {
            continue;
        };
        let Some(ident) = param.ident else {
            continue;
        };
        let binding_ident = injected_binding_ident(ident);

        match param.kind {
            ParamKind::Injected(InjectedParamKind::Env) => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = __ani_env;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = ani::env::Env::from_raw_unchecked(env);
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::This) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = &__ani_this;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = __ani_this;
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::Class) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
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

    quote! { #(#vars)* }
}

fn generate_async_blocking_injected_vars(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let mut vars = vec![quote! {
        let __ani_env = ani::env::Env::from_raw_unchecked(env);
        let env = __ani_env.as_raw();
    }];

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::This) {
        vars.push(quote! {
            let __ani_this: ani::types::AniObject<'_> = __ani_this_container
                .to_local(&__ani_env_outer)
                .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::Class) {
        vars.push(quote! {
            let __ani_class: ani::types::AniClass<'_> = __ani_class_container
                .to_local(&__ani_env_outer)
                .map_err(|e| -> ani::error::DynAniError { Box::new(e) })?;
        });
    }

    for param in params {
        let FnArg::Typed(pat_type) = param.arg else {
            continue;
        };
        let Some(ident) = param.ident else {
            continue;
        };
        let binding_ident = injected_binding_ident(ident);

        match param.kind {
            ParamKind::Injected(InjectedParamKind::Env) => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = &__ani_env;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = ani::env::Env::from_raw_unchecked(env);
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::This) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = &__ani_this;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = __ani_this;
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::Class) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
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

    quote! { #(#vars)* }
}

fn promise_reject_return() -> TokenStream {
    quote! {
        return match ani::conversions::PromiseRaw::<()>::reject_with_error(&__ani_env, e) {
            Ok(promise) => promise.into_raw(),
            Err(_) => std::ptr::null_mut(),
        };
    }
}

fn generate_injected_vars(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
) -> TokenStream {
    let mut vars = Vec::new();

    if has_injected_param(params, InjectedParamKind::Env) {
        vars.push(quote! {
            let __ani_env = ani::env::Env::from_raw_unchecked(env);
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::This) {
        vars.push(quote! {
            let __ani_this = ani::types::AniObject::from_raw(this);
        });
    }

    if binding_kind.is_class() && has_injected_param(params, InjectedParamKind::Class) {
        vars.push(quote! {
            let __ani_class = ani::types::AniClass::from_raw(_class);
        });
    }

    for param in params {
        let FnArg::Typed(pat_type) = param.arg else {
            continue;
        };
        let Some(ident) = param.ident else {
            continue;
        };
        let binding_ident = injected_binding_ident(ident);

        match param.kind {
            ParamKind::Injected(InjectedParamKind::Env) => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = &__ani_env;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = __ani_env;
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::This) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
                    vars.push(quote! {
                        let #binding_ident = &__ani_this;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = __ani_this;
                    });
                }
            }
            ParamKind::Injected(InjectedParamKind::Class) if binding_kind.is_class() => {
                if ty_is_ref(&pat_type.ty) {
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

    quote! { #(#vars)* }
}

fn build_wrapper_params(
    params: &[WrapperParam<'_>],
    binding_kind: WrapperBindingKind,
) -> Vec<TokenStream> {
    let mut wrapper_params = vec![quote! { env: *mut ani::sys::ani_env }];

    if binding_kind.is_class() {
        if binding_kind.is_static() {
            wrapper_params.push(quote! { _class: ani::sys::ani_class });
        } else {
            wrapper_params.push(quote! { this: ani::sys::ani_object });
        }
    }

    for (i, param) in params
        .iter()
        .filter(|param| param.kind == ParamKind::Regular)
        .enumerate()
    {
        if let FnArg::Typed(pat_type) = param.arg {
            let param_name = param
                .ident
                .cloned()
                .unwrap_or_else(|| format_ident!("arg{}", i));
            let ani_type = rust_type_to_ani_type(&pat_type.ty);
            wrapper_params.push(quote! { #param_name: #ani_type });
        }
    }

    wrapper_params
}

fn build_call_args(params: &[WrapperParam<'_>]) -> Vec<TokenStream> {
    let mut args = Vec::new();

    for param in params {
        match param.kind {
            ParamKind::SelfReceiver => {}
            ParamKind::Injected(_) => {
                if let Some(ident) = param.ident {
                    let ident = injected_binding_ident(ident);
                    args.push(quote! { #ident });
                }
            }
            ParamKind::Regular => {
                let param_name = param
                    .ident
                    .map(|ident| format_ident!("{}_converted", ident))
                    .unwrap_or_else(|| format_ident!("arg_converted"));
                args.push(quote! { #param_name });
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
    classify_sig_param(arg) != ParamKind::Regular
}
