# YAYA Direct Provider

YAYA 的最小 Rust 参考 Provider，用于下载普通 HTTP/HTTPS 文件。

## 能力

- 匹配 `http://` 和 `https://`
- 读取远程文件名、类型与大小
- 支持进度、取消、临时文件和安全重名
- 提供由 Provider 自己实现的下载列表 UI
- 同时可作为外部进程或内嵌 Provider 使用

## 构建

从 YAYA 根目录构建 UI：

```bash
npm run build:provider-ui:direct
```

构建后端：

```bash
cd providers/direct
cargo build -p provider-direct
```

`provider.json` 已包含 `target/debug/provider-direct` 开发候选路径。从 YAYA 根目录启动 Host 后即可自动发现。

## 目录

```text
provider.json
crates/provider-direct/    Rust Provider、CLI 和内嵌实现
ui/src/                    Vue + UnoCSS resolve surface
ui/dist/                   发布用 JS/CSS bundle
docs/development.md        本 Provider 的开发教学
```

通用开发流程见 [开发教学](docs/development.md)，完整协议见 [YAYA Provider Protocol](../../docs/provider-protocol.md)。

## 检查

```bash
npm run build:provider-ui:direct
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## License

GPL-3.0-only。
