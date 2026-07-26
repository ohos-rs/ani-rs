---
title: 测试与 ArkVM 回归
description: Rust、ETS、ArkVM Docker 与真实 OpenHarmony QEMU 的四层验证路径。
---

当前仓库的验证路径已经收敛成四层：

1. Rust 单测和 workspace 测试
2. `.ets` 输出检查
3. Docker + ArkVM example smoke
4. OpenHarmony QEMU 系统镜像回归

这样区分的好处是：一旦失败，你能立刻判断是 Rust 逻辑、ETS 生成，还是 ArkVM 运行时链路出了问题。

## 1. Rust 单测

本地先跑 workspace 测试：

```bash
cargo test --workspace -j 1
```

如果你在 Docker 环境里复现 example 测试链路，记得显式设置：

```bash
export ANI_TEST_MODULE_NAME=arkvm_test
```

这个变量是为了避免测试阶段的模块 descriptor 和 ArkVM smoke 环境互相污染。

## 2. ETS 输出检查

仓库脚本会：

- 清掉旧的 `.d.ets`
- 构建所有 example
- 检查每个 example 都产出了 `.ets`
- 确认没有 `.d.ets`
- 确认 `.ets` 里包含 `native` 和 `loadLibrary(...)`

命令：

```bash
bash ./scripts/check_example_ets.sh
```

## 3. ArkVM Docker 回归

真实运行时回归走 `scripts/run_arkvm_examples_ubuntu.sh`。脚本会：

- 启动 clean Ubuntu amd64 容器
- 安装 Rust toolchain 与构建依赖
- 构建所有 example
- 生成 `arkvm_test.ets`
- 用 `es2panda` 编译出 `.abc`
- 用 `ark` 逐个运行 example
- 输出 `examples/arkvm_report.txt` 和 `examples/arkvm_report.tsv`

基本用法：

```bash
ARKVM_DIR=/path/to/x64_linux_static \
ARK_SRC_ROOT=/path/to/arkcompiler_runtime_core \
ANI_TEST_MODULE_NAME=arkvm_test \
./scripts/run_arkvm_examples_ubuntu.sh
```

如果你手里只有打包好的 ArkVM tarball，也可以通过 `ARKVM_TARBALL` 让脚本自动解压。

## 4. OpenHarmony QEMU 系统镜像回归

`scripts/run_arkvm_examples_ohos_qemu.sh` 用于验证 ARM64 OpenHarmony QEMU
系统镜像，而不是 DevEco Studio 模拟器或宿主机 ArkVM。脚本会：

- 用 OpenHarmony SDK 交叉编译全部 ANI 动态库
- 用与系统镜像同源的 `es2panda` 生成 `.abc`
- 编译一个设备侧 ANI runner
- 把 runner、launcher ABC、测试 ABC 和 `.so` 推送到 QEMU
- 通过系统 `libarkruntime.so`、`etsstdlib.abc` 和
  `std.core.AbcRuntimeLinker` 加载并执行测试 ABC
- 把逐项结果写入 `target/ohos-qemu/report.tsv`

系统镜像通常不会安装独立的 `ark` 和 `es2panda` 命令，因此这条路径不能简化成
`hdc shell ark test.abc`。它验证的是系统 Ark Runtime 直接加载外部 ABC 的能力。

基本用法：

```bash
HDC_TARGET=127.0.0.1:5557 \
OHOS_SOURCE_ROOT=/path/to/openharmony \
DEVECO_SDK_ROOT=/path/to/openharmony-sdk \
./scripts/run_arkvm_examples_ohos_qemu.sh
```

可以用 `OHOS_QEMU_PACKAGE_FILTER` 只跑部分 package，或用
`OHOS_QEMU_CASE_TIMEOUT` 调整单项超时。

这不是 HAP 安装测试。正式应用分发仍应建立 Stage 模型 HAP，把生成的 ABC
和 `arm64-v8a` 动态库放入应用模块后，再走签名、`hdc install` 和 Ability
拉起流程。

### QEMU HAP 安装验证

除了外部 ABC runner，API 26 的真实 QEMU 镜像也已验证静态 ArkTS HAP：

- `module.json` 的 module 和 Ability 均使用 `arkTSMode: "static"`
- 合并后的 ABC 放在 `ets/modules_static.abc`
- ANI 动态库放在 `libs/arm64-v8a`
- HAP 完成签名校验后，可通过 `hdc install -r` 安装并用 `aa start` 拉起
- Ability 内调用 Rust native 方法，设备日志得到
  `[ANI_HAP_PASS] add=42;greet=Hello, QEMU!`

HAP 中的 ArkTS `native` 声明必须和 Rust `ANI_Constructor` 一次绑定的函数表
完整对应。只声明实际调用的两个方法、但绑定包含更多方法时，
`Module_BindNativeFunctions` 会返回 `ANI_NOT_FOUND`，导致模块初始化失败。

## 文档当前对齐到的验证基线

仓库现在记录的基线是：

- `cargo test --workspace -j 1` 可在 Ubuntu Docker 内通过
- `bash ./scripts/check_example_ets.sh` 通过
- `./scripts/run_arkvm_examples_ubuntu.sh` 已跑通
- `examples/arkvm_report.tsv` 中 52 个 example 均为 `build=OK / abc_compile=OK / runtime=OK`
- OpenHarmony QEMU 中 52 个 example 均为
  `cross_build=OK / abc_compile=OK / qemu_runtime=OK`，共 393 条断言通过、0 条失败
- 静态 ArkTS HAP 已完成签名、安装、Ability 拉起和 ANI 调用验证

## 常用定位顺序

如果某个能力回归失败，通常按这个顺序查：

1. 先看 Rust 构建日志，确认是不是 derive 展开或 trait bound 出错
2. 再看生成的 `.ets`，确认 public type、native decl 和 `loadLibrary(...)` 是否符合预期
3. 最后看 ArkVM `.log`，确认是 `es2panda` 编译问题还是 runtime 执行问题

## 相关文件

- `scripts/check_example_ets.sh`
- `scripts/generate_arkvm_smoke_ets.sh`
- `scripts/run_arkvm_examples_ubuntu.sh`
- `scripts/run_arkvm_examples_ohos_qemu.sh`
- `scripts/ohos_ani_abc_runner.cpp`
- `scripts/ohos_qemu_abc_launcher.ets`
- `examples/arkvm_report.txt`
- `examples/arkvm_report.tsv`
