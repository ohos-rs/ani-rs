---
title: 构建与加载
description: 生成 ETS 声明、交叉编译 OpenHarmony 动态库并接入 ArkTS 应用。
---

一次可加载的 ani-rs 构建包含两个产物：

| 产物 | 用途 |
| --- | --- |
| `lib<crate_name>.so` | 设备侧原生动态库 |
| `<crate_name>.ets` | ArkTS native 声明与 `loadLibrary(...)` |

## 控制 ETS 输出

默认情况下，声明写入 crate 自己的 `target/ani-ets/`。在应用工程中，推荐把输出路径固定到模块源码目录：

```bash
ANI_ETS_OUTPUT=entry/src/main/ets/native/my_module.ets cargo build
```

常用环境变量：

| 变量 | 作用 |
| --- | --- |
| `ANI_ETS_OUTPUT` | 设置 `.ets` 输出文件，允许相对 crate 根目录 |
| `ANI_ETS_LIBRARY` | 覆盖生成文件中的 `loadLibrary` 名称 |
| `ANI_MODULE_DESCRIPTOR` | 固定运行时注册使用的完整 module descriptor |
| `CARGO_TARGET_DIR` | 修改 Cargo 产物目录，同时影响默认 ETS 目录 |

:::caution
ArkTS 声明中的库名、动态库文件名和运行时模块 descriptor 必须一致。使用自定义 `ANI_ETS_LIBRARY` 后，也要同步调整 HAP 中的 `.so` 名称或加载配置。
:::

## 为 OpenHarmony ARM64 构建

下面以 DevEco/OpenHarmony SDK 的 Clang 为例：

```bash
export OHOS_SDK=/path/to/openharmony-sdk
export OHOS_CLANG="$OHOS_SDK/native/llvm/bin"

CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER="$OHOS_CLANG/aarch64-unknown-linux-ohos-clang" \
CC_aarch64_unknown_linux_ohos="$OHOS_CLANG/aarch64-unknown-linux-ohos-clang" \
CXX_aarch64_unknown_linux_ohos="$OHOS_CLANG/aarch64-unknown-linux-ohos-clang++" \
cargo build --release --target aarch64-unknown-linux-ohos
```

动态库位于：

```text
target/aarch64-unknown-linux-ohos/release/libmy_ani_module.so
```

## 接入 Stage 模型 HAP

通常需要完成以下映射：

```text
entry/
├── libs/
│   └── arm64-v8a/
│       └── libmy_ani_module.so
└── src/main/ets/
    └── native/
        └── my_ani_module.ets
```

ArkTS 代码导入生成模块后，`loadLibrary` 会加载动态库，并由 `ANI_Constructor` 完成 native function 或 method 绑定。

动态库还会导出 `ANI_Destructor`。VM 卸载模块时，它执行 `#[ani(finalize)]`，关闭 RuntimeDomain，取消并 exactly-once reject Promise/Task/TSFN/Stream，join 自定义或内置 backend，再释放 `ManagedResource`。若非协作任务超过 shutdown deadline，watchdog 会 fail-fast，绝不会让线程在 native image 卸载后继续执行。

如果项目走静态 ArkTS 编译链，`.ets` 会随应用构建进入模块 ABC；不要把源 `.ets` 当成设备侧独立脚本直接执行。

## 独立编译 ABC

需要在 HAP 之外验证时，可使用与目标系统匹配的 `es2panda`：

```bash
es2panda \
  --extension=ets \
  --arktsconfig /path/to/arktsconfig.json \
  --output test.abc \
  test.ets
```

系统镜像通常不提供独立的 `ark` 或 `es2panda` 命令。QEMU 中运行外部 ABC 时，需要设备侧 runner 使用系统 Ark Runtime 加载 ABC；仓库里的 `scripts/run_arkvm_examples_ohos_qemu.sh` 可以作为完整参考。

## 独立 CLI

安装后的 CLI 不依赖源码仓库，可查看支持合同、检查工具链并交叉编译当前项目：

```bash
ani-rs support
ani-rs doctor --arch arm64
ani-rs build \
  --arch arm64 \
  --module-descriptor entry.src.main.ets.native \
  --ets-output entry/src/main/ets/native/index.ets \
  --library my_ani_module \
  --release
```

源码仓库中的 `build_hap_smoke.sh`、`check_hap_reproducible.sh` 和 QEMU 脚本只用于 ani-rs 自身的发布资格测试，不属于已发布 CLI 的公共命令。

## 检查构建结果

在进入应用打包前，先确认：

1. `.so` 的目标架构与设备一致。
2. `.ets` 中的 `loadLibrary` 名称正确。
3. `.ets` 声明覆盖同一 target 下注册的全部 native 成员。
4. `ANI_Constructor` 和 `ANI_Destructor` 都可从动态库导出。

```bash
llvm-readelf -h target/aarch64-unknown-linux-ohos/release/libmy_ani_module.so
llvm-nm -D target/aarch64-unknown-linux-ohos/release/libmy_ani_module.so \
  | grep -E 'ANI_(Constructor|Destructor)'
```

遇到加载或绑定错误时，继续看 [故障排查](/guide/troubleshooting/)。
