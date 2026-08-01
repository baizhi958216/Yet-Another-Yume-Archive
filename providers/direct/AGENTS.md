# AGENTS.md

本文件适用于 `providers/direct`。

Direct 是兜底 HTTP/HTTPS Provider，优先级必须保持较低，不能抢占更具体 Provider 的 URL。

## 边界

- Rust 业务位于 `crates/provider-direct`，不要把来源逻辑移入 Host。
- `TaskDraft.payload` 只能包含执行下载所需的公开 URL、文件名和类型，不得加入凭据。
- 临时文件写 `workDir`，成功产物写 `outputDir`，重名时不得覆盖。
- 长下载必须响应 `CancellationToken` 并通过共享下载引擎报告进度。
- `provider.json`、CLI `describe`、内嵌 descriptor 的 id、版本和 surface 必须一致。

## UI

- UI 使用 Vue 3、UnoCSS 和 `@yaya/provider-ui`，入口为 `ui/src/main.ts`。
- UnoCSS 配置继承 `packages/provider-ui/uno.config.ts`；主题只使用 `--yaya-*` token。
- Direct 只需要 `resolve` surface，不要为简单直链增加无意义的管理界面。
- 布局必须允许收缩，不得出现横向滚动。
- Bridge 状态必须为纯 JSON。

## 验证

```bash
npm run build:provider-ui:direct
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

修改 UI 时还需在 YAYA 中手工检查浅色、深色和窄宽度。
