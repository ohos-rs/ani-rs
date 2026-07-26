---
title: 文档站开发
description: 使用 pnpm workspace、Astro 与 Starlight 维护和构建 ani-rs 文档。
---

文档站是根目录 pnpm workspace 中的独立包，源码统一放在 `website/`，不再维护第二份 `docs/`。

## 目录结构

```text
ani-rs/
├── package.json
├── pnpm-workspace.yaml
└── website/
    ├── astro.config.mjs
    ├── package.json
    ├── public/
    └── src/
        ├── assets/
        ├── content/
        │   └── docs/
        └── styles/
```

- 根 `package.json` 提供统一的 `docs:*` 命令。
- `website/astro.config.mjs` 管理 Starlight 导航、站点元数据和自定义样式。
- `website/src/content/docs/` 是唯一的文档内容源。
- `website/src/styles/shadcn.css` 定义 shadcn 风格的颜色、边框、圆角和页面组件。

## 本地开发

先在仓库根目录安装依赖：

```bash
pnpm install
```

启动支持热更新的开发服务器：

```bash
pnpm docs:dev
```

默认访问地址为 `http://localhost:4321`。

## 检查与构建

提交文档前运行：

```bash
pnpm docs:check
pnpm docs:build
```

`docs:check` 会检查 Astro、MDX 和内容集合；`docs:build` 会生成可发布的静态站点到 `website/dist/`。本地检查生产产物可运行：

```bash
pnpm docs:preview
```

## 新增页面

在 `website/src/content/docs/` 中新增 Markdown 或 MDX 文件，并至少声明标题：

```md
---
title: 页面标题
description: 页面摘要
---
```

如果页面需要出现在左侧导航，再更新 `website/astro.config.mjs` 中的 `sidebar`。普通 Markdown 用于内容页；只有需要自定义首页卡片等组件时才使用 MDX。

## 样式约定

站点以 Starlight 的可访问性和文档布局为基础，视觉层保持 shadcn 风格：

- 中性色背景与细边框
- 小圆角、低阴影
- emerald 强调色
- 清晰的代码、表格和提示块层级

优先复用 Markdown、Starlight aside 和现有 CSS class，不在内容页复制内联样式。

## GitHub Pages 发布

`.github/workflows/website.yml` 参考同组织 `arkit` 的发布流程：

1. `master` 分支的网站相关文件发生变化时触发
2. 使用锁文件安装 pnpm workspace 依赖
3. 运行 `pnpm docs:check`
4. 从 `actions/configure-pages` 取得仓库的 base path
5. 构建并上传 `website/dist/`
6. 通过 `actions/deploy-pages` 发布

Astro 配置通过 `SITE_BASE_PATH` 同时兼容本地根路径和 GitHub Pages 的 `/ani-rs/` 子路径。仓库的 Pages Source 需要设置为 **GitHub Actions**。
