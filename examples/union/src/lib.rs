//! Union 示例 - 使用 Either 类型处理联合类型
//!
//! 演示如何使用 `Either` 类型优雅地处理 ArkTS 的联合类型 (Union Types)
//! 联合类型在 ANI 中统一映射为 Lstd/core/Object;
//!
//! Either 类型自动处理：
//! - 类型检查 (instanceof)
//! - 装箱/拆箱 (boxing/unboxing)
//! - 类型安全的模式匹配

use ani::conversions::{Either, Either3, Either4};
use ani_derive::ani;

// ============================================================================
// 使用 #[ani] 宏 + Either 类型处理联合类型
// ============================================================================

/// 使用 Either 处理 string | int 联合类型
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type StringOrInt = string | int;
/// native function handleStringOrIntEither(value: StringOrInt): string;
/// ```
#[ani]
pub fn handle_string_or_int_either(value: Either<String, i32>) -> String {
    match value {
        Either::A(s) => format!("String: {}", s),
        Either::B(i) => format!("Int: {}", i),
    }
}

/// 使用 Either3 处理三类型联合
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type ThreeTypes = string | int | boolean;
/// native function handleThreeTypes(value: ThreeTypes): string;
/// ```
#[ani]
pub fn handle_three_types(value: Either3<String, i32, bool>) -> String {
    match value {
        Either3::A(s) => format!("String: {}", s),
        Either3::B(i) => format!("Int: {}", i),
        Either3::C(b) => format!("Boolean: {}", b),
    }
}

/// 使用 Either4 处理四类型联合
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// type FourTypes = string | int | boolean | double;
/// native function handleFourTypes(value: FourTypes): string;
/// ```
#[ani]
pub fn handle_four_types(value: Either4<String, i32, bool, f64>) -> String {
    match value {
        Either4::A(s) => format!("String: {}", s),
        Either4::B(i) => format!("Int: {}", i),
        Either4::C(b) => format!("Boolean: {}", b),
        Either4::D(d) => format!("Double: {}", d),
    }
}

/// 返回联合类型 - 使用 Either
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function returnEither(useString: boolean): string | int;
/// ```
#[ani]
pub fn return_either(use_string: bool) -> Either<String, i32> {
    if use_string {
        Either::A("Hello from Either!".to_string())
    } else {
        Either::B(42)
    }
}

// ============================================================================
// 辅助函数示例
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
