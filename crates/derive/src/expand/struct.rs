//! Struct Expansion
//!
//! Expands `#[ani]` macro for structs and derive macros.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, ItemStruct};

use crate::parser::BindgenAttrs;
use crate::types::ani_type::register_object_type_alias;
use crate::types::{
    current_module_name, emit_compile_ets_object, generate_object_field_ets_decl,
    qualify_member_descriptor,
};

/// Expand `#[ani]` for structs
pub fn expand_struct(
    attrs: BindgenAttrs,
    struct_item: ItemStruct,
    prepare: TokenStream,
) -> TokenStream {
    if let Err(err) = validate_object_struct(&struct_item) {
        return err.to_compile_error();
    }

    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    register_object_type_alias(&struct_name.to_string(), &class_name);
    let module_name = current_module_name();
    let qualified_name = qualify_member_descriptor(&class_name, &module_name);
    let class_descriptor = object_descriptor_signature(&qualified_name);
    let qualified_name_lit = syn::LitStr::new(&qualified_name, Span::call_site());
    let class_descriptor_lit = syn::LitStr::new(&class_descriptor, Span::call_site());

    let Fields::Named(fields) = &struct_item.fields else {
        return syn::Error::new_spanned(
            &struct_item,
            "#[ani(object)] currently only supports structs with named fields",
        )
        .to_compile_error();
    };

    let object_field_decls = fields
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("named field").to_string();
            generate_object_field_ets_decl(&field_name, &field.ty)
        })
        .collect::<Vec<_>>();
    emit_compile_ets_object(&class_name, &object_field_decls);

    let field_reads = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("named field");
        let field_ty = &field.ty;
        let field_literal = syn::LitStr::new(&field_name.to_string(), field_name.span());
        quote! {
            #field_name: <#field_ty as ani::conversions::ObjectField<'env>>::get_named_field(
                env,
                &obj,
                #field_literal,
            )?
        }
    });

    let field_writes = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("named field");
        let field_ty = &field.ty;
        let field_literal = syn::LitStr::new(&field_name.to_string(), field_name.span());
        quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#field_name,
                env,
                &obj,
                #field_literal,
            )?;
        }
    });

    quote! {
        #prepare

        #struct_item

        impl #struct_name {
            /// ANI class descriptor usable with `Env::find_class`.
            pub const fn class_descriptor() -> &'static str {
                #class_descriptor_lit
            }

            /// Qualified ArkTS object name.
            pub const fn arkts_name() -> &'static str {
                #qualified_name_lit
            }
        }

        impl ani::conversions::TypeInfo for #struct_name {
            fn type_signature() -> &'static str {
                Self::class_descriptor()
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        impl<'env> ani::conversions::FromAni<'env> for #struct_name {
            type Input = ani::sys::ani_object;

            fn from_ani(env: &ani::env::Env<'env>, value: Self::Input) -> ani::error::Result<Self> {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#struct_name)),
                    ));
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                let class = env.find_class(Self::arkts_name())?;
                if !env.object_instance_of(&obj, &class)? {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidType,
                        format!(
                            "Expected object of type {}, got incompatible instance",
                            Self::arkts_name(),
                        ),
                    ));
                }

                Ok(Self {
                    #(#field_reads,)*
                })
            }
        }

        impl<'env> ani::conversions::ToAni<'env> for #struct_name {
            type Output = ani::sys::ani_object;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let class = env.find_class(Self::arkts_name())?;
                let ctor = env.find_constructor(&class, ":")?;
                let obj = env.new_object(&class, &ctor, &[])?;
                #(#field_writes)*
                Ok(obj.into_raw())
            }
        }

        impl<'env> ani::conversions::ValidateFromAni<'env> for #struct_name {
            fn validate(env: &ani::env::Env<'env>, value: ani::sys::ani_object) -> bool {
                if value.is_null() {
                    return false;
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                let class = match env.find_class(Self::arkts_name()) {
                    Ok(class) => class,
                    Err(_) => return false,
                };
                env.object_instance_of(&obj, &class).unwrap_or(false)
            }
        }

        impl<'env> ani::conversions::FromAniObject<'env> for #struct_name {
            fn from_ani_object(
                env: &ani::env::Env<'env>,
                value: ani::sys::ani_object,
            ) -> ani::error::Result<Self> {
                <Self as ani::conversions::FromAni<'env>>::from_ani(env, value)
            }
        }

        impl<'env> ani::conversions::ToAniObject<'env> for #struct_name {
            fn to_ani_object(self, env: &ani::env::Env<'env>) -> ani::error::Result<ani::sys::ani_object> {
                <Self as ani::conversions::ToAni<'env>>::to_ani(self, env)
            }
        }
    }
}

fn validate_object_struct(struct_item: &ItemStruct) -> syn::Result<()> {
    validate_no_field_getter_setter(struct_item)?;

    if !struct_item.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &struct_item.generics,
            "#[ani(object)] does not support generic structs yet",
        ));
    }

    match &struct_item.fields {
        Fields::Named(_) => Ok(()),
        Fields::Unnamed(_) | Fields::Unit => Err(syn::Error::new_spanned(
            &struct_item.fields,
            "#[ani(object)] currently only supports structs with named fields",
        )),
    }
}

fn object_descriptor_signature(qualified_name: &str) -> String {
    if qualified_name.starts_with('L') && qualified_name.ends_with(';') {
        qualified_name.to_string()
    } else {
        format!("L{};", qualified_name.replace('.', "/"))
    }
}

fn validate_no_field_getter_setter(struct_item: &ItemStruct) -> syn::Result<()> {
    let Fields::Named(fields) = &struct_item.fields else {
        return Ok(());
    };

    for field in &fields.named {
        let Some(field_name) = &field.ident else {
            continue;
        };

        let has_getter_or_setter = field.attrs.iter().any(has_getter_or_setter_mark);
        if has_getter_or_setter {
            return Err(syn::Error::new_spanned(
                field_name,
                "#[ani(getter)] / #[ani(setter)] on struct fields are not implemented yet",
            ));
        }
    }

    Ok(())
}

fn has_getter_or_setter_mark(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("ani") {
        return false;
    }
    let text = quote!(#attr).to_string();
    text.contains("getter") || text.contains("setter")
}

fn validate_derive_input(input: &DeriveInput) -> syn::Result<()> {
    if let Data::Struct(data) = &input.data {
        let Fields::Named(fields) = &data.fields else {
            return Ok(());
        };

        for field in &fields.named {
            let Some(field_name) = &field.ident else {
                continue;
            };
            if field.attrs.iter().any(has_getter_or_setter_mark) {
                return Err(syn::Error::new_spanned(
                    field_name,
                    "#[derive(AniClass)] field getter/setter generation is not implemented yet",
                ));
            }
        }
    }

    Ok(())
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    if let Err(err) = validate_derive_input(&input) {
        return err.to_compile_error();
    }

    let name = &input.ident;
    let class_name = name.to_string();
    let module_name = current_module_name();
    let qualified_name = qualify_member_descriptor(&class_name, &module_name);
    let class_descriptor = object_descriptor_signature(&qualified_name);
    let descriptor_lit = syn::LitStr::new(&class_descriptor, Span::call_site());

    quote! {
        impl ani::conversions::TypeInfo for #name {
            fn type_signature() -> &'static str {
                #descriptor_lit
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }
    }
}
