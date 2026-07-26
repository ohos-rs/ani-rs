---
title: 故障排查
description: 处理 ETS 未生成、动态库加载失败、绑定错误、Promise reject 和 QEMU 连接问题。
---

## 没有生成 ETS

先确认：

- crate 使用了至少一个 `#[ani]`。
- 当前构建目标包含这个 `cdylib` crate。
- `ANI_ETS_OUTPUT` 没有指向其他位置。
- 查看的是 crate 自己的 `target/ani-ets/`，而不是 workspace 根目录。

```bash
cargo clean -p my-ani-module
ANI_ETS_OUTPUT=ets/index.ets cargo build -p my-ani-module -vv
```

## `loadLibrary` 找不到动态库

检查三个名称：

1. Cargo package `my-ani-module`
2. 生成的默认库名 `my_ani_module`
3. 文件名 `libmy_ani_module.so`

连字符会转换成下划线。若使用 `ANI_ETS_LIBRARY` 覆盖名称，HAP 中的库名也要同步。

同时检查目标 ABI：

```bash
llvm-readelf -h libmy_ani_module.so
llvm-readelf -d libmy_ani_module.so
```

ARM64 OpenHarmony 应使用 `aarch64-unknown-linux-ohos` 目标和 SDK 提供的 linker。

## `ANI_Constructor` 不存在

确认 crate 中确实展开了 `#[ani]`，并检查导出符号：

```bash
llvm-nm -D libmy_ani_module.so | grep ANI_Constructor
```

还要确认应用没有 strip 掉必需的动态符号。

## 绑定返回 `ANI_NOT_FOUND`

常见原因：

- `module`、`namespace` 或 `class` descriptor 写错。
- ArkTS 声明中的函数名与 `name = "..."` 不一致。
- 参数或返回值导致 ANI 签名不一致。
- 同一 target 的 native 声明不完整。

同一个 Module 或 Class 的一次 native bind，应与 ArkTS 侧可查找到的声明集合保持一致。不要只复制部分生成声明，再加载包含更多注册项的动态库。

优先对比实际生成的 `.ets`，不要手写猜测签名。

## 重复绑定

相同 target 下，`name + signature` 必须唯一：

```rust
// 正确：参数数量不同
#[ani(name = "sum")]
fn sum2(a: i32, b: i32) -> i32 { a + b }

#[ani(name = "sum")]
fn sum3(a: i32, b: i32, c: i32) -> i32 { a + b + c }
```

如果两个 Rust 函数生成相同签名，请修改 ArkTS 名称或参数模型。

## Promise 立即 reject

首先检查 Cargo feature：

```toml
ani = { git = "https://github.com/ohos-rs/ani-rs", features = ["async"] }
```

使用 `tokio::time`、`tokio::fs` 等模块时，还要开启对应 `ani` passthrough feature 和 Tokio 自身 feature。

然后确认返回错误是否来自 Rust `Result`：

```ts
try {
  await delayed_square(4, 10)
} catch (error) {
  console.error(error)
}
```

## 跨线程或异步生命周期错误

不要跨线程保存：

- `Env<'_>`
- `AniObject<'_>`
- `AniString<'_>`
- `Function<'_, ...>`

改用拥有生命周期的值：

- `Ref<T>`
- `GlobalRef`
- `FunctionRef<...>`
- 普通 owned Rust 类型

见 [引用与生命周期](/guide/references/)。

## ABC 编译失败

确认 `es2panda` 与 `arktsconfig.json` 来自同一 OpenHarmony 输出，并和目标运行时兼容。先单独编译最小 ETS 文件，再逐步加回生成声明和业务代码。

```bash
es2panda \
  --extension=ets \
  --arktsconfig /path/to/arktsconfig.json \
  --output smoke.abc \
  smoke.ets
```

## QEMU 无法连接或运行

列出真实连接目标：

```bash
hdc list targets
hdc -t 127.0.0.1:5557 shell uname -a
hdc -t 127.0.0.1:5557 shell ls /system/lib64/libarkruntime.so
```

端口号本身不能证明目标是 OpenHarmony QEMU。确认系统版本、CPU 架构和 Ark Runtime 文件均符合测试要求。

系统镜像通常没有 `ark` 命令。不能运行 `hdc shell ark smoke.abc` 时，使用 HAP 或通过系统 Runtime 加载 ABC 的设备侧 runner。

## 查看设备日志

```bash
hdc -t 127.0.0.1:5557 shell hilog -r
# 复现问题
hdc -t 127.0.0.1:5557 shell hilog -x
```

重点查找：

- dynamic linker 的缺失依赖
- `ANI_Constructor` 返回状态
- module / class bind 状态
- ArkTS exception
- Rust panic 转换后的错误信息
