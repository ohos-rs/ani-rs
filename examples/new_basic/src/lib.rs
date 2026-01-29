//! 基础示例 - 展示 ani-rs 的简单使用方法
//!
//! 这个示例演示了如何使用 `#[ani]` 宏创建 ANI 绑定，类似于 napi-rs。
//!
//! `#[ani]` 是一个统一的宏，可以用于：
//! - 模块级函数绑定（自动注册，无需手动列出！）
//! - 类方法绑定（实例方法和静态方法）
//! - 命名空间函数绑定
//! - 初始化函数标记
//!
//! 使用 `ctor` crate 实现类似 napi-rs 的自动注册机制，
//! 所有标记了 `#[ani]` 的函数会在库加载时自动注册到全局注册表中。

use ani_derive::ani;

// ============================================================================
// 基础数学函数 - 模块级别（自动注册！）
// ============================================================================

/// 两数相加
#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 两数相减
#[ani]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 两数相乘
#[ani]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 两数相除（b 为 0 时返回 0）
#[ani]
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

// ============================================================================
// 字符串操作
// ============================================================================

/// 问候函数
#[ani]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// 字符串长度
#[ani]
pub fn string_length(s: String) -> i32 {
    s.len() as i32
}

// ============================================================================
// 高级数学函数
// ============================================================================

/// 计算阶乘
#[ani]
pub fn factorial(n: i32) -> i64 {
    if n <= 1 {
        1
    } else {
        (1..=n as i64).product()
    }
}

/// 判断是否为质数
#[ani]
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// 计算最大公约数
#[ani]
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// 计算斐波那契数
#[ani]
pub fn fibonacci(n: i32) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0i64;
    let mut b = 1i64;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

// ============================================================================
// 不再需要 ani_module! 宏！
// ANI_Constructor 在第一个 #[ani] 宏展开时自动生成
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World".to_string()), "Hello, World!");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
