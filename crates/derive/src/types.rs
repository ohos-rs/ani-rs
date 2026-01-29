//! Type Helper Utilities

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath};

/// Parsed Type Information (reserved for future advanced type analysis)
#[allow(dead_code)]
pub struct TypeInfo {
    /// Whether this is a primitive type
    pub is_primitive: bool,
    /// Whether this is an Option type
    pub is_option: bool,
    /// Whether this is a Result type
    pub is_result: bool,
    /// Whether this is a Vec type
    pub is_vec: bool,
    /// Whether this is a reference type
    pub is_reference: bool,
    /// Whether this is a String type
    pub is_string: bool,
    /// Inner type (for Option, Result, Vec)
    pub inner_type: Option<Box<TypeInfo>>,
}

impl TypeInfo {
    /// Analyze type from syn::Type
    /// Reserved for future advanced type analysis
    #[allow(dead_code)]
    pub fn from_type(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => Self::from_type_path(type_path),
            Type::Reference(type_ref) => {
                let mut info = Self::from_type(&type_ref.elem);
                info.is_reference = true;
                info
            }
            Type::Tuple(tuple) if tuple.elems.is_empty() => Self {
                is_primitive: true,
                is_option: false,
                is_result: false,
                is_vec: false,
                is_reference: false,
                is_string: false,
                inner_type: None,
            },
            _ => Self::default(),
        }
    }

    #[allow(dead_code)]
    fn from_type_path(type_path: &TypePath) -> Self {
        let type_name = type_path
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();

        let mut info = TypeInfo {
            is_primitive: false,
            is_option: false,
            is_result: false,
            is_vec: false,
            is_reference: false,
            is_string: false,
            inner_type: None,
        };

        match type_name.as_str() {
            // Primitive types
            "bool" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32"
            | "f64" | "char" | "isize" | "usize" => {
                info.is_primitive = true;
            }

            // String type
            "String" => {
                info.is_string = true;
            }
            "str" => {
                info.is_string = true;
                info.is_reference = true;
            }

            // Option type
            "Option" => {
                info.is_option = true;
                info.inner_type = Self::extract_first_generic(type_path);
            }

            // Result type
            "Result" => {
                info.is_result = true;
                info.inner_type = Self::extract_first_generic(type_path);
            }

            // Vec type
            "Vec" => {
                info.is_vec = true;
                info.inner_type = Self::extract_first_generic(type_path);
            }

            _ => {}
        }

        info
    }

    #[allow(dead_code)]
    fn extract_first_generic(type_path: &TypePath) -> Option<Box<TypeInfo>> {
        if let Some(segment) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(ty)) = args.args.first() {
                    return Some(Box::new(TypeInfo::from_type(ty)));
                }
            }
        }
        None
    }

    /// Get ANI type signature
    #[allow(dead_code)]
    pub fn get_signature(&self, ty: &Type) -> String {
        if self.is_option {
            // Option type uses boxed signature
            if let Some(inner) = &self.inner_type {
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return get_boxed_signature_for_type(inner_ty, inner);
                            }
                        }
                    }
                }
            }
            return "Lstd/core/Object;".to_string();
        }

        if self.is_result {
            // Result returns Ok type
            if let Some(inner) = &self.inner_type {
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return inner.get_signature(inner_ty);
                            }
                        }
                    }
                }
            }
            return "Lstd/core/Object;".to_string();
        }

        if self.is_vec {
            // Vec type uses array signature
            if let Some(inner) = &self.inner_type {
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return format!("[{}", inner.get_signature(inner_ty));
                            }
                        }
                    }
                }
            }
            return "[Lstd/core/Object;".to_string();
        }

        if self.is_string {
            return "Lstd/core/String;".to_string();
        }

        if self.is_primitive {
            return get_primitive_signature(ty);
        }

        // Default object type
        "Lstd/core/Object;".to_string()
    }
}

impl Default for TypeInfo {
    fn default() -> Self {
        Self {
            is_primitive: false,
            is_option: false,
            is_result: false,
            is_vec: false,
            is_reference: false,
            is_string: false,
            inner_type: None,
        }
    }
}

/// Get primitive type signature
#[allow(dead_code)]
fn get_primitive_signature(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => "Z".to_string(),
        "i8" | "u8" => "B".to_string(),
        "i16" => "S".to_string(),
        "u16" | "char" => "C".to_string(),
        "i32" | "u32" => "I".to_string(),
        "i64" | "u64" => "J".to_string(),
        "f32" => "F".to_string(),
        "f64" => "D".to_string(),
        _ => "Lstd/core/Object;".to_string(),
    }
}

/// Get boxed type signature
#[allow(dead_code)]
fn get_boxed_signature_for_type(ty: &Type, info: &TypeInfo) -> String {
    if info.is_primitive {
        let type_str = quote!(#ty).to_string().replace(" ", "");

        match type_str.as_str() {
            "bool" => "Lstd/core/Boolean;".to_string(),
            "i8" | "u8" => "Lstd/core/Byte;".to_string(),
            "i16" => "Lstd/core/Short;".to_string(),
            "u16" | "char" => "Lstd/core/Char;".to_string(),
            "i32" => "Lstd/core/Int;".to_string(),
            "u32" => "Lstd/core/Int;".to_string(),
            "i64" => "Lstd/core/Long;".to_string(),
            "u64" => "Lstd/core/Long;".to_string(),
            "f32" => "Lstd/core/Float;".to_string(),
            "f64" => "Lstd/core/Double;".to_string(),
            _ => "Lstd/core/Object;".to_string(),
        }
    } else if info.is_string {
        "Lstd/core/String;".to_string()
    } else {
        info.get_signature(ty)
    }
}

/// Generate type conversion code (ANI to Rust)
#[allow(dead_code)]
pub fn generate_from_ani_conversion(ty: &Type, param_name: &syn::Ident) -> TokenStream {
    let info = TypeInfo::from_type(ty);
    let converted_name = quote::format_ident!("{}_converted", param_name);

    if info.is_primitive {
        quote! { let #converted_name = #param_name; }
    } else if info.is_string {
        quote! {
            let #converted_name = {
                let env_wrapper = ani::Env::from_raw_unchecked(env);
                let ani_str = ani::types::AniString::from_raw(#param_name);
                env_wrapper.get_string(&ani_str).unwrap_or_default()
            };
        }
    } else if info.is_option {
        quote! {
            let #converted_name = {
                // TODO: Implement Option type conversion
                None
            };
        }
    } else if info.is_vec {
        quote! {
            let #converted_name = {
                // TODO: Implement Vec type conversion
                Vec::new()
            };
        }
    } else {
        quote! { let #converted_name = #param_name; }
    }
}

/// Generate type conversion code (Rust to ANI)
#[allow(dead_code)]
pub fn generate_to_ani_conversion(ty: &Type, value_name: TokenStream) -> TokenStream {
    let info = TypeInfo::from_type(ty);

    if info.is_primitive {
        quote! { #value_name }
    } else if info.is_string {
        quote! {
            {
                let env_wrapper = ani::Env::from_raw_unchecked(env);
                match env_wrapper.create_string(&#value_name) {
                    Ok(s) => s.into_raw(),
                    Err(_) => std::ptr::null_mut()
                }
            }
        }
    } else if info.is_option {
        quote! {
            match #value_name {
                Some(v) => {
                    // TODO: Convert Some value
                    std::ptr::null_mut()
                }
                None => std::ptr::null_mut()
            }
        }
    } else if info.is_vec {
        quote! {
            {
                // TODO: Convert Vec
                std::ptr::null_mut()
            }
        }
    } else if info.is_result {
        quote! {
            match #value_name {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    Default::default()
                }
            }
        }
    } else {
        quote! { #value_name }
    }
}

/// Check if type needs boxing
#[allow(dead_code)]
pub fn needs_boxing(ty: &Type) -> bool {
    let info = TypeInfo::from_type(ty);
    info.is_option
        && info
            .inner_type
            .as_ref()
            .map(|i| i.is_primitive)
            .unwrap_or(false)
}

/// Check if type is void/unit
#[allow(dead_code)]
pub fn is_void_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_primitive_type_info() {
        let ty: Type = parse_quote!(i32);
        let info = TypeInfo::from_type(&ty);
        assert!(info.is_primitive);
        assert!(!info.is_option);
        assert!(!info.is_string);
    }

    #[test]
    fn test_string_type_info() {
        let ty: Type = parse_quote!(String);
        let info = TypeInfo::from_type(&ty);
        assert!(!info.is_primitive);
        assert!(info.is_string);
    }

    #[test]
    fn test_option_type_info() {
        let ty: Type = parse_quote!(Option<i32>);
        let info = TypeInfo::from_type(&ty);
        assert!(info.is_option);
        assert!(info.inner_type.is_some());
        assert!(info.inner_type.as_ref().unwrap().is_primitive);
    }

    #[test]
    fn test_vec_type_info() {
        let ty: Type = parse_quote!(Vec<String>);
        let info = TypeInfo::from_type(&ty);
        assert!(info.is_vec);
        assert!(info.inner_type.is_some());
        assert!(info.inner_type.as_ref().unwrap().is_string);
    }

    #[test]
    fn test_result_type_info() {
        let ty: Type = parse_quote!(Result<i32, String>);
        let info = TypeInfo::from_type(&ty);
        assert!(info.is_result);
        assert!(info.inner_type.is_some());
        assert!(info.inner_type.as_ref().unwrap().is_primitive);
    }

    #[test]
    fn test_signature_generation() {
        let ty: Type = parse_quote!(i32);
        let info = TypeInfo::from_type(&ty);
        assert_eq!(info.get_signature(&ty), "I");

        let ty: Type = parse_quote!(String);
        let info = TypeInfo::from_type(&ty);
        assert_eq!(info.get_signature(&ty), "Lstd/core/String;");
    }
}
