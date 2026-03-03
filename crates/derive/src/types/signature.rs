//! Signature Generation
//!
//! Generates ANI type signatures from Rust types using the structured AniType system.

use syn::{FnArg, ReturnType, Signature, Type};

use super::ani_type::AniType;
use crate::codegen::should_skip_in_signature;

// ============================================================================
// Type Signature Generation
// ============================================================================

/// Generate ANI signature from Rust type
pub fn rust_type_to_signature(ty: &Type) -> String {
    let ani_type = AniType::from_syn_type(ty);
    ani_type.to_signature()
}

// ============================================================================
// Function Signature Generation
// ============================================================================

/// Generate ANI signature from function signature
///
/// This function automatically skips injected parameters (Env, This, Class)
/// since they are not part of the ArkTS function signature.
pub fn generate_fn_signature(sig: &Signature, skip_first: bool) -> String {
    generate_signature_from_inputs(&sig.inputs, &sig.output, skip_first)
}

/// Generate ANI signature for a constructor.
///
/// Constructor signatures always return void (`:` with no return type suffix).
pub fn generate_ctor_signature(sig: &Signature, skip_first: bool) -> String {
    let mut ctor_sig = String::new();

    let params: Vec<_> = sig
        .inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .collect();

    for input in params {
        if let FnArg::Typed(pat_type) = input {
            ctor_sig.push_str(&rust_type_to_signature(&pat_type.ty));
        }
    }

    ctor_sig.push(':');
    ctor_sig
}

/// Generate signature from parameter list
fn generate_signature_from_inputs(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    output: &ReturnType,
    skip_first: bool,
) -> String {
    let mut sig = String::new();

    // Filter out:
    // 1. First parameter if skip_first is true (for class methods)
    // 2. All injected parameters (Env, This, Class)
    let params: Vec<_> = inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .collect();

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

// ============================================================================
// Descriptor Conversion
// ============================================================================

/// Convert class name to ANI descriptor
/// Example: "MyModule.MyClass" -> "LMyModule/MyClass;"
pub fn class_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert namespace to ANI descriptor
pub fn namespace_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert module to ANI descriptor
pub fn module_to_descriptor(name: &str) -> String {
    format!("L{};", name)
}

// ============================================================================
// Tests
// ============================================================================

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

    #[test]
    fn test_option_type_signature() {
        // Option<i32> should use boxed Int type
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Option<i32>)),
            "Lstd/core/Int;"
        );
        // Option<String> should use String type
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Option<String>)),
            "Lstd/core/String;"
        );
    }

    #[test]
    fn test_vec_type_signature() {
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(Vec<i32>)), "[I");
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Vec<String>)),
            "[Lstd/core/String;"
        );
    }

    #[test]
    fn test_record_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(HashMap<String, i32>)),
            "Lescompat/Record;"
        );
    }

    #[test]
    fn test_result_type_signature() {
        // Result<String, Error> should use the Ok type signature
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Result<String, Error>)),
            "Lstd/core/String;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Result<i32, Error>)),
            "I"
        );
    }

    #[test]
    fn test_function_type_signature() {
        // Function<Args, Return> types
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<(), ()>)),
            "Lstd/core/Function;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<(i32,), String>)),
            "Lstd/core/Function;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<FnArgs<(i32, i32)>, i32>)),
            "Lstd/core/Function;"
        );

        // FunctionRef types
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(FunctionRef<(i32,), String>)),
            "Lstd/core/Function;"
        );
    }

    #[test]
    fn test_either_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either<i32, String>)),
            "Lstd/core/Object;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either3<i32, String, bool>)),
            "Lstd/core/Object;"
        );
    }

    #[test]
    fn test_promise_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(PromiseRaw<String>)),
            "Lstd/core/Object;"
        );
    }

    #[test]
    fn test_is_either_type() {
        use super::super::ani_type::AniType;

        // Test using AniType parsing
        let either_ty: syn::Type = syn::parse_quote!(Either<i32, String>);
        assert!(AniType::from_syn_type(&either_ty).is_either());

        let either3_ty: syn::Type = syn::parse_quote!(Either3<i32, String, bool>);
        assert!(AniType::from_syn_type(&either3_ty).is_either());

        let option_ty: syn::Type = syn::parse_quote!(Option<i32>);
        assert!(!AniType::from_syn_type(&option_ty).is_either());

        let vec_ty: syn::Type = syn::parse_quote!(Vec<String>);
        assert!(!AniType::from_syn_type(&vec_ty).is_either());
    }

    #[test]
    fn test_ctor_signature_generation() {
        let sig: Signature = syn::parse_quote! {
            fn init(this: i64, name: String, age: i32)
        };
        assert_eq!(generate_ctor_signature(&sig, true), "Lstd/core/String;I:");
    }
}
