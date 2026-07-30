# Release 签名配置

发布工作流从 GitHub Actions Repository Secrets 读取签名材料。进入仓库的
**Settings → Secrets and variables → Actions → New repository secret** 添加下列值。
证书、私钥、密码和 keystore 不得提交到仓库。

应用使用 Application ID / Bundle ID `com.zhi.yaya`。

## Android APK

必需 Secrets：

| Secret | 内容 |
| --- | --- |
| `ANDROID_KEYSTORE_B64` | `.jks` keystore 的单行 Base64 |
| `ANDROID_KEYSTORE_PASSWORD` | keystore 密码 |
| `ANDROID_KEY_ALIAS` | 签名 key 的 alias |
| `ANDROID_KEY_PASSWORD` | key 密码；仅在它与 keystore 密码不同时设置 |

首次创建 keystore：

```bash
keytool -genkey -v \
  -keystore upload-keystore.jks \
  -storetype JKS \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -alias upload
```

在 macOS 生成单行 Base64：

```bash
base64 -i upload-keystore.jks | tr -d '\n' | pbcopy
```

请离线备份原始 keystore、alias 和密码。应用更新必须继续使用同一个签名 key。
工作流构建后会使用 Android SDK 的 `apksigner verify` 验证每个 release APK。

## macOS DMG

当前发布的是网站直接下载的 DMG，因此使用 **Developer ID Application** 证书，
并通过 App Store Connect API Key 完成 Apple 公证和 stapling。

全部必需 Secrets：

| Secret | 内容 |
| --- | --- |
| `APPLE_CERTIFICATE` | Developer ID Application `.p12` 的单行 Base64 |
| `APPLE_CERTIFICATE_PASSWORD` | 导出 `.p12` 时设置的密码 |
| `APPLE_SIGNING_IDENTITY` | 完整签名身份，例如 `Developer ID Application: Name (TEAMID)` |
| `KEYCHAIN_PASSWORD` | CI 临时 Keychain 的随机强密码 |
| `APPLE_API_ISSUER` | App Store Connect API Issuer ID |
| `APPLE_API_KEY` | App Store Connect API Key ID |
| `APPLE_API_KEY_B64` | 对应 `AuthKey_<KEY_ID>.p8` 文件的单行 Base64 |

在 macOS 查看完整签名身份：

```bash
security find-identity -v -p codesigning
```

生成证书和 API 私钥的 Secret 内容：

```bash
base64 -i DeveloperIDApplication.p12 | tr -d '\n' | pbcopy
base64 -i AuthKey_KEYID.p8 | tr -d '\n' | pbcopy
```

API Key 在 App Store Connect 的 **Users and Access → Integrations** 中创建。私钥
只能下载一次，应同时离线备份。工作流会将证书导入临时 Keychain，将 `.p8`
写入 runner 临时目录，构建时让 Tauri 完成签名、公证和 stapling，随后运行：

```bash
codesign --verify --deep --strict YAYA.app
xcrun stapler validate YAYA.dmg
```

若 macOS 的七个 Secret 全部缺失，工作流会回退到 ad-hoc 签名；只配置其中一部分
会直接失败并列出缺失配置，避免发布看似成功但未正确签名或公证的 DMG。

## 触发发布

签名 Secret 配好后，可以推送 `v*` 标签，或在 Actions 页面手动运行 Release
workflow。手动运行必须填写 Release tag。不要在来自 fork 的 workflow 中传递
签名 Secret。
