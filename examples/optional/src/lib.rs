//! Optional 参数示例 - 处理可选参数
//!
//! 演示如何处理 ArkTS 的可选参数 (?)
//! 使用泛型 Option<T> 实现，所有实现了 ToAni/FromAni 的类型都支持 Option
//!
//! 注意: 可选的基本类型会被自动装箱，如 int? -> Lstd/core/Int;

use ani_derive::ani;

// ============================================================================
// 基本可选参数 - 使用 #[ani] 宏和 Option<T>
// ============================================================================

/// 处理可选的 int 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalInt(required: int, optional?: int): int;
/// ```
///
/// Mangling: ILstd/core/Int;:I
/// Option<i32> 会自动处理装箱的 Int，None 表示 null
#[ani]
pub fn with_optional_int(required: i32, optional: Option<i32>) -> i32 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
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
#[ani]
pub fn with_optional_double(required: f64, optional: Option<f64>) -> f64 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
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
#[ani]
pub fn with_optional_boolean(value: i32, flag: Option<bool>) -> i32 {
    match flag {
        Some(true) => value * 2,
        Some(false) | None => value,
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
/// 引用类型的可选参数不会装箱，直接传 null
#[ani]
pub fn with_optional_string(required: String, optional: Option<String>) -> String {
    match optional {
        Some(opt_value) => format!("{} {}", required, opt_value),
        None => required,
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
#[ani]
pub fn with_multiple_optional(a: i32, b: Option<i32>, c: Option<i32>, d: Option<i32>) -> i32 {
    a + b.unwrap_or(0) + c.unwrap_or(0) + d.unwrap_or(0)
}

// ============================================================================
// 更多类型示例 - 展示泛型支持的其他类型
// ============================================================================

/// 处理可选的 long (i64) 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalLong(required: long, optional?: long): long;
/// ```
#[ani]
pub fn with_optional_long(required: i64, optional: Option<i64>) -> i64 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
    }
}

/// 处理可选的 float (f32) 参数
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function withOptionalFloat(required: float, optional?: float): float;
/// ```
#[ani]
pub fn with_optional_float(required: f32, optional: Option<f32>) -> f32 {
    match optional {
        Some(opt_value) => required + opt_value,
        None => required,
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
