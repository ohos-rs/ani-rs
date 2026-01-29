//! 异步包装示例 - 封装同步接口实现异步接口
//!
//! 演示如何在 Rust 中包装同步操作为异步 Promise

use ani_derive::ani;
use std::thread;
use std::time::Duration;

// ============================================================================
// 同步操作 - 模拟耗时计算
// ============================================================================

/// 模拟一个耗时的计算任务
fn expensive_computation(input: i32) -> i32 {
    // 模拟耗时操作
    thread::sleep(Duration::from_millis(100));
    input * input
}

/// 模拟网络请求
fn fetch_data(url: &str) -> String {
    // 模拟网络延迟
    thread::sleep(Duration::from_millis(50));
    format!("Response from: {}", url)
}

// ============================================================================
// 异步包装函数
// ============================================================================

/// 异步计算平方
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function asyncSquare(n: int): Promise<int>;
/// ```
///
/// 注意: 这里返回 Promise 的实际实现需要通过 ANI 的 Promise API
/// 简化版本直接返回结果，实际需要使用 Promise.resolve()
#[ani]
pub fn async_square(n: i32) -> i32 {
    expensive_computation(n)
}

/// 异步获取数据
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function asyncFetch(url: string): Promise<string>;
/// ```
#[ani]
pub fn async_fetch(url: String) -> String {
    fetch_data(&url)
}

// ============================================================================
// 回调风格的异步操作
// ============================================================================

/// 带回调的异步操作 - 存储回调信息
///
/// 实际使用中，这需要通过 ANI 的 FnObject 来调用 ArkTS 回调
///
/// 对应的 ArkTS 定义:
/// ```typescript
/// native function asyncWithCallback(input: int, callback: (result: int) => void): void;
/// ```
#[ani]
pub fn async_compute_start(input: i32) -> i32 {
    // 启动异步计算并返回任务 ID
    // 实际实现中会创建线程并存储回调
    let task_id = input.abs() % 1000;
    task_id
}

/// 检查异步任务状态
#[ani]
pub fn async_check_status(task_id: i32) -> bool {
    // 模拟检查任务状态
    task_id > 0
}

/// 获取异步任务结果
#[ani]
pub fn async_get_result(task_id: i32) -> i32 {
    // 模拟获取结果
    task_id * task_id
}

// ============================================================================
// 批量异步操作
// ============================================================================

/// 批量计算 - 返回所有结果的和
#[ani]
pub fn batch_compute(count: i32) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..count {
        sum += expensive_computation(i) as i64;
    }
    sum
}

// ============================================================================
// 模块初始化
// ============================================================================
