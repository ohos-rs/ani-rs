import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'zh-CN',
  title: 'ani-rs',
  description: 'Rust 到 ArkTS Native Interface 的工程化绑定层，围绕 #[ani] / #[ani(async)] 构建。',
  cleanUrls: true,
  lastUpdated: true,
  head: [
    ['meta', { name: 'theme-color', content: '#0d6f5f' }],
    ['meta', { name: 'apple-mobile-web-app-title', content: 'ani-rs' }],
  ],
  themeConfig: {
    logo: '/ani-mark.svg',
    nav: [
      { text: '开始使用', link: '/guide/getting-started', activeMatch: '^/guide/' },
      { text: '能力总览', link: '/reference/capabilities', activeMatch: '^/reference/' },
      { text: '参考', link: '/reference/crates', activeMatch: '^/reference/' },
      { text: '设计', link: '/design', activeMatch: '^/(design|capability-gap|napi-rs-diff)' },
      { text: 'GitHub', link: 'https://github.com/ohos-rs/ani-rs' },
    ],
    sidebar: [
      {
        text: '开始使用',
        items: [
          { text: '概览', link: '/' },
          { text: '快速开始', link: '/guide/getting-started' },
          { text: '使用须知', link: '/guide/compatibility' },
          { text: '绑定模型', link: '/guide/binding-model' },
          { text: '异步与 Tokio', link: '/guide/async' },
          { text: '测试与 ArkVM 回归', link: '/guide/testing' },
          { text: '示例索引', link: '/guide/examples' },
        ],
      },
      {
        text: '参考',
        items: [
          { text: '支持能力总览', link: '/reference/capabilities' },
          { text: 'Workspace 结构', link: '/reference/crates' },
          { text: '宏与派生', link: '/reference/macros' },
          { text: '类型系统与 ETS 面', link: '/reference/type-system' },
          { text: '运行时句柄', link: '/reference/runtime-handles' },
        ],
      },
      {
        text: '深入说明',
        items: [
          { text: '设计说明', link: '/design' },
          { text: '能力缺口清单', link: '/capability-gap' },
          { text: 'ani-rs vs napi-rs', link: '/napi-rs-diff' },
        ],
      },
    ],
    socialLinks: [{ icon: 'github', link: 'https://github.com/ohos-rs/ani-rs' }],
    search: {
      provider: 'local',
    },
    editLink: {
      pattern: 'https://github.com/ohos-rs/ani-rs/edit/main/docs/:path',
      text: '在 GitHub 上编辑此页',
    },
    outline: {
      level: [2, 3],
      label: '本页目录',
    },
    docFooter: {
      prev: '上一页',
      next: '下一页',
    },
    lastUpdatedText: '最后更新',
    darkModeSwitchLabel: '切换主题',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    sidebarMenuLabel: '文档导航',
    returnToTopLabel: '回到顶部',
    outlineTitle: '本页目录',
    footer: {
      message: 'MIT OR Apache-2.0',
      copyright: 'Copyright © 2026 ani-rs contributors',
    },
  },
})
