<script setup lang="ts">
const stats = [
  { value: '3', label: 'Workspace crates', note: 'ani / ani-derive / ani-sys' },
  { value: '52', label: 'ArkVM examples', note: '全量 smoke 已接入脚本回归' },
  { value: '.ets', label: '发布面', note: '当前仅生成 ETS，不生成 .d.ets' },
  { value: 'Docker', label: '验证环境', note: 'clean Ubuntu amd64 + ArkVM' },
]

const tracks = [
  {
    title: '绑定模型',
    body: '围绕 Module、Namespace、Class 三类目标组织导出，支持自动注册、overload、constructor、getter 和 setter。',
    link: '/guide/binding-model',
    cta: '查看绑定模型',
  },
  {
    title: '异步桥接',
    body: '#[ani(async)]、Deferred、AniResolver 与 tokio bridge 已统一到同一条 Promise 路径。',
    link: '/guide/async',
    cta: '查看异步能力',
  },
  {
    title: '能力清单',
    body: '把当前已支持、已验证和明确保留边界的能力拆成独立总览，不需要从 design 文档里反推。',
    link: '/reference/capabilities',
    cta: '查看能力总览',
  },
  {
    title: '测试闭环',
    body: '仓库内已经收敛出 cargo test、ETS 生成检查和 ArkVM Docker 回归三条主路径。',
    link: '/guide/testing',
    cta: '查看测试链路',
  },
]

const docMap = [
  { title: '快速开始', path: '/guide/getting-started', tag: '从零开始集成 ani-rs' },
  { title: '支持能力总览', path: '/reference/capabilities', tag: '按导出、类型、异步、句柄分组查看' },
  { title: '示例索引', path: '/guide/examples', tag: '52 个 example 的能力归类' },
  { title: '运行时句柄', path: '/reference/runtime-handles', tag: 'GlobalRef / WeakRef / Env / Resolver 的用法' },
]
</script>

<template>
  <section class="home-overview">
    <div class="home-overview__intro">
      <p class="eyebrow">Current Baseline</p>
      <h2>先把能力边界讲清楚，再谈 API 体验。</h2>
      <p>
        这套文档把 ani-rs 当前已经稳定可用的能力、运行时边界和验证路径拆开写。
        你可以直接从上手页开始，也可以跳到能力差异和 ArkVM 回归说明。
      </p>
    </div>

    <div class="home-overview__stats">
      <article v-for="item in stats" :key="item.label" class="stat-card">
        <p class="stat-card__value">{{ item.value }}</p>
        <h3>{{ item.label }}</h3>
        <p>{{ item.note }}</p>
      </article>
    </div>

    <div class="home-overview__tracks">
      <article v-for="item in tracks" :key="item.title" class="track-card">
        <h3>{{ item.title }}</h3>
        <p>{{ item.body }}</p>
        <a :href="item.link">{{ item.cta }}</a>
      </article>
    </div>

    <div class="home-overview__map">
      <article v-for="item in docMap" :key="item.title" class="map-card">
        <p class="map-card__tag">{{ item.tag }}</p>
        <h3>{{ item.title }}</h3>
        <a :href="item.path">打开文档</a>
      </article>
    </div>
  </section>
</template>

<style scoped>
.home-overview {
  margin: 0 auto;
  padding: 1rem 0 3rem;
}

.home-overview__intro {
  max-width: 760px;
  margin-bottom: 1.75rem;
}

.eyebrow {
  margin: 0 0 0.5rem;
  color: var(--vp-c-brand-1);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.home-overview__intro h2 {
  margin: 0;
  font-size: clamp(1.9rem, 3vw, 2.8rem);
  line-height: 1.05;
}

.home-overview__intro p:last-child {
  margin: 0.9rem 0 0;
  color: var(--vp-c-text-2);
  font-size: 1.03rem;
  line-height: 1.75;
}

.home-overview__stats,
.home-overview__tracks,
.home-overview__map {
  display: grid;
  gap: 1rem;
}

.home-overview__stats {
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  margin-bottom: 1rem;
}

.home-overview__tracks,
.home-overview__map {
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  margin-top: 1rem;
}

.stat-card,
.track-card,
.map-card {
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(13, 111, 95, 0.14);
  border-radius: 24px;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(247, 251, 249, 0.94)),
    radial-gradient(circle at top right, rgba(13, 111, 95, 0.12), transparent 45%);
  padding: 1.2rem 1.15rem;
  box-shadow: 0 18px 45px rgba(13, 111, 95, 0.08);
}

.stat-card__value {
  margin: 0 0 0.2rem;
  color: var(--vp-c-brand-1);
  font-size: clamp(1.8rem, 5vw, 2.5rem);
  font-weight: 700;
  line-height: 1;
}

.stat-card h3,
.track-card h3,
.map-card h3 {
  margin: 0;
  font-size: 1rem;
}

.stat-card p:last-child,
.track-card p,
.map-card__tag {
  color: var(--vp-c-text-2);
  line-height: 1.65;
}

.track-card a,
.map-card a {
  color: var(--vp-c-brand-1);
  font-weight: 600;
}

.map-card__tag {
  margin: 0 0 0.35rem;
  font-size: 0.9rem;
}

@media (max-width: 640px) {
  .home-overview {
    padding-bottom: 2rem;
  }

  .stat-card,
  .track-card,
  .map-card {
    border-radius: 20px;
  }
}
</style>
