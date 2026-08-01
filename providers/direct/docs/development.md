# Direct Provider 开发教学

Direct Provider 是理解 YAYA Provider 的最小入口。

## 1. 数据流

```text
http(s) URL
  → provider.json matches
  → DirectProvider::inspect
  → 单个 TaskDraft
  → ResolveView.vue
  → DirectProvider::run
  → Artifact
```

## 2. 修改输入解析

入口位于 `crates/provider-direct/src/lib.rs`。`supports` 只判断能否处理，`inspect` 负责完整解析和远程探测。

保持以下规则：

- 只接受具有 host 的 HTTP/HTTPS URL；
- 通用 Provider 优先级保持 `-100`；
- 文件名来自响应头或 URL 时必须净化；
- 网络错误返回 `network`，非法 URL 返回 `invalid_params`。

## 3. 修改任务模型

`inspect` 返回单个 `TaskDraft`。如果为任务新增选项：

1. 在 `TaskDraft.fields` 或 `ProviderView.fields` 声明字段；
2. 在 resolve surface 通过 `bridge.updateState()` 更新；
3. 在 `run` 的 `request.options` 中读取；
4. 为默认值和非法值添加测试。

不要让 `run` 依赖上一次 `inspect` 的内存；外部调用每次都会启动新进程。

## 4. 修改下载逻辑

下载必须继续使用 `yaya-download-engine`：

- 目标临时路径在 `request.work_dir`；
- 下载完成后再移动到 `request.output_dir`；
- reporter 映射 completed、total、rate 和 message；
- cancellation 必须传入下载引擎；
- 返回的 Artifact 指向已经存在的最终文件。

## 6. 构建和调试

```bash
# YAYA 根目录
npm run build:provider-ui:direct

# Provider 目录
cargo build -p provider-direct
cargo test --workspace
```

直接测试 CLI：

```bash
printf '%s\n' \
  '{"protocolVersion":2,"method":"inspect","params":{"value":"https://example.com/file.zip"}}' \
  | target/debug/provider-direct
```