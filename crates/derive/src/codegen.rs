//! Code Generation Utilities

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, ReturnType};

/// Generate wrapper function
pub fn generate_wrapper(
    func: &ItemFn,
    wrapper_name: &Ident,
    is_class_method: bool,
    is_static: bool,
) -> TokenStream {
    let func_name = &func.sig.ident;
    let return_type = &func.sig.output;

    // 生成 wrapper 参数
    let mut wrapper_params = vec![quote! { env: *mut ani_sys::ani_env }];

    // 类方法需要额外的 this/class 参数
    if is_class_method {
        if is_static {
            wrapper_params.push(quote! { _class: ani_sys::ani_class });
        } else {
            wrapper_params.push(quote! { this: ani_sys::ani_object });
        }
    }

    // 收集用户参数（跳过 self）
    let user_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter(|arg| !matches!(arg, FnArg::Receiver(_)))
        .collect();

    // 添加用户参数到 wrapper
    for (i, param) in user_params.iter().enumerate() {
        if let FnArg::Typed(pat_type) = param {
            let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                pat_ident.ident.clone()
            } else {
                format_ident!("arg{}", i)
            };

            let ani_type = rust_type_to_ani_type(&pat_type.ty);
            wrapper_params.push(quote! { #param_name: #ani_type });
        }
    }

    // 生成参数转换代码
    let conversions = generate_conversions(&user_params);

    // 生成调用参数
    let call_args: Vec<_> = user_params
        .iter()
        .enumerate()
        .map(|(i, param)| {
            if let FnArg::Typed(pat_type) = param {
                let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    format_ident!("{}_converted", pat_ident.ident)
                } else {
                    format_ident!("arg{}_converted", i)
                };
                quote! { #param_name }
            } else {
                quote! {}
            }
        })
        .collect();

    // 生成函数调用
    let func_call = quote! {
        let result = #func_name(#(#call_args),*);
    };

    // 生成返回值转换
    let return_conversion = generate_return_conversion(return_type);

    // 生成返回类型
    let wrapper_return = match return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            let ani_type = rust_type_to_ani_type(ty);
            quote! { -> #ani_type }
        }
    };

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables)]
        unsafe extern "C" fn #wrapper_name(#(#wrapper_params),*) #wrapper_return {
            #conversions
            #func_call
            #return_conversion
        }
    }
}

/// Generate parameter conversion code
fn generate_conversions(params: &[&FnArg]) -> TokenStream {
    let conversions: Vec<TokenStream> = params
        .iter()
        .enumerate()
        .map(|(i, param)| {
            if let FnArg::Typed(pat_type) = param {
                let param_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    pat_ident.ident.clone()
                } else {
                    format_ident!("arg{}", i)
                };
                let converted_name = format_ident!("{}_converted", param_name);
                let ty = &pat_type.ty;

                let type_str = quote!(#ty).to_string().replace(" ", "");

                match type_str.as_str() {
                    "i32" | "i8" | "i16" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                        quote! { let #converted_name = #param_name; }
                    }
                    "f32" | "f64" => {
                        quote! { let #converted_name = #param_name; }
                    }
                    "bool" => {
                        quote! { let #converted_name = #param_name != 0; }
                    }
                    "String" => {
                        quote! {
                            let #converted_name = {
                                // Convert ani_string to Rust String
                                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                                let ani_str = ani::types::AniString::from_raw(#param_name);
                                env_wrapper.get_string(&ani_str).unwrap_or_default()
                            };
                        }
                    }
                    "&str" => {
                        quote! {
                            let #converted_name = {
                                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                                let ani_str = ani::types::AniString::from_raw(#param_name);
                                env_wrapper.get_string(&ani_str).unwrap_or_default()
                            };
                            let #converted_name = #converted_name.as_str();
                        }
                    }
                    _ => {
                        quote! { let #converted_name = #param_name; }
                    }
                }
            } else {
                quote! {}
            }
        })
        .collect();

    quote! { #(#conversions)* }
}

/// Generate return value conversion code
fn generate_return_conversion(return_type: &ReturnType) -> TokenStream {
    match return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            let type_str = quote!(#ty).to_string().replace(" ", "");

            // Handle Result type
            if type_str.starts_with("Result<") {
                return quote! {
                    match result {
                        Ok(val) => val,
                        Err(e) => {
                            // TODO: Throw ANI exception
                            eprintln!("Error: {:?}", e);
                            Default::default()
                        }
                    }
                };
            }

            match type_str.as_str() {
                "i32" | "i64" | "i8" | "i16" | "u8" | "u16" | "u32" | "u64" => {
                    quote! { result }
                }
                "f32" | "f64" => {
                    quote! { result }
                }
                "bool" => {
                    quote! { if result { 1 } else { 0 } }
                }
                "String" => {
                    quote! {
                        // Convert String to ani_string
                        let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                        match env_wrapper.create_string(&result) {
                            Ok(s) => s.into_raw(),
                            Err(_) => std::ptr::null_mut()
                        }
                    }
                }
                "()" => quote! {},
                _ => quote! { result },
            }
        }
    }
}

/// Rust type to ANI C type
fn rust_type_to_ani_type(ty: &syn::Type) -> TokenStream {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => quote! { ani_sys::ani_boolean },
        "i8" => quote! { ani_sys::ani_byte },
        "u8" => quote! { ani_sys::ani_byte },
        "i16" => quote! { ani_sys::ani_short },
        "u16" => quote! { ani_sys::ani_char },
        "char" => quote! { ani_sys::ani_char },
        "i32" | "u32" => quote! { ani_sys::ani_int },
        "i64" | "u64" => quote! { ani_sys::ani_long },
        "f32" => quote! { ani_sys::ani_float },
        "f64" => quote! { ani_sys::ani_double },
        "String" | "&str" => quote! { ani_sys::ani_string },
        "()" => quote! { () },
        _ => {
            if type_str.starts_with("Vec<") {
                quote! { ani_sys::ani_array }
            } else if type_str.starts_with("Option<") {
                quote! { ani_sys::ani_object }
            } else if type_str.starts_with("Result<") {
                // Extract Ok type
                if let syn::Type::Path(type_path) = ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(ok_type)) = args.args.first() {
                                return rust_type_to_ani_type(ok_type);
                            }
                        }
                    }
                }
                quote! { ani_sys::ani_object }
            } else {
                quote! { ani_sys::ani_object }
            }
        }
    }
}

/// Generate class registration function
pub fn generate_class_register(
    register_name: &Ident,
    class_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani_sys::ani_env) -> ani_sys::ani_status {
            let mut cls: ani_sys::ani_class = std::ptr::null_mut();
            let class_name = concat!(#class_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindClass.unwrap())(
                env,
                class_name.as_ptr() as *const std::os::raw::c_char,
                &mut cls
            );

            if status != ani_sys::ani_status_ANI_OK {
                return status;
            }

            let methods = [
                ani_sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            (api.Class_BindNativeMethods.unwrap())(
                env,
                cls,
                methods.as_ptr(),
                methods.len()
            )
        }
    }
}

/// Generate namespace registration function
pub fn generate_namespace_register(
    register_name: &Ident,
    namespace_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani_sys::ani_env) -> ani_sys::ani_status {
            let mut ns: ani_sys::ani_namespace = std::ptr::null_mut();
            let ns_name = concat!(#namespace_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindNamespace.unwrap())(
                env,
                ns_name.as_ptr() as *const std::os::raw::c_char,
                &mut ns
            );

            if status != ani_sys::ani_status_ANI_OK {
                return status;
            }

            let functions = [
                ani_sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            (api.Namespace_BindNativeFunctions.unwrap())(
                env,
                ns,
                functions.as_ptr(),
                functions.len()
            )
        }
    }
}

/// Generate module registration function
pub fn generate_module_register(
    register_name: &Ident,
    module_descriptor: &str,
    func_name: &str,
    signature: &str,
    wrapper_name: &Ident,
) -> TokenStream {
    let find_module = if module_descriptor.is_empty() {
        // If no module specified, infer from filename
        quote! {
            // Use default module
            let mut module: ani_sys::ani_module = std::ptr::null_mut();
        }
    } else {
        quote! {
            let mut module: ani_sys::ani_module = std::ptr::null_mut();
            let module_name = concat!(#module_descriptor, "\0");

            let api = &*(*env);
            let status = (api.FindModule.unwrap())(
                env,
                module_name.as_ptr() as *const std::os::raw::c_char,
                &mut module
            );

            if status != ani_sys::ani_status_ANI_OK {
                return status;
            }
        }
    };

    quote! {
        #[doc(hidden)]
        pub unsafe extern "C" fn #register_name(env: *mut ani_sys::ani_env) -> ani_sys::ani_status {
            #find_module

            let functions = [
                ani_sys::ani_native_function {
                    name: concat!(#func_name, "\0").as_ptr() as *const std::os::raw::c_char,
                    signature: concat!(#signature, "\0").as_ptr() as *const std::os::raw::c_char,
                    pointer: #wrapper_name as *const std::os::raw::c_void,
                }
            ];

            let api = &*(*env);
            (api.Module_BindNativeFunctions.unwrap())(
                env,
                module,
                functions.as_ptr(),
                functions.len()
            )
        }
    }
}
