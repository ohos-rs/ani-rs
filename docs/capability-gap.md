# ANI-RS 能力缺口清单

## 目的

本文档记录当前 ani-rs 相对于目标设计和 OpenHarmony ANI 能力面的已实现能力、明确缺失能力、设计债，以及建议的落地优先级。

本文档聚焦以下问题：

- 当前哪些能力已经可以稳定使用
- 哪些能力底层已有，但宏层和类型生成还不够好用
- 哪些能力尚未实现
- 哪些 OpenHarmony ANI 测试目录中的能力，当前 ani-rs 还没有形成对应的上层封装或验证覆盖

## 当前基线

当前代码基线已经满足以下条件：

- 仅生成 `.ets`，不生成 `.d.ets`
- 不生成 `declare` 风格代码
- example 的 ETS smoke 测试不再只是导入模块，而是会真实调用 native 导出并根据返回值做断言
- 本地 Docker + Linux x64 ArkVM 链路已经打通
- Rust `cargo test --workspace` 已可在 Ubuntu Docker 中通过
- `scripts/run_arkvm_examples_ubuntu.sh` 现已支持直接使用 `ARKVM_TARBALL=/path/to/arkvm_static_linux_x64.tar.gz`
- 当前 ArkVM example 回归仍停留在 “Rust 构建 + ETS 生成通过，ABC 编译失败” 状态；失败点来自 `es2panda` 与外部 `runtime_core` stdlib 源码不兼容，而不是仓库内 Rust 宏/运行时改动本身

最终回归结果见：

- `examples/arkvm_report.txt`
- `examples/arkvm_report.tsv`

## 已验证能力

以下能力已经具备 example 级别和 ArkVM 级别的验证：

- 模块级函数导出
- namespace 绑定
- class instance/static/constructor 绑定
- getter/setter
- overload 绑定
- object/class nominal 类型生成
- record/set/map/fixed array/tuple/enum item/any value 等类型的基础生成
- PromiseRaw/Deferred 驱动的异步包装 example
- module/member、function variable、class static by name 等运行时查找型能力
- reference/ref scope/weak ref 基础使用
- error / resolver 句柄基础使用

对应 example 位于 `examples/` 目录下。

## 缺失能力分类

### 一、应优先补齐的高价值能力

这些能力对外使用价值高，并且最符合“继续向 napi-rs 风格收敛”的目标。

#### 1. `#[ani(async)]` 宏级异步绑定

现状：

- 已实现 `#[ani(async)] async fn foo(...) -> Result<T>`：宏会自动导出为 ArkTS `Promise<T>` 形式的 native binding
- 当前实现基于 `ani::tokio`（建议为 `ani` 依赖开启 `tokio_rt` feature；未开启时 Promise 会直接 reject 并提示开启）
- 当前约束：
  - 不支持注入 `env` / `this` / `class`，也不支持 Rust `self` receiver（避免跨线程捕获线程关联的 VM 句柄）
  - 参数和返回值需满足 `Send + 'static`（由 Rust 编译器在 `tokio` spawn 处强约束）
  - 仍不建议在 async 任务中直接持有 `AniObject/AniRef` 等 VM handle；后续可以参考 napi-rs 的 “ref container” 思路，用 `GlobalRef` 做显式生命周期托管再放开

代码位置：

- `crates/derive/src/expand/function.rs`
- `crates/derive/src/codegen/wrapper.rs`
- `crates/ani/src/conversions/promise.rs`
- `crates/ani/src/tokio.rs`

问题：

- 仍有进一步对齐 napi-rs 的空间（例如 async 参数引用保活、允许更多 handle/引用类参数等）

建议目标：

- 在当前基础上继续增强 async 能力面：
  - 为引用/handle 参数引入显式托管策略（类似 napi-rs 的 ref container），避免 GC 或线程语义问题
  - 结合 `GlobalRef/WeakRef` 补齐可用模式与 example 回归覆盖

#### 2. `Unknown -> Object` fallback 仍然偏多

现状：

- derive 类型系统仍然保留了部分 `Unknown` 回退路径
- 但过去一轮已经先收掉了一批高频误判，ETS public type 精度明显高于最初状态

当前已经收敛的类型面：

- nominal 自定义对象不再默认退化为 `Unknown/Object`，会优先保留注册别名或 Rust 路径名
- `Mutex` / `RwLock` / `RefCell` / `Cell` / `UnsafeCell` / `ManuallyDrop` / `MaybeUninit` / `OnceLock` / `LazyLock` 等透明包装会继续透传 inner type
- `HashMap<String, V>` / `HashSet<T>` / `BTreeSet<T>` / `BTreeMap<K, V>` 会保留为 `Record` / `Set` / `Map`，而不是对象兜底
- `FixedIntArray` / `FixedBooleanArray` / `AniFixedArrayInt` 等 fixed array wrapper 已提升为正式类型分支，不再依赖 `Unknown` 二次兜底
- `AniArray` / `AniArrayRef` / `AniFixedArray` / `AniFixedArrayRef` 也已提升为正式类型分支，ETS public type 会保持 `Array<Object>` / `FixedArray<Object>`，不再靠 runtime-name fallback
- 函数级泛型参数 `T` / `U` 以及 `Function<(T,), T>` 这类回调泛型，ETS 已能保留类型参数而不是回落到 `Object`
- `Either<T, U>` / `HashMap<String, T>` / `HashSet<T>` / `BTreeMap<String, Either<T, U>>` 这类嵌套容器现在也会继续保留函数级类型参数，不再在容器内部把 `T/U` 误判成 nominal object
- 已支持但过去未识别的 `CString` / `isize` / `usize` 现在也会生成 `string` / `long`，不再走未知对象兜底

代码位置：

- `crates/derive/src/types/ani_type.rs`
- `crates/derive/src/types/ets.rs`
- `crates/derive/src/types/conversion.rs`

问题：

- ETS public signature 不够精确
- 与“命名尽量和 Rust/注册别名一致、不要全部变成 Object”的目标仍有距离
- 后续扩展成本高

建议目标：

- 继续减少 `AniType::Unknown`
- 对可以识别的对象类型、handle 类型、record/union/either/result 进一步细化 public ETS type
- 将 fallback 变成真正少数兜底，而不是常规路径

#### 3. 显式 `module = ...` 绑定缺少 example 和回归覆盖

现状：

- 文档中提到 `#[ani(module = ...)]`
- 已补齐 `examples/module_binding`，并接入 ArkVM smoke（`scripts/generate_arkvm_smoke_ets.sh`）
- 当前更多覆盖的是 `namespace` 和运行时 `find_module`

问题：

- 能力面对用户并不完整
- 文档存在但没有实际可验证用例

建议目标：

- 已落地：新增 `examples/module_binding` + ArkVM smoke 回归
- 后续可增强：补一个“descriptor 与 crate 名不一致”的真实场景用例（避免只覆盖等价路径）

#### 4. `GlobalRef / WeakRef` 需要更明确的高层能力面

现状：

- runtime API 已有
- `examples/reference` 主要覆盖的是 `Ref<T>` 使用模式
- `GlobalRef` / `WeakRef` 现在已经接入 `examples/reference` 的 smoke，用于验证 native 内部 create/use/delete 基础链路；但仍缺少更完整的 weak invalidation 场景
- 真实 ArkVM 验证表明 raw `GlobalRef` / `WeakRef` 不适合作为 ETS public value 直接 roundtrip；当前 example 已改为 ETS 传 `Object`，native 内部完成 low-level handle 操作并返回断言结果

代码位置：

- `crates/ani/src/env.rs`
- `crates/ani/src/types.rs`
- `examples/reference/src/lib.rs`

建议目标：

- 继续补 `WeakRef` 的失效/GC 语义 example，而不只是 upgrade 成功路径
- 视需要补单独 example，或继续扩展 `examples/reference` 覆盖更多生命周期行为
- 明确 public ETS type 与 Rust 句柄的预期使用边界

### 二、底层已具备，但宏层和类型层还没完全压平的能力

这些能力不是“完全没有”，而是当前用户使用姿势还不够统一。

#### 1. Promise / Resolver 高层模式未统一

现状：

- `PromiseRaw`、`Deferred`、`AniResolver` 都可用
- 已有 `async_wrapper`、`error` example 覆盖一部分场景
- 但缺少统一的 derive 级设计

问题：

- 同一类能力存在多套使用路径
- 用户必须理解底层 ANI Promise 细节

建议目标：

- 先完成 `#[ani(async)]`
- 再决定是否统一 `PromiseRaw<'static, T>` 的导出建模

#### 2. `AniModule / AniNamespace / AniVariable` 缺少更好的 public ETS type model

现状：

- runtime handle 已经识别
- runtime env API 已有 `find_module`、`find_namespace`、`find_*_variable`、`get/set_variable_*`
- 但导出到 ETS 时仍然偏向 `Object`

问题：

- public API 对 ArkTS 使用者不够直观
- runtime handle 虽然能传，但类型表达不够强

建议目标：

- 明确这些 handle 在 ETS public signature 里的表现形式
- 判断哪些仍应保持 opaque object
- 判断哪些可以升级为更明确的 nominal public type

#### 3. class/property/static property metadata 模型仍可继续统一

现状：

- 相关结构已经开始往统一模型收敛
- 功能上可用
- 但设计层还有进一步整理空间

问题：

- 未来继续加 class op / property / static property 能力时，扩展面仍偏散

建议目标：

- 继续统一 class callable / property / op descriptor 模型
- 让宏展开更接近 napi-rs 的 class/property 组织方式

### 三、宏层明确尚不支持的结构限制

这些限制当前是显式存在的。

#### 1. `#[ani(object)]` 的结构限制

当前限制：

- 不支持 generic struct
- 仅支持 named fields
- 不支持 unnamed / unit struct

代码位置：

- `crates/derive/src/expand/struct.rs`

#### 2. `#[derive(AniClass)]` 的结构限制

当前限制：

- 不支持 generic struct
- 仅支持 named fields
- 不支持 unnamed / unit struct

代码位置：

- `crates/derive/src/expand/struct.rs`

#### 3. `#[derive(AniEnum)]` 的结构限制

当前限制：

- 不支持 generic enum
- 仅支持 unit variants

代码位置：

- `crates/derive/src/expand/struct.rs`

#### 4. `impl` 方法 receiver 限制

当前限制：

- 仅支持 `&self` 和 `&mut self`
- 不支持 by-value `self`

代码位置：

- `crates/derive/src/expand/impl_block.rs`

#### 5. `#[ani(constructor)]` 的命名限制

当前限制：

- 不能配合 `#[ani(name = ...)]`

代码位置：

- `crates/derive/src/expand/function.rs`

## 当前仍然粗糙的类型映射

以下类型当前在 ETS public type 上仍然偏向 `Object`，后续需要判断是否要做更精细的建模：

- `AniRef`
- `AniObject`
- `AniModule`
- `AniNamespace`
- `AniEnum`
- `AniError`
- `AniMethod`
- `AniStaticMethod`
- `AniField`
- `AniStaticField`
- `AniVariable`
- `AniResolver`
- `GlobalRef`
- `WeakRef`

相关代码位置：

- `crates/derive/src/types/ets.rs`
- `crates/derive/src/types/ani_type.rs`

说明：

- 这里并不意味着这些类型一定都要暴露成更强的 ArkTS nominal type
- 但至少需要明确哪些应该是 opaque handle，哪些应该继续向更强的 public type 靠拢

## 对比 OpenHarmony ANI 测试目录后的能力缺口

OpenHarmony ANI 原生测试目录包含以下能力面：

- `any_ops`
- `array_ops`
- `arraybuffer_ops`
- `bind_ops`
- `class_ops`
- `enum_ops`
- `error_ops`
- `find_ops`
- `fixedarray_ops`
- `fn_object_ops`
- `function_ops`
- `gref_ops`
- `local_scope_ops`
- `module_ops`
- `namespace_ops`
- `object_ops`
- `promise_ops`
- `ref_ops`
- `string_ops`
- `tuple_ops`
- `type_ops`
- `var_ops`
- `version_ops`
- `vm_ops`
- `wref_ops`
- 以及 `verifyani/*`、`bridges/*`、`cframe_iterator`、`load_library`、`native_library`、`options/*` 等

当前 ani-rs 的结论如下：

### 已经形成上层能力或 example 覆盖的

- `any_ops`
- `arraybuffer_ops`
- `class_ops` 的常用子集
- `enum_ops`
- `error_ops`
- `fixedarray_ops`
- `module_ops` 的运行时查找子集
- `namespace_ops` 的运行时查找子集
- `object_ops`
- `promise_ops` 的手工包装子集
- `ref_ops`
- `string_ops`
- `tuple_ops`
- `type_ops`
- `vm_ops`
- `wref_ops` 的基础使用子集

### 仍未形成完整上层封装或缺少明确回归覆盖的

- `gref_ops`
- 显式 `module` 绑定（已补 example + smoke，仍可补更复杂的 descriptor 场景）
- `AniVariable` 为核心的变量型 API 对外建模
- `fn_object_ops` 的系统化 example
- `promise_ops` 的 derive 级封装
- `load_library` / `native_library` 多库装载边界
- `cframe_iterator`
- `options/*`
- `version_ops`
- `verifyani/*`
- `bridges/*`

说明：

- 其中相当一部分更偏底层 runtime 验证，不一定都应该上升为 derive 能力
- 但 `gref_ops`、`promise_ops`、`module_ops`、`var_ops` 这类能力与用户侧 API 更接近，优先级更高

## 建议优先级

### P0

- 已实现 `#[ani(async)]`（后续补齐 ref/handle 友好模式与回归覆盖）
- 继续收缩 `Unknown -> Object`
- 已补齐显式 `module = ...` 绑定 example 与 ArkVM smoke（后续补复杂 descriptor 场景）
- 为 `GlobalRef / WeakRef` 增加更明确的 example 和行为验证

### P1

- 统一 `AniModule / AniNamespace / AniVariable / AniResolver` 的 public type model
- 继续统一 class/property/static property metadata 模型
- 系统化 `fn object` / `variable` / `module` / `namespace` 类型生成

### P2

- 评估 `version_ops`、`options/*`、`load_library`、`native_library` 是否需要上升为 ani-rs runtime 层 API
- 评估 `verifyani/*`、`bridges/*`、`cframe_iterator` 是否值得进入对外能力面

## 建议落地顺序

建议按以下顺序推进：

1. `#[ani(async)]`
2. `Unknown -> Object` 继续收口
3. `module = ...` example 与回归
4. `GlobalRef / WeakRef` example 与回归
5. `AniModule / AniNamespace / AniVariable / AniResolver` 的 public type model 统一
6. class/property/static property metadata 继续统一

## 维护建议

后续每补一项能力，建议同步更新本文档中的以下内容：

- 能力状态：未实现 / 部分实现 / 已实现
- 对应 example
- 对应 ArkVM 回归情况
- 是否仍存在 `Object` 级 fallback

这样可以把“功能是否已做完”和“类型是否已经做对”分开跟踪，避免只看 example 通过而忽略 public API 质量。
