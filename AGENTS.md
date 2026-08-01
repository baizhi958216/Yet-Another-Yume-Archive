# AGENTS.md

本文件适用于 `Yet-Another-Yume-Archive` (YAYA/娅娅) 根工作区。任何 AI Agent 在本仓库进行开发、修改或重构时，必须严格遵守以下规则与规范。

---

## 架构边界与核心原则

### 1. 宿主与 Provider 绝对解耦
- **零站点代码原则**：宿主 Core（`crates/app-core`）、Host（`crates/provider-host`）、API（`crates/provider-api`）、Task Runtime（`crates/task-runtime`）与 Download Engine（`crates/download-engine`）必须保持完全的**内容中立（Content-Agnostic）**。
- **不得在宿主中硬编码站点逻辑**：严禁在宿主 crate 中编写任何特定网站（如 Bilibili、YouTube、Pixiv 等）的域名判断、URL 解析、API 请求或数据结构。所有特定站点的逻辑必须且只能封装在 `providers/` 下的独立 Provider 中。

### 2. Provider 隔离与安全性
- **进程隔离**：Desktop 和 Web 模式下，Provider 作为独立子进程运行。Provider 崩溃或超时不得拖垮宿主进程。
- **凭据与敏感数据**：`TaskDraft.payload` 只能包含执行任务所需的公开数据（如 URL、文件名、流地址）。敏感凭据（如 Cookie、Token）不得暴露给宿主前端或其他未授权组件。
- **文件路径安全**：
  - 临时与中间产物必须写入 `request.work_dir`。
  - 最终下载产物写入 `request.output_dir`。
  - 遇到同名文件时必须采用安全的重命名策略，严禁覆盖既有文件。

### 3. Provider 响应性与取消支持
- **进度与取消**：所有长时运行的任务（如解析、网络下载）必须时刻响应 `CancellationToken`，并通过 `ProgressReporter` 汇报进度。
- **无状态执行**：`run` 方法不得依赖上一次 `inspect` 在内存中留下的全局状态。子进程每次 RPC 调用均可能是全新启动的独立进程。

---

## Provider UI 开发规范

- **技术栈**：Provider UI 必须使用 Vue 3、UnoCSS 以及 `@yaya/provider-ui` 软件包。
- **主题 Token 约束**：UI 样式必须严格使用 `--yaya-*` 提供的语义化 Design Tokens（如 `text-ink`、`text-muted`、`bg-soft`、`border-line`、`text-accent` 等）。严禁直接硬编码宿主系统的私有 CSS 类名。
- **布局约束**：UI 组件必须具备自适应收缩能力，在窄宽度界面（如侧边栏、模态框）下不得出现横向滚动条。
- **Bridge 通信**：通过 Provider UI Bridge 传输的状态与 Payload 必须为纯 JSON 可序列化对象。

---

## 代码构建与验证指令

无论作出任何修改，除非在开发者的明确要求下，都不进行编译运行和检查。

如果用户提出以下要求则可以进行：
### 1. TypeScript & Provider UI 检查

```bash
# 检查 Provider UI 类型与编译
npm run build:provider-ui

# 检查宿主前端类型
npm run dev -- --no-emit
```

### 2. Rust 代码格式与 Check

```bash
# 检查根工作区格式与 Lint
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# 检查参考 Provider (direct)
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---