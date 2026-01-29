//! Macro Attribute Parsing

use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ============================================================================
// Unified #[ani] Macro Attributes
// ============================================================================

/// Macro type identifier
#[derive(Debug, Clone, Default, PartialEq)]
pub enum AniMacroKind {
    /// Default: bind function/method/struct
    #[default]
    Bindgen,
    /// Initialization function
    Init,
    /// Object/class definition
    Object,
}

/// Unified `#[ani]` macro attributes
#[derive(Debug, Default, Clone)]
pub struct AniAttrs {
    /// Macro type
    pub kind: AniMacroKind,
    /// Namespace
    pub namespace: Option<String>,
    /// Class name
    pub class: Option<String>,
    /// Module name
    pub module: Option<String>,
    /// Whether it's a static method
    pub is_static: bool,
    /// Custom function name
    pub name: Option<String>,
    /// Custom signature
    pub signature: Option<String>,
    /// Whether to skip generation
    pub skip: bool,
    /// Whether it's a constructor
    pub constructor: bool,
    /// Getter property name
    pub getter: Option<String>,
    /// Setter property name
    pub setter: Option<String>,
    /// Whether it's an async function
    pub r#async: bool,
    /// init before_bindings option
    pub before_bindings: bool,
    /// object field configuration
    pub object_name: Option<String>,
}

impl Parse for AniAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = AniAttrs::default();

        if input.is_empty() {
            return Ok(attrs);
        }

        let items: Punctuated<AttrItem, Token![,]> = Punctuated::parse_terminated(input)?;

        for item in items {
            match item.key.to_string().as_str() {
                // 类型标识符
                "init" => {
                    attrs.kind = AniMacroKind::Init;
                }
                "object" => {
                    attrs.kind = AniMacroKind::Object;
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.object_name = Some(s);
                    }
                }
                // 原有属性
                "namespace" | "ns" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.namespace = Some(s);
                    }
                }
                "class" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.class = Some(s);
                    }
                }
                "module" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.module = Some(s);
                    } else {
                        attrs.module = Some(String::new());
                    }
                }
                "static" | "is_static" => {
                    attrs.is_static = true;
                }
                "name" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.name = Some(s);
                    }
                }
                "signature" | "sig" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.signature = Some(s);
                    }
                }
                "skip" => {
                    attrs.skip = true;
                }
                "constructor" | "ctor" => {
                    attrs.constructor = true;
                }
                "getter" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.getter = Some(s);
                    } else {
                        attrs.getter = Some(String::new());
                    }
                }
                "setter" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.setter = Some(s);
                    } else {
                        attrs.setter = Some(String::new());
                    }
                }
                "async" => {
                    attrs.r#async = true;
                }
                "before_bindings" => {
                    attrs.before_bindings = true;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        item.key,
                        format!("Unknown attribute: {}", other),
                    ));
                }
            }
        }

        Ok(attrs)
    }
}

impl From<AniAttrs> for AniBindgenAttrs {
    fn from(attrs: AniAttrs) -> Self {
        AniBindgenAttrs {
            namespace: attrs.namespace,
            class: attrs.class,
            module: attrs.module,
            is_static: attrs.is_static,
            name: attrs.name,
            signature: attrs.signature,
            skip: attrs.skip,
            constructor: attrs.constructor,
            getter: attrs.getter,
            setter: attrs.setter,
            r#async: attrs.r#async,
        }
    }
}

impl From<AniAttrs> for AniInitAttrs {
    fn from(attrs: AniAttrs) -> Self {
        AniInitAttrs {
            before_bindings: attrs.before_bindings,
        }
    }
}

// ============================================================================
// Original Attribute Definitions (maintained for backward compatibility)
// ============================================================================

/// `#[ani_bindgen]` macro attributes
#[derive(Debug, Default, Clone)]
pub struct AniBindgenAttrs {
    /// Namespace
    pub namespace: Option<String>,
    /// Class name
    pub class: Option<String>,
    /// Module name
    pub module: Option<String>,
    /// Whether it's a static method
    pub is_static: bool,
    /// Custom function name
    pub name: Option<String>,
    /// Custom signature
    pub signature: Option<String>,
    /// Whether to skip generation
    pub skip: bool,
    /// Whether it's a constructor
    pub constructor: bool,
    /// Getter property name
    pub getter: Option<String>,
    /// Setter property name
    pub setter: Option<String>,
    /// Whether it's an async function
    pub r#async: bool,
}

impl Parse for AniBindgenAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = AniBindgenAttrs::default();

        if input.is_empty() {
            return Ok(attrs);
        }

        let items: Punctuated<AttrItem, Token![,]> = Punctuated::parse_terminated(input)?;

        for item in items {
            match item.key.to_string().as_str() {
                "namespace" | "ns" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.namespace = Some(s);
                    }
                }
                "class" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.class = Some(s);
                    }
                }
                "module" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.module = Some(s);
                    } else {
                        attrs.module = Some(String::new());
                    }
                }
                "static" | "is_static" => {
                    attrs.is_static = true;
                }
                "name" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.name = Some(s);
                    }
                }
                "signature" | "sig" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.signature = Some(s);
                    }
                }
                "skip" => {
                    attrs.skip = true;
                }
                "constructor" | "ctor" => {
                    attrs.constructor = true;
                }
                "getter" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.getter = Some(s);
                    } else {
                        // If no value, use function name as property name
                        attrs.getter = Some(String::new());
                    }
                }
                "setter" => {
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.setter = Some(s);
                    } else {
                        attrs.setter = Some(String::new());
                    }
                }
                "async" => {
                    attrs.r#async = true;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        item.key,
                        format!("Unknown attribute: {}", other),
                    ));
                }
            }
        }

        Ok(attrs)
    }
}

#[derive(Debug)]
pub struct AttrItem {
    pub key: Ident,
    pub value: Option<AttrValue>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AttrValue {
    Str(String),
    Bool(bool),
    Ident(Ident),
}

impl Parse for AttrItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 处理 static 关键字特殊情况
        let key: Ident = if input.peek(Token![static]) {
            let _: Token![static] = input.parse()?;
            Ident::new("static", proc_macro2::Span::call_site())
        } else if input.peek(Token![async]) {
            let _: Token![async] = input.parse()?;
            Ident::new("async", proc_macro2::Span::call_site())
        } else {
            input.parse()?
        };

        let value = if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                Some(AttrValue::Str(lit.value()))
            } else if input.peek(syn::LitBool) {
                let lit: syn::LitBool = input.parse()?;
                Some(AttrValue::Bool(lit.value()))
            } else {
                let ident: Ident = input.parse()?;
                Some(AttrValue::Ident(ident))
            }
        } else {
            None
        };

        Ok(AttrItem { key, value })
    }
}

/// ANI init attributes
#[derive(Debug, Default)]
pub struct AniInitAttrs {
    /// Whether to call before bindings
    pub before_bindings: bool,
}

impl Parse for AniInitAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = AniInitAttrs::default();

        if input.is_empty() {
            return Ok(attrs);
        }

        let items: Punctuated<AttrItem, Token![,]> = Punctuated::parse_terminated(input)?;

        for item in items {
            match item.key.to_string().as_str() {
                "before_bindings" => {
                    attrs.before_bindings = true;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        item.key,
                        format!("Unknown attribute: {}", other),
                    ));
                }
            }
        }

        Ok(attrs)
    }
}
