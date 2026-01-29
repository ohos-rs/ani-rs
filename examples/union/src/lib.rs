//! Union 示例 - 处理 union 参数
//!
//! 演示如何处理 ArkTS 的联合类型 (Union Types)
//! 联合类型在 ANI 中统一映射为 Lstd/core/Object;

use ani::prelude::*;
use ani::sys;

// ============================================================================
// 基本联合类型处理
// ============================================================================

/// 处理 string | int 联合类型
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type StringOrInt = string | int;
/// native function handleStringOrInt(value: StringOrInt): string;
/// ```
///
/// Mangling: Lstd/core/Object;:Lstd/core/String;
#[no_mangle]
pub extern "C" fn handle_string_or_int(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    value: sys::ani_object,
) -> sys::ani_string {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return std::ptr::null_mut(),
        };

        let value_obj = AniObject::from_raw(value);

        // 检查是否是 String 类型
        let string_class = match env.find_class("Lstd/core/String;") {
            Ok(c) => c,
            Err(_) => return std::ptr::null_mut(),
        };

        let is_string = match env.object_instance_of(&value_obj, &string_class) {
            Ok(b) => b,
            Err(_) => false,
        };

        let result = if is_string {
            // 是字符串，直接获取
            let str_ref = AniString::from_raw(value as sys::ani_string);
            match env.get_string(&str_ref) {
                Ok(s) => format!("String: {}", s),
                Err(_) => "Error getting string".to_string(),
            }
        } else {
            // 检查是否是 Int 装箱类型
            let int_class = match env.find_class("Lstd/core/Int;") {
                Ok(c) => c,
                Err(_) => return std::ptr::null_mut(),
            };

            let is_int = match env.object_instance_of(&value_obj, &int_class) {
                Ok(b) => b,
                Err(_) => false,
            };

            if is_int {
                // 拆箱获取 int 值
                match env.call_method_by_name_int(&value_obj, "unboxed", Some(":I")) {
                    Ok(i) => format!("Int: {}", i),
                    Err(_) => "Error unboxing int".to_string(),
                }
            } else {
                "Unknown type".to_string()
            }
        };

        match env.create_string(&result) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// 处理 string | ArrayBuffer 联合类型
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type DataType = string | ArrayBuffer;
/// native function handleData(data: DataType): int;
/// ```
#[no_mangle]
pub extern "C" fn handle_data(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    data: sys::ani_object,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return -1,
        };

        let data_obj = AniObject::from_raw(data);

        // 检查是否是 String
        let string_class = match env.find_class("Lstd/core/String;") {
            Ok(c) => c,
            Err(_) => return -2,
        };

        if let Ok(true) = env.object_instance_of(&data_obj, &string_class) {
            let str_ref = AniString::from_raw(data as sys::ani_string);
            return match env.get_string(&str_ref) {
                Ok(s) => s.len() as i32,
                Err(_) => -3,
            };
        }

        // 检查是否是 ArrayBuffer
        let ab_class = match env.find_class("Lescompat/ArrayBuffer;") {
            Ok(c) => c,
            Err(_) => return -4,
        };

        if let Ok(true) = env.object_instance_of(&data_obj, &ab_class) {
            // 获取 ArrayBuffer 的长度
            return match env.call_method_by_name_int(&data_obj, "getByteLength", None) {
                Ok(len) => len,
                Err(_) => -5,
            };
        }

        // 未知类型
        0
    }
}

// ============================================================================
// 返回联合类型
// ============================================================================

/// 返回联合类型值
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function returnUnion(useString: boolean): string | int;
/// ```
///
/// 返回 Object 类型，可以是 String 或装箱的 Int
#[no_mangle]
pub extern "C" fn return_union(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    use_string: sys::ani_boolean,
) -> sys::ani_object {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return std::ptr::null_mut(),
        };

        if use_string != 0 {
            // 返回字符串
            match env.create_string("Hello from union!") {
                Ok(s) => s.into_raw() as sys::ani_object,
                Err(_) => std::ptr::null_mut(),
            }
        } else {
            // 返回装箱的 Int
            let int_class = match env.find_class("Lstd/core/Int;") {
                Ok(c) => c,
                Err(_) => return std::ptr::null_mut(),
            };

            let ctor = match env.find_constructor(&int_class, "I:V") {
                Ok(c) => c,
                Err(_) => return std::ptr::null_mut(),
            };

            let args = [ani_value_int(42)];
            match env.new_object(&int_class, &ctor, &args) {
                Ok(obj) => obj.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

// ============================================================================
// 多类型联合
// ============================================================================

/// 处理多类型联合
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type MultiType = string | int | boolean | double;
/// native function identifyType(value: MultiType): string;
/// ```
#[no_mangle]
pub extern "C" fn identify_type(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    value: sys::ani_object,
) -> sys::ani_string {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return std::ptr::null_mut(),
        };

        let value_obj = AniObject::from_raw(value);

        // 检查各种类型
        let type_name = check_type(&env, &value_obj);

        match env.create_string(&type_name) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

fn check_type(env: &Env, obj: &AniObject) -> String {
    // 检查 String
    if let Ok(cls) = env.find_class("Lstd/core/String;") {
        if let Ok(true) = env.object_instance_of(obj, &cls) {
            return "String".to_string();
        }
    }

    // 检查 Int
    if let Ok(cls) = env.find_class("Lstd/core/Int;") {
        if let Ok(true) = env.object_instance_of(obj, &cls) {
            return "Int".to_string();
        }
    }

    // 检查 Boolean
    if let Ok(cls) = env.find_class("Lstd/core/Boolean;") {
        if let Ok(true) = env.object_instance_of(obj, &cls) {
            return "Boolean".to_string();
        }
    }

    // 检查 Double
    if let Ok(cls) = env.find_class("Lstd/core/Double;") {
        if let Ok(true) = env.object_instance_of(obj, &cls) {
            return "Double".to_string();
        }
    }

    "Unknown".to_string()
}

// ============================================================================
// 简化版本 - 使用宏
// ============================================================================

/// 处理简单的数值或字符串
/// 返回类型标识码: 0=unknown, 1=int, 2=string
#[ani]
pub fn get_type_code(value_type: i32) -> i32 {
    match value_type {
        1 => 1, // int
        2 => 2, // string
        _ => 0, // unknown
    }
}

/// 根据类型创建不同的值
#[ani]
pub fn create_by_type(type_code: i32, int_val: i32, str_val: String) -> String {
    match type_code {
        1 => format!("Int: {}", int_val),
        2 => format!("String: {}", str_val),
        _ => "Unknown type".to_string(),
    }
}

// ============================================================================
// 模块初始化
// ============================================================================
