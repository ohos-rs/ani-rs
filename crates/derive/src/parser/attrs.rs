//! Attribute Parsing
//!
//! Parses `#[ani(...)]` macro attributes into structured data.

use syn::{
    Attribute, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ============================================================================
// Macro Kind
// ============================================================================

/// Identifies the type of `#[ani]` macro usage
#[derive(Debug, Clone, Default, PartialEq)]
pub enum AniMacroKind {
    /// Default: bind function/method/struct
    #[default]
    Bindgen,
    /// Initialization function
    Init,
    /// Module finalization function
    Finalize,
    /// Object/class definition
    Object,
}

// ============================================================================
// Unified Attributes
// ============================================================================

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
    pub is_async: bool,
    /// init before_bindings option
    pub before_bindings: bool,
    /// object field configuration
    pub object_name: Option<String>,
    /// Delegate a single-field newtype directly to its inner ANI type.
    pub transparent: bool,
    /// Delegate a single-field collection newtype to its inner ANI array type.
    pub array: bool,
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
                // Type identifiers
                "init" => {
                    attrs.kind = AniMacroKind::Init;
                }
                "finalize" | "fini" => {
                    attrs.kind = AniMacroKind::Finalize;
                }
                "object" => {
                    attrs.kind = AniMacroKind::Object;
                    if let Some(AttrValue::Str(s)) = item.value {
                        attrs.object_name = Some(s);
                    }
                }
                // Binding attributes
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
                    attrs.getter = match item.value {
                        Some(AttrValue::Str(s)) => Some(s),
                        _ => Some(String::new()),
                    };
                }
                "setter" => {
                    attrs.setter = match item.value {
                        Some(AttrValue::Str(s)) => Some(s),
                        _ => Some(String::new()),
                    };
                }
                "async" => {
                    attrs.is_async = true;
                }
                "before_bindings" => {
                    attrs.before_bindings = true;
                }
                "transparent" => {
                    attrs.transparent = true;
                }
                "array" => {
                    attrs.array = true;
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

// ============================================================================
// Bindgen Attributes (for code generation)
// ============================================================================

/// Attributes for binding code generation
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct BindgenAttrs {
    pub namespace: Option<String>,
    pub class: Option<String>,
    pub module: Option<String>,
    pub is_static: bool,
    pub name: Option<String>,
    pub signature: Option<String>,
    pub skip: bool,
    pub constructor: bool,
    pub getter: Option<String>,
    pub setter: Option<String>,
    pub is_async: bool,
    pub transparent: bool,
    pub array: bool,
}

impl From<AniAttrs> for BindgenAttrs {
    fn from(attrs: AniAttrs) -> Self {
        BindgenAttrs {
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
            is_async: attrs.is_async,
            transparent: attrs.transparent,
            array: attrs.array,
        }
    }
}

pub fn parse_bindgen_attrs_from_attribute(attr: &Attribute) -> syn::Result<BindgenAttrs> {
    if matches!(attr.meta, syn::Meta::Path(_)) {
        return Ok(BindgenAttrs::default());
    }
    let attrs = attr.parse_args::<AniAttrs>()?;
    Ok(BindgenAttrs::from(attrs))
}

// ============================================================================
// Init Attributes
// ============================================================================

/// Attributes for init functions
#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct InitAttrs {
    pub before_bindings: bool,
}

/// Attributes for module finalization functions.
#[derive(Debug, Default, Clone)]
pub struct FinalizeAttrs;

impl From<AniAttrs> for InitAttrs {
    fn from(attrs: AniAttrs) -> Self {
        InitAttrs {
            before_bindings: attrs.before_bindings,
        }
    }
}

// ============================================================================
// Attribute Parsing Helpers
// ============================================================================

/// A single attribute item (key = value or just key)
#[derive(Debug)]
pub struct AttrItem {
    pub key: Ident,
    pub value: Option<AttrValue>,
}

/// Attribute value types
#[derive(Debug)]
#[allow(dead_code)]
pub enum AttrValue {
    Str(String),
    Bool(bool),
    Ident(Ident),
}

impl Parse for AttrItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Handle `static` and `async` keywords specially
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
