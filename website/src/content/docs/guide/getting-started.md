---
title: 快速开始
description: 创建一个最小 ani-rs 动态库，导出 Rust 函数并从 ArkTS 调用。
---

`ani-rs` 使用属性宏生成 ANI wrapper、注册信息和 ArkTS `.ets` 声明。你只需要创建一个 `cdylib` crate、添加依赖并为公开函数标记 `#[ani]`。

## 前置条件

- 当前 Rust 工具链和 Cargo
- HarmonyOS 或 OpenHarmony SDK，包含目标平台的 Clang 与 sysroot
- ArkTS 1.2 / ANI 兼容的应用或运行时
- 真机或 QEMU 验证时需要 `hdc`

:::note
当前项目没有脚手架 CLI。已有 Cargo 项目可以直接手动接入；如果从零开始，执行 `cargo new --lib my-ani-module` 即可。
:::

## 创建动态库

```bash
cargo new --lib my-ani-module
cd my-ani-module
```

把 crate 配置成动态库并添加运行时与宏依赖：

```toml title="Cargo.toml"
[package]
name = "my-ani-module"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs" }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
```

## 导出第一个函数

```rust title="src/lib.rs"
use ani_derive::ani;

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}
```

不需要手动维护注册表。第一个 `#[ani]` 会生成 `ANI_Constructor`，每个导出函数会自动加入当前模块的绑定信息。

## 本地检查

```bash
cargo check
cargo test
```

构建时会生成 ArkTS 声明。默认输出在当前 crate 的 `target/ani-ets/`：

```text
target/ani-ets/my_ani_module.ets
```

文件名和默认 native library 名都由 Cargo package 名转换而来：连字符会变成下划线。可以通过环境变量覆盖输出位置或库名：

```bash
ANI_ETS_OUTPUT=ets/index.ets \
ANI_ETS_LIBRARY=my_native_library \
cargo build
```

## ArkTS 侧调用

生成文件包含 `loadLibrary(...)` 与 `native` 声明，可以直接在 ArkTS 模块中使用：

```ts
import { add, greet } from './ets/index'

console.log(add(20, 22))
console.log(greet('ArkTS'))
```

打包应用时，需要同时包含：

- 为设备架构编译的 `libmy_ani_module.so`
- 生成的 `.ets` 声明或编译后的 ABC

具体路径和交叉编译命令见 [构建与加载](/guide/build-and-load/)。

## 下一步

- 使用 Module、Namespace 和 Class 组织 API：[导出与命名](/guide/exports/)
- 定义带状态的类和对象：[Class、对象与枚举](/guide/classes-and-objects/)
- 添加异步方法：[异步与 Promise](/guide/async/)
- 查找完整示例：[示例](/guide/examples/)
