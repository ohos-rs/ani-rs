//! Signature Generation
//!
//! Generates ANI type signatures from Rust types using the structured AniType system.

use std::collections::HashSet;

use syn::{FnArg, GenericParam, ReturnType, Signature, Type};

use super::ani_type::AniType;
use crate::codegen::should_skip_in_signature;

// ============================================================================
// Type Signature Generation
// ============================================================================

/// Generate ANI signature from Rust type
#[cfg(test)]
pub fn rust_type_to_signature(ty: &Type) -> String {
    rust_type_to_signature_with_type_params(ty, &HashSet::new())
}

#[cfg(test)]
pub fn rust_type_to_signature_with_type_params(ty: &Type, type_params: &HashSet<String>) -> String {
    let ani_type = AniType::from_syn_type_with_type_params(ty, type_params);
    ani_type.to_signature()
}

fn rust_parameter_type_to_signature_with_type_params(
    ty: &Type,
    type_params: &HashSet<String>,
) -> String {
    let ani_type = AniType::from_syn_type_with_type_params(ty, type_params);
    match ani_type {
        // A standalone ArkTS `undefined` parameter is lowered to the `Any`
        // slot by es2panda. `U` is reserved for an undefined member inside a
        // union descriptor such as `X{C{std.core.String}U}`.
        AniType::Undefined => "Y".to_string(),
        _ => ani_type.to_signature(),
    }
}

fn rust_return_type_to_signature_with_type_params(
    ty: &Type,
    type_params: &HashSet<String>,
) -> String {
    let ani_type = AniType::from_syn_type_with_type_params(ty, type_params);
    match ani_type {
        // The runtime represents a standalone ArkTS `undefined` return using
        // the same empty return descriptor as `void`; the native ABI still
        // returns the undefined reference value.
        AniType::Undefined => String::new(),
        _ => ani_type.to_signature(),
    }
}

// ============================================================================
// Function Signature Generation
// ============================================================================

/// Generate ANI signature from function signature
///
/// This function automatically skips injected parameters (Env, This, Class)
/// since they are not part of the ArkTS function signature.
pub fn generate_fn_signature(sig: &Signature, skip_first: bool) -> String {
    generate_signature_from_inputs(
        &sig.inputs,
        &sig.output,
        skip_first,
        &collect_sig_type_params(sig),
    )
}

/// Generate ANI signature for a constructor.
///
/// Constructor signatures always return void (`:` with no return type suffix).
pub fn generate_ctor_signature(sig: &Signature, skip_first: bool) -> String {
    let mut ctor_sig = String::new();
    let type_params = collect_sig_type_params(sig);

    let params: Vec<_> = sig
        .inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .collect();

    for input in params {
        if let FnArg::Typed(pat_type) = input {
            ctor_sig.push_str(&rust_parameter_type_to_signature_with_type_params(
                &pat_type.ty,
                &type_params,
            ));
        }
    }

    ctor_sig.push(':');
    normalize_bind_signature(&ctor_sig)
}

fn collect_sig_type_params(sig: &Signature) -> HashSet<String> {
    sig.generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .collect()
}

/// Generate signature from parameter list
fn generate_signature_from_inputs(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    output: &ReturnType,
    skip_first: bool,
    type_params: &HashSet<String>,
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
            sig.push_str(&rust_parameter_type_to_signature_with_type_params(
                &pat_type.ty,
                type_params,
            ));
        }
    }

    sig.push(':');

    match output {
        ReturnType::Default => sig.push('V'),
        ReturnType::Type(_, ty) => {
            sig.push_str(&rust_return_type_to_signature_with_type_params(
                ty,
                type_params,
            ));
        }
    }

    normalize_bind_signature(&sig)
}

fn map_primitive_sig(ch: char) -> char {
    match ch {
        'Z' => 'z',
        'B' => 'b',
        'S' => 's',
        'C' => 'c',
        'I' => 'i',
        'J' => 'l',
        'F' => 'f',
        'D' => 'd',
        other => other,
    }
}

fn parse_array_token_len(bytes: &[u8], mut i: usize) -> usize {
    let start = i;
    while i < bytes.len() && bytes[i] == b'[' {
        i += 1;
    }
    if i >= bytes.len() {
        return i - start;
    }
    if bytes[i] == b'L' {
        i += 1;
        while i < bytes.len() && bytes[i] != b';' {
            i += 1;
        }
        if i < bytes.len() {
            i += 1;
        }
    } else {
        i += 1;
    }
    i - start
}

fn parse_braced_token_len(bytes: &[u8], i: usize) -> Option<usize> {
    if i + 2 >= bytes.len() || bytes[i + 1] != b'{' {
        return None;
    }
    if !matches!(bytes[i], b'A' | b'C' | b'E' | b'P' | b'X') {
        return None;
    }

    let mut depth = 0usize;
    let mut j = i + 1;
    while j < bytes.len() {
        match bytes[j] {
            b'{' => depth += 1,
            b'}' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                if depth == 0 {
                    return Some(j - i + 1);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

fn normalize_sig_side(side: &str) -> String {
    let bytes = side.as_bytes();
    let mut out = String::with_capacity(side.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if let Some(len) = parse_braced_token_len(bytes, i) {
            out.push_str(&side[i..i + len]);
            i += len;
            continue;
        }
        match bytes[i] {
            b'[' => {
                let len = parse_array_token_len(bytes, i);
                out.push_str(&side[i..i + len]);
                i += len;
            }
            b'L' => {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] != b';' {
                    j += 1;
                }
                let inner = side[i + 1..j].replace('/', ".");
                out.push_str("C{");
                out.push_str(&inner);
                out.push('}');
                i = if j < bytes.len() { j + 1 } else { j };
            }
            b => {
                out.push(map_primitive_sig(b as char));
                i += 1;
            }
        }
    }
    out
}

fn normalize_bind_signature(signature: &str) -> String {
    let (params, ret) = signature.split_once(':').unwrap_or((signature, ""));
    let params = normalize_sig_side(params);
    let ret = if ret == "V" {
        String::new()
    } else {
        normalize_sig_side(ret)
    };
    format!("{params}:{ret}")
}

// ============================================================================
// Descriptor Conversion
// ============================================================================

/// Convert class name to ANI descriptor
/// Example: "MyModule.MyClass" -> "MyModule.MyClass"
pub fn class_to_descriptor(name: &str) -> String {
    name.to_string()
}

/// Convert namespace to ANI descriptor
pub fn namespace_to_descriptor(name: &str) -> String {
    name.to_string()
}

/// Convert module to ANI descriptor
pub fn module_to_descriptor(name: &str) -> String {
    name.to_string()
}

/// Get current crate module name used by generated ETS file.
///
/// ANI examples and generated declaration file names use package name with
/// `-` converted to `_`.
pub fn current_module_name() -> String {
    for key in ["ANI_MODULE_DESCRIPTOR", "ANI_TEST_MODULE_NAME"] {
        if let Ok(override_name) = std::env::var(key) {
            let trimmed = override_name.trim();
            if !trimmed.is_empty() {
                return trimmed.replace('-', "_");
            }
        }
    }
    std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| String::from("entry"))
        .replace('-', "_")
}

/// Qualify class/namespace names with current module if needed.
///
/// ANI `FindClass`/`FindNamespace` expect descriptors in dotted notation.
/// For user-provided relative names (for example `Calculator` or
/// `example.Person`), prepend the current module name.
pub fn qualify_member_descriptor(name: &str, module_name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('@')
        || trimmed.starts_with("std.")
        || trimmed.starts_with("escompat.")
        || trimmed.starts_with("arkts.")
        || trimmed.starts_with(&format!("{module_name}."))
    {
        trimmed.to_string()
    } else {
        format!("{module_name}.{trimmed}")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_descriptor() {
        assert_eq!(class_to_descriptor("hello.Foo"), "hello.Foo");
        assert_eq!(class_to_descriptor("std.core.String"), "std.core.String");
    }

    #[test]
    fn test_qualify_member_descriptor() {
        assert_eq!(
            qualify_member_descriptor("Calculator", "ani_example_new_class"),
            "ani_example_new_class.Calculator"
        );
        assert_eq!(
            qualify_member_descriptor("example.Person", "ani_example_new_class"),
            "ani_example_new_class.example.Person"
        );
        assert_eq!(
            qualify_member_descriptor("@defModule.foo.Bar", "ani_example_new_class"),
            "@defModule.foo.Bar"
        );
        assert_eq!(
            qualify_member_descriptor("std.core.String", "ani_example_new_class"),
            "std.core.String"
        );
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
        // Option<T> is lowered to nullable union (T | null) at the ANI binding layer
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Option<i32>)),
            "X{C{std.core.Int}C{std.core.Null}}"
        );
        // Option<String> should use string | null union signature at the binding layer
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Option<String>)),
            "X{C{std.core.String}C{std.core.Null}}"
        );
    }

    #[test]
    fn test_explicit_nullish_type_signatures() {
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(Undefined)), "U");
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Null)),
            "C{std.core.Null}"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either<String, Undefined>)),
            "X{C{std.core.String}U}"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either3<String, Null, Undefined>)),
            "X{C{std.core.String}C{std.core.Null}U}"
        );
    }

    #[test]
    fn test_standalone_undefined_function_signature() {
        let sig: Signature = syn::parse_quote! {
            fn round_trip(value: Undefined) -> Undefined
        };
        assert_eq!(generate_fn_signature(&sig, false), "Y:");

        let union_sig: Signature = syn::parse_quote! {
            fn union_round_trip(value: Either<String, Undefined>) -> Either<String, Undefined>
        };
        assert_eq!(
            generate_fn_signature(&union_sig, false),
            "X{C{std.core.String}U}:X{C{std.core.String}U}"
        );
    }

    #[test]
    fn test_vec_type_signature() {
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(Vec<i32>)), "A{i}");
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Vec<String>)),
            "Lstd/core/Array;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Vec<crate::models::UserInfo>)),
            "Lstd/core/Array;"
        );
    }

    #[test]
    fn test_generic_type_param_signature_uses_object_slot() {
        let sig: Signature = syn::parse_quote! {
            fn identity<T>(value: T, values: Vec<T>) -> T
        };
        assert_eq!(
            generate_fn_signature(&sig, false),
            "C{std.core.Object}C{std.core.Array}:C{std.core.Object}"
        );
    }

    #[test]
    fn test_record_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(HashMap<String, i32>)),
            "Lstd/core/Record;"
        );
    }

    #[test]
    fn test_reference_handle_type_signatures() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(GlobalRef)),
            "Lstd/core/Object;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(WeakRef)),
            "Lstd/core/WeakRef;"
        );

        let sig: Signature = syn::parse_quote! {
            fn inspect(global: GlobalRef, weak: WeakRef) -> WeakRef
        };
        assert_eq!(
            generate_fn_signature(&sig, false),
            "C{std.core.Object}C{std.core.WeakRef}:C{std.core.WeakRef}"
        );
    }

    #[test]
    fn test_raw_array_handle_bind_signatures() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(values: AniArray<'_>, refs: AniArrayRef<'_>, fixed: AniFixedArray<'_>, fixed_refs: AniFixedArrayRef<'_>) -> AniFixedArrayRef<'_>
        };
        assert_eq!(
            generate_fn_signature(&sig, false),
            "A{C{std.core.Object}}A{C{std.core.Object}}A{C{std.core.Object}}A{C{std.core.Object}}:A{C{std.core.Object}}"
        );
    }

    #[test]
    fn test_set_and_map_type_signatures() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(HashSet<String>)),
            "Lstd/core/Set;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(BTreeSet<String>)),
            "Lstd/core/Set;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(BTreeMap<String, i32>)),
            "Lstd/core/Map;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(
                ani::conversions::NativePointer<crate::NativeResource>
            )),
            "J"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(
                ani::conversions::ManagedResource<crate::NativeResource>
            )),
            "J"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(ani::conversions::BigInt)),
            "Lstd/core/BigInt;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(ani::conversions::AnyValue)),
            "Lstd/core/Object;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(ani::conversions::TupleValue)),
            "Lstd/core/Object;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(ani::conversions::EnumItem)),
            "Lstd/core/Object;"
        );
    }

    #[test]
    fn test_object_container_type_signatures_stay_container_shaped() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(HashMap<String, crate::models::UserInfo>)),
            "Lstd/core/Record;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(HashSet<crate::models::UserInfo>)),
            "Lstd/core/Set;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(BTreeSet<crate::models::UserInfo>)),
            "Lstd/core/Set;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(BTreeMap<String, crate::models::UserInfo>)),
            "Lstd/core/Map;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(
                ani::conversions::NativePointer<crate::models::UserInfo>
            )),
            "J"
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
            "Lstd/core/Function0;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<(i32,), String>)),
            "Lstd/core/Function1;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<FnArgs<(i32, i32)>, i32>)),
            "Lstd/core/Function2;"
        );

        // FunctionRef types
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(FunctionRef<(i32,), String>)),
            "Lstd/core/Function1;"
        );
    }

    #[test]
    fn test_either_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either<i32, String>)),
            "X{C{std.core.Int}C{std.core.String}}"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Either3<i32, String, bool>)),
            "X{C{std.core.Int}C{std.core.String}C{std.core.Boolean}}"
        );
    }

    #[test]
    fn test_promise_type_signature() {
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(PromiseRaw<String>)),
            "Lstd/core/Promise;"
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
        assert_eq!(generate_ctor_signature(&sig, true), "C{std.core.String}i:");
    }
}
