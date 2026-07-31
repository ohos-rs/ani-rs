//! Impl Block Expansion
//!
//! Expands `#[ani]` macro for impl blocks.

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, FnArg, ImplItem, ItemFn, ItemImpl, Pat, ReturnType, Type};

use crate::codegen::{
    ClassDescriptorMember, ClassMemberScope, ClassPropertyDescriptor, emit_export_plan_ets,
    generate_async_blocking_wrapper_with_target, generate_async_ref_container_captures,
    generate_async_ref_container_restores, generate_async_wrapper_with_target,
    generate_register_call, generate_wrapper_with_target,
};
use crate::parser::{BindgenAttrs, parse_bindgen_attrs_from_attribute};
use crate::types::{
    ani_type::resolve_object_type_fields, generate_param_conversions,
    generate_param_conversions_with_custom_error, generate_return_conversion,
    rust_type_to_ani_type,
};

use super::function::{
    AsyncExportMode, BindingOwner, BindingResolveInput, CallableKind, SignatureBindingStyle,
    async_export_mode, resolve_binding_plan_with_class_plan, resolve_class_member_plan,
    signature_for_export, validate_constructor_usage, validate_unsupported_bind_attrs,
};

/// Expand `#[ani]` for impl blocks
pub fn expand_impl(attrs: BindgenAttrs, impl_block: ItemImpl, prepare: TokenStream) -> TokenStream {
    let struct_name = match extract_struct_name(&impl_block) {
        Ok(name) => name,
        Err(err) => return err,
    };

    let mut wrappers = Vec::new();
    let mut queue_entries = Vec::new();
    let mut original_items = Vec::new();
    let mut errors = Vec::new();
    let mut property_slots: BTreeMap<(String, ClassMemberScope, String), ClassPropertyDescriptor> =
        BTreeMap::new();

    for item in &impl_block.items {
        match item {
            ImplItem::Fn(method) => {
                if !has_bind_marker(&method.attrs) {
                    original_items.push(quote! { #method });
                    continue;
                }

                match process_method(&attrs, &impl_block, &struct_name, method) {
                    Ok(processed) => {
                        if let Err(err) = validate_property_slot_conflict(
                            &mut property_slots,
                            processed.class_descriptor.as_ref(),
                            method,
                        ) {
                            errors.push(err.to_compile_error());
                            original_items.push(processed.original_method);
                            continue;
                        }
                        wrappers.push(processed.wrapper);
                        queue_entries.push(processed.queue_entry);
                        original_items.push(processed.original_method);
                    }
                    Err(err) => {
                        errors.push(err.to_compile_error());
                        let sanitized_method = sanitize_method(method);
                        original_items.push(quote! { #sanitized_method });
                    }
                }
            }
            _ => original_items.push(quote! { #item }),
        }
    }

    let impl_attrs = &impl_block.attrs;
    let impl_generics = &impl_block.generics;
    let impl_self_ty = &impl_block.self_ty;
    let register_name = format_ident!("__ani_register_impl_{}", struct_name.to_lowercase());
    let ctor_register_name =
        format_ident!("__ani_ctor_register_impl_{}", struct_name.to_lowercase());
    let callback_name = format!("impl::{struct_name}");

    quote! {
        #prepare

        #(#errors)*

        #(#impl_attrs)*
        impl #impl_generics #impl_self_ty {
            #(#original_items)*
        }

        #(#wrappers)*

        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(_env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            #(#queue_entries)*
            ani::sys::ani_status_ANI_OK
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[::ani::ctor::ctor(crate_path = ::ani::ctor)]
        fn #ctor_register_name() {
            ::ani::module_register::register_module_export(
                #callback_name,
                #register_name,
            );
        }
    }
}

struct ProcessedMethod {
    wrapper: TokenStream,
    queue_entry: TokenStream,
    original_method: TokenStream,
    class_descriptor: Option<ClassDescriptorMember>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MethodReceiver {
    None,
    Owned,
    Ref,
    RefMut,
}

impl MethodReceiver {
    fn has_receiver(self) -> bool {
        !matches!(self, Self::None)
    }

    fn is_mut(self) -> bool {
        matches!(self, Self::RefMut)
    }
}

fn process_method(
    impl_attrs: &BindgenAttrs,
    impl_block: &ItemImpl,
    struct_name: &str,
    method: &syn::ImplItemFn,
) -> syn::Result<ProcessedMethod> {
    let receiver = analyze_method_receiver(method)?;

    let method_attrs = parse_method_bind_attrs(&method.attrs)?;
    let merged_attrs = merge_bindgen_attrs(impl_attrs, &method_attrs, struct_name);
    validate_receiver_usage(method, &merged_attrs, receiver)?;

    let method_fn = to_item_fn(method);
    validate_unsupported_bind_attrs(&merged_attrs, &method_fn)?;
    validate_constructor_usage(&merged_attrs, &method_fn)?;

    let method_name = &method.sig.ident;
    let binding_input = BindingResolveInput {
        callable_kind: if merged_attrs.constructor {
            CallableKind::Constructor
        } else {
            CallableKind::Function
        },
        owner: BindingOwner::Class {
            scope: if merged_attrs.constructor || !merged_attrs.is_static {
                ClassMemberScope::Instance
            } else {
                ClassMemberScope::Static
            },
        },
        signature_style: if receiver.has_receiver() {
            SignatureBindingStyle::SkipRustReceiver
        } else {
            SignatureBindingStyle::Direct
        },
    };

    let ets_name = merged_attrs
        .name
        .clone()
        .unwrap_or_else(|| method_name.to_string());
    let class_member_plan = resolve_class_member_plan(
        &merged_attrs,
        &method_name.to_string(),
        &method.sig,
        binding_input,
        &ets_name,
    )?;

    if let Some(property_name) = class_member_plan
        .as_ref()
        .and_then(|plan| plan.property_name())
        && !binding_input.is_static()
    {
        validate_accessor_backing_field_conflict(struct_name, property_name, method)?;
    }

    let signature_for_binding = signature_for_export(&merged_attrs, &method.sig)?;
    let binding = resolve_binding_plan_with_class_plan(
        &merged_attrs,
        &ets_name,
        &signature_for_binding,
        binding_input,
        class_member_plan.as_ref(),
    )?;
    emit_export_plan_ets(&binding);

    let wrapper_name = format_ident!("__ani_{}_{}", struct_name, method_name);
    let sanitized_method = sanitize_method(method);
    let wrapper = if receiver.has_receiver() {
        match async_export_mode(&merged_attrs) {
            Some(AsyncExportMode::Promise) => generate_async_receiver_wrapper(
                &to_item_fn(&sanitized_method),
                &wrapper_name,
                &impl_block.self_ty,
                receiver,
            ),
            Some(AsyncExportMode::Blocking) => generate_async_blocking_receiver_wrapper(
                &to_item_fn(&sanitized_method),
                &wrapper_name,
                &impl_block.self_ty,
                receiver,
            ),
            None => generate_receiver_wrapper(
                &to_item_fn(&sanitized_method),
                &wrapper_name,
                &impl_block.self_ty,
                receiver,
            ),
        }
    } else {
        let call_target = {
            let self_ty = &impl_block.self_ty;
            quote! { <#self_ty>::#method_name }
        };
        if merged_attrs.is_async {
            match async_export_mode(&merged_attrs) {
                Some(AsyncExportMode::Promise) => generate_async_wrapper_with_target(
                    &to_item_fn(&sanitized_method),
                    &wrapper_name,
                    binding_input.wrapper_binding_kind(),
                    call_target,
                ),
                Some(AsyncExportMode::Blocking) => generate_async_blocking_wrapper_with_target(
                    &to_item_fn(&sanitized_method),
                    &wrapper_name,
                    binding_input.wrapper_binding_kind(),
                    call_target,
                ),
                None => unreachable!("async wrapper requested without async attrs"),
            }
        } else {
            generate_wrapper_with_target(
                &to_item_fn(&sanitized_method),
                &wrapper_name,
                binding_input.wrapper_binding_kind(),
                call_target,
            )
        }
    };

    let register_call = generate_register_call(
        &binding.register_target,
        &binding.register_symbol_name,
        &binding.signature,
        quote! { #wrapper_name as *const std::os::raw::c_void },
    );
    let queue_entry = quote! {
        {
            let status = #register_call;
            if status != ani::sys::ani_status_ANI_OK {
                return status;
            }
        }
    };

    Ok(ProcessedMethod {
        wrapper,
        queue_entry,
        original_method: quote! { #sanitized_method },
        class_descriptor: binding.class_descriptor.clone(),
    })
}

fn validate_property_slot_conflict(
    property_slots: &mut BTreeMap<(String, ClassMemberScope, String), ClassPropertyDescriptor>,
    class_descriptor: Option<&ClassDescriptorMember>,
    method: &syn::ImplItemFn,
) -> syn::Result<()> {
    let Some(property_descriptor) = class_descriptor.and_then(ClassDescriptorMember::property)
    else {
        return Ok(());
    };

    let key = (
        property_descriptor.metadata.owner.clone(),
        property_descriptor.metadata.scope,
        property_descriptor.metadata.public_name.clone(),
    );

    if let Some(existing) = property_slots.get_mut(&key) {
        if let Err(message) = existing.merge(property_descriptor) {
            return Err(syn::Error::new_spanned(&method.sig.ident, message));
        }
        return Ok(());
    }

    property_slots.insert(key, property_descriptor.clone());
    Ok(())
}

fn generate_receiver_wrapper(
    func: &ItemFn,
    wrapper_name: &syn::Ident,
    self_ty: &Type,
    receiver: MethodReceiver,
) -> TokenStream {
    let return_type = &func.sig.output;
    let wrapper_return = build_wrapper_return(return_type);
    let param_error_return = build_param_error_return(return_type);
    let regular_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| matches!(classify_receiver_arg(arg), ReceiverArgKind::Regular))
        .collect();
    let conversions = generate_param_conversions(&regular_params, &param_error_return);
    let wrapper_params = regular_wrapper_params(func);
    let injected_env_bindings = generate_receiver_env_bindings(func);
    let call_args = build_receiver_call_args(func);
    let method_name = &func.sig.ident;
    let receiver_load_error = generate_throw_error_and_return(&param_error_return);
    let writeback = if receiver.is_mut() {
        let writeback_error = generate_throw_error_and_return(&param_error_return);
        quote! {
            if let Err(e) = ani::conversions::WriteBackToAniObject::write_back_to_ani_object(
                __ani_self,
                &__ani_env,
                &__ani_this,
            ) {
                #writeback_error
            }
        }
    } else {
        quote! {}
    };
    let return_conversion = generate_return_conversion(return_type);

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(
            env: *mut ani::sys::ani_env,
            this: ani::sys::ani_object
            #(, #wrapper_params)*
        ) #wrapper_return {
            let __ani_env = ani::env::Env::from_raw_unchecked(env);
            let __ani_this = ani::types::AniObject::from_raw(this);
            let mut __ani_self: #self_ty = match ani::conversions::FromAni::from_ani(
                &__ani_env,
                this as <#self_ty as ani::conversions::FromAni<'_>>::Input,
            ) {
                Ok(value) => value,
                Err(e) => {
                    #receiver_load_error
                }
            };
            #injected_env_bindings
            #conversions
            let result = match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                __ani_self.#method_name(#(#call_args),*)
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
                    #param_error_return
                }
            };
            #writeback
            #return_conversion
        }
    }
}

fn generate_async_blocking_receiver_wrapper(
    func: &ItemFn,
    wrapper_name: &syn::Ident,
    self_ty: &Type,
    receiver: MethodReceiver,
) -> TokenStream {
    let return_type = &func.sig.output;
    let wrapper_return = build_wrapper_return(return_type);
    let param_error_return = build_param_error_return(return_type);
    let regular_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| matches!(classify_receiver_arg(arg), ReceiverArgKind::Regular))
        .collect();
    let conversions = generate_param_conversions(&regular_params, &param_error_return);
    let wrapper_params = regular_wrapper_params(func);
    let injected_env_bindings = generate_receiver_env_bindings(func);
    let call_args = build_receiver_call_args(func);
    let method_name = &func.sig.ident;
    let writeback = if receiver.is_mut() {
        quote! {
            ani::conversions::WriteBackToAniObject::write_back_to_ani_object(
                __ani_self,
                &__ani_env,
                &__ani_this,
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        quote! {}
    };
    let return_conversion = generate_return_conversion(return_type);

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(
            env: *mut ani::sys::ani_env,
            this: ani::sys::ani_object
            #(, #wrapper_params)*
        ) #wrapper_return {
            let __ani_env_outer = ani::env::Env::from_raw_unchecked(env);
            let __ani_this_container = {
                let __ani_this_ref =
                    unsafe { ani::types::AniRef::from_raw(this as ani::sys::ani_ref) };
                match ani::conversions::RefContainer::new(&__ani_env_outer, &__ani_this_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        let _ = ani::conversions::throw_error(&__ani_env_outer, &e.to_string());
                        #param_error_return
                    }
                }
            };
            #conversions
            let __ani_future = async move {
                let __ani_env = ani::env::Env::from_raw_unchecked(env);
                let env = __ani_env.as_raw();
                let __ani_this: ani::types::AniObject<'_> = __ani_this_container
                    .to_local(&__ani_env)
                    .map_err(|e| e.to_string())?;
                let mut __ani_self: #self_ty = #self_ty::__ani_from_bound_ani_object(
                    &__ani_env,
                    __ani_this.as_raw(),
                )
                .map_err(|e| e.to_string())?;
                #injected_env_bindings
                let result = __ani_self.#method_name(#(#call_args),*).await;
                #writeback
                result.map_err(|e| e.to_string())
            };
            let result = match ani::tokio::block_on_future_result(__ani_future) {
                Ok(result) => result,
                Err(e) => {
                    let _ = ani::conversions::throw_error(&__ani_env_outer, &e.to_string());
                    #param_error_return
                }
            };
            #return_conversion
        }
    }
}

fn generate_async_receiver_wrapper(
    func: &ItemFn,
    wrapper_name: &syn::Ident,
    self_ty: &Type,
    receiver: MethodReceiver,
) -> TokenStream {
    let regular_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| matches!(classify_receiver_arg(arg), ReceiverArgKind::Regular))
        .collect();
    let conversion_error = generate_promise_reject_and_return();
    let conversions =
        generate_param_conversions_with_custom_error(&regular_params, &conversion_error);
    let async_param_captures = generate_async_ref_container_captures(
        &regular_params,
        &format_ident!("__ani_env"),
        &conversion_error,
    );
    let wrapper_params = regular_wrapper_params(func);
    let injected_env_bindings = generate_receiver_env_bindings(func);
    let call_args = build_receiver_call_args(func);
    let method_name = &func.sig.ident;
    let async_param_restores =
        generate_async_ref_container_restores(&regular_params, &format_ident!("__ani_env"));
    let writeback = if receiver.is_mut() {
        quote! {
            ani::conversions::WriteBackToAniObject::write_back_to_ani_object(
                __ani_self,
                __ani_env,
                &__ani_this,
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        quote! {}
    };

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables, clippy::needless_lifetimes)]
        unsafe extern "C" fn #wrapper_name(
            env: *mut ani::sys::ani_env,
            this: ani::sys::ani_object
            #(, #wrapper_params)*
        ) -> ani::sys::ani_object {
            let __ani_env = ani::env::Env::from_raw_unchecked(env);
            let __ani_vm = match __ani_env.get_vm() {
                Ok(vm) => vm,
                Err(e) => {
                    #conversion_error
                }
            };
            let __ani_this_container = {
                let __ani_this_ref = unsafe { ani::types::AniRef::from_raw(this as ani::sys::ani_ref) };
                match ani::conversions::RefContainer::new(&__ani_env, &__ani_this_ref) {
                    Ok(value) => value,
                    Err(e) => {
                        #conversion_error
                    }
                }
            };
            #conversions
            #async_param_captures

            match ani::tokio::spawn_future_result_factory(&__ani_env, move || async move {
                let __ani_attach = __ani_vm.attach_current_thread_scoped().map_err(|e| e.to_string())?;
                let __ani_env = __ani_attach.env();
                let env = __ani_env.as_raw();
                let __ani_this: ani::types::AniObject<'_> = __ani_this_container
                    .to_local(&__ani_env)
                    .map_err(|e| e.to_string())?;
                let mut __ani_self: #self_ty = #self_ty::__ani_from_bound_ani_object(
                    __ani_env,
                    __ani_this.as_raw(),
                )
                .map_err(|e| e.to_string())?;
                #injected_env_bindings
                #async_param_restores
                let result = __ani_self.#method_name(#(#call_args),*).await;
                #writeback
                result.map_err(|e| e.to_string())
            }) {
                Ok(promise) => promise.into_raw(),
                Err(e) => {
                    match ani::conversions::PromiseRaw::<()>::reject(&__ani_env, e.to_string()) {
                        Ok(promise) => promise.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                }
            }
        }
    }
}

fn regular_wrapper_params(func: &ItemFn) -> Vec<TokenStream> {
    let mut params = Vec::new();

    for (i, param) in func
        .sig
        .inputs
        .iter()
        .filter(|arg| matches!(classify_receiver_arg(arg), ReceiverArgKind::Regular))
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

fn generate_receiver_env_bindings(func: &ItemFn) -> TokenStream {
    let mut vars = Vec::new();

    for arg in &func.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let Pat::Ident(pat_ident) = &*pat_type.pat else {
                continue;
            };
            let ident = &pat_ident.ident;
            let binding_ident = format_ident!("__ani_injected_{}", ident);
            if matches!(classify_receiver_arg(arg), ReceiverArgKind::InjectedEnv) {
                let ty = &pat_type.ty;
                let ty_str = quote!(#ty).to_string().replace(' ', "");
                if ty_str.starts_with('&') {
                    vars.push(quote! {
                        let #binding_ident = &__ani_env;
                    });
                } else {
                    vars.push(quote! {
                        let #binding_ident = ani::env::Env::from_raw_unchecked(env);
                    });
                }
            }
        }
    }

    quote! { #(#vars)* }
}

fn build_receiver_call_args(func: &ItemFn) -> Vec<TokenStream> {
    let mut args = Vec::new();

    for arg in &func.sig.inputs {
        match classify_receiver_arg(arg) {
            ReceiverArgKind::Receiver => {}
            ReceiverArgKind::InjectedEnv => {
                if let FnArg::Typed(pat_type) = arg
                    && let Pat::Ident(pat_ident) = &*pat_type.pat
                {
                    let ident = format_ident!("__ani_injected_{}", pat_ident.ident);
                    args.push(quote! { #ident });
                }
            }
            ReceiverArgKind::Regular => {
                if let FnArg::Typed(pat_type) = arg {
                    let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        format_ident!("{}_converted", pat_ident.ident)
                    } else {
                        format_ident!("arg_converted")
                    };
                    args.push(quote! { #param_name });
                }
            }
            ReceiverArgKind::InjectedThis | ReceiverArgKind::InjectedClass => {}
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

fn generate_throw_error_and_return(on_error_return: &TokenStream) -> TokenStream {
    quote! {
        let env_wrapper = ani::env::Env::from_raw_unchecked(env);
        let _ = ani::conversions::throw_error(&env_wrapper, &e.to_string());
        #on_error_return
    }
}

fn generate_promise_reject_and_return() -> TokenStream {
    quote! {
        return match ani::conversions::PromiseRaw::<()>::reject(&__ani_env, e.to_string()) {
            Ok(promise) => promise.into_raw(),
            Err(_) => std::ptr::null_mut(),
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReceiverArgKind {
    Receiver,
    InjectedEnv,
    InjectedThis,
    InjectedClass,
    Regular,
}

fn classify_receiver_arg(arg: &FnArg) -> ReceiverArgKind {
    match arg {
        FnArg::Receiver(_) => ReceiverArgKind::Receiver,
        FnArg::Typed(pat_type) => {
            let ty = &*pat_type.ty;
            let name = get_param_name(arg);

            if is_env_type(ty) {
                return ReceiverArgKind::InjectedEnv;
            }
            if is_this_type(ty) {
                return ReceiverArgKind::InjectedThis;
            }
            if let Some(name) = name.as_deref() {
                if name == "this" && is_ani_object_type(ty) {
                    return ReceiverArgKind::InjectedThis;
                }
                if (name == "class" || name == "_class") && is_class_type(ty) {
                    return ReceiverArgKind::InjectedClass;
                }
            }

            ReceiverArgKind::Regular
        }
    }
}

fn validate_receiver_usage(
    method: &syn::ImplItemFn,
    attrs: &BindgenAttrs,
    receiver: MethodReceiver,
) -> syn::Result<()> {
    if !receiver.has_receiver() {
        return Ok(());
    }

    if attrs.constructor {
        return Err(syn::Error::new_spanned(
            &method.sig,
            "#[ani(constructor)] cannot use a Rust self receiver; initialize instance state via injected `this` instead",
        ));
    }

    if attrs.is_static {
        return Err(syn::Error::new_spanned(
            &method.sig,
            "#[ani(static)] cannot be combined with a Rust self receiver",
        ));
    }

    for arg in &method.sig.inputs {
        match classify_receiver_arg(arg) {
            ReceiverArgKind::InjectedThis => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "Rust self receiver methods must not also request injected `this`; use `self` / `&mut self` only",
                ));
            }
            ReceiverArgKind::InjectedClass => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "Rust self receiver methods must not request injected `class`",
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

fn validate_accessor_backing_field_conflict(
    struct_name: &str,
    property_name: &str,
    method: &syn::ImplItemFn,
) -> syn::Result<()> {
    let Some(fields) = resolve_object_type_fields(struct_name) else {
        return Ok(());
    };

    if fields.iter().any(|field| field == property_name) {
        return Err(syn::Error::new_spanned(
            &method.sig,
            format!(
                "#[ani(getter)] / #[ani(setter)] property `{property_name}` conflicts with generated object backing field `{property_name}` on `{struct_name}`; use a distinct Rust backing field name such as `_{property_name}`"
            ),
        ));
    }

    Ok(())
}

fn analyze_method_receiver(method: &syn::ImplItemFn) -> syn::Result<MethodReceiver> {
    let Some(receiver) = method.sig.receiver() else {
        return Ok(MethodReceiver::None);
    };

    if receiver.reference.is_none() {
        return Ok(MethodReceiver::Owned);
    }

    if receiver.mutability.is_some() {
        Ok(MethodReceiver::RefMut)
    } else {
        Ok(MethodReceiver::Ref)
    }
}

fn extract_struct_name(impl_block: &ItemImpl) -> Result<String, TokenStream> {
    if let syn::Type::Path(type_path) = &*impl_block.self_ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return Ok(segment.ident.to_string());
    }
    Err(syn::Error::new_spanned(&impl_block.self_ty, "Expected type path").to_compile_error())
}

fn has_bind_marker(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("ani_bindgen") || attr.path().is_ident("ani"))
}

fn parse_method_bind_attrs(attrs: &[Attribute]) -> syn::Result<BindgenAttrs> {
    let mut merged = BindgenAttrs::default();
    for attr in attrs {
        if attr.path().is_ident("ani") {
            let parsed = parse_bindgen_attrs_from_attribute(attr)?;
            merged = merge_bindgen_attrs(&merged, &parsed, "");
        }
    }
    Ok(merged)
}

fn merge_bindgen_attrs(
    base: &BindgenAttrs,
    extra: &BindgenAttrs,
    default_class: &str,
) -> BindgenAttrs {
    BindgenAttrs {
        namespace: extra.namespace.clone().or_else(|| base.namespace.clone()),
        class: extra
            .class
            .clone()
            .or_else(|| base.class.clone())
            .or_else(|| (!default_class.is_empty()).then(|| default_class.to_string())),
        module: extra.module.clone().or_else(|| base.module.clone()),
        is_static: base.is_static || extra.is_static,
        name: extra.name.clone().or_else(|| base.name.clone()),
        signature: extra.signature.clone().or_else(|| base.signature.clone()),
        skip: base.skip || extra.skip,
        constructor: base.constructor || extra.constructor,
        getter: extra.getter.clone().or_else(|| base.getter.clone()),
        setter: extra.setter.clone().or_else(|| base.setter.clone()),
        is_async: base.is_async || extra.is_async,
    }
}

fn sanitize_method(method: &syn::ImplItemFn) -> syn::ImplItemFn {
    let mut sanitized = method.clone();
    sanitized
        .attrs
        .retain(|attr| !attr.path().is_ident("ani_bindgen") && !attr.path().is_ident("ani"));
    sanitized
}

fn to_item_fn(method: &syn::ImplItemFn) -> ItemFn {
    ItemFn {
        attrs: method.attrs.clone(),
        vis: method.vis.clone(),
        sig: method.sig.clone(),
        block: Box::new(method.block.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn impl_expansion_registers_via_ctor_and_emits_queue_entries() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl Widget {
                #[ani]
                fn get_name() -> String {
                    "ok".to_string()
                }

                #[ani(static)]
                fn sum(a: i32, b: i32) -> i32 {
                    a + b
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("register_module_export"));
        assert!(expanded.contains("queue_binding"));
        assert!(expanded.contains("BindingTarget :: Class"));
        assert!(expanded.contains("ClassBindingScope :: Instance"));
        assert!(expanded.contains("ClassBindingScope :: Static"));
        assert!(expanded.contains("get_name"));
        assert!(expanded.contains("sum"));
        assert!(expanded.contains("ii:i"));
    }

    #[test]
    fn impl_expansion_supports_receiver_methods() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl Widget {
                #[ani]
                fn get_name(&self) -> String {
                    self.name.clone()
                }

                #[ani]
                fn bump(&mut self, delta: i32) -> i32 {
                    self.count += delta;
                    self.count
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("FromAni :: from_ani"));
        assert!(expanded.contains("__ani_self . get_name"));
        assert!(expanded.contains("write_back_to_ani_object"));
    }

    #[test]
    fn impl_expansion_supports_owned_receiver_methods() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl Widget {
                #[ani]
                fn consume(self) -> String {
                    self.name
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("FromAni :: from_ani"));
        assert!(expanded.contains("__ani_self . consume"));
        assert!(!expanded.contains("write_back_to_ani_object"));
    }

    #[test]
    fn impl_expansion_rejects_accessor_backing_field_conflicts() {
        crate::types::ani_type::register_object_type_fields(
            "ConflictWidget",
            &["count".to_string(), "name".to_string()],
        );

        let attrs = BindgenAttrs {
            class: Some("ConflictWidget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl ConflictWidget {
                #[ani(getter)]
                fn get_count(&self) -> i32 {
                    self.count
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("conflicts with generated object backing field `count`"));
    }

    #[test]
    fn impl_expansion_allows_getter_setter_pair_for_same_property() {
        let attrs = BindgenAttrs {
            class: Some("PairWidget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl PairWidget {
                #[ani(getter = "count")]
                fn get_count(&self) -> i32 {
                    self.count
                }

                #[ani(setter = "count")]
                fn set_count(&mut self, value: i32) {
                    self.count = value;
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("__ani_native_get_count"));
        assert!(expanded.contains("__ani_native_set_count"));
        assert!(!expanded.contains("compile_error"));
    }

    #[test]
    fn impl_expansion_rejects_duplicate_property_getters() {
        let attrs = BindgenAttrs {
            class: Some("DuplicateGetterWidget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl DuplicateGetterWidget {
                #[ani(getter = "count")]
                fn get_count(&self) -> i32 {
                    self.count
                }

                #[ani(getter = "count")]
                fn read_count(&self) -> i32 {
                    self.count
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("compile_error"));
        assert!(expanded.contains("duplicate"));
        assert!(expanded.contains("getter"));
        assert!(expanded.contains("count"));
        assert!(expanded.contains("DuplicateGetterWidget"));
    }
}
