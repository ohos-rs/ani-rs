//! Impl Block Expansion
//!
//! Expands `#[ani]` macro for impl blocks.

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, ItemImpl};

use crate::codegen::generate_wrapper;
use crate::parser::BindgenAttrs;
use crate::types::{class_to_descriptor, generate_fn_signature};

/// Expand `#[ani]` for impl blocks
pub fn expand_impl(attrs: BindgenAttrs, impl_block: ItemImpl, prepare: TokenStream) -> TokenStream {
    let struct_name = match extract_struct_name(&impl_block) {
        Ok(name) => name,
        Err(err) => return err,
    };

    let class_name = attrs.class.clone().unwrap_or_else(|| struct_name.clone());
    let class_descriptor = class_to_descriptor(&class_name);

    let mut wrappers = Vec::new();
    let mut method_entries = Vec::new();
    let mut original_methods = Vec::new();

    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            let has_bindgen = method
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("ani_bindgen"));

            if !has_bindgen {
                original_methods.push(quote! { #method });
                continue;
            }

            let (wrapper, entry) = process_method(&struct_name, method);
            wrappers.push(wrapper);
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
        pub unsafe fn #bind_fn_name(env: *mut ani::sys::ani_env) -> ani::sys::ani_status {
            let mut cls: ani::sys::ani_class = std::ptr::null_mut();
            let class_name = concat!(#class_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindClass.unwrap())(
                env,
                class_name.as_ptr() as *const std::os::raw::c_char,
                &mut cls
            );

            if status != ani::sys::ani_status_ANI_OK {
                return status;
            }

            let methods: [ani::sys::ani_native_function; #methods_count] = [
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

/// Extract struct name from impl block
fn extract_struct_name(impl_block: &ItemImpl) -> Result<String, TokenStream> {
    if let syn::Type::Path(type_path) = &*impl_block.self_ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Ok(segment.ident.to_string());
        }
    }
    Err(syn::Error::new_spanned(&impl_block.self_ty, "Expected type path").to_compile_error())
}

/// Process a single method in impl block
fn process_method(struct_name: &str, method: &syn::ImplItemFn) -> (TokenStream, TokenStream) {
    let method_name = &method.sig.ident;
    let ets_name = method_name.to_string().to_case(Case::Camel);

    let has_self = method
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

    let is_static = !has_self;
    let signature = generate_fn_signature(&method.sig, true);
    let wrapper_name = format_ident!("__ani_{}_{}", struct_name, method_name);

    // Create ItemFn for wrapper generation
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

    let entry = quote! {
        ani::sys::ani_native_function {
            name: concat!(#ets_name, "\0").as_ptr() as *const std::os::raw::c_char,
            signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
            pointer: #wrapper_name as *const std::os::raw::c_void,
        }
    };

    (wrapper, entry)
}
