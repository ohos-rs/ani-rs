//! ETS native stub generation from Rust function signatures.
//!
//! Stubs are emitted during macro expansion (compile phase) so no runtime
//! registration/writing is required.

use std::collections::{BTreeMap, btree_map::Entry};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::codegen::should_skip_in_signature;

use super::ani_type::{AniType, PrimitiveType, StringType, WrapperType};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtsDeclKind {
    Global,
    Namespace,
    Class,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsDecl {
    kind: EtsDeclKind,
    target: String,
    signature: String,
    is_static: bool,
}

static ETS_STATE: OnceLock<Mutex<BTreeMap<PathBuf, Vec<EtsDecl>>>> = OnceLock::new();

fn state() -> &'static Mutex<BTreeMap<PathBuf, Vec<EtsDecl>>> {
    ETS_STATE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn output_path() -> Option<PathBuf> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from);
    let pkg = std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| "ani".to_string())
        .replace('-', "_");

    if let Ok(path) = std::env::var("ANI_ETS_OUTPUT") {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            return Some(path);
        }
        if let Some(manifest_dir) = manifest_dir {
            return Some(manifest_dir.join(path));
        }
        return Some(path);
    }

    if let Some(out_dir) = std::env::var("OUT_DIR").ok().map(PathBuf::from) {
        return Some(out_dir.join(format!("{pkg}.ets")));
    }

    let manifest_dir = manifest_dir?;
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .ok()
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                manifest_dir.join(path)
            }
        })
        .unwrap_or_else(|| manifest_dir.join("target"));

    Some(target_dir.join("ani-ets").join(format!("{pkg}.ets")))
}

fn library_name() -> String {
    std::env::var("ANI_ETS_LIBRARY")
        .or_else(|_| std::env::var("CARGO_PKG_NAME").map(|name| name.replace('-', "_")))
        .unwrap_or_else(|_| "ani".to_string())
}

fn namespace_child_mut<'a>(
    map: &'a mut BTreeMap<String, NamespaceNode>,
    key: &str,
) -> &'a mut NamespaceNode {
    match map.entry(key.to_string()) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(NamespaceNode::default()),
    }
}

#[derive(Default)]
struct NamespaceNode {
    functions: Vec<String>,
    classes: BTreeMap<String, Vec<(bool, String)>>,
    children: BTreeMap<String, NamespaceNode>,
}

impl NamespaceNode {
    fn insert_namespace_fn(&mut self, path: &str, signature: &str) {
        let mut node = self;
        for seg in path.split('.').filter(|s| !s.is_empty()) {
            node = namespace_child_mut(&mut node.children, seg);
        }
        node.functions.push(signature.to_string());
    }

    fn insert_class_method(&mut self, path: &str, is_static: bool, signature: &str) {
        let mut parts = path
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let class_name = parts.pop().unwrap_or("NativeClass").to_string();

        let mut node = self;
        for seg in parts {
            node = namespace_child_mut(&mut node.children, seg);
        }

        node.classes
            .entry(class_name)
            .or_default()
            .push((is_static, signature.to_string()));
    }
}

fn render_class_block(
    out: &mut String,
    indent: usize,
    class_name: &str,
    methods: &[(bool, String)],
    lib_name: &str,
) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}class {class_name} {{\n"));

    let mut methods = methods.to_vec();
    methods.sort_by(|a, b| a.1.cmp(&b.1));
    let inner = " ".repeat(indent + 2);
    out.push_str(&format!(
        "{inner}static {{ loadLibrary(\"{lib_name}\"); }}\n"
    ));
    for (is_static, sig) in methods {
        if is_static {
            out.push_str(&format!("{inner}native static {sig};\n"));
        } else {
            out.push_str(&format!("{inner}native {sig};\n"));
        }
    }
    out.push_str(&format!("{pad}}}\n"));
}

fn render_namespace(
    out: &mut String,
    indent: usize,
    name: &str,
    node: &NamespaceNode,
    lib_name: &str,
) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}namespace {name} {{\n"));
    let inner = " ".repeat(indent + 2);

    let mut fns = node.functions.clone();
    fns.sort();
    if !fns.is_empty() {
        out.push_str(&format!("{inner}loadLibrary(\"{lib_name}\");\n"));
    }
    for sig in fns {
        out.push_str(&format!("{inner}native function {sig};\n"));
    }

    for (class_name, methods) in &node.classes {
        render_class_block(out, indent + 2, class_name, methods, lib_name);
    }

    for (child, child_node) in &node.children {
        render_namespace(out, indent + 2, child, child_node, lib_name);
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn section_break(out: &mut String, has_content: &mut bool) {
    if *has_content {
        out.push('\n');
    }
    *has_content = true;
}

fn render_decls(decls: &[EtsDecl]) -> String {
    let mut out = String::from("// Auto-generated by ani-rs at compile time.\n\n");
    let lib_name = library_name();

    let mut globals: Vec<String> = Vec::new();
    let mut root = NamespaceNode::default();

    for decl in decls {
        match decl.kind {
            EtsDeclKind::Global => globals.push(decl.signature.clone()),
            EtsDeclKind::Namespace => root.insert_namespace_fn(&decl.target, &decl.signature),
            EtsDeclKind::Class => {
                root.insert_class_method(&decl.target, decl.is_static, &decl.signature)
            }
        }
    }

    globals.sort();
    let mut has_content = false;

    if !globals.is_empty() {
        section_break(&mut out, &mut has_content);
        out.push_str(&format!("loadLibrary(\"{lib_name}\");\n"));
        for sig in globals {
            out.push_str(&format!("native function {sig};\n"));
        }
    }

    for (class_name, methods) in &root.classes {
        section_break(&mut out, &mut has_content);
        render_class_block(&mut out, 0, class_name, methods, &lib_name);
    }

    for (name, node) in &root.children {
        section_break(&mut out, &mut has_content);
        render_namespace(&mut out, 0, name, node, &lib_name);
    }

    out
}

pub fn emit_compile_ets_decl(kind: EtsDeclKind, target: &str, signature: &str, is_static: bool) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let decls = state.entry(path.clone()).or_default();
    let item = EtsDecl {
        kind,
        target: target.to_string(),
        signature: signature.to_string(),
        is_static,
    };
    if !decls.contains(&item) {
        decls.push(item);
    }
    let content = render_decls(decls);
    drop(state);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, content);
}

fn rust_type_to_ets(ty: &Type) -> String {
    ani_type_to_ets(&AniType::from_syn_type(ty))
}

fn ani_type_to_ets(ty: &AniType) -> String {
    match ty {
        AniType::Primitive(p) => primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "string".to_string(),
        AniType::Unit => "void".to_string(),
        AniType::Wrapper(WrapperType::Option(inner)) => {
            format!("{} | null", ani_type_to_ets(inner))
        }
        AniType::Wrapper(WrapperType::Vec(inner)) => format!("Array<{}>", ani_type_to_ets(inner)),
        AniType::Wrapper(WrapperType::Result(inner)) => ani_type_to_ets(inner),
        AniType::Wrapper(WrapperType::Ref(inner)) => ani_type_to_ets(inner),
        AniType::Function(_) => "Function".to_string(),
        AniType::FnArgs(_) => "Array<Object>".to_string(),
        AniType::Either(either) => either
            .types
            .iter()
            .map(|item| rust_type_to_ets(item.as_ref()))
            .collect::<Vec<_>>()
            .join(" | "),
        AniType::Promise(promise) => {
            let inner = promise
                .inner
                .as_ref()
                .map(|t| ani_type_to_ets(t))
                .unwrap_or_else(|| "void".to_string());
            format!("Promise<{}>", inner)
        }
        AniType::Record(record) => format!("Record<string, {}>", ani_type_to_ets(&record.value)),
        AniType::AniObject => "Object".to_string(),
        AniType::ArrayBuffer => "ArrayBuffer".to_string(),
        AniType::Tuple(items) => format!(
            "[{}]",
            items
                .iter()
                .map(ani_type_to_ets)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        AniType::Unknown(ty) => unknown_type_to_ets(ty).unwrap_or_else(|| "Object".to_string()),
    }
}

fn unknown_type_to_ets(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => path_type_to_ets(type_path),
        Type::Reference(type_ref) => unknown_type_to_ets(type_ref.elem.as_ref()),
        Type::Paren(type_paren) => unknown_type_to_ets(type_paren.elem.as_ref()),
        Type::Group(type_group) => unknown_type_to_ets(type_group.elem.as_ref()),
        _ => None,
    }
}

fn path_type_to_ets(type_path: &syn::TypePath) -> Option<String> {
    let segment = type_path.path.segments.last()?;
    let ident = segment.ident.to_string();

    if let Some(mapped) = known_ani_runtime_type(&ident) {
        return Some(mapped.to_string());
    }

    if is_custom_object_name(&ident) {
        return Some(ident);
    }

    None
}

fn known_ani_runtime_type(ident: &str) -> Option<&'static str> {
    match ident {
        "AniString" => Some("string"),
        "AniArrayBuffer" => Some("ArrayBuffer"),
        "AniFnObject" | "AniFunction" => Some("Function"),
        "AniArray" | "AniArrayRef" | "AniFixedArray" | "AniFixedArrayRef" => Some("Array<Object>"),
        "AniArrayInt" | "AniFixedArrayInt" => Some("Array<int>"),
        "AniArrayLong" | "AniFixedArrayLong" => Some("Array<long>"),
        "AniArrayDouble" | "AniFixedArrayDouble" => Some("Array<double>"),
        "AniFixedArrayFloat" => Some("Array<float>"),
        "AniFixedArrayByte" => Some("Array<byte>"),
        "AniFixedArrayShort" => Some("Array<short>"),
        "AniFixedArrayChar" => Some("Array<char>"),
        "AniFixedArrayBoolean" => Some("Array<boolean>"),
        "AniRef" | "AniObject" | "AniClass" | "AniType" | "AniModule" | "AniNamespace"
        | "AniEnum" | "AniError" | "AniEnumItem" | "AniTupleValue" | "AniMethod"
        | "AniStaticMethod" | "AniField" | "AniStaticField" | "AniVariable" | "AniResolver"
        | "GlobalRef" | "WeakRef" => Some("Object"),
        _ => None,
    }
}

fn is_custom_object_name(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let Some(first) = ident.chars().next() else {
        return false;
    };
    if !first.is_ascii_uppercase() {
        return false;
    }

    !matches!(
        ident,
        "String"
            | "Option"
            | "Result"
            | "Vec"
            | "Box"
            | "Rc"
            | "Arc"
            | "HashMap"
            | "BTreeMap"
            | "HashSet"
            | "BTreeSet"
            | "VecDeque"
            | "LinkedList"
            | "Cow"
            | "PathBuf"
    )
}

fn primitive_to_ets(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "boolean",
        PrimitiveType::I8 | PrimitiveType::U8 => "byte",
        PrimitiveType::I16 => "short",
        PrimitiveType::U16 | PrimitiveType::Char => "char",
        PrimitiveType::I32 | PrimitiveType::U32 => "int",
        PrimitiveType::I64 | PrimitiveType::U64 => "long",
        PrimitiveType::F32 => "float",
        PrimitiveType::F64 => "double",
    }
}

/// Generate ETS declaration signature string for a function.
///
/// Output example: `add(a: int, b: int): int`
pub fn generate_fn_ets_decl(sig: &Signature, ets_name: &str, skip_first: bool) -> String {
    let mut params: Vec<String> = Vec::new();

    for (idx, arg) in sig
        .inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .enumerate()
    {
        if let FnArg::Typed(pat_type) = arg {
            let name = match pat_type.pat.as_ref() {
                Pat::Ident(ident) => ident.ident.to_string(),
                _ => format!("arg{}", idx),
            };
            let ty = rust_type_to_ets(&pat_type.ty);
            params.push(format!("{name}: {ty}"));
        }
    }

    let ret = match &sig.output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => rust_type_to_ets(ty),
    };

    format!("{ets_name}({}): {ret}", params.join(", "))
}

/// Generate ETS declaration signature string for a constructor.
///
/// Output example: `constructor(name: string, age: int)`
pub fn generate_ctor_ets_decl(sig: &Signature, skip_first: bool) -> String {
    let mut params: Vec<String> = Vec::new();

    for (idx, arg) in sig
        .inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .enumerate()
    {
        if let FnArg::Typed(pat_type) = arg {
            let name = match pat_type.pat.as_ref() {
                Pat::Ident(ident) => ident.ident.to_string(),
                _ => format!("arg{}", idx),
            };
            let ty = rust_type_to_ets(&pat_type.ty);
            params.push(format!("{name}: {ty}"));
        }
    }

    format!("constructor({})", params.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fn_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn add(a: i32, b: i32) -> i32
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "add", false),
            "add(a: int, b: int): int"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_keeps_custom_object_types() {
        let sig: Signature = syn::parse_quote! {
            fn process_user(user: UserInfo, maybe: Option<UserInfo>, list: Vec<UserInfo>) -> UserInfo
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "process_user", false),
            "process_user(user: UserInfo, maybe: UserInfo | null, list: Array<UserInfo>): UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_known_ani_handle_types() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(s: AniString<'_>, buffer: AniArrayBuffer<'_>, field: AniField, cb: AniFunction) -> AniFunction
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "inspect", false),
            "inspect(s: string, buffer: ArrayBuffer, field: Object, cb: Function): Function"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_hashmap_to_record() {
        let sig: Signature = syn::parse_quote! {
            fn summarize(input: HashMap<String, i32>) -> HashMap<String, String>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "summarize", false),
            "summarize(input: Record<string, int>): Record<string, string>"
        );
    }

    #[test]
    fn test_render_decls_uses_ani_native_style() {
        let decls = vec![
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                signature: "add(a: int, b: int): int".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "Math.Utils".to_string(),
                signature: "sqrt(x: double): double".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                signature: "getName(): string".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                signature: "create(name: string): long".to_string(),
                is_static: true,
            },
        ];

        let rendered = render_decls(&decls);

        assert!(!rendered.contains("declare "));
        assert!(rendered.contains("loadLibrary(\""));
        assert!(rendered.contains("native function add(a: int, b: int): int;"));
        assert!(rendered.contains("namespace Math {"));
        assert!(rendered.contains("native function sqrt(x: double): double;"));
        assert!(rendered.contains("class Person {"));
        assert!(rendered.contains("native getName(): string;"));
        assert!(rendered.contains("native static create(name: string): long;"));
    }

    #[test]
    fn test_generate_ctor_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn person_new(this: i64, name: String, age: i32)
        };
        assert_eq!(
            generate_ctor_ets_decl(&sig, true),
            "constructor(name: string, age: int)"
        );
    }
}
