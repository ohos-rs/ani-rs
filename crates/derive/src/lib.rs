//! # ANI Derive Macros
//!
//! Provides simple and easy-to-use macros similar to napi-rs for generating ANI binding code.
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use ani_derive::ani;
//!
//! // Module-level function
//! #[ani]
//! fn add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//!
//! // Class method
//! #[ani(class = "Calculator")]
//! fn multiply(&self, a: f64, b: f64) -> f64 {
//!     a * b
//! }
//!
//! // Namespace function
//! #[ani(namespace = "Math")]
//! fn sqrt(x: f64) -> f64 {
//!     x.sqrt()
//! }
//!
//! // Initialization function
//! #[ani(init)]
//! fn setup() {
//!     println!("Module initialized!");
//! }
//! ```

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, ItemImpl, ItemStruct, parse_macro_input};

mod attrs;
mod codegen;
mod expand;
mod signature;
mod types;

use attrs::*;
use expand::*;

/// Unified ANI binding macro
///
/// Exports Rust functions, structs, and impl blocks as ArkTS native bindings.
/// Supports module-level functions, class methods, namespaces, and more.
///
/// # Basic Usage (Function Binding)
///
/// - `#[ani]` - Bind to module level
/// - `#[ani(namespace = "MyNamespace")]` - Bind to namespace
/// - `#[ani(class = "MyClass")]` - Bind as class instance method
/// - `#[ani(class = "MyClass", static)]` - Bind as class static method
/// - `#[ani(class = "MyClass", constructor)]` - Bind as constructor
/// - `#[ani(getter = "propertyName")]` - Bind as property getter
/// - `#[ani(setter = "propertyName")]` - Bind as property setter
///
/// # Initialization Function
///
/// - `#[ani(init)]` - Mark as initialization function (called during module load)
/// - `#[ani(init, before_bindings)]` - Call before bindings are registered
///
/// # Object/Class Definition
///
/// - `#[ani(object)]` - Define ANI object type
/// - `#[ani(object = "CustomName")]` - Use custom name
///
/// # Examples
///
/// ```rust,ignore
/// // Module-level function
/// #[ani]
/// fn greet(name: String) -> String {
///     format!("Hello, {}!", name)
/// }
///
/// // Class instance method
/// #[ani(class = "Person")]
/// fn get_age(this: i64) -> i32 {
///     // ... implementation
///     42
/// }
///
/// // Initialization function
/// #[ani(init)]
/// fn setup() {
///     println!("Module initialized!");
/// }
///
/// // Object definition
/// #[ani(object)]
/// struct Person {
///     name: String,
///     age: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn ani(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AniAttrs);

    // 根据宏类型分发处理
    match attrs.kind {
        AniMacroKind::Init => {
            // Initialization function
            if let Ok(func) = syn::parse::<ItemFn>(item.clone()) {
                return expand_init(attrs.into(), func).into();
            }
            TokenStream::from(
                syn::Error::new_spanned(
                    proc_macro2::TokenStream::from(item),
                    "#[ani(init)] can only be applied to functions",
                )
                .to_compile_error(),
            )
        }
        AniMacroKind::Object => {
            // Object/class definition
            if let Ok(struct_item) = syn::parse::<ItemStruct>(item.clone()) {
                let mut bindgen_attrs: AniBindgenAttrs = attrs.clone().into();
                if let Some(ref obj_name) = attrs.object_name {
                    bindgen_attrs.class = Some(obj_name.clone());
                }
                return expand_struct(bindgen_attrs, struct_item).into();
            }
            TokenStream::from(
                syn::Error::new_spanned(
                    proc_macro2::TokenStream::from(item),
                    "#[ani(object)] can only be applied to structs",
                )
                .to_compile_error(),
            )
        }
        AniMacroKind::Bindgen => {
            // Default binding logic
            let bindgen_attrs: AniBindgenAttrs = attrs.into();

            // Try to parse as function
            if let Ok(func) = syn::parse::<ItemFn>(item.clone()) {
                return expand_function(bindgen_attrs, func).into();
            }

            // Try to parse as impl block
            if let Ok(impl_block) = syn::parse::<ItemImpl>(item.clone()) {
                return expand_impl(bindgen_attrs, impl_block).into();
            }

            // Try to parse as struct
            if let Ok(struct_item) = syn::parse::<ItemStruct>(item.clone()) {
                return expand_struct(bindgen_attrs, struct_item).into();
            }

            TokenStream::from(
                syn::Error::new_spanned(
                    proc_macro2::TokenStream::from(item),
                    "#[ani] can only be applied to functions, impl blocks, or structs",
                )
                .to_compile_error(),
            )
        }
    }
}

/// Derive macro to generate ANI class bindings for structs
///
/// # Examples
///
/// ```rust,ignore
/// #[derive(AniClass)]
/// #[ani(class = "Person")]
/// struct Person {
///     #[ani(getter, setter)]
///     name: String,
///     
///     #[ani(getter)]
///     age: i32,
/// }
/// ```
#[proc_macro_derive(AniClass, attributes(ani))]
pub fn derive_ani_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_class_derive(input).into()
}

/// Module initialization macro
///
/// Used to generate the `ANI_Constructor` entry function.
///
/// # Examples
///
/// ```rust,ignore
/// ani_module! {
///     name: "my_module",
///     functions: [add, subtract, multiply],
///     classes: [Calculator, Person],
/// }
/// ```
#[proc_macro]
pub fn ani_module(input: TokenStream) -> TokenStream {
    let module_def = parse_macro_input!(input as AniModuleDef);
    expand_module(module_def).into()
}
