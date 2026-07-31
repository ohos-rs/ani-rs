---
title: 支持矩阵
description: ANI-RS 的 API、架构、运行时和验证层级。
---

## OpenHarmony 与 ANI

| 项目 | 支持范围 |
| --- | --- |
| 最低 ANI 能力 | API 24 |
| 当前基准头文件 | OpenHarmony API 26 源码中的 `interface/sdk_c/ani/ani.h` |
| 兼容模式 | 关闭默认 feature 后保留 API 24 之前的 primitive wrapper 路径 |
| ArkTS | ArkTS 1.2 / ETS |

`scripts/header.sh --check` 会同时验证头文件校验和、API 24 符号和 bindgen 输出漂移。

## 目标架构

| QEMU guest | Rust target | HAP ABI 目录 |
| --- | --- | --- |
| ARM64 | `aarch64-unknown-linux-ohos` | `arm64-v8a` |
| x86_64 | `x86_64-unknown-linux-ohos` | `x86_64` |
| ARMv7A | `armv7-unknown-linux-ohos` | `armeabi-v7a` |

使用 CLI 检查或构建：

```bash
cargo run -p ani-cli -- doctor --arch arm64
cargo run -p ani-cli -- build --arch x86_64 --release -- -p my-module
```

QEMU 运行套件通过 `OHOS_QEMU_GUEST_ARCH` 选择架构，并在发送到 guest 前用 `llvm-readelf` 验证每个 `.so` 的 ELF machine：

```bash
HDC_TARGET=127.0.0.1:5558 \
OHOS_QEMU_GUEST_ARCH=arm64 \
OHOS_SOURCE_ROOT=/path/to/openharmony \
scripts/run_arkvm_examples_ohos_qemu.sh
```

## 验证层级

| 层级 | 命令 |
| --- | --- |
| Rust 单测与宏测试 | `cargo test --workspace --all-features` |
| 全 feature lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| ETS 确定性与原子写入 | `scripts/check_example_ets.sh` |
| ABC 与真实 QEMU Runtime | `scripts/run_arkvm_examples_ohos_qemu.sh` |
| HAP 中 ABC、ABI 与入口符号 | `scripts/verify_hap.sh app.hap arm64` |
| 发布前完整检查 | `scripts/check_release.sh` |

HAP 验证要求至少包含一个 `.abc`，并检查 `libs/<abi>/` 下每个 ANI 动态库的架构和 `ANI_Constructor` 导出。
