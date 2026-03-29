//! Struct Expansion
//!
//! Expands `#[ani]` macro for structs and derive macros.

use std::collections::BTreeSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Field, Fields, GenericParam, Generics, Ident, Index,
    ItemStruct, Lit, Member, Token, Variant, punctuated::Punctuated,
};

use crate::parser::{AniAttrs, AttrItem, AttrValue, BindgenAttrs};
use crate::types::ani_type::{
    ObjectMemberAccessKind, ObjectMemberDescriptor, register_object_type_alias,
    register_object_type_members,
};
use crate::types::{
    EtsDeclKind, EtsObjectMemberDecl, EtsObjectMemberKind, current_module_name,
    emit_compile_ets_object, emit_compile_ets_rendered_decl, generate_object_field_ets_decl,
    generate_object_property_ets_decl, qualify_member_descriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectAccessKind {
    Field,
    Property,
}

#[derive(Clone)]
struct ObjectFieldSpec {
    member: Member,
    rust_name: String,
    arkts_name: String,
    ty: syn::Type,
    access: ObjectAccessKind,
    emit_private: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectStructKind {
    Named,
    Unnamed,
    Unit,
}

#[derive(Default)]
struct ObjectFieldAttrs {
    name: Option<String>,
    property: bool,
    property_name: Option<String>,
}

#[derive(Clone)]
struct EnumVariantSpec {
    rust_ident: Ident,
    arkts_name: String,
    discriminant: i32,
}

/// Expand `#[ani]` for structs
pub fn expand_struct(
    attrs: BindgenAttrs,
    struct_item: ItemStruct,
    prepare: TokenStream,
) -> TokenStream {
    if let Err(err) = validate_object_item_struct(&struct_item) {
        return err.to_compile_error();
    }

    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    let object_impls = match expand_object_type_impls(
        struct_name,
        &class_name,
        &struct_item.generics,
        &struct_item.fields,
    ) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error(),
    };

    quote! {
        #prepare
        #struct_item
        #object_impls
    }
}

fn expand_object_type_impls(
    struct_name: &syn::Ident,
    class_name: &str,
    generics: &Generics,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    let type_params = collect_struct_type_params(generics)?;
    let (struct_kind, field_specs) = collect_object_field_specs(fields)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let env_generics = generics_with_env(generics);
    let (env_impl_generics, _, env_where_clause) = env_generics.split_for_impl();
    let field_bounds = object_field_bounds(&field_specs);
    let method_where_clause = render_where_clause(None, &field_bounds);
    let env_where_with_fields = render_where_clause(env_where_clause, &field_bounds);
    let env_where_with_record_bounds = render_where_clause(
        env_where_clause,
        &[
            quote! { Self: ani::conversions::ToAni<'env, Output = ani::sys::ani_object> },
            quote! { Self: ani::conversions::FromAni<'env, Input = ani::sys::ani_object> },
        ],
    );
    let env_where_with_to_ani = render_where_clause(
        env_where_clause,
        &[quote! { Self: ani::conversions::ToAni<'env, Output = ani::sys::ani_object> }],
    );

    register_object_type_alias(&struct_name.to_string(), class_name);
    register_object_type_members(
        &struct_name.to_string(),
        &field_specs
            .iter()
            .map(|field| ObjectMemberDescriptor {
                rust_name: field.rust_name.clone(),
                arkts_name: field.arkts_name.clone(),
                access: match field.access {
                    ObjectAccessKind::Field => ObjectMemberAccessKind::Field,
                    ObjectAccessKind::Property => ObjectMemberAccessKind::Property,
                },
            })
            .collect::<Vec<_>>(),
    );
    emit_object_decl(class_name, &type_params, &field_specs);

    let module_name = current_module_name();
    let qualified_name = qualify_member_descriptor(class_name, &module_name);
    let class_descriptor = object_descriptor_signature(&qualified_name);
    let qualified_name_lit = syn::LitStr::new(&qualified_name, Span::call_site());
    let class_descriptor_lit = syn::LitStr::new(&class_descriptor, Span::call_site());

    let field_reads = render_field_reads(struct_kind, &field_specs);
    let field_writes_to_new = field_specs.iter().map(generate_field_write_to_new);
    let field_writes_back = field_specs.iter().map(generate_field_write_back);

    Ok(quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// ANI class descriptor usable with `Env::find_class`.
            pub const fn class_descriptor() -> &'static str {
                #class_descriptor_lit
            }

            /// Qualified ArkTS object name.
            pub const fn arkts_name() -> &'static str {
                #qualified_name_lit
            }

            #[doc(hidden)]
            pub fn __ani_from_bound_ani_object<'env>(
                env: &ani::env::Env<'env>,
                value: ani::sys::ani_object,
            ) -> ani::error::Result<Self>
            #method_where_clause
            {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#struct_name)),
                    ));
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                Ok(#field_reads)
            }
        }

        impl #impl_generics ani::conversions::TypeInfo for #struct_name #ty_generics #where_clause {
            fn type_signature() -> &'static str {
                Self::class_descriptor()
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        impl #env_impl_generics ani::conversions::FromAni<'env> for #struct_name #ty_generics
        #env_where_with_fields
        {
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

                Self::__ani_from_bound_ani_object(env, value)
            }
        }

        impl #env_impl_generics ani::conversions::ToAni<'env> for #struct_name #ty_generics
        #env_where_with_fields
        {
            type Output = ani::sys::ani_object;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let class = env.find_class(Self::arkts_name())?;
                let ctor = env.find_constructor(&class, ":")?;
                let obj = env.new_object(&class, &ctor, &[])?;
                #(#field_writes_to_new)*
                Ok(obj.into_raw())
            }
        }

        impl #env_impl_generics ani::conversions::WriteBackToAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_fields
        {
            fn write_back_to_ani_object(
                self,
                env: &ani::env::Env<'env>,
                obj: &ani::types::AniObject<'_>,
            ) -> ani::error::Result<()> {
                #(#field_writes_back)*
                Ok(())
            }
        }

        impl #env_impl_generics ani::conversions::ValidateFromAni<'env>
            for #struct_name #ty_generics
        #env_where_clause
        {
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

        impl #env_impl_generics ani::conversions::FromAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_record_bounds
        {
            fn from_ani_object(
                env: &ani::env::Env<'env>,
                value: ani::sys::ani_object,
            ) -> ani::error::Result<Self> {
                <Self as ani::conversions::FromAni<'env>>::from_ani(env, value)
            }
        }

        impl #env_impl_generics ani::conversions::RecordValue<'env>
            for #struct_name #ty_generics
        #env_where_with_record_bounds
        {
            fn to_record_ref(self, env: &ani::env::Env<'env>) -> ani::error::Result<ani::types::AniRef<'env>> {
                let raw = <Self as ani::conversions::ToAni<'env>>::to_ani(self, env)?;
                Ok(unsafe { ani::types::AniRef::from_raw(raw as ani::sys::ani_ref) })
            }

            fn from_record_ref(
                env: &ani::env::Env<'env>,
                value: &ani::types::AniRef<'env>,
            ) -> ani::error::Result<Self> {
                <Self as ani::conversions::FromAni<'env>>::from_ani(
                    env,
                    value.as_raw() as ani::sys::ani_object,
                )
            }
        }

        impl #env_impl_generics ani::conversions::ToAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_to_ani
        {
            fn to_ani_object(self, env: &ani::env::Env<'env>) -> ani::error::Result<ani::sys::ani_object> {
                <Self as ani::conversions::ToAni<'env>>::to_ani(self, env)
            }
        }
    })
}

fn generate_field_read_expr(field: &ObjectFieldSpec) -> TokenStream {
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::get_named_field(
                env,
                &obj,
                #field_literal,
            )?
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::get_named_property(
                env,
                &obj,
                #field_literal,
            )?
        },
    }
}

fn generate_field_write_to_new(field: &ObjectFieldSpec) -> TokenStream {
    let member = &field.member;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#member,
                env,
                &obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#member,
                env,
                &obj,
                #field_literal,
            )?;
        },
    }
}

fn generate_field_write_back(field: &ObjectFieldSpec) -> TokenStream {
    let member = &field.member;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#member,
                env,
                obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#member,
                env,
                obj,
                #field_literal,
            )?;
        },
    }
}

fn render_field_reads(kind: ObjectStructKind, fields: &[ObjectFieldSpec]) -> TokenStream {
    match kind {
        ObjectStructKind::Named => {
            let initializers = fields.iter().map(|field| {
                let Member::Named(field_name) = &field.member else {
                    unreachable!("named struct field should use named member");
                };
                let value = generate_field_read_expr(field);
                quote! { #field_name: #value }
            });
            quote! {
                Self {
                    #(#initializers,)*
                }
            }
        }
        ObjectStructKind::Unnamed => {
            let values = fields.iter().map(generate_field_read_expr);
            quote! {
                Self(
                    #(#values,)*
                )
            }
        }
        ObjectStructKind::Unit => quote! { Self },
    }
}

fn emit_object_decl(class_name: &str, type_params: &[String], fields: &[ObjectFieldSpec]) {
    let object_field_decls = fields
        .iter()
        .map(|field| {
            let (kind, rendered) = match field.access {
                ObjectAccessKind::Field => (
                    EtsObjectMemberKind::Field,
                    generate_object_field_ets_decl(
                        &field.arkts_name,
                        &field.ty,
                        field.emit_private,
                    ),
                ),
                ObjectAccessKind::Property => (
                    EtsObjectMemberKind::Property,
                    generate_object_property_ets_decl(&field.arkts_name, &field.ty),
                ),
            };
            EtsObjectMemberDecl {
                name: field.arkts_name.clone(),
                kind,
                is_private: field.emit_private,
                rendered,
            }
        })
        .collect::<Vec<_>>();
    emit_compile_ets_object(class_name, type_params, &object_field_decls);
}

fn collect_object_field_specs(
    fields: &Fields,
) -> syn::Result<(ObjectStructKind, Vec<ObjectFieldSpec>)> {
    let mut member_names = BTreeSet::new();
    let mut push_field =
        |member: Member, rust_name: String, field: &Field, default_name: String| {
            let attrs = parse_object_field_attrs(field)?;
            let access = if attrs.property {
                ObjectAccessKind::Property
            } else {
                ObjectAccessKind::Field
            };
            let arkts_name = attrs.property_name.or(attrs.name).unwrap_or(default_name);
            if !member_names.insert(arkts_name.clone()) {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("duplicate ArkTS object member name `{arkts_name}`"),
                ));
            }
            let emit_private = matches!(access, ObjectAccessKind::Field)
                && rust_name.starts_with('_')
                && arkts_name == rust_name;
            Ok(ObjectFieldSpec {
                member,
                rust_name,
                arkts_name,
                ty: field.ty.clone(),
                access,
                emit_private,
            })
        };

    match fields {
        Fields::Named(fields) => {
            let mut out = Vec::with_capacity(fields.named.len());
            for field in &fields.named {
                let Some(rust_ident) = field.ident.clone() else {
                    continue;
                };
                let rust_name = rust_ident.to_string();
                out.push(push_field(
                    Member::Named(rust_ident),
                    rust_name.clone(),
                    field,
                    rust_name,
                )?);
            }
            Ok((ObjectStructKind::Named, out))
        }
        Fields::Unnamed(fields) => {
            let mut out = Vec::with_capacity(fields.unnamed.len());
            for (index, field) in fields.unnamed.iter().enumerate() {
                let rust_name = format!("field{index}");
                out.push(push_field(
                    Member::Unnamed(Index::from(index)),
                    rust_name.clone(),
                    field,
                    rust_name,
                )?);
            }
            Ok((ObjectStructKind::Unnamed, out))
        }
        Fields::Unit => Ok((ObjectStructKind::Unit, Vec::new())),
    }
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

fn collect_struct_type_params(generics: &Generics) -> syn::Result<Vec<String>> {
    let mut out = Vec::new();
    for param in &generics.params {
        match param {
            GenericParam::Type(ty) => out.push(ty.ident.to_string()),
            GenericParam::Lifetime(lifetime) => {
                return Err(syn::Error::new_spanned(
                    lifetime,
                    "ANI object/class derives support type parameters only; lifetime parameters are not supported",
                ));
            }
            GenericParam::Const(const_param) => {
                return Err(syn::Error::new_spanned(
                    const_param,
                    "ANI object/class derives support type parameters only; const generics are not supported",
                ));
            }
        }
    }
    Ok(out)
}

fn validate_object_struct_shape(generics: &Generics) -> syn::Result<()> {
    collect_struct_type_params(generics).map(|_| ())
}

fn validate_object_item_struct(struct_item: &ItemStruct) -> syn::Result<()> {
    validate_object_struct_shape(&struct_item.generics)
}

fn object_descriptor_signature(qualified_name: &str) -> String {
    if qualified_name.starts_with('L') && qualified_name.ends_with(';') {
        qualified_name.to_string()
    } else {
        format!("L{};", qualified_name.replace('.', "/"))
    }
}

fn derive_named_type_name(input: &DeriveInput) -> syn::Result<String> {
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
        if let Some(name) = parsed.name {
            return Ok(name);
        }
    }

    Ok(input.ident.to_string())
}

fn validate_derive_input(input: &DeriveInput) -> syn::Result<()> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(AniClass)] can only be applied to structs",
        ));
    };

    let _ = data;
    validate_object_struct_shape(&input.generics)
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    if let Err(err) = validate_derive_input(&input) {
        return err.to_compile_error();
    }

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(
            &input,
            "#[derive(AniClass)] can only be applied to structs",
        )
        .to_compile_error();
    };

    let class_name = match derive_named_type_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error(),
    };

    match expand_object_type_impls(&input.ident, &class_name, &input.generics, &data.fields) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn generics_with_env(generics: &Generics) -> Generics {
    let mut out = generics.clone();
    out.params.insert(0, syn::parse_quote!('env));
    out
}

fn render_where_clause(base: Option<&syn::WhereClause>, extra: &[TokenStream]) -> TokenStream {
    if extra.is_empty() {
        return base.map_or_else(TokenStream::new, |clause| quote! { #clause });
    }

    if let Some(base) = base {
        let predicates = &base.predicates;
        quote! {
            where
                #predicates,
                #(#extra),*
        }
    } else {
        quote! {
            where
                #(#extra),*
        }
    }
}

fn object_field_bounds(fields: &[ObjectFieldSpec]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            match field.access {
                ObjectAccessKind::Field => {
                    quote! { #field_ty: ani::conversions::ObjectField<'env> }
                }
                ObjectAccessKind::Property => {
                    quote! { #field_ty: ani::conversions::ObjectProperty<'env> }
                }
            }
        })
        .collect()
}

fn validate_enum_derive_input(input: &DeriveInput) -> syn::Result<&syn::DataEnum> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(AniEnum)] can only be applied to enums",
        ));
    };

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "#[derive(AniEnum)] does not support generic enums yet",
        ));
    }

    Ok(data)
}

fn parse_enum_variant_arkts_name(variant: &Variant) -> syn::Result<Option<String>> {
    for attr in &variant.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let parsed = attr.parse_args::<AniAttrs>()?;
        if let Some(name) = parsed.name {
            return Ok(Some(name));
        }
    }

    Ok(None)
}

fn parse_enum_discriminant(expr: &Expr) -> syn::Result<i32> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(value),
            ..
        }) => value.base10_parse::<i32>(),
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(value),
                ..
            }) = unary.expr.as_ref()
            {
                Ok(-value.base10_parse::<i32>()?)
            } else {
                Err(syn::Error::new_spanned(
                    expr,
                    "#[derive(AniEnum)] only supports integer literal discriminants",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "#[derive(AniEnum)] only supports integer literal discriminants",
        )),
    }
}

fn collect_enum_variant_specs(data: &syn::DataEnum) -> syn::Result<Vec<EnumVariantSpec>> {
    let mut specs = Vec::new();
    let mut next_discriminant = 0_i32;

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant.fields,
                "#[derive(AniEnum)] currently only supports unit variants",
            ));
        }

        let discriminant = if let Some((_, expr)) = &variant.discriminant {
            parse_enum_discriminant(expr)?
        } else {
            next_discriminant
        };
        next_discriminant = discriminant.checked_add(1).ok_or_else(|| {
            syn::Error::new_spanned(
                variant,
                "#[derive(AniEnum)] discriminant overflow while deriving ANI enum",
            )
        })?;

        let arkts_name =
            parse_enum_variant_arkts_name(variant)?.unwrap_or_else(|| variant.ident.to_string());
        specs.push(EnumVariantSpec {
            rust_ident: variant.ident.clone(),
            arkts_name,
            discriminant,
        });
    }

    Ok(specs)
}

fn render_enum_decl(enum_name: &str, variants: &[EnumVariantSpec]) -> String {
    let members = variants
        .iter()
        .map(|variant| format!("  {} = {}", variant.arkts_name, variant.discriminant))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("enum {enum_name} {{\n{members}\n}}")
}

fn emit_enum_decl(qualified_name: &str, variants: &[EnumVariantSpec]) {
    let mut parts = qualified_name
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let enum_name = parts.pop().unwrap_or(qualified_name);
    let rendered = render_enum_decl(enum_name, variants);

    if parts.is_empty() {
        emit_compile_ets_rendered_decl(EtsDeclKind::Global, "", &rendered);
    } else {
        emit_compile_ets_rendered_decl(EtsDeclKind::Namespace, &parts.join("."), &rendered);
    }
}

fn expand_enum_type_impls(
    enum_name: &Ident,
    qualified_name: &str,
    variants: &[EnumVariantSpec],
) -> TokenStream {
    register_object_type_alias(&enum_name.to_string(), qualified_name);
    emit_enum_decl(qualified_name, variants);

    let descriptor = object_descriptor_signature(&qualify_member_descriptor(
        qualified_name,
        &current_module_name(),
    ));
    let descriptor_lit = syn::LitStr::new(&descriptor, Span::call_site());
    let qualified_name_lit = syn::LitStr::new(qualified_name, Span::call_site());

    let to_ani_arms = variants.iter().map(|variant| {
        let rust_ident = &variant.rust_ident;
        let arkts_name = syn::LitStr::new(&variant.arkts_name, Span::call_site());
        quote! { Self::#rust_ident => #arkts_name }
    });
    let from_ani_arms = variants.iter().map(|variant| {
        let rust_ident = &variant.rust_ident;
        let arkts_name = syn::LitStr::new(&variant.arkts_name, Span::call_site());
        quote! { #arkts_name => Ok(Self::#rust_ident) }
    });

    quote! {
        impl #enum_name {
            /// ANI enum descriptor usable with `Env::find_enum`.
            pub const fn enum_descriptor() -> &'static str {
                #descriptor_lit
            }

            /// Qualified ArkTS enum name.
            pub const fn arkts_name() -> &'static str {
                #qualified_name_lit
            }
        }

        impl ani::conversions::TypeInfo for #enum_name {
            fn type_signature() -> &'static str {
                Self::enum_descriptor()
            }

            fn ani_c_type() -> &'static str {
                "ani_enum_item"
            }
        }

        impl<'env> ani::conversions::ToAni<'env> for #enum_name {
            type Output = ani::sys::ani_enum_item;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let enm = env.find_enum(Self::arkts_name())?;
                let item_name = match self {
                    #(#to_ani_arms,)*
                };
                let item = env.get_enum_item_by_name(&enm, item_name)?;
                Ok(item.into_raw())
            }
        }

        impl<'env> ani::conversions::FromAni<'env> for #enum_name {
            type Input = ani::sys::ani_enum_item;

            fn from_ani(env: &ani::env::Env<'env>, value: Self::Input) -> ani::error::Result<Self> {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#enum_name)),
                    ));
                }

                let item = ani::conversions::EnumItem::from_handle(unsafe {
                    ani::types::AniEnumItem::from_raw(value)
                });
                let name = item.name(env)?;
                match name.as_str() {
                    #(#from_ani_arms,)*
                    _ => Err(ani::error::Error::new(
                        ani::error::Status::InvalidType,
                        format!("Unknown enum item `{}` for {}", name, stringify!(#enum_name)),
                    )),
                }
            }
        }
    }
}

/// Expand AniEnum derive macro
pub fn expand_enum_derive(input: DeriveInput) -> TokenStream {
    let data = match validate_enum_derive_input(&input) {
        Ok(data) => data,
        Err(err) => return err.to_compile_error(),
    };

    let enum_name = match derive_named_type_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error(),
    };
    let variants = match collect_enum_variant_specs(data) {
        Ok(variants) => variants,
        Err(err) => return err.to_compile_error(),
    };

    expand_enum_type_impls(&input.ident, &enum_name, &variants)
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
        assert!(
            expanded.contains("impl ani :: conversions :: TypeInfo for ExplicitDerivedProfile")
        );
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
    fn derive_ani_class_keeps_underscore_backing_fields_for_runtime() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct BackingObject {
                _name: String,
                value: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("get_named_field"));
        assert!(expanded.contains("set_named_field"));
        assert!(expanded.contains("_name"));
    }

    #[test]
    fn derive_ani_class_supports_generic_tuple_and_unit_structs() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct TupleUser<T>(T, #[ani(property)] bool);
        };
        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("impl < T > ani :: conversions :: TypeInfo for TupleUser < T >"));
        assert!(expanded.contains(
            "impl < 'env , T > ani :: conversions :: ToAni < 'env > for TupleUser < T >"
        ));
        assert!(expanded.contains("T : ani :: conversions :: ObjectField < 'env >"));
        assert!(expanded.contains("bool : ani :: conversions :: ObjectProperty < 'env >"));
        assert!(expanded.contains("self . 0"));
        assert!(expanded.contains("self . 1"));
        assert!(expanded.contains("field0"));
        assert!(expanded.contains("field1"));

        let unit_input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct Marker;
        };
        let unit_expanded = expand_class_derive(unit_input).to_string();
        assert!(unit_expanded.contains("impl ani :: conversions :: TypeInfo for Marker"));
        assert!(unit_expanded.contains("Ok (Self)"));
    }

    #[test]
    fn derive_ani_class_rejects_non_type_generics() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct Borrowed<'a> {
                name: &'a str,
            }
        };
        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("lifetime parameters are not supported"));
    }

    #[test]
    fn derive_ani_enum_emits_type_info_and_conversion_impls() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniEnum)]
            #[ani(name = "models.Status")]
            enum Status {
                Idle,
                Running = 4,
                Stopped,
            }
        };

        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("impl ani :: conversions :: TypeInfo for Status"));
        assert!(expanded.contains("pub const fn enum_descriptor () -> & 'static str"));
        assert!(expanded.contains("pub const fn arkts_name () -> & 'static str"));
        assert!(expanded.contains("ani_enum_item"));
        assert!(expanded.contains("env . find_enum (Self :: arkts_name ())"));
        assert!(expanded.contains("env . get_enum_item_by_name (& enm , item_name)"));
        assert!(expanded.contains("models.Status"));
    }

    #[test]
    fn derive_ani_enum_supports_variant_name_override() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniEnum)]
            enum Outcome {
                #[ani(name = "Ok")]
                Success,
                Failure,
            }
        };

        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("\"Ok\" => Ok (Self :: Success)"));
        assert!(expanded.contains("Self :: Success => \"Ok\""));
    }

    #[test]
    fn derive_ani_enum_rejects_non_unit_variants() {
        let input: DeriveInput = parse_quote! {
            enum BadEnum {
                Value(i32),
            }
        };
        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("currently only supports unit variants"));
    }
}
