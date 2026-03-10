//! Impl Block Expansion
//!
//! Expands `#[ani]` macro for impl blocks.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, FnArg, ImplItem, ItemFn, ItemImpl, ReturnType};

use crate::codegen::{generate_register_call, generate_wrapper_with_target};
use crate::parser::{BindgenAttrs, parse_bindgen_attrs_from_attribute};

use super::function::{
    emit_binding_plan_ets, resolve_binding_plan, validate_constructor_usage,
    validate_unsupported_bind_attrs,
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

    for item in &impl_block.items {
        match item {
            ImplItem::Fn(method) => {
                if !has_bind_marker(&method.attrs) {
                    original_items.push(quote! { #method });
                    continue;
                }

                match process_method(&attrs, &impl_block, &struct_name, method) {
                    Ok(processed) => {
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
}

fn process_method(
    impl_attrs: &BindgenAttrs,
    impl_block: &ItemImpl,
    struct_name: &str,
    method: &syn::ImplItemFn,
) -> syn::Result<ProcessedMethod> {
    validate_receiver_free_method(method)?;

    let method_attrs = parse_method_bind_attrs(&method.attrs)?;
    let merged_attrs = merge_bindgen_attrs(impl_attrs, &method_attrs, struct_name);

    let method_fn = to_item_fn(method);
    validate_unsupported_bind_attrs(&merged_attrs, &method_fn)?;
    validate_constructor_usage(&merged_attrs, &method_fn)?;

    let method_name = &method.sig.ident;
    let is_constructor = merged_attrs.constructor;
    let is_static = if is_constructor {
        false
    } else {
        merged_attrs.is_static
    };

    let binding = resolve_binding_plan(
        &merged_attrs,
        &method_name.to_string(),
        &method.sig,
        is_static,
        is_constructor,
        false,
    )?;
    emit_binding_plan_ets(&binding);

    let wrapper_name = format_ident!("__ani_{}_{}", struct_name, method_name);
    let call_target = {
        let self_ty = &impl_block.self_ty;
        quote! { <#self_ty>::#method_name }
    };
    let sanitized_method = sanitize_method(method);
    let wrapper = generate_wrapper_with_target(
        &to_item_fn(&sanitized_method),
        &wrapper_name,
        true,
        is_static,
        call_target,
    );

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
    })
}

fn extract_struct_name(impl_block: &ItemImpl) -> Result<String, TokenStream> {
    if let syn::Type::Path(type_path) = &*impl_block.self_ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Ok(segment.ident.to_string());
        }
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

fn validate_receiver_free_method(method: &syn::ImplItemFn) -> syn::Result<()> {
    if method
        .sig
        .inputs
        .iter()
        .any(|arg| matches!(arg, FnArg::Receiver(_)))
    {
        return Err(syn::Error::new_spanned(
            &method.sig,
            "#[ani] impl methods do not support Rust self receivers yet; use associated functions with injected `this` instead",
        ));
    }
    if let ReturnType::Type(_, _) = &method.sig.output {
        return Ok(());
    }
    Ok(())
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
        assert!(expanded.contains("queue_class_binding"));
        assert!(expanded.contains("get_name"));
        assert!(expanded.contains("sum"));
        assert!(expanded.contains("ii:i"));
    }

    #[test]
    fn impl_expansion_rejects_receiver_methods() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            ..Default::default()
        };
        let impl_block: ItemImpl = parse_quote! {
            impl Widget {
                #[ani]
                fn get_name(&self) -> String {
                    "ok".to_string()
                }
            }
        };

        let expanded = expand_impl(attrs, impl_block, TokenStream::new()).to_string();
        assert!(expanded.contains("do not support Rust self receivers yet"));
    }
}
