# ani-rs vs napi-rs 设计差异梳理与当前对齐结果

本文档以 `napi-rs` 的抽象层次与工程化实践为参照，梳理当前 `ani-rs`（ArkTS 1.2 ANI 绑定库）在设计与能力面上的差异点，并记录本轮已经完成的对齐项与保留下来的 runtime 边界。

## 1. 运行时与 ABI 约束差异

`napi-rs` 面向 Node.js N-API：

- ABI 入口是 `napi_register_module_v1`，由 Node 在加载 `.node` 模块时回调
- 导出是对 `exports` 对象挂属性
- 函数签名不需要 JNI 风格 mangling，参数解析来自 `napi_get_cb_info`

`ani-rs` 面向 ArkTS 1.2 ANI：

- ABI 入口是 `ANI_Constructor`，由 ArkTS `loadLibrary()` 触发
- 导出需要绑定到 **Module / Namespace / Class** 三类目标
- ABI 需要明确的 mangling 签名（类似 JNI），并且需要通过 descriptor 查找目标

这决定了两者即使宏形态相似（`#[napi]` vs `#[ani]`），底层注册与签名系统也必然不同。

## 2. 分层与 crate 组织差异

`napi-rs` 的典型分层：

- `napi-sys`：原始 FFI
- `napi`：安全封装 + bindgen runtime（Env/Js* 类型、Promise/Task 等）
- `napi-macro/backend`：宏解析 + wrapper 生成 + TS 类型生成

当前 `ani-rs` 的分层：

- `ani-sys`：bindgen 生成的 ANI C API
- `ani`：Env/VM/type wrappers + conversions + module_register + tokio bridge
- `ani-derive`：宏解析 + wrapper 生成 + ETS 导出

当前结果：

- 继续保持三层结构：`ani-sys` / `ani` / `ani-derive`
- `ani::prelude` 已收敛为高层默认入口，覆盖 `Env`、错误类型、VM、主要 runtime handle、conversion traits/helpers
- `ani-derive` 的 wrapper 现在已经承担 panic 边界、async Promise 包装、参数转换和一批 ref-backed 参数保活职责

## 3. 注册模型差异

`napi-rs`：

- 使用 `ctor`（或 wasm 下的替代路径）收集导出项
- Node 回调入口一次性执行注册，必要时维护 class metadata、hook 等

`ani-rs`：

- 使用 `ctor` 收集 per-function/per-impl 注册回调
- `ANI_Constructor` 中先执行 init，再执行 registrations
- 注册回调阶段只做 “enqueue”，真正 bind 阶段按目标分组后调用 `*_BindNativeFunctions/Methods`

当前结果：

- 当前 “enqueue + 分组 bind” 是合理的 ANI 适配方式
- 注册确定性已进一步收口：
  - 同 target 下的 pending bindings 会按 `name + signature + pointer` 稳定排序
  - 同 target 下重复的 `name + signature` 组合会在 bind 前诊断并返回 `ANI_ALREADY_BINDED`
  - 相关单测已覆盖排序稳定性、重复绑定拒绝、跨 target 同名共存

## 4. 类型系统与转换差异

`napi-rs`：

- 核心是 `ToNapiValue/FromNapiValue`，并辅以 `JsUnknown/JsObject/...` 的值模型
- TS 类型生成与转换系统强绑定

`ani-rs`：

- 核心是 `ToAni/FromAni` 与 `AniType`（用于签名与 ETS public type）
- 需要处理 `null/undefined` 的差异，并在 ETS 侧生成必要的 bridge wrapper
- `Vec<T>` / `VecDeque<T>` / `LinkedList<T>` 这类容器现在也开始区分 “public ETS type 精确表达” 与 “底层 bind signature 兼容 ArkVM”：
  - primitive element 仍走 fixed-array signature
  - ref/object element 的 ArkTS public type 仍保持 `Array<string>` / `Array<User>`，但 bind signature 会收敛为 `std.core.Array`

当前结果：

- 继续把 “类型表达精确度” 当作一等公民（减少 `Unknown -> Object`）
- 运行时 handle（`AniModule/AniNamespace/AniVariable/...`）的 ETS public type model 已定型，object-backed handles 会导出为显式 opaque public type

## 5. Async / Promise 语义差异

`napi-rs`：

- `#[napi] async fn` 可直接导出 `Promise<T>`
- 运行时会处理参数引用保活（ref container）与 panic 转 rejection，确保 Promise 一定 settle

`ani-rs` 当前对齐进展：

- 已支持 `#[ani(async)] async fn ... -> Result<T>` 自动导出 `Promise<T>`
- `ani::tokio` 已补齐 tokio task panic 捕获并转 Promise rejection（避免 Promise 永远 pending）
- 已引入 `RefContainer`，并在 `#[ani(async)]` 中自动托管一批 ref-backed 常规参数（含 scoped `Function<'_, ...>` 回调）与注入的 `this/class`
- `Ref<T>` / `FunctionRef<...>` 已开始向 napi-rs 风格的 managed owning handle 收敛：`FromAni` 会记录 VM，`Drop` 自动回收 global ref，返回 ArkTS 时会先还原 local handle；同步和异步 callback return 都已有 ArkVM smoke
- `RefContainer` 现在也能统一接住 `GlobalRef` / `Ref<T>` / `FunctionRef<...>` 作为 async bridge source，manual tokio helper 与宏路径之间的心智模型更接近
- Promise 路径开始收敛到同一套模型：`Deferred<T>` 已 typed 化，`Env::promise_new_typed<T>()` 和 `AniResolver` helper 能把 low-level resolver 流程桥回 `PromiseRaw<T> / Deferred<T>` 语义

当前结果：

- `#[ani(async)]` Promise 路径、manual `ani::tokio` helper、`Deferred<T>` / `AniResolver` helper 已形成统一模型
- `RefContainer` 已覆盖 local ref、scoped `Function<'_, ...>`、注入的 `this/class`，并且 manual async helper 现在已有以下 ArkVM smoke：
  - local object handle
  - typed `Ref<AniObject<'static>>`
  - `GlobalRef`
  - `FunctionRef<...>`
- `Ref<T>` / `FunctionRef<...>` 已具备 managed owning-handle 语义：`FromAni` 记录 VM，`Drop` 自动释放 global ref，返回 ArkTS 时重新 materialize local handle
- 因而本轮不再把 async 使用模式作为继续推进的 TODO；剩余差异主要只在 ANI 自身没有 `ThreadsafeFunction` 等能力面时需要另做设计

## 6. panic 边界与异常模型

`napi-rs` 默认在 wrapper 中 `catch_unwind`，避免 panic 穿越 FFI 边界。

`ani-rs` 已向该方向收敛：

- `#[ani]` 生成的同步 wrapper 已增加 `catch_unwind`，panic 会转为 ArkTS 异常（throw）
- tokio async 任务的 panic 会转为 Promise rejection

当前结果：

- 保持 “panic 永不跨 ABI” 的硬约束
- 默认安全策略已经落地：同步 wrapper `catch_unwind` 转 throw，tokio async panic 转 Promise rejection
- `#[ani(catch_unwind = false)]` 一类开关当前不作为待办推进项

## 7. 声明生成差异（TS vs ETS）

`napi-rs`：

- 侧重 `.d.ts` 生成与发布

`ani-rs`：

- 当前以 `.ets` 生成与 ArkVM smoke 验证为主
- 尚未生成 `.d.ets` / `declare` 风格

当前结果：

- 先把 `.ets` 的 public signature 做到足够精确、稳定
- `.ets` public signature 已作为当前主发布面
- `.d.ets` / `declare` 风格当前保留为 scope 选择，不再作为本轮待完成项

## 8. 能力缺口对照（精简版）

本轮已完成并完成回归覆盖的高价值对齐项：

- `#[ani(async)]` Promise 导出、panic->rejection、manual/global/typed/function ref-container 路径
- 继续收敛类型系统，减少 `Unknown -> Object`
- 显式 `module = ...` 绑定 example 与 ArkVM smoke（含 descriptor mismatch 场景）
- `GlobalRef/WeakRef` 更完整的语义覆盖（含失效/GC 行为、helper、ArkVM 回归）
- 注册阶段的稳定排序与重复绑定诊断

不一定能对齐或需要重设计：

- 类似 `ThreadsafeFunction` 的跨线程回调：取决于 ANI 是否提供等价能力面（需要先确认 sys API）

## 9. 收口结论

原先按 `docs/capability-gap.md` 列出的 P0 顺序项已经处理完成：

1. `#[ani(async)]` 的可用模式与回归覆盖已补齐
2. `Unknown -> Object` 已继续收口到少数兜底路径
3. `module = ...` example 与复杂 descriptor 回归已补齐
4. `GlobalRef/WeakRef` example 与生命周期/GC 回归已补齐
5. handle public type model 已统一

当前这份文档中不再保留可继续执行的任务列表；剩余内容仅是运行时边界或 public-API scope 决策。
