# YAYA Provider Protocol 规范

本文档定义了 YAYA 宿主（Host）与 Provider 之间的通信协议规范（Protocol Version: `2`）。

---

## 目录

1. [基本约定](#1-基本约定)
2. [JSON-RPC 消息封装](#2-json-rpc-消息封装)
3. [RPC 方法与参数](#3-rpc-方法与参数)
4. [流式进度汇报 (Streaming Progress)](#4-流式进度汇报-streaming-progress)
5. [错误码分类 (ProviderErrorCode)](#5-错误码分类-providererrorcode)
6. [数据模型定义 (Models)](#6-数据模型定义-models)
7. [Provider UI Bridge 协议](#7-provider-ui-bridge-协议)

---

## 1. 基本约定

- **字符集**：UTF-8。
- **传输媒介**：
  - **Desktop / Web**：基于标准的 `stdio` 管道。宿主发送请求至 Provider 的 `stdin`（单行 JSON），Provider 将进度与结果输出至 `stdout`。日志/调试输出必须定向至 `stderr`，严禁污染 `stdout`。
  - **Mobile**：内存异步 Channel（Serde JSON 结构体）。
- **字段命名标准**：Wire 上的 JSON 字段统一使用 **`camelCase`** 驼峰命名法。

---

## 2. JSON-RPC 消息封装

### 2.1 请求格式 (Request)

```json
{
  "protocolVersion": 2,
  "method": "inspect",
  "params": {
    "value": "https://example.com/video/123"
  }
}
```

- `protocolVersion`: 数字 `2`。
- `method`: 调用的 RPC 方法名称（字符串）。
- `params`: 方法对应的具体参数对象。

### 2.2 响应格式 (Response)

```json
{
  "result": { ... },
  "error": null
}
```

若成功，`result` 包含返回值，`error` 为 `null`；若失败，`result` 为 `null`，`error` 格式如下：

```json
{
  "result": null,
  "error": {
    "code": "invalid_params",
    "message": "无法识别的视频 ID 格式",
    "data": null
  }
}
```

---

## 3. RPC 方法与参数

### 3.1 `supports`

判断 Provider 是否能够处理给定的输入字符串。

- **Params**:
  ```json
  { "input": "https://example.com/v/123" }
  ```
- **Result**: `boolean` (`true` 或 `false`)

---

### 3.2 `inspect`

对输入内容进行元信息解析，生成渲染给用户选择的任务视图与交互表单。

- **Params** (`ProviderInput`):
  ```json
  { "value": "https://example.com/v/123" }
  ```
- **Result** (`ProviderView`):
  ```json
  {
    "provider": "example",
    "title": "示例视频作品",
    "description": "作品描述信息",
    "imageUrl": "https://example.com/cover.jpg",
    "tasks": [
      {
        "key": "1080p",
        "title": "1080P 全高清",
        "description": "MP4 格式",
        "size": 157286400,
        "imageUrl": "",
        "selected": true,
        "fields": [],
        "payload": { "quality": 80 }
      }
    ],
    "fields": [
      {
        "key": "embedSubtitles",
        "label": "嵌入字幕",
        "description": "下载时自动压制软字幕",
        "type": "toggle",
        "default": true
      }
    ]
  }
  ```

---

### 3.3 `run`

执行具体的下载或归档任务。

- **Params** (`ProviderTaskRequest`):
  ```json
  {
    "id": "task_uuid_123",
    "source": "https://example.com/v/123",
    "task": { ... },
    "options": {
      "embedSubtitles": true
    },
    "outputDir": "/downloads/yaya",
    "workDir": "/downloads/yaya/.tmp_task_uuid_123",
    "settings": {}
  }
  ```
- **Result** (`Vec<Artifact>`):
  ```json
  [
    {
      "path": "/downloads/yaya/video.mp4",
      "name": "video.mp4",
      "mimeType": "video/mp4",
      "size": 157286400,
      "metadata": {}
    }
  ]
  ```

---

### 3.4 `fetch_asset`

在 Provider 的网络上下文（携带 Cookie、代理等）中获取小规模二进制资源（如高精度封面图）。

- **Params**:
  ```json
  { "url": "https://example.com/asset.png" }
  ```
- **Result** (`BinaryAsset`):
  ```json
  {
    "contentType": "image/png",
    "bytes": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
  }
  ```
  `bytes` 字段为标准的 Base64 编码字符串。

---

### 3.5 `ui_descriptor`

获取 Provider UI 的元数据定义。

- **Params**: `{}`
- **Result** (`ProviderUiDescriptor`):
  ```json
  {
    "apiVersion": 1,
    "surfaces": [
      { "id": "resolve", "initialHeight": 120 },
      { "id": "management", "initialHeight": 320 }
    ]
  }
  ```

---

### 3.6 `ui_bundle`

获取 Provider 自包含编译的前端 JavaScript ES Module 代码与 CSS 样式。

- **Params**: `{}`
- **Result** (`ProviderUiBundle`):
  ```json
  {
    "apiVersion": 1,
    "surfaces": [ ... ],
    "module": "export default { mount(runtime) { ... } }",
    "style": ".provider-root { ... }"
  }
  ```

---

### 3.7 `ui_action`

前端 UI Bridge 发起的透明 RPC 动作调用。

- **Params** (`ProviderUiActionRequest`):
  ```json
  {
    "action": "check_cookie",
    "payload": { "cookie": "SESSDATA=xxx" }
  }
  ```
- **Result**: 任意 JSON 可序列化值。

---

## 4. 流式进度汇报 (Streaming Progress)

在执行 `run` 方法期间，Provider 子进程可以在输出最终 Response 之前，向 `stdout` 输出多行独立放置的 Event JSON 消息汇报实时进度：

```json
{ "event": "progress", "data": { "completed": 1048576, "total": 10485760, "bytesPerSecond": 524288, "message": "下载视频片段中 (1/10)" } }
```

### Event 结构

```json
{
  "event": "progress",
  "data": {
    "completed": 1048576,
    "total": 10485760,
    "bytesPerSecond": 524288,
    "message": "提示字符串"
  }
}
```

---

## 5. 错误码分类 (ProviderErrorCode)

协议使用固定封闭集的 `snake_case` 错误码：

| 错误码字符串 | 说明 |
| :--- | :--- |
| `unsupported_protocol` | 宿主与 Provider 协议版本不匹配 |
| `invalid_params` | 参数缺失、格式错误或非法输入 |
| `unsupported_method` | Provider 未实现或不支持请求的方法 |
| `auth_required` | 需要账号登录、Cookie 或 Token 授权 |
| `not_found` | 目标资源、视频或用户不存在 (404) |
| `network` | 网络连接超时、域名解析失败或 HTTP 状态码异常 |
| `canceled` | 任务被用户或系统主动取消 |
| `internal` | Provider 内部未捕获的运行时异常 |

---

## 6. 数据模型定义 (Models)

### 6.1 表单控件 (FormControl)

用于声明在 `ProviderView` 或 `TaskDraft` 中的配置选项，宿主渲染引擎据此生成对应 UI：

- **`toggle`**: 开关选择 (`default`: boolean)
- **`select`**: 下拉选择 (`options`: Array<{ label, value, description }>, `default`: any)
- **`text`**: 单行文本输入 (`default`: string, `placeholder`: string)
- **`textarea`**: 多行文本输入 (`default`: string, `placeholder`: string, `rows`: number)
- **`secret`**: 密码/Token 隐藏输入 (`default`: string, `placeholder`: string)
- **`number`**: 数字滑动/输入 (`default`: number, `min`: number, `max`: number, `step`: number)

---

## 7. Provider UI Bridge 协议

Provider 前端（在 Webview 沙箱中运行）通过 `ProviderUiBridge` 与 Host 交互：

1. **`invoke(action, payload)`**: 映射到后端的 `ui_action` 方法。
2. **`asset(url)`**: 映射到后端的 `fetch_asset` 方法，返回 Blob/Data URL。
3. **`openModal(options)`**: 请求宿主弹出一个模态框，渲染指定的 Surface。
4. **`closeModal(result)`**: 关闭当前模态框并向调用方返回结果。
5. **`updateState(state)`**: 将用户在该 Surface 中修改的 Form 字段状态同步给宿主任务配置。
