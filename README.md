<div align="center">
  <img src="public/yaya.png" width="112" alt="YAYA 图标">
  <h1>YAYA</h1>
  <p>一个跨平台下载器。</p>
</div>

YAYA（Yet Another Yume Archive，娅娅）希望把不同来源的下载任务放进同一个简单的工作流里。

> 娅娅目前处于早期开发阶段，功能和 Provider 协议仍可能变化。

## 主要功能

- 根据输入自动选择合适的 Provider
- 在下载前预览内容，并按 Provider 提供的选项进行配置
- 支持单个任务和批量任务
- 支持排队、并发下载、暂停、继续、重试和取消
- 实时显示进度、速度、当前阶段与输出文件
- 保留任务历史，应用重启后仍可查看和继续处理
- 支持明暗模式与主题色
- 可运行在桌面端、移动端或 Web

仓库默认内置直链 Provider，可以下载普通 HTTP/HTTPS 文件。其他内容来源需要安装或编译对应的 Provider；主仓库不包含任何特定内容站点的实现。

## 使用方式

1. 打开“新建下载”，粘贴地址。
2. 下载

桌面端可以选择保存目录。Web 端会在任务完成后通过浏览器交付文件。支持认证的 Provider 可以在“插件”页面完成登录。

## 从源码运行

需要准备：

- Node.js `^20.19.0` 或 `>=22.12.0`
- npm
- 当前稳定版 Rust 工具链
- 目标平台所需的 Tauri 2 系统依赖

先安装前端依赖：

```bash
npm ci
```

### 桌面端

启动开发版本：

```bash
npm run tauri dev
```

构建当前平台的安装包：

```bash
npm run tauri build
```

### Web 端

构建前端并在 `http://127.0.0.1:9527` 启动本地服务：

```bash
npm run web
```

需要前端热更新时，分别运行后端和 Vite：

```bash
# 终端 1
cargo run -p yaya-web

# 终端 2
npm run dev
```

然后访问 Vite 输出的地址，默认是 `http://localhost:1420`。

Web 服务默认只监听本机，定位为单用户应用。若要修改监听地址，可设置 `YAYA_WEB_ADDR`；若要修改数据目录，可设置 `YAYA_DATA_DIR`。

### Android 与 iOS

```bash
npm run android:dev
npm run android:build

npm run ios:dev
npm run ios:build
```

移动端开发还需要 Android SDK、Java 17、NDK，或 macOS、Xcode、CocoaPods 等相应平台环境。

## Provider

Provider 是 YAYA 的下载能力扩展。它负责识别输入、展示可选内容和参数，并执行实际下载；YAYA 负责统一呈现和管理任务。

Provider 有两种使用方式：

- **内嵌 Provider**：随 YAYA 一起编译，桌面端和移动端都可使用。
- **外部 Provider**：以独立程序安装，无需重新编译 YAYA，适用于桌面端和 Web 主机。

如果你想开发或分发 Provider，请阅读 [Provider 协议](docs/provider-protocol.md)。该文档包含包结构、Manifest、进程通信和数据模型等完整说明。

## 参与开发

前端使用 Vue 3、Pinia 和 UnoCSS，应用核心与宿主使用 Rust、Tauri 2 和 Axum。

常用检查命令：

```bash
npm run build
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

直链 Provider 是独立 workspace，修改后还需要单独检查：

```bash
cd providers/direct
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## License

GPL-3.0-only。
