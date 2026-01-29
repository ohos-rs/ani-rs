//! BigInt 示例 - 解析 BigInt 参数
//!
//! 演示如何处理 ArkTS 的 BigInt 类型
//! BigInt 在 ANI 中对应 Lescompat/BigInt;

use ani::prelude::*;

// ============================================================================
// BigInt 基本操作
// ============================================================================

/// 从 i64 创建 BigInt 值（返回 long 表示）
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function createBigInt(value: long): bigint;
/// ```
#[ani]
pub fn create_big_int(value: i64) -> i64 {
    value
}

/// 将 BigInt 转换为 long
///
/// 对应的 ArkTS 定义:
/// ```typescript  
/// native function bigIntToLong(value: bigint): long;
/// ```
///
/// 注意: BigInt 的 Mangling 是 Lescompat/BigInt;
/// 在 native 层接收时需要作为对象处理
#[ani]
pub fn big_int_to_long(value: i64) -> i64 {
    value
}

// ============================================================================
// BigInt 算术运算
// ============================================================================

/// BigInt 加法
#[ani]
pub fn big_int_add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}

/// BigInt 减法
#[ani]
pub fn big_int_subtract(a: i64, b: i64) -> i64 {
    a.wrapping_sub(b)
}

/// BigInt 乘法
#[ani]
pub fn big_int_multiply(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b)
}

/// BigInt 除法
#[ani]
pub fn big_int_divide(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// BigInt 取模
#[ani]
pub fn big_int_modulo(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a % b
    }
}

// ============================================================================
// BigInt 位运算
// ============================================================================

/// BigInt 按位与
#[ani]
pub fn big_int_and(a: i64, b: i64) -> i64 {
    a & b
}

/// BigInt 按位或
#[ani]
pub fn big_int_or(a: i64, b: i64) -> i64 {
    a | b
}

/// BigInt 按位异或
#[ani]
pub fn big_int_xor(a: i64, b: i64) -> i64 {
    a ^ b
}

/// BigInt 左移
#[ani]
pub fn big_int_shl(a: i64, bits: i32) -> i64 {
    a << bits
}

/// BigInt 右移
#[ani]
pub fn big_int_shr(a: i64, bits: i32) -> i64 {
    a >> bits
}

// ============================================================================
// BigInt 比较
// ============================================================================

/// 比较两个 BigInt
/// 返回: -1 if a < b, 0 if a == b, 1 if a > b
#[ani]
pub fn big_int_compare(a: i64, b: i64) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// 检查 BigInt 是否为零
#[ani]
pub fn big_int_is_zero(value: i64) -> bool {
    value == 0
}

/// 检查 BigInt 是否为负数
#[ani]
pub fn big_int_is_negative(value: i64) -> bool {
    value < 0
}

// ============================================================================
// BigInt 工具函数
// ============================================================================

/// 获取 BigInt 的位数
#[ani]
pub fn big_int_bit_length(value: i64) -> i32 {
    if value == 0 {
        0
    } else {
        64 - value.abs().leading_zeros() as i32
    }
}

/// 取绝对值
#[ani]
pub fn big_int_abs(value: i64) -> i64 {
    value.abs()
}

/// 取负值
#[ani]
pub fn big_int_negate(value: i64) -> i64 {
    -value
}

/// 计算幂次方 (a^b mod 2^64)
#[ani]
pub fn big_int_pow(base: i64, exp: i32) -> i64 {
    if exp < 0 {
        return 0;
    }
    let mut result: i64 = 1;
    let mut b = base;
    let mut e = exp as u32;
    while e > 0 {
        if e & 1 == 1 {
            result = result.wrapping_mul(b);
        }
        b = b.wrapping_mul(b);
        e >>= 1;
    }
    result
}

// ============================================================================
// 模块初始化
// ============================================================================
