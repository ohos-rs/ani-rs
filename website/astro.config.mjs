import { defineConfig } from 'astro/config';
import { unified } from '@astrojs/markdown-remark';
import starlight from '@astrojs/starlight';

const normalizeBasePath = (value) => {
  const path = value?.trim().replace(/^\/+|\/+$/g, '');
  return path ? `/${path}/` : '/';
};

const site = 'https://ohos-rs.github.io';
const base = normalizeBasePath(process.env.SITE_BASE_PATH);
const socialImage = new URL(`${base.replace(/^\/+/, '')}og.png`, `${site}/`).href;

const rebaseContentUrls = () => (tree) => {
  const visit = (node) => {
    if (node?.type === 'element' && node.properties) {
      for (const attribute of ['href', 'src']) {
        const value = node.properties[attribute];
        if (
          typeof value === 'string' &&
          value.startsWith('/') &&
          !value.startsWith('//') &&
          !value.startsWith(base)
        ) {
          node.properties[attribute] = `${base.replace(/\/$/, '')}${value}`;
        }
      }
    }

    node?.children?.forEach(visit);
  };

  visit(tree);
};

export default defineConfig({
  site,
  base,
  markdown: {
    processor: unified({
      rehypePlugins: [rebaseContentUrls],
    }),
  },
  integrations: [
    starlight({
      title: 'ANI-RS',
      description:
        '面向 ArkTS 1.2 Native Interface 的安全、低样板 Rust 绑定层。',
      logo: {
        src: './src/assets/ani-mark.svg',
        alt: 'ANI-RS',
      },
      favicon: '/favicon.svg',
      defaultLocale: 'root',
      locales: {
        root: {
          label: '简体中文',
          lang: 'zh-CN',
        },
      },
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/ohos-rs/ani-rs',
        },
      ],
      components: {
        PageTitle: './src/components/PageTitle.astro',
      },
      customCss: ['./src/styles/shadcn.css'],
      credits: false,
      lastUpdated: true,
      tableOfContents: {
        minHeadingLevel: 2,
        maxHeadingLevel: 3,
      },
      editLink: {
        baseUrl:
          'https://github.com/ohos-rs/ani-rs/edit/master/website/',
      },
      sidebar: [
        {
          label: '开始使用',
          items: [
            { label: '概览', link: '/' },
            { slug: 'guide/getting-started' },
            { slug: 'guide/build-and-load' },
          ],
        },
        {
          label: '核心概念',
          items: [
            { slug: 'guide/exports' },
            { slug: 'guide/classes-and-objects' },
            { slug: 'guide/type-conversions' },
            { slug: 'guide/async' },
            { slug: 'guide/errors' },
            { slug: 'guide/references' },
          ],
        },
        {
          label: '指南与帮助',
          items: [
            { slug: 'guide/testing' },
            { slug: 'guide/troubleshooting' },
            { slug: 'guide/examples' },
          ],
        },
        {
          label: '参考',
          items: [
            { slug: 'reference/macros' },
            { slug: 'reference/cargo-features' },
            { slug: 'reference/support-matrix' },
          ],
        },
      ],
      head: [
        {
          tag: 'meta',
          attrs: {
            name: 'theme-color',
            content: '#09090b',
            media: '(prefers-color-scheme: dark)',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'theme-color',
            content: '#ffffff',
            media: '(prefers-color-scheme: light)',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'apple-mobile-web-app-title',
            content: 'ANI-RS',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image',
            content: socialImage,
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image:width',
            content: '1200',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image:height',
            content: '630',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'twitter:card',
            content: 'summary_large_image',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'twitter:image',
            content: socialImage,
          },
        },
      ],
    }),
  ],
});
