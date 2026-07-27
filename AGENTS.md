# AGENTS.md

本文件是 YAYA 仓库的编码代理指南，适用于整个仓库。子目录中若以后出现更具体的 `AGENTS.md`，以离目标文件最近的指南为准。

## 项目原则

YAYA 是 Provider 驱动的通用下载宿主。主仓库只理解输入、任务、表单、进度、产物和认证等通用概念，不理解任何具体内容站点。

修改时必须保持以下边界：

- `src-tauri` 和 `src-web` 是薄适配层；可复用业务逻辑应进入 `crates/app-core`。
- 前端通过 `src/services/transport.ts` 和 `src/services/api/` 访问后端，不得在组件中直接拼接另一套 Tauri 或 REST 调用。
- Host 与 Provider 的共享词汇只定义在 `crates/provider-api`。
- Provider 发现、启停和进程通信只属于 `crates/provider-host`。
- 任务状态、调度、SQLite 持久化和事件只属于 `crates/task-runtime`。
- `crates/download-engine` 不依赖 Provider API，宿主也不直接依赖下载引擎；只有需要它的 Provider 使用它。
- 具体站点的 URL、标识、认证细节和下载逻辑必须留在独立 Provider 中。

不要把具体站点实现或私有 Provider 配置写入主仓库源码。通用协议文档可以说明外部仓库的接入方式，但不要加入真实私有配置。

## 仓库地图

- `src/`：Vue 3、Pinia、UnoCSS 和双宿主 transport。
- `src-tauri/`：Tauri 2 桌面/移动入口及命令桥接。
- `src-web/`：Axum 静态站点、REST、SSE 和浏览器文件传输。
- `crates/app-core/`：宿主无关的用户操作。
- `crates/provider-api/`：Provider trait 与稳定 wire model。
- `crates/provider-host/`：Provider Manifest、发现、管理和 stdio 协议。
- `crates/task-runtime/`：队列、状态机、持久化和执行。
- `crates/download-engine/`：HTTP 探测、分段、流式回退和恢复。
- `providers/direct/`：独立 Cargo workspace 和独立发布单元。
- `providers/bundle/`：静态 Provider 注册点。
- `docs/provider-protocol.md`：外部 Provider 协议的规范文档。

根 `Cargo.toml` 不包含 `providers/direct`；对根 workspace 执行 Cargo 命令不会完整检查 Direct Provider，必须进入该目录单独执行。

## 开发约束

### Rust

- 使用 workspace 中已有的依赖和版本；新增依赖前确认不能由现有 crate 或标准库完成。
- 公共 wire model 使用 `serde` 的 `camelCase` 字段；枚举值按现有模型使用 `snake_case`。
- 不随意修改 `Provider` trait、协议 envelope、错误码、任务状态或现有字段默认值。这些都是兼容性接口。
- 修改 wire shape 时，同步更新：
  - `docs/provider-protocol.md`
  - `src/types.ts`
  - 受影响的 Tauri/Web 适配
- Provider 失败应返回 `ProviderError` 和最准确的闭集错误码，不要靠解析错误字符串判断类型。
- 所有长时间 Provider 操作都必须响应 `CancellationToken`。暂停会取消当前执行；能否正确续传由 Provider 自己的磁盘状态保证。
- 不持有同步锁跨越 `.await`，不在异步路径执行可避免的阻塞 I/O。
- 用户可见的任务变更必须持久化并发出完整 `TaskSnapshot`。前端把后端快照视为唯一事实来源。
- 文件名、Provider 输入和远端元数据都不可信；写文件前必须限制路径穿越和非法路径字符。
- 不记录访问令牌、Cookie、二维码密钥或 Provider 私有数据。

### Vue 与 TypeScript

- 使用 `<script setup lang="ts">`、Composition API 和现有 Pinia store 模式。
- 组件负责展示与短生命周期交互；远端状态和共享操作放在 store/service 中。
- 后端类型集中在 `src/types.ts`，字段必须与 Rust 序列化结果一致。
- 所有后端调用经过 `src/services/api/`；所有运行环境判断集中使用 `isApp()`、`isDesktop()` 或 `isTauri()` 的现有封装。
- Tauri 事件和 Web SSE 都传递完整任务快照。不要在前端另建权威任务状态机。
- 新增用户操作时同时实现 Tauri command、Web route 和前端 service，除非该能力明确只属于一个宿主。
- 保持现有响应式桌面/移动布局、键盘可操作性、按钮禁用状态和错误展示方式。
- 用户界面当前以中文为主；新增文案沿用简洁中文。
- 使用现有 UnoCSS token 和组件样式，不引入并行的样式系统。

### Provider

- Provider 的 `supports`/Manifest `matches` 只负责快速判断；`inspect` 才执行完整解析和网络检查。
- 多个 Provider 匹配时优先级高者胜出。通用兜底 Provider 应使用较低优先级。
- `TaskDraft.payload` 对 Host 不透明，但必须是可序列化且足以让后续独立 `run` 调用完成任务的数据。
- Provider 声明的 `FormField.key` 和 `TaskDraft.key` 在所属视图内必须稳定且唯一。
- `run` 只能把最终产物写入 `outputDir`，临时文件和恢复信息写入 `workDir`。
- `run` 返回的 `Artifact.path` 必须指向真实产物；不要把临时文件作为 Artifact。
- 外部 Provider 严格遵守 `docs/provider-protocol.md` 的 JSON Lines 约定。stdout 只用于协议帧，诊断信息写 stderr。
- 认证凭据只保存在 `YAYA_PROVIDER_DATA_DIR`，停用 Provider 不应删除凭据。
- `fetch_asset` 只用于小型内存资源；大文件必须通过任务下载。

## 生成文件与本地配置

以下内容需特别处理：

- `providers/bundle/Cargo.toml` 和 `providers/bundle/src/lib.rs` 由 `scripts/gen-bundle.mjs` 管理。自定义 Bundle 时修改被忽略的 `bundle.config.json` 并运行脚本，不要手工拼接生成内容。
- `providers/bundle/Cargo.default.toml` 和 `providers/bundle/src/lib.default.rs` 是无本地配置时的受版本控制默认模板，必须保持只注册 Direct Provider。
- `src-tauri/gen/` 是 Tauri 生成的移动原生工程。只有任务明确涉及 Android/iOS 原生配置时才修改，并避免无关重生成。
- `src-tauri/gen/schemas/` 为生成的 Tauri schema，不要手工编辑。
- 不提交 `bundle.config.json`、`src-tauri/gen/android/keystore.properties`、密钥、签名文件、`.local` 配置、`target/`、`dist/` 或 `node_modules/`。
- 不改写用户已有的本地 Bundle 或签名配置。执行生成脚本前先确认任务确实需要它。

## 常用工作流

安装依赖：

```bash
npm ci
```

桌面开发：

```bash
npm run tauri dev
```

Web 开发需要两个进程：

```bash
cargo run -p yaya-web
npm run dev
```

`npm run dev` 只启动 Vite，不会自动启动 Rust Web 后端。

如果只修改前端，至少执行：

```bash
npm run build
```

如果修改根 Rust workspace，执行：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

如果修改 `providers/direct`、Provider API、Provider Host 或下载引擎，还要执行：

```bash
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

本仓库不保留测试代码，验证以 fmt、clippy、构建和手工验证为准。

## 按改动选择验证

| 改动 | 必需验证 |
| --- | --- |
| Vue、store、service、样式 | `npm run build` |
| 通用 Rust 逻辑 | 根 workspace 的 fmt、clippy |
| Provider wire model 或错误码 | 根 Rust 检查、前端构建、协议文档核对 |
| Tauri/Web API | 两个适配入口的类型检查，并验证对应前端 service |
| 任务状态或调度 | 根 workspace 的 fmt、clippy 及手工验证 |
| 下载引擎 | 根 workspace 的 fmt、clippy 及手工验证 |
| Direct Provider | 独立 workspace 的 fmt、clippy |
| Bundle 生成 | 在有配置和无配置两种情况下检查脚本输出；结束时保留任务要求的状态 |
| Android/iOS 配置 | 对应 Tauri 移动构建或最接近的原生配置验证 |

不要用 formatter 顺手改写与任务无关的文件。仓库可能有用户未提交改动，修改前查看 `git status`，只触碰任务需要的内容。

## 完成标准

提交工作前确认：

- 行为位于正确的架构层，没有复制 Tauri/Web 业务逻辑。
- 主仓库仍与具体站点无关。
- Rust、TypeScript 和协议中的同名字段保持一致。
- 新增失败路径有结构化错误，长任务可取消，文件路径经过约束。
- 用户可见行为有明确的手工验证方法。
- 相关文档、示例和环境变量说明已同步。
- 已报告实际执行的检查结果以及任何环境限制。
