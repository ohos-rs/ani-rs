//! Struct Expansion
//!
//! Expands `#[ani]` macro for structs and derive macros.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, ItemStruct};

use crate::parser::BindgenAttrs;
use crate::types::{class_to_descriptor, current_module_name, qualify_member_descriptor};

/// Expand `#[ani]` for structs
pub fn expand_struct(
    attrs: BindgenAttrs,
    struct_item: ItemStruct,
    prepare: TokenStream,
) -> TokenStream {
    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    let module_name = current_module_name();
    let class_descriptor =
        class_to_descriptor(&qualify_member_descriptor(&class_name, &module_name));

    // Generate field accessors (placeholder for future implementation)
    let _field_accessors = generate_field_accessors(&struct_item);

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

/// Generate field getter/setter accessors
fn generate_field_accessors(struct_item: &ItemStruct) -> Vec<TokenStream> {
    let Fields::Named(fields) = &struct_item.fields else {
        return Vec::new();
    };

    let mut accessors = Vec::new();

    for field in &fields.named {
        let Some(_field_name) = &field.ident else {
            continue;
        };

        let has_getter = field.attrs.iter().any(|a| {
            a.path().is_ident("ani") && a.to_token_stream().to_string().contains("getter")
        });
        let has_setter = field.attrs.iter().any(|a| {
            a.path().is_ident("ani") && a.to_token_stream().to_string().contains("setter")
        });

        if has_getter || has_setter {
            // TODO: Generate getter/setter implementations
            accessors.push(quote! {});
        }
    }

    accessors
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let class_name = name.to_string();
    let module_name = current_module_name();
    let class_descriptor =
        class_to_descriptor(&qualify_member_descriptor(&class_name, &module_name));

    quote! {
        impl ani::AniBindable for #name {
            fn class_descriptor() -> &'static str {
                #class_descriptor
            }
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
