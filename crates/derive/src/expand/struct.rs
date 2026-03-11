//! Struct Expansion
//!
//! Expands `#[ani]` macro for structs and derive macros.

use std::collections::BTreeSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident,
    ItemStruct, Token,
};

use crate::parser::{AniAttrs, AttrItem, AttrValue, BindgenAttrs};
use crate::types::ani_type::{
    register_object_type_alias, register_object_type_members, ObjectMemberAccessKind,
    ObjectMemberDescriptor,
};
use crate::types::{
    current_module_name, emit_compile_ets_object, generate_object_field_ets_decl,
    generate_object_property_ets_decl, qualify_member_descriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectAccessKind {
    Field,
    Property,
}

#[derive(Clone)]
struct ObjectFieldSpec {
    rust_ident: Ident,
    arkts_name: String,
    ty: syn::Type,
    access: ObjectAccessKind,
}

#[derive(Default)]
struct ObjectFieldAttrs {
    name: Option<String>,
    property: bool,
    property_name: Option<String>,
}

/// Expand `#[ani]` for structs
pub fn expand_struct(
    attrs: BindgenAttrs,
    struct_item: ItemStruct,
    prepare: TokenStream,
) -> TokenStream {
    let fields = match validate_object_item_struct(&struct_item) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error(),
    };

    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    let object_impls = expand_object_type_impls(struct_name, &class_name, fields);

    quote! {
        #prepare
        #struct_item
        #object_impls
    }
}

fn expand_object_type_impls(
    struct_name: &syn::Ident,
    class_name: &str,
    fields: &FieldsNamed,
) -> TokenStream {
    let field_specs = match collect_object_field_specs(fields) {
        Ok(specs) => specs,
        Err(err) => return err.to_compile_error(),
    };

    register_object_type_alias(&struct_name.to_string(), class_name);
    register_object_type_members(
        &struct_name.to_string(),
        &field_specs
            .iter()
            .map(|field| ObjectMemberDescriptor {
                rust_name: field.rust_ident.to_string(),
                arkts_name: field.arkts_name.clone(),
                access: match field.access {
                    ObjectAccessKind::Field => ObjectMemberAccessKind::Field,
                    ObjectAccessKind::Property => ObjectMemberAccessKind::Property,
                },
            })
            .collect::<Vec<_>>(),
    );
    emit_object_decl(class_name, &field_specs);

    let module_name = current_module_name();
    let qualified_name = qualify_member_descriptor(class_name, &module_name);
    let class_descriptor = object_descriptor_signature(&qualified_name);
    let qualified_name_lit = syn::LitStr::new(&qualified_name, Span::call_site());
    let class_descriptor_lit = syn::LitStr::new(&class_descriptor, Span::call_site());

    let field_reads = field_specs.iter().map(generate_field_read);
    let field_writes_to_new = field_specs.iter().map(generate_field_write_to_new);
    let field_writes_back = field_specs.iter().map(generate_field_write_back);

    quote! {
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
                #(#field_writes_to_new)*
                Ok(obj.into_raw())
            }
        }

        impl<'env> ani::conversions::WriteBackToAniObject<'env> for #struct_name {
            fn write_back_to_ani_object(
                self,
                env: &ani::env::Env<'env>,
                obj: &ani::types::AniObject<'_>,
            ) -> ani::error::Result<()> {
                #(#field_writes_back)*
                Ok(())
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

fn generate_field_read(field: &ObjectFieldSpec) -> TokenStream {
    let field_name = &field.rust_ident;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, field_name.span());
    match field.access {
        ObjectAccessKind::Field => quote! {
            #field_name: <#field_ty as ani::conversions::ObjectField<'env>>::get_named_field(
                env,
                &obj,
                #field_literal,
            )?
        },
        ObjectAccessKind::Property => quote! {
            #field_name: <#field_ty as ani::conversions::ObjectProperty<'env>>::get_named_property(
                env,
                &obj,
                #field_literal,
            )?
        },
    }
}

fn generate_field_write_to_new(field: &ObjectFieldSpec) -> TokenStream {
    let field_name = &field.rust_ident;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, field_name.span());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#field_name,
                env,
                &obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#field_name,
                env,
                &obj,
                #field_literal,
            )?;
        },
    }
}

fn generate_field_write_back(field: &ObjectFieldSpec) -> TokenStream {
    let field_name = &field.rust_ident;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, field_name.span());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#field_name,
                env,
                obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#field_name,
                env,
                obj,
                #field_literal,
            )?;
        },
    }
}

fn emit_object_decl(class_name: &str, fields: &[ObjectFieldSpec]) {
    let object_field_decls = fields
        .iter()
        .map(|field| match field.access {
            ObjectAccessKind::Field => generate_object_field_ets_decl(&field.arkts_name, &field.ty),
            ObjectAccessKind::Property => {
                generate_object_property_ets_decl(&field.arkts_name, &field.ty)
            }
        })
        .collect::<Vec<_>>();
    emit_compile_ets_object(class_name, &object_field_decls);
}

fn collect_object_field_specs(fields: &FieldsNamed) -> syn::Result<Vec<ObjectFieldSpec>> {
    let mut out = Vec::with_capacity(fields.named.len());
    let mut member_names = BTreeSet::new();

    for field in &fields.named {
        let Some(rust_ident) = field.ident.clone() else {
            continue;
        };
        let attrs = parse_object_field_attrs(field)?;
        let access = if attrs.property {
            ObjectAccessKind::Property
        } else {
            ObjectAccessKind::Field
        };
        let arkts_name = attrs
            .property_name
            .or(attrs.name)
            .unwrap_or_else(|| rust_ident.to_string());
        if !member_names.insert(arkts_name.clone()) {
            return Err(syn::Error::new_spanned(
                &rust_ident,
                format!("duplicate ArkTS object member name `{arkts_name}`"),
            ));
        }
        out.push(ObjectFieldSpec {
            rust_ident,
            arkts_name,
            ty: field.ty.clone(),
            access,
        });
    }

    Ok(out)
}

fn parse_object_field_attrs(field: &Field) -> syn::Result<ObjectFieldAttrs> {
    let mut out = ObjectFieldAttrs::default();

    for attr in &field.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let items = attr.parse_args_with(Punctuated::<AttrItem, Token![,]>::parse_terminated)?;
        for item in items {
            match item.key.to_string().as_str() {
                "name" => match item.value {
                    Some(AttrValue::Str(value)) => out.name = Some(value),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &item.key,
                            "#[ani(name = \"...\")] on object fields requires a string literal",
                        ));
                    }
                },
                "property" => {
                    out.property = true;
                    if let Some(AttrValue::Str(value)) = item.value {
                        out.property_name = Some(value);
                    } else if item.value.is_some() {
                        return Err(syn::Error::new_spanned(
                            &item.key,
                            "#[ani(property = \"...\")] on object fields requires a string literal",
                        ));
                    }
                }
                "getter" | "setter" | "constructor" | "module" | "namespace" | "class"
                | "static" | "async" => {
                    return Err(syn::Error::new_spanned(
                        &item.key,
                        format!("#[ani({})] is not supported on object fields", item.key),
                    ));
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        &item.key,
                        format!("Unknown object field attribute: {other}"),
                    ));
                }
            }
        }
    }

    Ok(out)
}

fn validate_object_struct_shape<'a>(
    generics: &'a Generics,
    fields: &'a Fields,
    generic_error: &str,
    fields_error: &str,
) -> syn::Result<&'a FieldsNamed> {
    if !generics.params.is_empty() {
        return Err(syn::Error::new_spanned(generics, generic_error));
    }

    match fields {
        Fields::Named(fields) => Ok(fields),
        Fields::Unnamed(_) | Fields::Unit => Err(syn::Error::new_spanned(fields, fields_error)),
    }
}

fn validate_object_item_struct(struct_item: &ItemStruct) -> syn::Result<&FieldsNamed> {
    validate_object_struct_shape(
        &struct_item.generics,
        &struct_item.fields,
        "#[ani(object)] does not support generic structs yet",
        "#[ani(object)] currently only supports structs with named fields",
    )
}

fn object_descriptor_signature(qualified_name: &str) -> String {
    if qualified_name.starts_with('L') && qualified_name.ends_with(';') {
        qualified_name.to_string()
    } else {
        format!("L{};", qualified_name.replace('.', "/"))
    }
}

fn derive_object_name(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let parsed = attr.parse_args::<AniAttrs>()?;
        if let Some(name) = parsed.object_name {
            return Ok(name);
        }
        if let Some(name) = parsed.class {
            return Ok(name);
        }
    }

    Ok(input.ident.to_string())
}

fn validate_derive_input(input: &DeriveInput) -> syn::Result<&FieldsNamed> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(AniClass)] can only be applied to structs",
        ));
    };

    validate_object_struct_shape(
        &input.generics,
        &data.fields,
        "#[derive(AniClass)] does not support generic structs yet",
        "#[derive(AniClass)] currently only supports structs with named fields",
    )
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    let fields = match validate_derive_input(&input) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error(),
    };

    let class_name = match derive_object_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error(),
    };

    let name = &input.ident;
    expand_object_type_impls(name, &class_name, fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn derive_ani_class_uses_explicit_class_name() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            #[ani(class = "models.ExplicitDerivedProfile")]
            struct ExplicitDerivedProfile {
                id: i32,
                name: String,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("impl ani :: conversions :: TypeInfo for ExplicitDerivedProfile"));
        assert!(expanded.contains("pub const fn arkts_name () -> & 'static str"));
        assert!(expanded.contains("models.ExplicitDerivedProfile"));
        assert!(expanded.contains(
            "impl < 'env > ani :: conversions :: FromAni < 'env > for ExplicitDerivedProfile"
        ));
        assert!(expanded.contains(
            "impl < 'env > ani :: conversions :: ToAni < 'env > for ExplicitDerivedProfile"
        ));
    }

    #[test]
    fn derive_ani_class_supports_object_name_override() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            #[ani(object = "models.ObjectAlias")]
            struct ObjectAlias {
                id: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("models.ObjectAlias"));
        assert!(expanded.contains("impl ani :: conversions :: TypeInfo for ObjectAlias"));
    }

    #[test]
    fn derive_ani_class_supports_field_level_property_metadata() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct PropertyObject {
                #[ani(property)]
                name: String,
                value: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("ObjectProperty"));
        assert!(expanded.contains("get_named_property"));
        assert!(expanded.contains("set_named_property"));
    }

    #[test]
    fn derive_ani_class_rejects_tuple_structs() {
        let input: DeriveInput = parse_quote! {
            struct TupleUser(i32, String);
        };
        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("currently only supports structs with named fields"));
    }
}
