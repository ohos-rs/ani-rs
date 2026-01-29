//! Optional 参数示例 - 处理可选参数
//!
//! 演示如何处理 ArkTS 的可选参数 (?)
//! 注意: 可选的基本类型会被装箱，如 int? -> Lstd/core/Int;

use ani::prelude::*;
use ani::sys;
use ani_derive::ani;

// ============================================================================
// 基本可选参数
// ============================================================================

/// 处理可选的 int 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalInt(required: int, optional?: int): int;
/// ```
///
/// Mangling: ILstd/core/Int;:I
/// 注意: optional? 参数会被装箱为 Int 类
///
/// 这里使用低级 API 来处理装箱的 Int
#[no_mangle]
pub extern "C" fn with_optional_int(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    required: sys::ani_int,
    optional: sys::ani_object, // 装箱的 Int 或 null
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return required,
        };

        // 检查 optional 是否为 null
        if optional.is_null() {
            return required;
        }

        // 拆箱获取值
        let optional_obj = AniObject::from_raw(optional);
        match env.call_method_by_name_int(&optional_obj, "unboxed", Some(":I")) {
            Ok(opt_value) => required + opt_value,
            Err(_) => required,
        }
    }
}

/// 处理可选的 double 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalDouble(required: double, optional?: double): double;
/// ```
///
/// Mangling: DLstd/core/Double;:D
#[no_mangle]
pub extern "C" fn with_optional_double(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    required: sys::ani_double,
    optional: sys::ani_object, // 装箱的 Double 或 null
) -> sys::ani_double {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return required,
        };

        if optional.is_null() {
            return required;
        }

        let optional_obj = AniObject::from_raw(optional);
        match env.call_method_by_name_double(&optional_obj, "unboxed", Some(":D")) {
            Ok(opt_value) => required + opt_value,
            Err(_) => required,
        }
    }
}

/// 处理可选的 boolean 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalBoolean(value: int, flag?: boolean): int;
/// ```
///
/// Mangling: ILstd/core/Boolean;:I
#[no_mangle]
pub extern "C" fn with_optional_boolean(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    value: sys::ani_int,
    flag: sys::ani_object, // 装箱的 Boolean 或 null
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return value,
        };

        if flag.is_null() {
            return value;
        }

        let flag_obj = AniObject::from_raw(flag);
        match env.call_method_by_name_boolean(&flag_obj, "unboxed", Some(":Z")) {
            Ok(flag_value) => {
                if flag_value {
                    value * 2
                } else {
                    value
                }
            }
            Err(_) => value,
        }
    }
}

// ============================================================================
// 可选字符串参数（引用类型不需要装箱）
// ============================================================================

/// 处理可选的 string 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalString(required: string, optional?: string): string;
/// ```
///
/// Mangling: Lstd/core/String;Lstd/core/String;:Lstd/core/String;
/// 注意: 引用类型的可选参数不会装箱，直接传 null
#[no_mangle]
pub extern "C" fn with_optional_string(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    required: sys::ani_string,
    optional: sys::ani_string, // String 或 null
) -> sys::ani_string {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return required,
        };

        let required_str = AniString::from_raw(required);
        let required_value = match env.get_string(&required_str) {
            Ok(s) => s,
            Err(_) => return required,
        };

        let result = if optional.is_null() {
            required_value
        } else {
            let optional_str = AniString::from_raw(optional);
            match env.get_string(&optional_str) {
                Ok(opt_value) => format!("{} {}", required_value, opt_value),
                Err(_) => required_value,
            }
        };

        match env.create_string(&result) {
            Ok(s) => s.into_raw(),
            Err(_) => required,
        }
    }
}

// ============================================================================
// 多个可选参数
// ============================================================================

/// 处理多个可选参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withMultipleOptional(
///     a: int,
///     b?: int,
///     c?: int,
///     d?: int
/// ): int;
/// ```
///
/// Mangling: ILstd/core/Int;Lstd/core/Int;Lstd/core/Int;:I
#[no_mangle]
pub extern "C" fn with_multiple_optional(
    env: *mut sys::ani_env,
    _obj: sys::ani_object,
    a: sys::ani_int,
    b: sys::ani_object,
    c: sys::ani_object,
    d: sys::ani_object,
) -> sys::ani_int {
    unsafe {
        let env = match Env::from_raw(env) {
            Ok(e) => e,
            Err(_) => return a,
        };

        let mut sum = a;

        // 拆箱 b
        if !b.is_null() {
            let b_obj = AniObject::from_raw(b);
            if let Ok(v) = env.call_method_by_name_int(&b_obj, "unboxed", Some(":I")) {
                sum += v;
            }
        }

        // 拆箱 c
        if !c.is_null() {
            let c_obj = AniObject::from_raw(c);
            if let Ok(v) = env.call_method_by_name_int(&c_obj, "unboxed", Some(":I")) {
                sum += v;
            }
        }

        // 拆箱 d
        if !d.is_null() {
            let d_obj = AniObject::from_raw(d);
            if let Ok(v) = env.call_method_by_name_int(&d_obj, "unboxed", Some(":I")) {
                sum += v;
            }
        }

        sum
    }
}

// ============================================================================
// 简化版本（使用宏包装）
// ============================================================================

/// 使用宏的简化版本 - 有默认值的参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withDefault(value: int, multiplier: int = 1): int;
/// ```
///
/// 注意: 带默认值的参数与可选参数处理方式相同
#[ani]
pub fn with_default_simple(value: i32, multiplier: i32) -> i32 {
    value * multiplier
}

/// 可选参数计数
///
/// 统计有多少个非 null 的可选参数
#[ani]
pub fn count_provided_args() -> i32 {
    // 这个函数只是演示目的
    // 实际需要检查每个参数是否为 null
    0
}

// ============================================================================
// 模块初始化
// ============================================================================
