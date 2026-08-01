---
title: 支持矩阵
description: ANI-RS 的 API、架构、运行时和验证层级。
---

## OpenHarmony 与 ANI

| 项目 | 支持范围 |
| --- | --- |
| 最低 ANI 能力 | API 23（`--no-default-features --features api23`） |
| 默认能力 | API 24（`api24`） |
| 发布验证能力 | API 26（`api26`） |
| 当前基准头文件 | OpenHarmony API 26 源码中的 `interface/sdk_c/ani/ani.h` |
| 兼容模式 | `api23` 使用 class constructor/method primitive wrapper 路径 |
| ArkTS | ArkTS 1.2 / ETS |

API 23/24 当前声明为“交叉编译兼容”；只有 API 26 具有真实 guest 运行时证据。常规 `.github/workflows/ci.yml` 只执行格式检查、全 feature Clippy、workspace 单测和一次依赖安全审计。`.github/workflows/qemu.yml` 使用 GitHub-hosted Ubuntu/macOS runner，由 action 安装 OpenHarmony SDK 与系统 QEMU，并固定下载、校验 [`harmony-contrib/ohos-qemu` `v20260731`](https://github.com/harmony-contrib/ohos-qemu/releases/tag/v20260731)。每个架构 leg 依次检查 API 23/24/26 编译 profile，再对 API 26 的 ARM64/x86_64/ARMv7A 镜像执行同一 commit、同一 52 场景、HAP、JIT 50/100 轮内存压力与性能报告。其他检查脚本保留为本地或发布诊断工具，不再拆成常规 Actions job。

`scripts/header.sh --check` 会同时验证头文件校验和、API 24 符号和 bindgen 输出漂移。

## 目标架构

| QEMU guest | Rust target | HAP ABI 目录 |
| --- | --- | --- |
| ARM64 | `aarch64-unknown-linux-ohos` | `arm64-v8a` |
| x86_64 | `x86_64-unknown-linux-ohos` | `x86_64` |
| ARMv7A | `armv7-unknown-linux-ohos` | `armeabi-v7a` |

使用 CLI 检查或构建：

```bash
ani-rs support
ani-rs doctor --arch arm64
ani-rs build --arch x86_64 --release
```

QEMU 运行套件通过 `OHOS_QEMU_GUEST_ARCH` 选择架构，并在发送到 guest 前用 `llvm-readelf` 验证每个 `.so` 的 ELF machine：

```bash
HDC_TARGET=127.0.0.1:5558 \
OHOS_QEMU_GUEST_ARCH=arm64 \
DEVECO_SDK_ROOT=/path/to/openharmony-sdk \
OHOS_QEMU_USE_ABC_FIXTURES=1 \
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
| HAP 解包内容可复现 | `scripts/check_hap_reproducible.sh arm64` |
| 执行 HAP 中实际 ABC 与 SO | `scripts/run_hap_abc_ohos_qemu.sh` |
| 50/100 轮 PSS 增长 | `scripts/check_qemu_memory.sh` |
| Rust 所有权计数 | `runtime_metrics` + `assert_no_runtime_leaks` |
| Host 性能回归 | `scripts/check_performance.sh` |
| 真实 QEMU 场景延迟上限 | `scripts/check_qemu_performance.sh` / `performance.tsv` |
| 公共 API semver | `scripts/check_semver.sh` |
| ETS/ANI/native ABI | `scripts/check_abi.sh` |
| 发布前完整检查 | `scripts/check_release.sh` |

异步运行时门禁还覆盖 RuntimeDomain shutdown/drain/join/restart、watchdog、closing 拒绝、Promise continuation、自定义 rejection decoder、并发 stream waiter、return/throw/cancel 竞态，以及 Promise/Task/TSFN/Stream 的统一析构取消。Loom 穷举 exactly-once terminal transition；guest 的 `runtime_task_leak_gate` 调用真实 `assert_no_runtime_leaks`，`runtime_global_reference_leak_gate` 使用 3 秒有界终态检查，允许 Promise 完成与 Rust 引用删除之间的正常调度间隔，但引用不回落仍会失败。TypedArray 门禁区分 owned、VM ref、scope view 与 Rust→ANI 必须复制的平台边界。

HAP 验证要求存在 `resources/rawfile/ani_rs_smoke.abc`，并检查 `libs/<abi>/` 下每个 ANI 动态库的架构以及 `ANI_Constructor`、`ANI_Destructor` 导出。CI 的 portable HAP 使用 SDK 自带 packing tool，不依赖 DevEco/Hvigor。仓库中的 52 个 ABC 夹具与对应 ArkTS 源码绑定校验和；这既防止测试源码与字节码漂移，也验证旧 ABC 与当前 native ABI 的兼容性。

当前运行基准为上游 `v20260731`、Linux 6.6.101、OpenHarmony 7.0.0.32 / API 26；ARM64、x86_64、ARMv7A 均以 JIT 开启状态使用同一 52 场景和 458 个断言通过真实 guest、HAP 解包执行，并完成 100 轮异步 RuntimeKernel 重启。发布结论仍以对应 commit 的 workflow 报告为准，不能用历史或其他架构报告替代。
