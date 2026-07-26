---
title: 测试与调试
description: 测试 Rust 逻辑、ETS 声明、ABC 和 OpenHarmony 设备侧加载。
---

Native 模块最好分层测试。先验证纯 Rust 逻辑，再检查生成声明，最后进入 ArkTS 与设备运行时。这样可以快速判断错误发生在哪一层。

## Rust 单元测试

导出函数仍是普通 Rust 函数，可以直接测试：

```rust
#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_values() {
        assert_eq!(add(20, 22), 42);
    }
}
```

```bash
cargo test
```

把不依赖 `Env` 的业务逻辑放在普通 Rust 函数中，native wrapper 只负责转换和调用，测试会更简单。

## 检查 ETS 声明

```bash
cargo build
sed -n '1,200p' target/ani-ets/my_ani_module.ets
```

至少检查：

- `loadLibrary(...)` 与实际 `.so` 名称一致。
- 参数、返回值和 nullish 类型符合预期。
- class、namespace 和 overload 出现在正确 target。
- 没有残留旧构建生成的声明。

可以把 `ANI_ETS_OUTPUT` 指到固定路径，并把生成文件纳入快照或 diff：

```bash
ANI_ETS_OUTPUT=tests/expected/module.ets cargo build
git diff -- tests/expected/module.ets
```

## ArkTS smoke test

为每个公开 API 写少量可观察断言：

```ts
import { add, greet } from './native/my_ani_module'

function assertTrue(value: boolean, message: string): void {
  if (!value) {
    throw new Error(message)
  }
}

assertTrue(add(20, 22) === 42, 'add failed')
assertTrue(greet('ArkTS') === 'Hello, ArkTS!', 'greet failed')
```

如果使用独立工具链，把 smoke `.ets` 编译成 ABC：

```bash
es2panda \
  --extension=ets \
  --arktsconfig /path/to/arktsconfig.json \
  --output smoke.abc \
  smoke.ets
```

`es2panda`、`arktsconfig.json` 和目标系统 Ark Runtime 应来自兼容的 OpenHarmony 构建。

## 在真实 OpenHarmony QEMU 上测试

先确认连接的 target：

```bash
hdc list targets
hdc -t 127.0.0.1:5557 shell uname -m
```

不要仅根据端口号判断环境类型。应确认它运行的是需要验证的 OpenHarmony QEMU 系统镜像，而不是 DevEco 模拟器或其他设备。

把 ARM64 动态库、ABC 和 runner 推送到设备：

```bash
hdc -t 127.0.0.1:5557 shell mkdir -p /data/local/tmp/my-ani
hdc -t 127.0.0.1:5557 file send \
  target/aarch64-unknown-linux-ohos/release/libmy_ani_module.so \
  /data/local/tmp/my-ani/libmy_ani_module.so
hdc -t 127.0.0.1:5557 file send \
  smoke.abc \
  /data/local/tmp/my-ani/smoke.abc
```

设备镜像通常没有可直接执行的 `ark` 命令。外部 ABC 需要 runner 通过系统 `libarkruntime.so` 加载；可以复用仓库的：

```bash
HDC_TARGET=127.0.0.1:5557 \
OHOS_SOURCE_ROOT=/path/to/openharmony \
DEVECO_SDK_ROOT=/path/to/openharmony-sdk \
OHOS_QEMU_PACKAGE_FILTER=my-package \
./scripts/run_arkvm_examples_ohos_qemu.sh
```

该脚本适合仓库 example。应用项目应把相同原则集成进自己的测试任务：同源工具链编译 ABC、目标架构 `.so`、系统 Runtime 执行和明确断言。

## HAP 集成测试

最终仍要在应用形态测试：

1. 把 `.so` 放入目标 ABI 的 `libs` 目录。
2. 让生成 `.ets` 进入静态 ArkTS 构建。
3. 签名并安装 HAP。
4. 启动 Ability，调用每个关键 native API。
5. 同时检查 ArkTS 异常和 `hilog`。

```bash
hdc -t 127.0.0.1:5557 install -r entry-default-signed.hap
hdc -t 127.0.0.1:5557 shell aa start \
  -b com.example.myani \
  -a EntryAbility
hdc -t 127.0.0.1:5557 shell hilog -x
```

## 推荐顺序

| 失败位置 | 首先检查 |
| --- | --- |
| `cargo test` | Rust 业务逻辑 |
| `cargo build` | 宏参数、trait bound、目标 linker |
| ETS diff | 类型推导、名称与 target |
| ABC 编译 | ArkTS 语法与工具链版本 |
| 动态库加载 | ABI、库名、依赖库 |
| native bind | descriptor、签名与声明完整性 |
| 调用崩溃或异常 | 生命周期、错误转换、线程使用 |

常见错误的具体处理方法见 [故障排查](/guide/troubleshooting/)。
