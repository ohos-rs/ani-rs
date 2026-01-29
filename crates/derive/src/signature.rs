//! Signature Generation

use quote::quote;
use syn::{FnArg, ReturnType, Signature, Type};

/// Generate ANI signature from Rust type
pub fn rust_type_to_signature(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        // Primitive types
        "bool" => "Z".to_string(),
        "i8" => "B".to_string(),
        "u8" => "B".to_string(),
        "i16" => "S".to_string(),
        "u16" => "C".to_string(),
        "char" => "C".to_string(),
        "i32" => "I".to_string(),
        "u32" => "I".to_string(),
        "i64" => "J".to_string(),
        "u64" => "J".to_string(),
        "f32" => "F".to_string(),
        "f64" => "D".to_string(),
        "()" => "V".to_string(),

        // String types
        "String" | "&str" | "&String" => "Lstd/core/String;".to_string(),

        // Special handling
        _ => {
            // Handle Option<T>
            if type_str.starts_with("Option<") {
                // Optional types need boxing
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                return get_boxed_signature(inner);
                            }
                        }
                    }
                }
                "Lstd/core/Object;".to_string()
            }
            // Handle Vec<T>
            else if type_str.starts_with("Vec<") {
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                let inner_sig = rust_type_to_signature(inner);
                                return format!("[{}", inner_sig);
                            }
                        }
                    }
                }
                "[Lstd/core/Object;".to_string()
            }
            // Handle Result<T, E>
            else if type_str.starts_with("Result<") {
                if let Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(ok_type)) = args.args.first() {
                                return rust_type_to_signature(ok_type);
                            }
                        }
                    }
                }
                "V".to_string()
            }
            // Handle Either types (union types in ArkTS)
            else if type_str.starts_with("Either<")
                || type_str.starts_with("Either3<")
                || type_str.starts_with("Either4<")
                || type_str.starts_with("Either5<")
                || type_str.starts_with("Either6<")
                || type_str.starts_with("Either7<")
                || type_str.starts_with("Either8<")
            {
                // All Either types map to Object in ANI
                "Lstd/core/Object;".to_string()
            }
            // Other types treated as objects
            else {
                "Lstd/core/Object;".to_string()
            }
        }
    }
}

/// Get boxed type signature
fn get_boxed_signature(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => "Lstd/core/Boolean;".to_string(),
        "i8" => "Lstd/core/Byte;".to_string(),
        "i16" => "Lstd/core/Short;".to_string(),
        "i32" => "Lstd/core/Int;".to_string(),
        "i64" => "Lstd/core/Long;".to_string(),
        "f32" => "Lstd/core/Float;".to_string(),
        "f64" => "Lstd/core/Double;".to_string(),
        _ => rust_type_to_signature(ty),
    }
}

/// Generate ANI signature from function signature
pub fn generate_signature_from_fn(sig: &Signature, skip_first: bool) -> String {
    generate_signature_from_inputs(&sig.inputs, &sig.output, skip_first)
}

/// Generate ANI signature from method signature
pub fn generate_signature_from_method_sig(sig: &Signature, skip_first: bool) -> String {
    generate_signature_from_inputs(&sig.inputs, &sig.output, skip_first)
}

/// Generate signature from parameter list
fn generate_signature_from_inputs(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    output: &ReturnType,
    skip_first: bool,
) -> String {
    let mut sig = String::new();

    let params: Vec<_> = inputs.iter().skip(if skip_first { 1 } else { 0 }).collect();

    for input in params {
        if let FnArg::Typed(pat_type) = input {
            sig.push_str(&rust_type_to_signature(&pat_type.ty));
        }
    }

    sig.push(':');

    match output {
        ReturnType::Default => sig.push('V'),
        ReturnType::Type(_, ty) => {
            sig.push_str(&rust_type_to_signature(ty));
        }
    }

    sig
}

/// Convert class name to descriptor
/// Example: "MyModule.MyClass" -> "LMyModule/MyClass;"
pub fn class_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert namespace to descriptor
pub fn namespace_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert module to descriptor
pub fn module_to_descriptor(name: &str) -> String {
    format!("L{};", name)
}

/// Get method call suffix for return type
#[allow(dead_code)]
pub fn get_call_suffix(ty: &Type) -> &'static str {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => "_Boolean",
        "i8" | "u8" => "_Byte",
        "u16" | "char" => "_Char",
        "i16" => "_Short",
        "i32" | "u32" => "_Int",
        "i64" | "u64" => "_Long",
        "f32" => "_Float",
        "f64" => "_Double",
        "()" => "_Void",
        _ => "_Ref",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_descriptor() {
        assert_eq!(class_to_descriptor("hello.Foo"), "Lhello/Foo;");
        assert_eq!(class_to_descriptor("std.core.String"), "Lstd/core/String;");
    }

    #[test]
    fn test_rust_type_to_signature() {
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(i32)), "I");
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(f64)), "D");
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(bool)), "Z");
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(String)),
            "Lstd/core/String;"
        );
    }
}
