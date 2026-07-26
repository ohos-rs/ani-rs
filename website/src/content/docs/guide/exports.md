---
title: 导出与命名
description: 使用 #[ani] 导出 Module、Namespace、Class 成员并控制 ArkTS 名称。
---

与 napi-rs 的自动导出体验类似，`ani-rs` 不要求手写 native 注册数组。区别在于 ANI 需要明确绑定目标：Module、Namespace 或 Class。

## Module 函数

普通 `#[ani]` 函数导出到当前模块：

```rust
use ani_derive::ani;

#[ani]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

模块名默认来自 Cargo package 名。需要绑定到显式 descriptor 时使用 `module`：

```rust
#[ani(module = "my_math")]
pub fn square(value: i32) -> i32 {
    value * value
}
```

`module` descriptor 必须能被运行时找到，并且与 ArkTS 模块定义一致。

## Namespace 函数

使用 `namespace` 把 API 按业务域组织：

```rust
#[ani(namespace = "Math")]
pub fn clamp_to_zero(value: i32) -> i32 {
    value.max(0)
}

#[ani(namespace = "Math.Format")]
pub fn format_score(value: i32) -> String {
    format!("score:{value}")
}
```

嵌套 namespace 使用点号分隔。生成的 ETS 会保留对应的 namespace 层级。

## Class 成员

独立函数也可以直接绑定成 class 成员：

```rust
#[ani(class = "Calculator", static)]
pub fn version() -> i32 {
    1
}

#[ani(class = "Calculator", name = "add")]
pub fn calculator_add(a: i32, b: i32) -> i32 {
    a + b
}
```

需要原生实例状态时，更推荐把 `#[ani]` 应用到 `impl`。完整方式见 [Class、对象与枚举](/guide/classes-and-objects/)。

## 重命名

默认导出名与 Rust 函数名一致。使用 `name` 指定 ArkTS 名称：

```rust
#[ani(name = "calculateTotal")]
pub fn calculate_total(price: f64, count: i32) -> f64 {
    price * count as f64
}
```

## Overload

同一 target 可以注册多个相同名称、不同签名的函数：

```rust
#[ani(name = "sum")]
pub fn sum_two(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(name = "sum")]
pub fn sum_three(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
```

相同 target 下的 `name + signature` 必须唯一。出现重复组合时，模块加载会在绑定前失败。

通常让 `ani-rs` 从 Rust 类型推导 ANI 签名即可。只有在对接已有 ArkTS 声明时才使用 `signature = "..."` 显式覆盖。

## 初始化回调

模块加载时可以在绑定前后执行初始化代码：

```rust
use ani::prelude::*;
use ani_derive::ani;

#[ani(init, before_bindings)]
fn prepare(env: &Env<'_>) -> Result<()> {
    let _ = env;
    Ok(())
}

#[ani(init)]
fn ready() {
    // native bindings 已完成
}
```

- `before_bindings` 适合准备绑定所需资源。
- 普通 `#[ani(init)]` 在所有 native 成员绑定后运行。
- 初始化失败时返回 `Result<()>`，错误会中止模块加载。

## 参数注入

以下参数由 wrapper 注入，不会出现在生成的 ArkTS 签名里：

| Rust 参数 | 用途 |
| --- | --- |
| `env: &Env<'_>` | 创建值、调用运行时 API、抛出错误 |
| `this: &AniObject<'_>` | 当前 class 实例 |
| `class: &AniClass<'_>` | 当前 class，常用于静态成员 |

下一步可以查阅 [宏属性参考](/reference/macros/) 或直接打开 `examples/new_basic`、`examples/module_binding` 和 `examples/bind_overload`。
