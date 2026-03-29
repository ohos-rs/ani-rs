---
layout: home

hero:
  name: ani-rs
  text: "面向 ArkTS Native Interface 的 Rust 绑定层"
  tagline: "以 #[ani]、#[ani(init)]、#[ani(async)] 为核心入口，把 ArkTS 1.2 ANI 的 Module / Namespace / Class 绑定、类型转换、ETS 导出和 Docker + ArkVM 回归收敛到同一套工程化流程里。"
  image:
    src: /ani-mark.svg
    alt: ani-rs
  actions:
    - theme: brand
      text: 快速开始
      link: /guide/getting-started
    - theme: alt
      text: 支持能力
      link: /reference/capabilities
    - theme: alt
      text: 浏览示例
      link: /guide/examples

features:
  - title: 低样板导出
    details: "#[ani] 自动注册导出项，统一承载 module、namespace、class、getter、setter、constructor 和 overload。"
  - title: 能力范围可查
    details: "支持能力、运行时句柄、已验证 example 和明确边界都拆成独立页面，不需要先翻设计文档。"
  - title: 可验证的类型面
    details: "ToAni / FromAni 与 AniType 同时驱动运行时转换、签名生成和 ETS public type，持续收缩 Unknown -> Object 兜底。"
  - title: 异步 Promise 路径
    details: "#[ani(async)]、Deferred<T>、AniResolver 和 ani::tokio 已统一到 Promise bridge，并覆盖注入与常见 ref-backed 参数托管。"
  - title: Docker + ArkVM 回归
    details: "仓库内脚本已经把 ETS 输出检查、Ubuntu Docker 构建和 ArkVM smoke 串起来，文档直接给出复现命令。"
---

<HomeOverview />
