---
title: 快速开始
description: 从依赖配置、Rust 导出到 ArkTS 加载，完成第一个 ani-rs 模块。
---

这一页只覆盖把一个最小 `ani-rs` 模块跑起来需要知道的内容。更细的设计约束、类型边界和 ArkVM 验证方式，分别放在后面的绑定模型、类型系统和测试章节里。

:::note
如果你已经能写 `napi-rs`，可以把 `ani-rs` 先理解成“宏体验相近，但导出目标换成 Module / Namespace / Class”。开始写代码前，建议再看一眼 [使用须知](/guide/compatibility)。
:::

## 前置要求

- Rust 1.85+
- Node.js 22.12+（用于本文档站点）
- ArkTS 1.2 兼容运行时
- 如果需要跑真实 ArkVM 回归，还需要准备 Linux x64 的 ArkVM 工具链和 `arkcompiler_runtime_core` 资源目录

## 添加依赖

如果你在仓库外直接消费 `ani-rs`，先把 crate 配进 `Cargo.toml`。

```toml title="同步导出"
[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", package = "ani" }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs", package = "ani-derive" }
```

```toml title="异步导出"
[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", package = "ani", features = ["async", "tokio_time"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs", package = "ani-derive" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

:::caution[特别注意]
如果你写了 `#[ani(async)]` 却没有开启 `ani` 的 `async` 或 `tokio_rt` feature，代码仍可能编译通过，但运行时返回的 Promise 会直接 reject。
:::

## 写一个最小导出

```rust
use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(namespace = "Math")]
pub fn square(input: i32) -> i32 {
    input * input
}
```

要点：

- `#[ani]` 第一次展开时会自动生成 `ANI_Constructor`
- 导出项会在加载 native library 时注册到全局 registry
- 默认目标是模块级函数；显式声明 `namespace`、`class`、`module` 后会走对应的绑定路径

## 编译与 ETS 输出

对 crate 执行常规构建后，`ani-rs` 会在该 crate 的 `target/ani-ets/` 下生成对应 `.ets`：

```bash
cargo check -p your-crate
```

典型输出路径：

```text
examples/new_basic/target/ani-ets/ani_example_new_basic.ets
```

当前基线：

- 只生成 `.ets`
- 不生成 `.d.ets`
- 不生成 `declare` 风格声明

仓库自带检查脚本会验证这些约束：

```bash
bash ./scripts/check_example_ets.sh
```

## ArkTS 侧调用

生成的 `.ets` 会包含 `loadLibrary(...)` 和 `native` 声明，ArkTS 侧按普通模块使用即可：

```ts
import { add } from './target/ani-ets/ani_example_new_basic'

let value = add(2, 3)
console.log(value)
```

如果你需要一个对照实现，可以直接看这些 example：

- `examples/new_basic`
- `examples/module_binding`
- `examples/ets_declaration`

## 下一步

- 想看 `#[ani]` 怎么映射到 Module / Namespace / Class：转到 [绑定模型](/guide/binding-model)
- 想看 `#[ani(async)]`、Tokio 和 Promise：转到 [异步与 Tokio](/guide/async)
- 想先看“现在支持什么”：转到 [支持能力总览](/reference/capabilities)
- 想直接找现成 example：转到 [示例索引](/guide/examples)
