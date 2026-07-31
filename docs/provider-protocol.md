# YAYA Provider Protocol

本文档定义 YAYA 外部 Provider 的包格式和子进程协议。当前唯一受支持的版本是：

- Manifest schema：`1`
- Subprocess protocol：`1`

本文档中的 JSON 字段使用 `camelCase`，枚举值使用 `snake_case`。除非字段说明允许省略，否则 Provider 应按示例输出完整且类型正确的数据。

## 1. 运行模型

YAYA Provider 有两种承载方式：

- **内嵌 Provider**：实现 Rust `Provider` 与 `ProviderControl`，通过 `providers/bundle` 静态链接。
- **外部 Provider**：以 `provider.json` 描述，由 Host 启动可执行文件并通过 stdin/stdout 交换 JSON Lines。

本文档的进程和 Manifest 章节针对外部 Provider；数据模型和方法语义同时适用于内嵌 Provider。

外部 Provider 采用 **spawn-per-call**：

1. Host 为一次方法调用启动一个新进程。
2. Host 在 stdin 写入一条以换行结束的 JSON 请求，然后关闭 stdin。
3. Provider 执行请求。
4. Provider 在 stdout 输出零条或多条事件，以及一条终止响应。
5. Provider 以退出码 `0` 结束。

Provider 不能依赖同一进程处理多次请求。需要跨调用保存的 Cookie、令牌、设备标识或缓存必须写入 `YAYA_PROVIDER_DATA_DIR`。

## 2. Provider 包与发现

### 2.1 包结构

一个外部 Provider 是 Provider 根目录下的一级子目录：

```text
providers/
└── example/
    ├── provider.json
    └── bin/
        ├── provider-example
        └── provider-example.exe
```

Host 依次扫描以下根目录：

1. `<data-dir>/providers`
2. `<current-working-directory>/providers`
3. `YAYA_PROVIDERS_DIR`

后扫描的根目录在相同 Provider ID 冲突时覆盖先扫描的根目录；外部 Provider 也覆盖同 ID 的内嵌 Provider。一个根目录内的同 ID 冲突行为不应依赖。

Host 会跳过以下包：

- 缺少 `provider.json`
- Manifest 不是有效 JSON 或不能反序列化
- `schemaVersion` 不是 `1`
- 没有当前目标对应的 `executables`
- 候选可执行文件均不存在

### 2.2 Manifest

最小示例：

```json
{
  "schemaVersion": 1,
  "id": "example",
  "name": "Example Provider",
  "executables": {
    "aarch64-apple-darwin": ["bin/provider-example"],
    "x86_64-apple-darwin": ["bin/provider-example"],
    "x86_64-unknown-linux-gnu": ["bin/provider-example"],
    "x86_64-pc-windows-msvc": ["bin/provider-example.exe"]
  }
}
```

完整字段：

| 字段 | 类型 | 必需 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `schemaVersion` | `u32` | 是 | — | 必须为 `1` |
| `id` | string | 是 | — | 稳定且全局唯一的 Provider ID |
| `name` | string | 是 | — | Host 界面显示名称 |
| `version` | string | 否 | `""` | Provider 版本 |
| `description` | string | 否 | `""` | 简短说明 |
| `capabilities` | object | 否 | `{}` | 可选能力 |
| `enabledByDefault` | boolean | 否 | `true` | 首次发现时是否启用 |
| `matches` | array | 否 | `[]` | 无需启动进程的输入预匹配规则 |
| `priority` | integer | 否 | `0` | 多个 Provider 匹配时数值高者优先 |
| `executables` | object | 是 | — | 目标三元组到候选相对路径数组的映射 |

`capabilities` 当前支持：

```json
{
  "authentication": true,
  "settings": true
}
```

`authentication` 缺省为 `false`。设为 `true` 表示 Provider 实现第 7 节的认证页面与 opaque action 方法。认证 UI 和逻辑完全属于 Provider，Host 不区分二维码、OAuth2、Cookie、Token 或 2FA。

`settings` 缺省为 `false`。设为 `true` 表示 Provider 实现第 8 节的设置描述、读取、保存和操作方法。字段、状态、按钮及可选自定义页面均由 Provider 声明，Host 不包含具体 Provider 的设置逻辑。

Host 支持以下目标键：

- `aarch64-apple-darwin`
- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `aarch64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`

每个值是按顺序尝试的候选路径，路径相对于 `provider.json` 所在目录。第一个存在的普通文件会被选择。Provider 的发布流程负责为 Unix 可执行文件设置执行权限。

### 2.3 输入匹配

外部 Provider 是否支持输入完全由 Manifest 的 `matches` 决定。Host 会先对输入执行 `trim`，再进行不区分 ASCII 大小写的匹配。

包含子串：

```json
{ "kind": "contains", "value": "example.com" }
```

匹配前缀：

```json
{ "kind": "prefix", "value": "ex:" }
```

匹配前缀且剩余部分必须为至少一位 ASCII 数字：

```json
{ "kind": "prefix", "value": "ep", "thenDigits": true }
```

匹配完全由 ASCII 数字组成的输入：

```json
{ "kind": "digits" }
```

同一 Provider 的任一规则命中即视为支持。若没有 Provider 命中，Host 返回 `not_found`；若多个 Provider 命中，选择 `priority` 最大者。不要依赖相同优先级时的选择顺序。

### 2.4 启用状态与数据目录

Host 将 Provider 启用集合保存在应用数据目录的 `providers.json`。首次运行使用 `enabledByDefault`；之后以持久化集合为准。停用 Provider 只把它从任务 Registry 中移除，不删除其数据。

启动外部 Provider 时，Host 设置：

```text
YAYA_PROVIDER_DATA_DIR=<provider-root>/.data/<provider-id>
```

Host 会在启动前创建该目录。Provider 应：

- 只在这里保存认证凭据和跨调用状态
- 自己管理文件格式与迁移
- 不假设当前工作目录
- 不把凭据输出到 stdout、stderr、任务 payload 或进度事件

## 3. 传输协议

### 3.1 请求

Host 在 stdin 写入一条 UTF-8 JSON，并追加 `\n`：

```json
{
  "protocolVersion": 1,
  "method": "inspect",
  "params": {
    "value": "https://example.com/file"
  }
}
```

字段均为必需：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `protocolVersion` | `u32` | 当前为 `1` |
| `method` | string | 第 6、7、8 节定义的方法名 |
| `params` | JSON value | 对应方法参数 |

不支持协议版本时返回 `unsupported_protocol`；方法未知时返回 `unsupported_method`；参数不能反序列化时返回 `invalid_params`。

### 3.2 终止响应

成功：

```json
{"result":{"title":"Example","tasks":[]}}
```

失败：

```json
{
  "error": {
    "code": "network",
    "message": "upstream request failed",
    "data": {
      "status": 503
    }
  }
}
```

终止响应必须满足：

- 是一行完整 JSON
- `result` 与 `error` 二选一
- 成功时 `result` 不得为 `null`
- 没有业务返回值的方法使用 `{}` 作为 `result`
- 作为 stdout 最后一条非空协议帧
- 输出后进程以退出码 `0` 结束

Host 在 `inspect`、`fetch_asset` 和认证调用中读取 stdout 的最后一个非空行作为终止响应。在 `run` 中，Host 会逐行处理事件和终止响应。为保证兼容性，Provider 不应在终止响应后继续输出。

退出码非 `0` 会被视为进程失败，即使 stdout 已包含成功结果。诊断信息应写入 stderr；stdout 只能写协议 JSON。避免向 stderr 写入大量数据，否则管道缓冲可能阻塞进程。

### 3.3 事件

只有 `run` 使用流式事件。进度事件：

```json
{
  "event": "progress",
  "progress": {
    "completed": 1048576,
    "total": 8388608,
    "rate": 524288,
    "message": "Downloading"
  }
}
```

Host 会忽略未知 `event`，便于以后增加日志或阶段事件；但 v1 只保证处理 `progress`。事件必须是一行有效 JSON。没有 `event` 字段的行会被当作终止响应解析。

### 3.4 取消

Host 只对 `run` 提供取消。任务暂停或取消时，Host 直接终止当前 Provider 进程，并把调用映射为 `canceled`。

v1 没有优雅退出消息或信号握手。Provider 必须：

- 把下载中间数据写到 `workDir`
- 以可恢复方式更新中间状态
- 避免先破坏已有最终产物再开始长操作
- 能在下次全新进程的 `run` 中从磁盘状态恢复

## 4. 通用数据模型

以下定义描述 JSON wire shape。`u64`/`i64` 在 JSON 中表现为 number；路径表现为平台路径字符串。

### 4.1 ProviderInput

```json
{
  "value": "raw user input"
}
```

`value` 是必需的原始输入。

### 4.2 ProviderView

```json
{
  "provider": "example",
  "title": "Example Collection",
  "description": "12 items",
  "imageUrl": "https://example.com/cover.jpg",
  "tasks": [],
  "fields": []
}
```

| 字段 | 类型 | 必需 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `provider` | string | 否 | `""` | Host 会覆盖为实际 Provider ID |
| `title` | string | 是 | — | 视图标题 |
| `description` | string | 否 | `""` | 视图说明 |
| `imageUrl` | string | 否 | `""` | 封面 URL；远程资源由 `fetch_asset` 获取 |
| `tasks` | `TaskDraft[]` | 是 | — | 可创建的任务 |
| `fields` | `FormField[]` | 否 | `[]` | 应用于所有已选任务的共享选项 |

### 4.3 TaskDraft

```json
{
  "key": "episode-1",
  "title": "Episode 1",
  "description": "1080p",
  "size": 8388608,
  "imageUrl": "",
  "selected": true,
  "fields": [],
  "payload": {
    "resourceId": "opaque-to-host"
  }
}
```

| 字段 | 类型 | 必需 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `key` | string | 是 | — | 当前视图内稳定且唯一 |
| `title` | string | 是 | — | 任务标题 |
| `description` | string | 否 | `""` | 任务说明 |
| `size` | number 或 `null` | 否 | `null` | 预估大小 |
| `imageUrl` | string | 否 | `""` | 任务图片 URL |
| `selected` | boolean | 否 | `true` | Host 初始是否选中 |
| `fields` | `FormField[]` | 否 | `[]` | 此任务独有选项 |
| `payload` | JSON value | 否 | `null` | Host 原样保存并在 `run` 时返回 |

`payload` 属于 Provider 私有数据，但不应用来保存认证秘密。一次 `inspect` 与对应 `run` 可能由不同进程完成，因此 `payload` 必须包含重建任务所需的非敏感上下文。

### 4.4 FormField

所有字段共有：

| 字段 | 类型 | 必需 | 默认值 |
| --- | --- | --- | --- |
| `key` | string | 是 | — |
| `label` | string | 是 | — |
| `description` | string | 否 | `""` |
| `type` | string | 是 | — |

Toggle：

```json
{
  "key": "includeMetadata",
  "label": "包含元数据",
  "type": "toggle",
  "default": false
}
```

`default` 缺省为 `false`。

Select：

```json
{
  "key": "quality",
  "label": "质量",
  "type": "select",
  "options": [
    {
      "label": "1080p",
      "value": 80,
      "description": "推荐"
    }
  ],
  "default": 80
}
```

`options` 必需；每个选项的 `label`、`value` 必需，`description` 缺省为 `""`。字段 `default` 缺省为 `null`。

Text：

```json
{
  "key": "filePrefix",
  "label": "文件名前缀",
  "type": "text",
  "default": "",
  "placeholder": "可选"
}
```

`default` 和 `placeholder` 均缺省为 `""`。

Number：

```json
{
  "key": "threads",
  "label": "线程数",
  "type": "number",
  "default": 4,
  "min": 1,
  "max": 16,
  "step": 1
}
```

`default` 必需；`min`、`max`、`step` 可省略或为 `null`。

设置表单还可使用 `secret`，结构与 `text` 相同，但 Host 必须按密码输入框渲染：

```json
{
  "key": "token",
  "label": "访问令牌",
  "type": "secret",
  "default": "",
  "placeholder": "输入令牌"
}
```

Host 把用户答案以 `FormField.key` 为键放入任务的 `options`。Provider 必须自行验证值类型、范围和仍然有效的 Select 选项，不能信任 Host 输入。

### 4.5 ProviderTaskRequest

```json
{
  "id": "7c7d356f-6482-44af-a8dd-458dd754df65",
  "source": "original input",
  "task": {
    "key": "episode-1",
    "title": "Episode 1",
    "payload": {
      "resourceId": "opaque-to-host"
    }
  },
  "options": {
    "quality": 80
  },
  "outputDir": "/path/to/output",
  "workDir": "/path/to/output/.yaya/7c7d356f-6482-44af-a8dd-458dd754df65",
  "settings": {}
}
```

| 字段 | 类型 | 必需 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `id` | string | 是 | — | Host 生成的任务 ID |
| `source` | string | 是 | — | 创建任务时的原始输入 |
| `task` | `TaskDraft` | 是 | — | `inspect` 产生的任务草稿 |
| `options` | object | 否 | `{}` | 共享字段与任务字段的用户答案 |
| `outputDir` | string | 是 | — | 最终产物目录 |
| `workDir` | string | 是 | — | 当前任务的临时及恢复目录 |
| `settings` | object | 否 | `{}` | 预留的通用 Provider 设置 |

Provider 应把临时文件放入 `workDir`，只把完成的产物放入 `outputDir`。任务成功后 Runtime 会清理 `workDir`；暂停时保留，取消或删除时清理。

### 4.6 TaskProgress

| 字段 | 类型 | 必需 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `completed` | number | 否 | `0` | 已完成单位 |
| `total` | number 或 `null` | 否 | `null` | 总单位 |
| `rate` | number | 否 | `0` | 每秒完成单位 |
| `message` | string | 否 | `""` | 当前阶段 |

单位由 Provider 定义，但通常使用字节。一次任务内的 `completed`、`total` 和 `rate` 必须使用相同单位，并应保持 `completed` 单调不减。

### 4.7 Artifact

```json
{
  "path": "/path/to/output/file.bin",
  "name": "file.bin",
  "mimeType": "application/octet-stream",
  "size": 8388608,
  "metadata": {}
}
```

| 字段 | 类型 | 必需 | 默认值 |
| --- | --- | --- | --- |
| `path` | string | 是 | — |
| `name` | string | 否 | `""` |
| `mimeType` | string | 否 | `""` |
| `size` | number 或 `null` | 否 | `null` |
| `metadata` | object | 否 | `{}` |

`path` 必须是已完成且 Host 可读取的真实文件。`run` 可以返回多个 Artifact。

### 4.8 BinaryAsset

```json
{
  "contentType": "image/png",
  "bytes": "iVBORw0KGgoAAA..."
}
```

`contentType` 和 `bytes` 都是必需字符串；`bytes` 使用标准 Base64。该类型只适合封面、头像等小型内存资源。

## 5. 错误

错误格式：

```json
{
  "code": "auth_required",
  "message": "login first",
  "data": {
    "action": "open_provider_settings"
  }
}
```

| `code` | 语义 |
| --- | --- |
| `unsupported_protocol` | 不支持请求中的 `protocolVersion` |
| `invalid_params` | JSON 或方法参数无效 |
| `unsupported_method` | 方法或可选能力未实现 |
| `auth_required` | 操作需要先认证 |
| `not_found` | 输入、资源或任务不存在 |
| `network` | 上游网络或 HTTP 失败 |
| `canceled` | Provider 自己检测到取消；外部 `run` 通常由 Host 终止 |
| `internal` | 无法归入以上类别的 Provider 内部错误 |

`message` 是面向用户或日志的简洁说明。`data` 可省略，应只包含安全、可序列化、无需 Host 理解的补充信息。不要用 `internal` 包装本可准确表达的认证、网络或参数错误。

## 6. 核心方法

### 6.1 `inspect`

把原始输入解析为 Host 可渲染的任务视图。

请求：

```json
{
  "protocolVersion": 1,
  "method": "inspect",
  "params": {
    "value": "ex:123"
  }
}
```

成功结果是 `ProviderView`：

```json
{
  "result": {
    "title": "Example Collection",
    "description": "2 items",
    "tasks": [
      {
        "key": "1",
        "title": "Item 1",
        "payload": {
          "id": 1
        }
      },
      {
        "key": "2",
        "title": "Item 2",
        "selected": false,
        "payload": {
          "id": 2
        }
      }
    ]
  }
}
```

Host 会把结果中的 `provider` 覆盖为实际 Provider ID。

### 6.2 `run`

执行一个已创建任务。

`params` 是完整的 `ProviderTaskRequest`。执行期间可以输出进度，最后返回 `Artifact[]`：

```jsonl
{"event":"progress","progress":{"completed":0,"total":8388608,"rate":0,"message":"Preparing"}}
{"event":"progress","progress":{"completed":4194304,"total":8388608,"rate":1048576,"message":"Downloading"}}
{"result":[{"path":"/downloads/file.bin","name":"file.bin","mimeType":"application/octet-stream","size":8388608,"metadata":{}}]}
```

即使没有产物，成功结果也应为 `[]`，不能为 `null`。

### 6.3 `fetch_asset`

在 Provider 的网络和认证上下文中获取小型远程资源：

```json
{
  "protocolVersion": 1,
  "method": "fetch_asset",
  "params": {
    "url": "https://example.com/cover.png"
  }
}
```

结果是 `BinaryAsset`。不支持时返回 `unsupported_method`。如果 Provider 在 `ProviderView.imageUrl`、`TaskDraft.imageUrl` 或认证用户信息中返回 HTTP(S) 图片，应实现此方法。

### 6.4 `describe`

参考 Direct Provider 接受 `describe` 并返回自身描述，但当前 Host **不调用该方法进行发现或展示**；Manifest 才是外部 Provider 元数据的唯一来源。

第三方 Provider 可以为调试实现 `describe`，但它不是 protocol v1 的必需方法，也不能替代 `provider.json`。

## 7. Provider 认证

Manifest 声明 `"authentication": true` 时，Provider 必须实现本节两个方法。认证表单、UI、步骤、轮询和状态机全部由 Provider 页面实现；Host 不渲染认证字段，也不理解二维码、OAuth2、Cookie、Token 或 2FA。

### 7.1 `auth_describe`

参数为 `{}`，结果是 Provider 自带的完整认证页面：

```json
{
  "html": "<main>...</main><script>...</script>",
  "height": 480
}
```

Host 把 HTML 放入无同源权限的沙箱 iframe，并注入禁止网络连接、表单提交、外部资源和子框架的 CSP。页面必须内联所需 CSS/JS；二维码、表单组件和认证流程代码都属于 Provider 发布物。OAuth 页面可用带 `target="_blank"` 和 `rel="noreferrer"` 的普通链接打开外部授权页；沙箱只允许弹出窗口逃逸，不赋予认证页面同源或顶层导航权限。

### 7.2 `auth_invoke`

认证页面只能通过 `postMessage` bridge 请求 Provider 操作：

```json
{
  "channel": "yaya-provider-auth",
  "version": 1,
  "type": "invoke",
  "requestId": "1",
  "action": "verify_2fa",
  "payload": {
    "code": "123456",
    "flowId": "opaque-id"
  }
}
```

Host 将 `action` 和 opaque `payload` 传给 `auth_invoke`：

```json
{
  "action": "verify_2fa",
  "payload": {
    "code": "123456",
    "flowId": "opaque-id"
  }
}
```

结果是 Provider 定义的任意 JSON。Host 不读取其内容，只通过 bridge 返回页面：

```json
{
  "state": "waiting_for_2fa",
  "message": "验证码错误"
}
```

bridge 响应格式为 `{ channel, version, type: "response", requestId, ok, result }`；失败时使用 `error`。Provider 页面自行处理结果、渲染下一步和安排轮询。Provider 必须验证所有 action/payload，长期凭据和真正的认证状态只能保存在 `YAYA_PROVIDER_DATA_DIR`，不得放入返回给页面的 payload。

页面内容高度改变时可发送 `{ channel, version, type: "resize", height }`。Host 只接受有限数值并约束在 96–800px；这不会开放其他宿主能力。

认证页面与第 8.5 节的复杂设置页面共用同一个通用沙箱运行时；Host 不为认证或设置维护独立 iframe 实现，只通过 channel 和受限 handler 配置区分 bridge。

## 8. Provider 设置

Manifest 声明 `"settings": true` 时，Provider 通过统一协议完整控制自己的设置。Host 只认识字段、状态、操作和沙箱 bridge，不解释 FFmpeg、BT、代理、镜像等 Provider 私有概念。

### 8.1 `settings_describe`

返回设置页结构。普通设置由 Host 通用渲染：

```json
{
  "sections": [
    {
      "key": "network",
      "title": "网络",
      "description": "仅影响当前 Provider",
      "fields": [
        {
          "key": "proxy",
          "label": "代理地址",
          "description": "",
          "type": "text",
          "default": "",
          "placeholder": "http://127.0.0.1:7890"
        }
      ],
      "statuses": [
        {
          "key": "runtime",
          "label": "运行环境",
          "available": true,
          "value": "可用",
          "description": "已找到所需组件"
        }
      ],
      "actions": [
        {
          "key": "test_proxy",
          "label": "测试连接",
          "description": "",
          "style": "secondary"
        }
      ]
    }
  ]
}
```

`sections`、`fields`、`statuses` 和 `actions` 缺省为 `[]`。各层 `key` 在所属视图内必须稳定且唯一。状态的 `label`、`available` 和 `value` 必需；操作 `style` 可为 `primary`、`secondary` 或 `danger`，缺省为 `secondary`。

### 8.2 `settings_get`

返回当前字段值：

```json
{
  "values": {
    "proxy": "http://127.0.0.1:7890"
  }
}
```

未保存的字段可以省略，Host 使用 `settings_describe` 中的 `default` 展示。Provider 不应把 `secret` 字段的明文值返回给 Host；可以返回空字符串或掩码，并自行定义空值在保存时代表“保持原值”还是“清空”。

### 8.3 `settings_update`

参数与结果均为 `ProviderSettingsState`：

```json
{
  "values": {
    "proxy": "http://127.0.0.1:7890"
  }
}
```

Provider 必须验证全部键和值，只持久化自己声明并接受的内容，并返回保存后的安全状态。Host 的字段过滤不是安全边界。

### 8.4 `settings_invoke`

执行 Provider 声明的操作：

```json
{
  "action": "test_proxy",
  "values": {
    "proxy": "http://127.0.0.1:7890"
  }
}
```

结果：

```json
{
  "message": "连接成功",
  "refresh": true
}
```

`message` 缺省为空；`refresh` 为 `true` 时 Host 重新调用 `settings_describe` 和 `settings_get`。Provider 必须拒绝未声明或不支持的操作。

### 8.5 自定义沙箱页面

复杂设置可在 `settings_describe` 中同时返回 `customPage`：

```json
{
  "sections": [
    {
      "key": "advanced",
      "title": "高级设置",
      "fields": [
        {
          "key": "proxy",
          "label": "代理地址",
          "type": "text",
          "default": "",
          "placeholder": ""
        }
      ],
      "statuses": [],
      "actions": [
        {
          "key": "test_proxy",
          "label": "测试",
          "style": "secondary"
        }
      ]
    }
  ],
  "customPage": {
    "height": 480,
    "html": "<main>...</main><script>...</script>"
  }
}
```

即使使用自定义页面，页面可访问的字段和操作也必须在 `sections` 中声明。Host 在无同源权限的 `sandbox=\"allow-scripts\"` iframe 中加载 HTML，注入禁止网络连接、表单提交、子框架、外部资源和顶层导航的 CSP，并仅开放以下 `postMessage` bridge：

```json
{"channel":"yaya-provider-settings","version":1,"type":"get","requestId":"1"}
{"channel":"yaya-provider-settings","version":1,"type":"update","requestId":"2","values":{"proxy":"..."}}
{"channel":"yaya-provider-settings","version":1,"type":"invoke","requestId":"3","action":"test_proxy","values":{"proxy":"..."}}
```

iframe 加载完成后 Host 主动发送当前状态，避免页面启动时序竞态：

```json
{"channel":"yaya-provider-settings","version":1,"type":"init","result":{"values":{}}}
```

页面也可以主动发送 `ready` 或 `get`，两者等价。Host 的响应为：

```json
{"channel":"yaya-provider-settings","version":1,"type":"response","requestId":"1","ok":true,"result":{"values":{}}}
```

失败时 `ok` 为 `false` 并带 `error` 字符串。由于沙箱页面是 opaque origin，双方发送消息时使用通配目标 origin；Host 必须校验消息来源确为当前 iframe、协议 channel/version、声明过的字段和操作。页面不得依赖外部脚本、样式或网络资源。

## 9. 最小外部 Provider 示例

`provider.json`：

```json
{
  "schemaVersion": 1,
  "id": "example",
  "name": "Example Provider",
  "version": "0.1.0",
  "description": "Example protocol implementation",
  "enabledByDefault": true,
  "priority": 10,
  "capabilities": {
    "authentication": false,
    "settings": false
  },
  "matches": [
    {
      "kind": "prefix",
      "value": "ex:"
    }
  ],
  "executables": {
    "aarch64-apple-darwin": ["bin/provider-example"],
    "x86_64-apple-darwin": ["bin/provider-example"],
    "x86_64-unknown-linux-gnu": ["bin/provider-example"],
    "aarch64-unknown-linux-gnu": ["bin/provider-example"],
    "x86_64-pc-windows-msvc": ["bin/provider-example.exe"],
    "aarch64-pc-windows-msvc": ["bin/provider-example.exe"]
  }
}
```

一次完整会话：

```text
stdin  → {"protocolVersion":1,"method":"inspect","params":{"value":"ex:1"}}
stdout ← {"result":{"title":"Example","tasks":[{"key":"1","title":"Item 1","payload":{"id":1}}]}}
exit   ← 0
```

稍后执行任务时是一个新进程：

```text
stdin  → {"protocolVersion":1,"method":"run","params":{"id":"...","source":"ex:1","task":{"key":"1","title":"Item 1","payload":{"id":1}},"options":{},"outputDir":"/downloads","workDir":"/downloads/.yaya/...","settings":{}}}
stdout ← {"event":"progress","progress":{"completed":50,"total":100,"rate":10,"message":"Downloading"}}
stdout ← {"result":[{"path":"/downloads/item-1.bin","name":"item-1.bin","mimeType":"application/octet-stream","size":100,"metadata":{}}]}
exit   ← 0
```

## 10. 兼容性检查清单

发布 Provider 前确认：

- `provider.json` 可按 schema v1 反序列化，并包含当前目标的现有可执行文件。
- `id` 稳定，`matches` 不会过度抢占其他 Provider，`priority` 有明确理由。
- 每次进程只读取一个请求，所有 stdout 非空行都是单行 JSON。
- 请求版本、未知方法和无效参数返回正确错误码。
- `inspect` 不依赖进程内状态，`run` 可仅凭请求和持久化目录工作。
- `TaskDraft.key`、`FormField.key` 唯一，Provider 验证所有 `options`。
- 进度使用一致单位，最终响应只输出一次，成功结果不是 `null`。
- 临时数据写入 `workDir`，最终 Artifact 位于 `outputDir`。
- 进程被强制终止后不会留下损坏的最终文件，并能按需要恢复。
- 凭据只写入 `YAYA_PROVIDER_DATA_DIR`，stdout/stderr 不泄露秘密。
- 远程图片需要展示时实现 `fetch_asset`。
- 声明认证能力时 `auth_describe` 和 `auth_invoke` 符合第 7 节，Host 无需理解具体认证方式。
- 声明设置能力时四个设置方法符合第 8 节；所有字段、状态和操作 key 稳定且不含秘密。
- 自定义设置页面只通过受限 bridge 访问已声明字段和操作，且不依赖网络或外部资源。

协议模型的 Rust 定义位于 `crates/provider-api`，传输实现位于 `crates/provider-host`。当实现与本文档不一致时，应把它视为兼容性问题并同时修正文档和模型。
