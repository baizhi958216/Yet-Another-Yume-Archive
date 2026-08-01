# YAYA Provider 开发者指南

本指南面向希望为 YAYA（Yet Another Yume Archive）开发新站点 Provider 的开发者。

YAYA 采用了完全**宿主与站点解耦**的架构。宿主不关心任何特定网站的逻辑；所有针对特定站点的链接识别、元信息解析、动态选项生成、资源下载以及自定义 Webview UI 均由 **Provider** 独立实现。

---

## 目录

1. [架构概述](#1-架构概述)
2. [Provider 目录结构](#2-provider-目录结构)
3. [Provider Manifest (provider.json)](#3-provider-manifest-providerjson)
4. [Rust 后端开发](#4-rust-后端开发)
5. [Provider UI 前端开发](#5-provider-ui-前端开发)
6. [移动端与静态内嵌 (Bundle)](#6-移动端与静态内嵌-bundle)
7. [构建、测试与调试](#7-构建测试与调试)

---

## 1. 架构概述

### 1.1 生命周期与数据流

当用户在 YAYA 宿主中输入链接或 ID 时，完整的数据交互流程如下：

```text
 用户输入 (URL / ID)
        │
        ▼
 宿主 ProviderManager 匹配 (provider.json matches / supports)
        │
        ▼
 调用 Provider::inspect (通过 Stdio JSON-RPC 或内嵌调用)
        │
        ▼
 返回 ProviderView (标题、封面、任务列表 TaskDraft、自定义表单 FormField)
        │
        ▼
 宿主渲染 UI / 载入 Provider 专属 resolve surface
        │
        ▼
 用户确认下载，触发 TaskRuntime -> Provider::run
        │
        ▼
 执行下载 (实时通过 ProgressReporter 汇报进度，响应 Cancellation)
        │
        ▼
 最终产物移动至 output_dir，返回 Vec<Artifact> 给宿主
```

### 1.2 运行传输模式

- **桌面端 / Web 端 (进程隔离模式)**：Provider 是一个独立的 CLI 可执行文件。宿主按需启动子进程，通过 `stdio` 传输 JSON-RPC 2.0 消息（协议规范见 [provider-protocol.md](provider-protocol.md)）。崩溃或异常不会影响宿主。
- **移动端 iOS / Android (静态内嵌模式)**：移动端操作系统限制创建子进程，Provider 的 Rust 代码将被直接静态编译链接入宿主 App，通过 `HostedProvider` 内存管道通信。

---

## 2. Provider 目录结构

建议在 `providers/<provider-id>/` 目录下组织独立的 Provider 仓库或子目录。一个标准的 Provider 项目结构如下：

```text
providers/my-site/
├── provider.json              # Provider 声明文件 (Manifest)
├── Cargo.toml                 # Cargo Workspace / Package 配置
├── README.md                  # 开发与使用说明
├── AGENTS.md                  # Agent / AI 协同开发规范
├── CLAUDE.md                  # 命令速查
├── crates/
│   └── provider-my-site/      # Rust 后端核心实现与 CLI 入口
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs         # 实现 Provider Trait
│           └── main.rs        # Stdio 子进程 JSON-RPC 入口
├── ui/                        # 前端 UI Bundle (Vue 3 + UnoCSS)
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   └── src/
│       ├── main.ts            # UI 入口，使用 defineProviderUi
│       └── views/
│           ├── ResolveView.vue     # 任务解析视图 (resolve surface)
│           └── ManagementView.vue  # 账号/设置管理视图 (management surface)
└── docs/
    └── development.md         # 本 Provider 的具体开发教学
```

---

## 3. Provider Manifest (provider.json)

`provider.json` 是宿主发现并加载 Provider 的清单文件。示例如下：

```json
{
  "schemaVersion": 2,
  "id": "my-site",
  "name": "My Site Downloader",
  "version": "0.1.0",
  "description": "解析并下载 My Site 的视频与图集",
  "enabledByDefault": true,
  "priority": 10,
  "ui": {
    "apiVersion": 1,
    "entry": "ui/dist/provider-ui.js",
    "style": "ui/dist/provider-ui.css",
    "surfaces": [
      { "id": "resolve", "initialHeight": 120 },
      { "id": "management", "initialHeight": 320 }
    ]
  },
  "matches": [
    { "kind": "prefix", "value": "https://mysite.com/v/" },
    { "kind": "contains", "value": "mysite.com" }
  ],
  "executables": {
    "aarch64-apple-darwin": ["target/release/provider-my-site", "target/debug/provider-my-site"],
    "x86_64-apple-darwin": ["target/release/provider-my-site", "target/debug/provider-my-site"],
    "x86_64-unknown-linux-gnu": ["target/release/provider-my-site", "target/debug/provider-my-site"],
    "x86_64-pc-windows-msvc": ["target/release/provider-my-site.exe", "target/debug/provider-my-site.exe"]
  }
}
```

### 字段说明

| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `schemaVersion` | `u32` | 必须为 `2`。 |
| `id` | `string` | 唯一标识符（例如 `direct`、`bilibili`）。 |
| `name` | `string` | 展示名称。 |
| `priority` | `i32` | 匹配优先级。值越大越优先。直链兜底为 `-100`，特定站点建议 `0 ~ 100`。 |
| `matches` | `Array` | 免启动进程的快速前置匹配规则列表（`contains`、`prefix`、`digits`）。 |
| `ui` | `Object` | 可选的前端 UI bundle 配置。 |
| `executables` | `Object` | 按 Target Triple 映射的可执行文件候选相对路径列表。 |

---

## 4. Rust 后端开发

### 4.1 引入 `yaya-provider-api`

在 Provider 的 Cargo.toml 中引入依赖：

```toml
[dependencies]
yaya-provider-api = { path = "../../../crates/provider-api" }
yaya-download-engine = { path = "../../../crates/download-engine" } # 可选，用于文件下载
async-trait = "0.1"
tokio = { version = "1", features = [ "full" ] }
serde = { version = "1", features = [ "derive" ] }
serde_json = "1"
```

### 4.2 实现 `Provider` Trait

```rust
use std::sync::Arc;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{
    Artifact, BinaryAsset, ProgressReporter, Provider, ProviderError,
    ProviderInput, ProviderTaskRequest, ProviderView, TaskDraft, FormField, FormControl,
};

pub struct MySiteProvider {
    client: reqwest::Client,
}

#[async_trait]
impl Provider for MySiteProvider {
    fn id(&self) -> &str {
        "my-site"
    }

    fn supports(&self, input: &str) -> bool {
        input.contains("mysite.com")
    }

    fn priority(&self) -> i32 {
        10
    }

    async fn inspect(&self, input: ProviderInput) -> Result<ProviderView, ProviderError> {
        // 1. 解析用户输入的 URL / ID
        // 2. 调用站点 API 提取画质、音频、文件元信息
        // 3. 构建 TaskDraft 列表与交互表单
        let task = TaskDraft {
            key: "video_1080p".into(),
            title: "高清 1080P 视频".into(),
            description: "MP4 格式 · 150MB".into(),
            size: Some(157286400),
            image_url: "https://mysite.com/cover.jpg".into(),
            selected: true,
            fields: vec![],
            payload: serde_json::json!({
                "download_url": "https://mysite.com/stream/1080p.mp4"
            }),
        };

        Ok(ProviderView {
            provider: self.id().to_string(),
            title: "示例视频标题".into(),
            description: "视频简介内容".into(),
            image_url: "https://mysite.com/cover.jpg".into(),
            tasks: vec![task],
            fields: vec![],
        })
    }

    async fn run(
        &self,
        request: ProviderTaskRequest,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError> {
        // 1. 从 request.task.payload 中提取下载所需信息
        let download_url = request.task.payload["download_url"]
            .as_str()
            .ok_or_else(|| ProviderError::invalid_params("missing download_url"))?;

        // 2. 临时文件写入 request.work_dir，最终产物移动至 request.output_dir
        let target_path = request.output_dir.join("video.mp4");

        // 3. 使用下载引擎或自研网络下载，实时通过 reporter 汇报进度
        reporter.report(yaya_provider_api::TaskProgress {
            completed: 1024,
            total: Some(2048),
            bytes_per_second: Some(512),
            message: "下载中...".into(),
        });

        // 4. 返回生成的产物列表
        Ok(vec![Artifact {
            path: target_path,
            name: "video.mp4".into(),
            mime_type: "video/mp4".into(),
            size: Some(2048),
            metadata: Default::default(),
        }])
    }
}
```

### 4.3 实现 CLI 子进程 Stdio 循环 (`main.rs`)

在 Desktop/Web 运行模式下，子进程通过 `main.rs` 从 `stdin` 逐行读取 JSON-RPC 请求，并将响应输出到 `stdout`。

`yaya-provider-api` 提供了标准的 Stdio 循环处理函数或手写 JSON-RPC 解包。格式详见 [provider-protocol.md](provider-protocol.md)。

---

## 5. Provider UI 前端开发

当 Provider 需要展示复杂的任务选择器（如多 Episode 列表、画质切换）或管理界面（如 Cookie 登录）时，可以构建前端 UI Bundle。

### 5.1 安装依赖与包引入

UI 使用 Vue 3 + UnoCSS，并引入官方 SDK `@yaya/provider-ui`（在工作区根 package.json 中管理）：

```json
{
  "dependencies": {
    "@yaya/provider-ui": "*",
    "vue": "^3.5.40"
  }
}
```

### 5.2 挂载 Surface (`main.ts`)

```typescript
import { defineProviderUi } from '@yaya/provider-ui'
import ManagementView from './views/ManagementView.vue'
import ResolveView from './views/ResolveView.vue'
import 'virtual:uno.css'

export default defineProviderUi({
  resolve: ResolveView,
  management: ManagementView,
})
```

### 5.3 在 Vue 视图中使用 Bridge

```vue
<script setup lang="ts">
import { useProviderContext, useProviderUi } from '@yaya/provider-ui'

const bridge = useProviderUi()
const context = useProviderContext<{ tasks: any[] }>()

async function onRefresh() {
  // 调用 Provider 后端的自定义 RPC action
  const result = await bridge.invoke('refresh_quality', { format: '4k' })
  // 更新表单状态给宿主
  bridge.updateState({ selectedFormat: '4k' })
}
</script>

<template>
  <div class="p-3 bg-soft text-ink rounded-lg border border-line">
    <h3 class="text-sm font-medium text-accent">
      解析选项
    </h3>
    <button class="mt-2 px-3 py-1 bg-accent text-white rounded" @click="onRefresh">
      刷新画质
    </button>
  </div>
</template>
```

### 5.4 UnoCSS 样式规范

样式必须继承 `@yaya/provider-ui` preset 提供的 semantic tokens：
- **文本颜色**：`text-ink`（主文本）、`text-muted`（次要文本）、`text-accent`（强调文本）。
- **背景与边框**：`bg-soft`、`border-line`、`bg-accent-soft`。
- 绝不写写死调色板（如 `text-red-500`），确保深色模式与浅色模式自动适配。

---

## 6. 移动端与静态内嵌 (Bundle)

针对 iOS / Android 移动端构建时，项目根目录的 `scripts/gen-bundle.mjs` 会读取配置并将该 Provider 注册到 `providers/bundle/src/lib.rs` 中：

```rust
pub fn providers() -> Vec<HostedProvider> {
    let mut values = Vec::new();
    values.push(HostedProvider::new(provider_my_site::MySiteProvider::new()));
    values
}
```

这使得完全相同的 Rust 代码既能作为 CLI 子进程独立编译，又能作为 Lib 静态嵌入移动端宿主。

---

## 7. 构建、测试与调试

### 7.1 构建步骤

在根目录下执行：

```bash
# 1. 编译 UI Bundle
npm run build:provider-ui

# 2. 编译 Provider 后端二进制
cd providers/my-site
cargo build --release
```

### 7.2 CLI 管道调试

由于桌面端子进程采用基于 Stdio 的 JSON-RPC 2.0，你可以直接使用命令行测试你的 Provider：

```bash
# 测试 supports
printf '{"protocolVersion":2,"method":"supports","params":{"input":"https://mysite.com/v/123"}}\n' \
  | target/debug/provider-my-site

# 测试 inspect
printf '{"protocolVersion":2,"method":"inspect","params":{"value":"https://mysite.com/v/123"}}\n' \
  | target/debug/provider-my-site
```

标准输出应返回 JSON 格式的 `Response` 对象。

### 7.3 宿主集成测试

将可执行文件编译至 `provider.json` 配置的路径中，启动 YAYA 桌面端或 Web 端，宿主启动时扫描发现该 Provider，即可在界面中粘贴对应 URL 进行完整联调。
