# 使用须知

> [!IMPORTANT]
> `ani-rs` 的目标是提供接近 `napi-rs` 的开发体验，但它最终落在 ArkTS 1.2 ANI 的运行时和 ABI 约束上。读文档时，优先把它理解成“相似的人体工学，不同的底层绑定面”。

## 文档该怎么读

如果你是第一次接触这个仓库，建议按下面顺序读：

1. 先看 [快速开始](/guide/getting-started)，把最小导出和 `.ets` 产物跑起来
2. 再看 [支持能力总览](/reference/capabilities)，确认当前哪些能力已经可用
3. 需要落代码时，再按专题看 [绑定模型](/guide/binding-model)、[异步与 Tokio](/guide/async)、[运行时句柄](/reference/runtime-handles)

## 与 napi-rs 相似的部分

当前已经尽量对齐的体验包括：

- `#[ani]` 的低样板导出方式
- `#[ani(async)]` 导出 `Promise<T>`
- 自动注册与初始化回调
- 以类型系统驱动的 public declaration 生成
- Tokio feature 的拆分方式

## 与 napi-rs 不同的地方

### 绑定目标不是单一 `exports`

ANI 导出分成三类目标：

- Module
- Namespace
- Class

这意味着：

- 你需要明确函数最终挂在哪个 descriptor 上
- overload 和重复绑定校验都要按 target 维度看

### 当前只生成 `.ets`

这是当前版本的明确发布策略，不是漏实现：

- 生成 `.ets`
- 不生成 `.d.ets`
- 不生成 `declare` 风格输出

### 异步桥接有 runtime 边界

`#[ani(async)]` 已经可用，但不是“所有东西都能随便跨线程搬运”。

当前规则：

- 已进入 `RefContainer` 覆盖面的 ref-backed 参数可以自动托管
- `env` / `this` / `class` 注入会在 runtime worker 上重建
- 没有自动托管到的自定义 wrapper 或句柄，仍建议显式使用 `GlobalRef` / `RefContainer`
- 相关捕获值在必要时仍需满足 `Send + 'static`

### 还没有 `ThreadsafeFunction` 等价公开能力

这一点和 ohos-rs / napi-rs 文档里的 TSFN 页面不同。当前 ani-rs 还没有稳定对外的“跨线程回调上层函数”公开能力面。

因此目前的建议是：

- Promise 型异步优先用 `#[ani(async)]`
- 需要跨线程托管对象时优先用 `GlobalRef`、`Ref<T>`、`FunctionRef<...>` 和 `RefContainer`

## 特别注意

### `#[derive(AniEnum)]` 当前仅支持 unit variants

这属于当前 public API scope 决策。

如果未来支持带数据的 enum，通常会是另一套 tagged-union / object model，而不会继续复用 ArkTS enum 语义。

### 泛型 object/class 仍受 ArkVM runtime 限制

已经支持：

- generic struct 的 public ETS declaration
- object-backed generic instantiation 的实际 roundtrip

仍需注意：

- primitive generic instantiation 仍受 ArkVM generic slot runtime model 约束

### ArkVM smoke 需要模块名隔离

在 Docker + ArkVM 回归里，记得设置：

```bash
export ANI_TEST_MODULE_NAME=arkvm_test
```

否则容易出现测试阶段和 smoke 阶段 descriptor 不匹配。

## 如果你只关心“现在能不能用”

直接看 [支持能力总览](/reference/capabilities)。那一页会把当前已经支持、已验证和明确边界的能力放在同一个视图里。
