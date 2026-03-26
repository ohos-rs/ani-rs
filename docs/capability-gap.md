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
- 已验证 clean Ubuntu 容器内复制并解压用户提供的 `x64_linux_static_fixed.zip` 后，可直接使用其中的 `es2panda` / `es2abc` / `resources/ets/{stdlib,sdk}` / `etsstdlib.abc` 进行 example 回归
- 当前 ArkVM example 回归已在 Docker 中跑通；`examples/arkvm_report_fixed3.tsv` 中 50 个 example 均为 `build=OK / abc_compile=OK / runtime=OK`
- 当前 ArkVM example 运行时需要显式设置 `ANI_TEST_MODULE_NAME=arkvm_test`，否则会出现 descriptor 不匹配导致的 runtime 失败

最终回归结果见：

- `examples/arkvm_report_fixed3.txt`
- `examples/arkvm_report_fixed3.tsv`

## 已验证能力

以下能力已经具备 example 级别和 ArkVM 级别的验证：

- 模块级函数导出
- namespace 绑定
- 显式 `#[ani(module = ...)]` 绑定
- class instance/static/constructor 绑定
- getter/setter
- overload 绑定
- object/class nominal 类型生成
- record/set/map/fixed array/tuple/enum item/any value 等类型的基础生成
- `PromiseRaw` / `Deferred` 以及 `#[ani(async)]` 驱动的异步包装 example
- module/member、function variable、namespace variable、class static by name 等运行时查找型能力
- reference/ref scope/`GlobalRef`/`WeakRef` 基础使用
- VM version / `VmOptions` 基础能力
- error / resolver 句柄基础使用

对应 example 位于 `examples/` 目录下。

## 缺失能力分类

### 一、应优先补齐的高价值能力

这些能力对外使用价值高，并且最符合“继续向 napi-rs 风格收敛”的目标。

#### 1. `#[ani(async)]` 宏级异步绑定

现状：

- 已实现 `#[ani(async)] async fn foo(...) -> Result<T>`：宏会自动导出为 ArkTS `Promise<T>` 形式的 native binding
- 当前实现基于 `ani::tokio`（建议为 `ani` 依赖开启 `async` feature；底层仍保留 `tokio_rt` 及对齐 napi-rs 的 `tokio_*` 子 feature；未开启时 Promise 会直接 reject 并提示开启）
- 当前约束：
  - 宏层不再拒绝注入 `env` / `this` / `class`，也不再拒绝 Rust `self` receiver；Rust 单测已经覆盖这部分展开与基础逻辑
  - 已支持 `constructor/getter/setter/signature` 与 `#[ani(async)]` 组合；其中 constructor/getter/setter 会保留同步 ArkTS 形态，并在 wrapper 内阻塞等待 future 完成
  - Promise 形态下，常规参数仍会在调用线程先完成转换，再跨线程移交到 dedicated local runtime worker；因此这部分捕获值当前仍需满足 `Send + 'static`
  - 对 `AniObject / AniRef / AnyValue / AniArray* / AniFixedArray* / AniString / AniClass / AniModule / AniNamespace / AniError / AniFnObject / Function<'_, ...>` 等 ref-backed 常规参数，宏现在会自动借助 `RefContainer` 做跨线程托管与恢复
  - `FunctionRef<...>` 作为 owned global callback 已可直接跨线程捕获；同时 `FunctionRef<...>` 已支持作为 `#[ani(async)]` 的 Promise 输出值返回到 ArkTS，Docker/ArkVM smoke 已覆盖真实回调 roundtrip
  - `RefContainer::new(...)` 现在也可直接接住 `GlobalRef` / `Ref<T>` / `FunctionRef<...>` 这类已拥有 global 的句柄，manual async 场景可以统一走 “container restore local handle” 模式
  - future 的输出值和错误值本身不再因为 runtime bridge 被统一强制要求 `Send + 'static`
  - 当前 Docker/ArkVM 稳定回归已经覆盖全局 env 注入、async constructor/getter/setter 组合、static class 注入，以及 class-instance 的 `this/self` 路径与直接 `AniObject/AniRef` 常规参数
  - 对尚未纳入 `RefContainer` 自动托管的类型，仍不建议在 async 任务中手写跨线程持有 VM handle；优先使用显式 `GlobalRef/RefContainer`

代码位置：

- `crates/derive/src/expand/function.rs`
- `crates/derive/src/codegen/wrapper.rs`
- `crates/ani/src/conversions/promise.rs`
- `crates/ani/src/tokio.rs`

问题：

- 仍有进一步对齐 napi-rs 的空间（例如扩大自动托管类型面、补更多自定义 wrapper/引用类参数策略、runtime backend 抽象等）
- 现阶段自动托管仍主要覆盖 ref-backed handle；非该类参数仍依赖显式 `Send + 'static` 约束或用户手写托管

建议目标：

- 在当前基础上继续增强 async 能力面：
  - 扩大跨线程常规参数与引用/handle 参数的自动托管覆盖面
  - 继续把 `RefContainer` 能力与 `GlobalRef/WeakRef` 生命周期模型收敛成更统一的用户层模式

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

#### 3. 显式 `module = ...` 绑定已落地，剩余是复杂 descriptor 场景覆盖

现状：

- `examples/module_binding` 已落地，并接入 ArkVM smoke（`scripts/generate_arkvm_smoke_ets.sh`）
- 基础等价 descriptor 场景已经通过 Docker ArkVM 回归验证
- 当前更多覆盖的是等价 module descriptor，复杂映射场景仍较少

问题：

- “有无该能力” 已经不是问题
- 仍缺一个 “descriptor 与 crate 名不一致” 的真实场景回归，避免只覆盖最简单路径

建议目标：

- 维持现有 `examples/module_binding` + ArkVM smoke 回归
- 后续补一个 “descriptor 与 crate 名不一致” 的真实场景用例

#### 4. `GlobalRef / WeakRef` 需要更明确的高层能力面

现状：

- runtime API 已有
- `examples/reference` 主要覆盖的是 `Ref<T>` 使用模式
- `examples/reference` / `examples/weak_ref` 已覆盖 `GlobalRef` / `WeakRef` 的 create/use/delete/upgrade 基础链路
- `GlobalRef / WeakRef` 现在具备句柄自身的 helper 方法：
  - `GlobalRef::to_local / to_object / to_class / clone_ref / delete`
  - `WeakRef::upgrade / is_alive / is_released / delete`
- `Ref<T>` / `FunctionRef<...>` 现在会在 `FromAni` 场景下记录 owning `AniVm`，`Drop` 时自动删除持有的 global ref；`ToAni` 返回 ArkTS 时会先还原 thread-local handle，避免把 raw global handle 直接当作业务返回值泄露出去
- `examples/weak_ref` 已补齐更完整的 weak invalidation / GC 语义场景，包括：
  - 仅弱引用下的失效回归
  - `GlobalRef` 存活期间弱引用仍可 upgrade
  - 删除 `GlobalRef` 后在压力窗口内最终失效
- 真实 ArkVM 验证表明 raw `GlobalRef` / `WeakRef` 不适合作为 ETS public value 直接 roundtrip；当前 example 已改为 ETS 传 `Object`，native 内部完成 low-level handle 操作并返回断言结果

代码位置：

- `crates/ani/src/env.rs`
- `crates/ani/src/types.rs`
- `examples/reference/src/lib.rs`

建议目标：

- 维持当前 `WeakRef` 的失效/GC 回归
- 视需要继续扩展 `examples/reference` 覆盖更多 `GlobalRef` 高层生命周期模式（如 clone / local restore）
- 明确 public ETS type 与 Rust 句柄的预期使用边界

### 二、底层已具备，但宏层和类型层还没完全压平的能力

这些能力不是“完全没有”，而是当前用户使用姿势还不够统一。

#### 1. Promise / Resolver 高层模式未统一

现状：

- `PromiseRaw`、`Deferred`、`AniResolver` 都可用
- 已有 `async_wrapper`、`error` example 覆盖一部分场景
- 现在已补一层统一桥接：
  - `Deferred<T>` 已 typed 化，并与 `PromiseRaw<T>` 对齐
  - `Env::promise_new_typed<T>()` 可直接把 low-level `promise_new()` 桥接到 `Deferred<T> + PromiseRaw<T>`
  - `AniResolver` 已补 `resolve_value / reject_error / reject_message / into_deferred` helper，可与 `Deferred<T>` 互转
- 但仍缺少完全统一的 derive 级设计

问题：

- 同一类能力存在多套使用路径
- 用户必须理解底层 ANI Promise 细节

建议目标：

- 先完成 `#[ani(async)]`
- 再决定是否统一 `PromiseRaw<'static, T>` 的导出建模

#### 2. `AniModule / AniNamespace / AniVariable / AniResolver / GlobalRef / WeakRef` 的 public ETS type model 已明确

现状：

- runtime handle 已经识别
- runtime env API 已有 `find_module`、`find_namespace`、`find_*_variable`、`get/set_variable_*`
- 现在这批 handle 的 public ETS 暴露已经统一：
  - `AniModule / AniNamespace / AniVariable / AniResolver / GlobalRef`：保留命名 opaque handle，ETS 里导出为 `export type X = Object`
  - `WeakRef`：保持 `WeakRef<Object>`，不再和 object-backed handle 混成同一套写法

问题：

- 其余 runtime handle（例如 `AniRef / AniError / AniField / AniMethod`）虽然也已有明确 surface，但仍然属于 coarse-grained opaque handle
- 这批类型本质上还是 low-level runtime handle，不适合作为高层业务模型直接 roundtrip

建议目标：

- 继续保持 “命名明确，但语义 opaque” 的边界
- 仅在 ANI/ArkTS 语义本身足够稳定时，再评估是否要把更多 handle 提升成更强的 nominal public type

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
- `AniEnum`
- `AniError`
- `AniMethod`
- `AniStaticMethod`
- `AniField`
- `AniStaticField`

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
- `function_ops` 的查找/调用子集
- `fixedarray_ops`
- `gref_ops` 的 create/use/delete 基础子集
- `module_ops` 的运行时查找子集
- `namespace_ops` 的运行时查找子集
- `object_ops`
- `promise_ops` 的手工包装子集
- `ref_ops`
- `string_ops`
- `tuple_ops`
- `type_ops`
- `var_ops` 的 get/set 基础子集
- `version_ops` 的基础查询子集
- `vm_ops`
- `options/*` 的基础构建子集
- `wref_ops` 的基础使用子集

### 仍未形成完整上层封装或缺少明确回归覆盖的

- `AniVariable` 为核心的变量型 API 对外建模
- `fn_object_ops` 的系统化 example
- `promise_ops` 的统一高层封装（虽已有 `#[ani(async)]`，但 `PromiseRaw` / `Deferred` / `AniResolver` 仍是多套路径）
- `gref_ops` / `wref_ops` 更完整的 invalidation / GC / 生命周期行为回归
- `load_library` / `native_library` 多库装载边界
- `cframe_iterator`
- `verifyani/*`
- `bridges/*`

说明：

- 其中相当一部分更偏底层 runtime 验证，不一定都应该上升为 derive 能力
- 但 `gref_ops`、`promise_ops`、`module_ops`、`var_ops` 这类能力与用户侧 API 更接近，优先级更高

## 建议优先级

### P0

- 已实现 `#[ani(async)]`（后续补齐 ref/handle 友好模式与回归覆盖）
- 继续收缩 `Unknown -> Object`
- 为 `GlobalRef / WeakRef` 增加更明确的 example 和行为验证

### P1

- 继续统一 class/property/static property metadata 模型
- 系统化 `fn object` / `variable` / `module` / `namespace` 类型生成
- 补显式 `module = ...` 的复杂 descriptor 场景回归

### P2

- 评估 `load_library`、`native_library` 是否需要上升为 ani-rs runtime 层 API
- 评估 `verifyani/*`、`bridges/*`、`cframe_iterator` 是否值得进入对外能力面

## 建议落地顺序

建议按以下顺序推进：

1. `#[ani(async)]`
2. `Unknown -> Object` 继续收口
3. `GlobalRef / WeakRef` example 与回归
4. class/property/static property metadata 继续统一
5. 显式 `module = ...` 的复杂 descriptor 场景回归

## 维护建议

后续每补一项能力，建议同步更新本文档中的以下内容：

- 能力状态：未实现 / 部分实现 / 已实现
- 对应 example
- 对应 ArkVM 回归情况
- 是否仍存在 `Object` 级 fallback

这样可以把“功能是否已做完”和“类型是否已经做对”分开跟踪，避免只看 example 通过而忽略 public API 质量。
