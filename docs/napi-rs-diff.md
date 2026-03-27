# ani-rs vs napi-rs 设计差异梳理与对齐建议

本文档以 `napi-rs` 的抽象层次与工程化实践为参照，梳理当前 `ani-rs`（ArkTS 1.2 ANI 绑定库）在设计与能力面上的差异点，并给出在不违背 ArkTS/ANI 语义约束下可落地的优化方向。

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

对齐建议：

- 保持三层结构即可，但要把 “用户能直接用的高层模式” 尽量收敛到 `ani::prelude`（类似 `napi::bindgen_prelude`）
- `ani-derive` 的 wrapper 生成应承担更多 “安全边界” 工作（panic 边界、async 语义、参数保活策略等）

## 3. 注册模型差异

`napi-rs`：

- 使用 `ctor`（或 wasm 下的替代路径）收集导出项
- Node 回调入口一次性执行注册，必要时维护 class metadata、hook 等

`ani-rs`：

- 使用 `ctor` 收集 per-function/per-impl 注册回调
- `ANI_Constructor` 中先执行 init，再执行 registrations
- 注册回调阶段只做 “enqueue”，真正 bind 阶段按目标分组后调用 `*_BindNativeFunctions/Methods`

对齐建议：

- 当前 “enqueue + 分组 bind” 是合理的 ANI 适配方式
- 持续保证注册的确定性（同 target 下排序、重复绑定诊断）可以进一步向 napi-rs 的稳定性靠拢

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

对齐建议：

- 继续把 “类型表达精确度” 当作一等公民（减少 `Unknown -> Object`）
- 对运行时 handle（`AniModule/AniNamespace/AniVariable/...`）明确 “ETS public 表达” 与 “opaque handle 边界”，避免用户误用跨线程/跨 scope 的句柄

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

仍待对齐点：

- 自动托管仍未覆盖所有可能的 wrapper/handle 组合
- 仍需继续把 `RefContainer`、`GlobalRef`、`WeakRef` 收敛成更统一的 async 使用模式

## 6. panic 边界与异常模型

`napi-rs` 默认在 wrapper 中 `catch_unwind`，避免 panic 穿越 FFI 边界。

`ani-rs` 已向该方向收敛：

- `#[ani]` 生成的同步 wrapper 已增加 `catch_unwind`，panic 会转为 ArkTS 异常（throw）
- tokio async 任务的 panic 会转为 Promise rejection

对齐建议：

- 保持 “panic 永不跨 ABI” 的硬约束
- 后续可考虑提供可选开关（例如 `#[ani(catch_unwind = false)]`）但默认应安全

## 7. 声明生成差异（TS vs ETS）

`napi-rs`：

- 侧重 `.d.ts` 生成与发布

`ani-rs`：

- 当前以 `.ets` 生成与 ArkVM smoke 验证为主
- 尚未生成 `.d.ets` / `declare` 风格

对齐建议：

- 先把 `.ets` 的 public signature 做到足够精确、稳定
- 再评估是否需要 `.d.ets` 或 `declare` 风格作为发布形态（能力缺口文档已有追踪）

## 8. 能力缺口对照（精简版）

可以直接对齐且高价值（P0/P1）：

- `#[ani(async)]`（已落地，后续继续扩大自动托管参数/handle 范围）
- 继续收敛类型系统，减少 `Unknown -> Object`
- 显式 `module = ...` 绑定 example 与 ArkVM smoke（已补齐，含 descriptor mismatch 场景）
- `GlobalRef/WeakRef` 更完整的语义覆盖（含失效/GC 行为，已补基础 helper 与 ArkVM 回归）

不一定能对齐或需要重设计：

- 类似 `ThreadsafeFunction` 的跨线程回调：取决于 ANI 是否提供等价能力面（需要先确认 sys API）

## 9. 推荐落地顺序

建议继续沿用 `docs/capability-gap.md` 的 P0 顺序推进：

1. 完善 `#[ani(async)]` 的可用模式与回归覆盖
2. `Unknown -> Object` 继续收口
3. `module = ...` example 与回归（已补齐 smoke，含复杂 descriptor 场景）
4. `GlobalRef/WeakRef` example 与回归
5. handle public type model 统一（`AniModule/AniNamespace/AniVariable/AniResolver` 等）
