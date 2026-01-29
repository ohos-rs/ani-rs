//! Macro Expansion Implementation

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Fields, FnArg, ItemFn, ItemImpl, ItemStruct};

use crate::attrs::*;
use crate::codegen::*;
use crate::signature::*;

/// Expand ani_bindgen for functions
/// The `prepare` parameter contains auto-generated ANI_Constructor code (only for first invocation)
pub fn expand_function(attrs: AniBindgenAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    if attrs.skip {
        return quote! { #prepare #func };
    }

    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();

    // 确定 ArkTS 中的函数名（驼峰命名）
    let ets_name = attrs
        .name
        .clone()
        .unwrap_or_else(|| func_name.to_string().to_case(Case::Camel));

    // 检查是否有 self 参数（实例方法）
    let has_self = func
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

    let is_static = attrs.is_static || !has_self;
    let is_class_method = attrs.class.is_some();

    // 生成签名
    let signature = attrs
        .signature
        .clone()
        .unwrap_or_else(|| generate_signature_from_fn(&func.sig, has_self || is_class_method));

    // 生成 wrapper 函数名
    let wrapper_name = format_ident!("__ani_native_{}", func_name);
    let register_name = format_ident!("__ani_register_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_{}", func_name);

    // 生成 wrapper 函数
    let wrapper = generate_wrapper(&func, &wrapper_name, is_class_method, is_static);

    // 生成注册函数
    let register_fn = if let Some(ref class) = attrs.class {
        let descriptor = class_to_descriptor(class);
        generate_class_register(
            &register_name,
            &descriptor,
            &ets_name,
            &signature,
            &wrapper_name,
        )
    } else if let Some(ref ns) = attrs.namespace {
        let descriptor = namespace_to_descriptor(ns);
        generate_namespace_register(
            &register_name,
            &descriptor,
            &ets_name,
            &signature,
            &wrapper_name,
        )
    } else {
        let descriptor = attrs
            .module
            .as_ref()
            .map(|m| module_to_descriptor(m))
            .unwrap_or_default();
        generate_module_register(
            &register_name,
            &descriptor,
            &ets_name,
            &signature,
            &wrapper_name,
        )
    };

    // 生成 ctor 自动注册函数
    // 使用 #[ctor::ctor(crate_path = ::ani::ctor)] 在库加载时自动注册到全局注册表
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

/// Expand ani_bindgen for impl blocks
/// The `prepare` parameter contains auto-generated ANI_Constructor code (only for first invocation)
pub fn expand_impl(attrs: AniBindgenAttrs, impl_block: ItemImpl, prepare: TokenStream) -> TokenStream {
    let struct_name = if let syn::Type::Path(type_path) = &*impl_block.self_ty {
        type_path
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    } else {
        return syn::Error::new_spanned(impl_block.self_ty, "Expected type path")
            .to_compile_error();
    };

    let class_name = attrs.class.clone().unwrap_or_else(|| struct_name.clone());
    let class_descriptor = class_to_descriptor(&class_name);

    let mut wrappers = Vec::new();
    let mut method_entries = Vec::new();
    let mut original_methods = Vec::new();

    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            // 检查是否有 ani_bindgen 属性
            let has_bindgen = method
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("ani_bindgen"));

            if !has_bindgen {
                original_methods.push(quote! { #method });
                continue;
            }

            let method_name = &method.sig.ident;
            let ets_name = method_name.to_string().to_case(Case::Camel);

            // 检查是否有 self 参数
            let has_self = method
                .sig
                .inputs
                .first()
                .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

            let is_static = !has_self;

            // 生成签名
            let signature = generate_signature_from_method_sig(&method.sig, true);

            let wrapper_name = format_ident!("__ani_{}_{}", struct_name, method_name);

            // 创建 ItemFn 用于生成 wrapper
            let func = ItemFn {
                attrs: method
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|a| !a.path().is_ident("ani_bindgen"))
                    .collect(),
                vis: method.vis.clone(),
                sig: method.sig.clone(),
                block: Box::new(method.block.clone()),
            };

            let wrapper = generate_wrapper(&func, &wrapper_name, true, is_static);
            wrappers.push(wrapper);

            let entry = quote! {
                ani_sys::ani_native_function {
                    name: concat!(#ets_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            };
            method_entries.push(entry);

            original_methods.push(quote! { #method });
        }
    }

    let methods_count = method_entries.len();
    let bind_fn_name = format_ident!("__ani_bind_{}", struct_name.to_lowercase());

    let impl_attrs = &impl_block.attrs;
    let impl_generics = &impl_block.generics;
    let impl_self_ty = &impl_block.self_ty;

    quote! {
        #prepare

        #(#impl_attrs)*
        impl #impl_generics #impl_self_ty {
            #(#original_methods)*
        }

        #(#wrappers)*

        #[doc(hidden)]
        pub unsafe fn #bind_fn_name(env: *mut ani_sys::ani_env) -> ani_sys::ani_status {
            let mut cls: ani_sys::ani_class = std::ptr::null_mut();
            let class_name = concat!(#class_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindClass.unwrap())(
                env,
                class_name.as_ptr() as *const std::os::raw::c_char,
                &mut cls
            );

            if status != ani_sys::ani_status_ANI_OK {
                return status;
            }

            let methods: [ani_sys::ani_native_function; #methods_count] = [
                #(#method_entries),*
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

/// Expand ani_bindgen for structs
/// The `prepare` parameter contains auto-generated ANI_Constructor code (only for first invocation)
pub fn expand_struct(attrs: AniBindgenAttrs, struct_item: ItemStruct, prepare: TokenStream) -> TokenStream {
    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    let class_descriptor = class_to_descriptor(&class_name);

    // 生成字段的 getter/setter
    let _field_accessors = match &struct_item.fields {
        Fields::Named(fields) => {
            let mut accessors = Vec::new();
            for field in &fields.named {
                if let Some(_field_name) = &field.ident {
                    let _field_type = &field.ty;
                    // 检查字段属性
                    let has_getter = field.attrs.iter().any(|a| {
                        a.path().is_ident("ani")
                            && a.to_token_stream().to_string().contains("getter")
                    });
                    let has_setter = field.attrs.iter().any(|a| {
                        a.path().is_ident("ani")
                            && a.to_token_stream().to_string().contains("setter")
                    });

                    if has_getter || has_setter {
                        // TODO: 生成 getter/setter
                        accessors.push(quote! {});
                    }
                }
            }
            accessors
        }
        _ => Vec::new(),
    };

    quote! {
        #prepare

        #struct_item

        impl #struct_name {
            /// Get class descriptor
            pub const fn class_descriptor() -> &'static str {
                #class_descriptor
            }
        }
    }
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let class_name = name.to_string();
    let class_descriptor = class_to_descriptor(&class_name);

    quote! {
        impl ani::AniBindable for #name {
            fn class_descriptor() -> &'static str {
                #class_descriptor
            }
        }
    }
}

/// Expand ani_init attribute
/// The `prepare` parameter contains auto-generated ANI_Constructor code (only for first invocation)
pub fn expand_init(_attrs: AniInitAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
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

// ============================================================================
// Helper Traits
// ============================================================================

trait ToTokenStreamExt {
    fn to_token_stream(&self) -> TokenStream;
}

impl ToTokenStreamExt for syn::Attribute {
    fn to_token_stream(&self) -> TokenStream {
        quote! { #self }
    }
}
