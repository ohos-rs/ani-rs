# 测试与 ArkVM 回归

当前仓库的验证路径已经收敛成三层：

1. Rust 单测和 workspace 测试
2. `.ets` 输出检查
3. Docker + ArkVM example smoke

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

## 文档当前对齐到的验证基线

仓库现在记录的基线是：

- `cargo test --workspace -j 1` 可在 Ubuntu Docker 内通过
- `bash ./scripts/check_example_ets.sh` 通过
- `./scripts/run_arkvm_examples_ubuntu.sh` 已跑通
- `examples/arkvm_report.tsv` 中 52 个 example 均为 `build=OK / abc_compile=OK / runtime=OK`

## 常用定位顺序

如果某个能力回归失败，通常按这个顺序查：

1. 先看 Rust 构建日志，确认是不是 derive 展开或 trait bound 出错
2. 再看生成的 `.ets`，确认 public type、native decl 和 `loadLibrary(...)` 是否符合预期
3. 最后看 ArkVM `.log`，确认是 `es2panda` 编译问题还是 runtime 执行问题

## 相关文件

- `scripts/check_example_ets.sh`
- `scripts/generate_arkvm_smoke_ets.sh`
- `scripts/run_arkvm_examples_ubuntu.sh`
- `examples/arkvm_report.txt`
- `examples/arkvm_report.tsv`
