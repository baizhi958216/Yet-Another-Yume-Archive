# Yet Another Yume Archive (YAYA / 娅娅)

> Yet Another Yume Archive（YAYA/娅娅）是一款面向多平台的现代化下载工具。

---

## 亮点

- **现代化 UI 设计**：基于 Vue 3 + UnoCSS 打造的优雅交互界面，响应式布局，支持极简主题与动态视觉体验。
- **高性能下载引擎**：内置 HTTP/HTTPS 多线程分片下载引擎，支持断点续传、实时网速限制与文件校验。
- **全平台覆盖**：同时提供 Tauri v2 桌面应用、移动端 APP 以及轻量级的 Web 网页版服务。
- **插件化扩展能力**：支持灵活的 Provider 机制，可根据需求扩展不同站点的资源解析与下载能力。

---

## 快速开始

### 前置要求

- **Node.js**: `^20.19.0` 或 `>=22.12.0`
- **Rust**: `>= 1.80`
- **npm** / **cargo**

### 1. 安装依赖

```bash
npm install
```

### 2. 构建 Provider UI 插件库

```bash
npm run build:provider-ui
```

### 3. 运行开发环境

- **Web 网页版模式** (Vite + Axum Server):
  ```bash
  npm run dev
  # 在另一个终端中启动 Web 宿主
  npm run web
  ```

- **Tauri 桌面版**:
  ```bash
  npm run tauri dev
  ```

### 4. 运行测试

```bash
cargo test --workspace
```

---

## 开发与扩展

想要为娅娅开发插件或贡献代码？请查阅以下开发文档：

- [Provider 开发者指南](docs/provider-development.md) — 了解如何编写自己的资源解析 Provider。
- [Provider Protocol 规范](docs/provider-protocol.md) — 详细的通信协议规范。
- [参考实现 (Direct Provider)](providers/direct/README.md) — 官方 HTTP/HTTPS 直链下载插件实现。

---

## License

GPL-3.0-only
