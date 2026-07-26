---
title: ANI-RS 能力缺口清单
description: 已实现能力、明确语义边界与仍需关注的 ANI runtime 能力面。
---

## 目的

本文档记录当前 ani-rs 相对于目标设计和 OpenHarmony ANI 能力面的已实现能力、已完成收口项、明确语义边界，以及 public API scope 决策。

本文档聚焦以下问题：

- 当前哪些能力已经可以稳定使用
- 哪些能力底层已有，但宏层和类型生成还不够好用
- 当前还剩哪些只是运行时边界或 scope 决策，而不是继续推进的待办
- 哪些 OpenHarmony ANI 测试目录中的能力，当前 ani-rs 还没有形成对应的上层封装或验证覆盖

## 当前基线

当前代码基线已经满足以下条件：

- 仅生成 `.ets`，不生成 `.d.ets`
- 不生成 `declare` 风格代码
- example 的 ETS smoke 测试不再只是导入模块，而是会真实调用 native 导出并根据返回值做断言
- 本地 Docker + Linux x64 ArkVM 链路已经打通
- Rust `cargo test --workspace -j 1` 已在 Ubuntu Docker 中通过；相关单测已做好 `ANI_TEST_MODULE_NAME` 环境隔离，不再和 ArkVM 回归环境互相污染
- 已验证 clean Ubuntu 容器内复制并解压用户提供的 `x64_linux_static_fixed.zip` 后，可直接使用其中的 `es2panda` / `es2abc` / `resources/ets/{stdlib,sdk}` / `etsstdlib.abc` 进行 example 回归
- 当前 ArkVM example 回归已在 Docker 中跑通；`examples/arkvm_report.tsv` 中 52 个 example 均为 `build=OK / abc_compile=OK / runtime=OK`
- 当前 ArkVM example 运行时需要显式设置 `ANI_TEST_MODULE_NAME=arkvm_test`，否则会出现 descriptor 不匹配导致的 runtime 失败
- OpenHarmony 7.0.0.32（API 26）ARM64 QEMU 系统镜像已完成 52 / 52 个 example 的 `cross_build / abc_compile / qemu_runtime` 回归
- QEMU runtime 共执行 393 条断言，结果为 393 条通过、0 条失败
- 静态 ArkTS HAP 已完成签名、安装、Ability 启动和 ANI 调用验证，设备日志输出 `[ANI_HAP_PASS]`

最终回归结果见：

- `examples/arkvm_report.txt`
- `examples/arkvm_report.tsv`

## 已验证能力

以下能力已经具备 example、ArkVM 和 OpenHarmony QEMU 级别的验证：

- 模块级函数导出
- namespace 绑定
- 显式 `#[ani(module = ...)]` 绑定
- class instance/static/constructor 绑定
- `impl` receiver 方法绑定（`self` / `&self` / `&mut self`）
- getter/setter
- overload 绑定
- object/class nominal 类型生成
- `#[ani(object)]` / `#[derive(AniClass)]` 的 named/tuple/unit struct 与 type-parameter generic struct
- `Vec<T>` / `VecDeque<T>` / `LinkedList<T>` 的 primitive/ref/object array 导出与回归
- record/set/map/fixed array/tuple/enum item/any value 等类型的基础生成
- `PromiseRaw` / `Deferred` 以及 `#[ani(async)]` 驱动的异步包装 example
- module/member、function variable、namespace variable、class static by name 等运行时查找型能力
- reference/ref scope/`GlobalRef`/`WeakRef` 基础使用
- VM version / `VmOptions` 基础能力
- error / resolver 句柄基础使用
- `#[ani(constructor)]` 与 `#[ani(name = ...)]` 组合

对应 example 位于 `examples/` 目录下。

## 本轮收口结果

本文档里原先列出的任务项已经完成收口；剩余内容不再按“待办”跟踪，而是分为“已落地能力”和“明确语义边界”。

### 1. `#[ani(async)]` 与 Promise 路径已形成统一可用面

当前状态：

- `#[ani(async)] async fn ... -> Result<T>` 已稳定导出为 ArkTS `Promise<T>`
- `env` / `this` / `class` 注入、Rust `self` receiver、`constructor/getter/setter/signature` 组合都已支持
- `RefContainer` 已覆盖主要 ref-backed 常规参数与 callback/global-handle 托管路径
- `PromiseRaw<T>` / `Deferred<T>` / `AniResolver` 现在形成一套统一桥接：
  - `Deferred<T>` 已具备 `TypeInfo` / `ToAni` / `FromAni`，derive/type-system/ETS 会把它识别为 typed resolver handle，而不是未知对象
  - `Env::promise_new_typed<T>()`、`Env::promise_resolved(...)`、`Env::promise_rejected(...)`、`Env::promise_rejected_with_error(...)` 已形成 env-rooted 高层入口
  - `AniResolver::resolve_value / reject_error / reject_message / into_deferred` 与 `Deferred<T>::new / from_resolver` 可互相桥接
- `examples/async_wrapper` 已同时覆盖：
  - `Env::promise_new_typed`
  - `Env::promise_resolved`
  - `Env::promise_rejected`
  - `AniResolver::resolve_value`
  - `AniResolver::into_deferred`
  - `#[ani(async)]` Promise wrapper
- 新增 Promise/Resolver/Deferred 路径已接入 ArkVM smoke 生成脚本与 ETS 回归

仍然成立的运行时约束：

- Promise 形态下，调用线程先完成常规参数转换，再把可捕获值移交给 dedicated local runtime worker；因此尚未自动托管的那部分捕获值仍需满足 `Send + 'static`
- 对尚未进入 `RefContainer` 自动托管覆盖面的自定义 wrapper/handle，仍建议显式使用 `GlobalRef` / `RefContainer`

代码位置：

- `crates/derive/src/expand/function.rs`
- `crates/derive/src/codegen/wrapper.rs`
- `crates/ani/src/conversions/promise.rs`
- `crates/ani/src/tokio.rs`
- `examples/async_wrapper/src/lib.rs`

### 2. `Unknown -> Object` 已继续收缩到少数兜底路径

当前状态：

- nominal 自定义对象、透明 wrapper、record/set/map、array/fixed-array wrapper、函数级泛型参数、常见字符串/路径类 owned/borrowed wrapper 均已进入正式类型分支
- `Vec<T>` / `VecDeque<T>` / `LinkedList<T>` 已完成 “ETS public type 精确表达 + ArkVM bind signature 兼容收敛”
- `Deferred<T>` 现在也不再落到 `Unknown/Object`，而是进入 resolver handle surface
- `AniRef` / `AniType` / `AniModule` / `AniNamespace` / `AniEnum` / `AniError` / `AniMethod` / `AniStaticMethod` / `AniField` / `AniStaticField` / `AniVariable` / `AniResolver` / `GlobalRef` / `WeakRef` 都已有明确 public ETS model：
  - object-backed handle 统一导出为命名 opaque alias，例如 `export type AniResolver = Object`
  - `WeakRef` 保持 `WeakRef<Object>`
- `AniObject` 继续直接映射为 ArkTS `Object`；这是有意保持高层对象语义，而不是遗漏的 nominal handle

当前 `Unknown` 的剩余用途：

- genuinely unknown 的 Rust 自定义路径，且未注册对象别名
- 当前 ANI/ArkTS 语义本身不适合进一步 nominal 化的兜底对象面

代码位置：

- `crates/derive/src/types/ani_type.rs`
- `crates/derive/src/types/ets.rs`
- `crates/derive/src/types/conversion.rs`

### 3. class/property/static property metadata 已统一到 descriptor 模型

当前状态：

- class member 元数据已统一收敛到：
  - `ClassMemberMetadata`
  - `ClassCallableDescriptor`
  - `ClassPropertyDescriptor`
  - `ClassOpDescriptor`
- property getter/setter merge、slot key、sort key、register descriptor、ETS emission 都统一走 descriptor 模型
- constructor / method / property / iterator op 的 native symbol 与 public ETS surface 已集中由 `ClassMemberPlanKind` 和 descriptor constructor 生成
- `render_decls` / `emit_export_plan_ets` / impl property slot merge 已在单测中稳定覆盖

代码位置：

- `crates/derive/src/codegen/export.rs`
- `crates/derive/src/expand/function.rs`
- `crates/derive/src/expand/impl_block.rs`
- `crates/derive/src/types/ets.rs`

### 4. `module = ...`、`GlobalRef / WeakRef`、runtime handle surface 已完成任务清单中的收口

当前状态：

- `#[ani(module = ...)]` 的复杂 descriptor remap / override / suffix 场景已有 example、单测和 ArkVM smoke
- `GlobalRef / WeakRef` 已覆盖 create/use/delete/upgrade、weak invalidation/GC、global 存活/释放阶段差异
- `AniModule / AniNamespace / AniVariable / AniResolver / GlobalRef / WeakRef` 的 public ETS type model 已定型

代码位置：

- `crates/ani/src/env.rs`
- `crates/ani/src/types.rs`
- `crates/ani/src/conversions/reference.rs`
- `examples/module_binding`
- `examples/reference`
- `examples/weak_ref`

## 明确语义边界

以下内容不再按“缺失能力”追踪，而是当前版本有意识保留的边界：

### 1. `#[ani(object)]` / `#[derive(AniClass)]` generic struct 的 ArkVM runtime 边界

- named / tuple / unit struct 与 type-parameter generic struct 已补齐
- ETS public declaration 已保留 `class Foo<T>`
- `examples/derive_shapes` 与 Docker/ArkVM smoke 已验证 object-backed generic instantiation（例如 `String`）的真实 roundtrip
- generic field 的 primitive instantiation（如 `T = int/bool`）仍受 ArkVM generic slot runtime model 约束；这不是 derive 漏实现，而是运行时能力边界

### 2. `#[derive(AniEnum)]` 当前保持 unit variant 语义

- 当前只支持 unit variants
- 这是 ANI enum / ArkTS enum 语义边界，而不是简单未完成的宏展开
- generic enum 或非 unit variant 如果继续支持，通常已经不再是 ArkTS enum，而是另一套 tagged-union / object model 设计

## 对比 OpenHarmony ANI 测试目录后的最终结论

OpenHarmony ANI 原生测试目录里的高价值用户面能力，当前 ani-rs 已形成上层 API、example 或 ArkVM 回归覆盖：

- `any_ops`
- `arraybuffer_ops`
- `class_ops` 常用子集
- `enum_ops`
- `error_ops`
- `function_ops` 查找/调用子集
- `fn_object_ops` callback storage / return / roundtrip 子集
- `fixedarray_ops`
- `gref_ops` create/use/delete 基础子集
- `module_ops` 运行时查找子集
- `namespace_ops` 运行时查找子集
- `object_ops`
- `promise_ops` 的高层 Promise/Resolver/Deferred 路径
- `ref_ops`
- `string_ops`
- `tuple_ops`
- `type_ops`
- `var_ops` get/set 基础子集
- `version_ops` 基础查询子集
- `vm_ops`
- `options/*` 基础构建子集
- `wref_ops` 基础使用子集

以下条目经评估后不进入当前 ani-rs 对外能力面，因此不再作为本文档待办项：

- `load_library`
- `native_library`
- `cframe_iterator`
- `verifyani/*`
- `bridges/*`

评估结论：

- 当前 `crates/sys/src/lib.rs` 的 bindgen ANI surface 中并没有这组 API 对应的可封装 C 入口
- 它们更接近 runtime/平台侧验证能力，而不是当前 ani-rs 在 Rust/derive/ETS 层应补的一层 safe wrapper
- 因此本文档不再把它们列为“ani-rs 缺口”，而是明确归类为“当前不纳入 public API surface”

## 当前结论

截至本轮收口，本文档中的任务列表已经处理完成；当前文档中不再保留需要继续执行的 TODO：

- 能落代码的能力项已经补齐，并接入 Rust 单测、example 或 ArkVM smoke
- 不适合继续作为任务推进的部分，已经改写为明确语义边界或 public-API scope 决策

后续如果继续扩能力，建议只增量维护以下信息：

- 是否新增 example / ArkVM smoke
- 是否引入新的 public ETS surface
- 是否出现新的 `Unknown` 兜底路径
