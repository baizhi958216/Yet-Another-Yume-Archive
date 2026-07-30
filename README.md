# YAYA

YAYA（Yet Another Yume Archive，娅娅）是一个由 Provider 驱动的跨平台下载器。用户只需输入地址或内容标识，匹配的 Provider 会解析可下载任务、声明可配置选项并负责实际下载；宿主只处理通用的界面、队列、进度、持久化和文件交付。

项目使用同一套 Vue 前端和 Rust 业务核心，同时提供：

- Tauri 2 桌面应用（macOS、Windows、Linux）
- Tauri 2 移动应用（Android、iOS）
- 本地 Axum Web 主机
- 可静态链接或作为外部进程安装的 Provider

> 项目目前处于 `0.1.0` 阶段。仓库暂未提供稳定版下载地址，请从源码运行或使用 CI 生成的构建产物。

## 功能

- 根据输入自动选择优先级最高的可用 Provider
- 由 Provider 动态声明单选、多选、文本、数字等下载选项
- 支持一条输入生成单任务或批量任务
- 任务排队、并发限制、暂停、继续、重试、取消和历史记录
- 实时显示进度、总量、速度、阶段消息和输出文件
- HTTP 分段下载、流式回退、失败重试和磁盘断点信息
- Provider 启用/停用、独立数据目录及可选扫码认证
- 明暗模式和主题色
- 内置 HTTP/HTTPS 直链下载 Provider
- Tauri IPC 与 Web REST/SSE 使用同一业务模型

## 架构

```text
Vue 3 + Pinia
      │
      ├── Tauri IPC / task://event
      └── REST / Server-Sent Events
                    │
              yaya-app-core
              ├── provider-host ── provider-api
              └── task-runtime ── SQLite
                         │
                      Provider
                         │
                  download-engine（可选）
```

核心模块的职责如下：

| 目录 | 职责 |
| --- | --- |
| `src/` | Vue 界面、Pinia 状态以及 Tauri/Web 双 transport |
| `src-tauri/` | 桌面和移动端宿主，负责 Tauri 命令与事件桥接 |
| `src-web/` | 本地 Web 宿主，负责静态资源、REST、SSE 和浏览器文件下载 |
| `crates/app-core/` | 与宿主无关的应用命令层 |
| `crates/task-runtime/` | 任务状态机、调度、SQLite 持久化和进度事件 |
| `crates/provider-api/` | Host 与 Provider 共享的 trait 和 wire model |
| `crates/provider-host/` | Provider 发现、启停管理和子进程协议 |
| `crates/download-engine/` | 可供 Provider 复用的 HTTP 下载引擎 |
| `providers/direct/` | 独立发布的直链下载 Provider workspace |
| `providers/bundle/` | 编译进宿主的 Provider 静态注册点 |

`src-tauri` 与 `src-web` 都只做薄适配，用户操作最终进入 `AppCore`。前端的 `src/services/transport.ts` 根据运行环境选择 Tauri IPC 或 Web API，因此两种宿主共享页面、类型和交互逻辑。

## 环境要求

基础开发环境：

- Node.js `^20.19.0` 或 `>=22.12.0`
- npm
- 当前稳定版 Rust 工具链
- Tauri 2 对目标平台要求的系统依赖

Linux 桌面构建可参考 CI 安装：

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf
```

移动端还需要对应的 Tauri 前置环境：

- Android：Android SDK、Java 17、NDK；CI 使用 NDK `27.1.12297006`
- iOS：macOS、Xcode、CocoaPods 以及有效的开发者签名配置

## 快速开始

安装前端依赖：

```bash
npm ci
```

### 桌面应用

```bash
npm run tauri dev
```

构建当前平台的安装包：

```bash
npm run tauri build
```

### Web 主机

最简单的方式会先构建前端，再在 `127.0.0.1:9527` 启动 Axum：

```bash
npm run web
```

开发时建议分别启动后端和 Vite。Vite 会把 `/api` 代理到 `127.0.0.1:9527`：

```bash
# 终端 1
cargo run -p yaya-web

# 终端 2
npm run dev
```

然后访问 Vite 输出的地址，默认是 `http://localhost:1420`。

Web 主机默认只监听本机，定位为单用户应用。浏览器创建任务时，输出会进入独立临时目录；文件完成并通过 `/api/tasks/{id}/file` 传输后，对应文件和请求目录会被清理。

### 移动应用

仓库已包含 Tauri 生成的 Android 与 iOS 工程：

```bash
npm run android:dev
npm run android:build

npm run ios:dev
npm run ios:build
```

移动端不能依赖桌面式外部可执行文件分发，Provider 应通过 `providers/bundle` 静态链接进应用。

## 使用方式

1. 在“新建下载”中输入 HTTP/HTTPS 地址或已安装 Provider 支持的内容标识。
2. YAYA 根据匹配规则和优先级选择 Provider，并请求任务预览。
3. 选择任务并填写 Provider 声明的选项。
4. 桌面端可选择保存目录；Web 和移动端由宿主决定输出位置。
5. 创建任务后在“任务中心”查看进度并执行暂停、继续、重试、取消或删除。
6. 支持身份能力的 Provider 可在“插件”页面扫码登录。

默认并发数为 3，可在“偏好设置”中调整为 1–10。

## Provider

YAYA 支持两种装载方式：

- **内嵌 Provider**：通过 `providers/bundle` 编译进宿主，适用于所有平台，也是移动端的主要方式。
- **外部 Provider**：带有 `provider.json` 和目标平台可执行文件的包；宿主按调用启动独立子进程，适用于桌面和 Web 主机。

默认 Bundle 只注册 `providers/direct`，用于下载 HTTP/HTTPS 直链。主仓库刻意保持与具体内容站点无关；站点 Provider 应位于独立仓库或独立发布单元。

### 外部 Provider 搜索位置

应用启动时依次扫描：

1. `<data-dir>/providers`
2. 当前工作目录下的 `providers/`
3. `YAYA_PROVIDERS_DIR` 指定的目录

后扫描的根目录在 Provider ID 冲突时优先，外部 Provider 也会覆盖同 ID 的内嵌 Provider。启用状态保存在数据目录的 `providers.json`。

完整的包结构、Manifest、JSON Lines 协议和数据模型见 [Provider Protocol](docs/provider-protocol.md)。

### 自定义静态 Bundle

`scripts/gen-bundle.mjs` 根据本地 `bundle.config.json` 生成 `providers/bundle/Cargo.toml` 和 `providers/bundle/src/lib.rs`。配置文件已被 Git 忽略，避免主仓库历史耦合私有或站点专用 Provider。

示例：

```json
{
  "providers": [
    {
      "crate": "yaya-extra-provider-example",
      "path": "/absolute/path/to/yaya-extra-provider-example"
    }
  ]
}
```

生成 Bundle：

```bash
node scripts/gen-bundle.mjs
```

删除本地配置后再次执行同一命令，会恢复仅包含 Direct Provider 的默认文件。

## 配置与数据

| 环境变量 | 作用 | 默认值 |
| --- | --- | --- |
| `YAYA_WEB_ADDR` | Web 主机监听地址 | `127.0.0.1:9527` |
| `YAYA_DATA_DIR` | Web 主机数据目录 | 系统本地数据目录下的 `YAYA` |
| `YAYA_PROVIDERS_DIR` | 额外外部 Provider 根目录 | 未设置 |
| `YAYA_PROVIDER_DATA_DIR` | 宿主传给外部 Provider 的独立持久化目录 | 由宿主设置 |

数据目录中包含：

- `yaya.db`：任务快照和运行时设置
- `settings.json`：应用设置
- `providers.json`：Provider 启用状态
- `providers/`：用户安装的外部 Provider
- `providers/.data/<provider-id>/`：外部 Provider 的持久化数据

不要把 Provider 凭据写进任务 `payload`、日志或主应用配置；需要持久化的认证信息应保存在 `YAYA_PROVIDER_DATA_DIR`。

## 开发与检查

前端：

```bash
npm run build
```

Rust 主 workspace：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

Direct Provider 是独立 workspace，需要单独检查：

```bash
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

更完整的代码修改约束和验证要求见 [AGENTS.md](AGENTS.md)。

## 发布

`.github/workflows/release.yml` 在 `v*` 标签或手动触发时构建：

- macOS（Apple Silicon 与 Intel）
- Linux
- Windows
- Android APK

所有构建成功后，workflow 会汇总产物、生成 `SHA256SUMS.txt` 并创建或更新对应的 GitHub Release。推送 `v*` 标签时使用该标签；手动触发时必须填写 Release tag。

额外 Provider、私有仓库访问和平台签名均通过可选 GitHub Actions Secret 注入：

- Provider：`PROVIDERS_REPO`、`PROVIDERS_GIT_TOKEN`、`BUNDLE_CONFIG_JSON`
- Android：`ANDROID_KEYSTORE_B64`、`ANDROID_KEYSTORE_PASSWORD`、`ANDROID_KEY_ALIAS`，key 密码不同时另设 `ANDROID_KEY_PASSWORD`
- macOS Developer ID 签名：`APPLE_CERTIFICATE`、`APPLE_CERTIFICATE_PASSWORD`、`APPLE_SIGNING_IDENTITY`、`KEYCHAIN_PASSWORD`
- macOS 公证：`APPLE_API_ISSUER`、`APPLE_API_KEY`、`APPLE_API_KEY_B64`

没有 Provider Secret 时仍会发布只包含 Direct Provider 的版本。Android 未配置签名时产物不带 release 签名；macOS 未配置完整签名和公证 Secret 时回退到 ad-hoc 签名。Secret 的生成方式、证书类型和平台要求见 [Release 签名配置](docs/release-signing.md)。

## 许可证

本项目采用 [GNU General Public License v3.0 only](https://www.gnu.org/licenses/gpl-3.0.html)。
