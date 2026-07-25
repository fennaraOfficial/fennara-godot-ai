<!-- fennara-i18n: locale=zh-CN source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# 脚本

<!-- fennara-doc-nav:start -->
[English](../../../../scripts/README.md) · **简体中文** · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

此目录包含本地开发、包预览和发布工作流共享的仓库自动化。

除非帮助文本另有说明，脚本应小巧、确定且可安全地从仓库根目录运行。它们不应把用户特定状态写到仓库外。

<a id="version-scripts"></a>
## 版本脚本

- `set-version.mjs`：更新仓库 `VERSION`、插件 `VERSION`、本地 Rust 工作区元数据、锁文件包版本和 C++ 插件版本常量。
- `check-version.mjs`：验证这些版本化文件仍保持同步。

在 CI 和发布打包前运行 `check-version.mjs`。只有在有意更改 Fennara 版本时才使用 `set-version.mjs`。

<a id="packaging-scripts"></a>
## 打包脚本

- `package-preview.mjs`：同步已提交的插件负载，并在 GDExtension 和本地 Rust 二进制文件构建后组装各平台预览归档。
- `package-addon-all.mjs`：把各平台插件部分合并为最终的全平台插件归档。
- `release-policy.mjs`：定义各发布轨道最低兼容的已发布 CLI。
- `write-release-manifest.mjs`：根据发布资源写入 `fennara-release-manifest-v<version>.json`，并验证每个引用的 SHA-256。

这些脚本使用 `.package-preview/` 临时暂存，并把 ZIP 输出写入仓库根目录 `dist/`。这些输出被忽略，不应提交。

打包脚本必须保持插件负载较小。特别是 `libcef.so` 和 `fennara_cef_helper` 等 Linux CEF 运行时文件不得打包进 `fennara-addon-*`。CEF 只安装一次，位于用户共享的 Fennara 应用数据目录中。

<a id="staging-release-scripts"></a>
## 预发布脚本

- `write-staging-candidate.mjs`：为一个拉取请求和冻结的源提交创建精确预发布身份。
- `validate-staging-build.mjs`：发布前检查插件部分、平台归档、组装后的插件、发布清单和 Linux CEF。
- `smoke-public-release.mjs`：通过未认证浏览器 URL 下载每个已发布候选，并在推进频道前验证可信资源及清单哈希。
- `write-staging-pointer.mjs`：对精确发布清单做哈希后写入小型 PR 专属指针。
- `check-staging-channel-advance.mjs`：拒绝向后或冲突的频道移动。
- `validate-staging-publish-bundle.mjs`：不执行候选代码，重新验证最终资源包。
- `verify-published-assets.mjs`：比较预期和已下载 GitHub Release 资源的名称及 SHA-256。

这些脚本支持 `.github/workflows/staging-release.yml`。候选构建作业没有发布凭据。只有受信任的最终作业可以发布，并且仅在下载和验证精确发布后推进每个频道的 Git ref。

<a id="linux-cef-scripts"></a>
## Linux CEF 脚本

- `prepare-linux-cef-sdk.mjs`：下载并解压用于构建 Linux CEF 桥接的固定官方 Linux x64 CEF SDK。
- `prepare-linux-cef-runtime.mjs`：暂存独立 Linux CEF 运行时 ZIP、验证必需文件、在 Linux 上剥离暂存的 ELF 二进制文件，并可为发布打包写入生成的 `local/webview-runtimes/linux-cef.json` 清单。
- `check-linux-cef-runtime-release.mjs`：验证发布资源包含启用清单指定的 CEF 运行时 ZIP，且 SHA-256 一致。
- `cef/linux/fennara_cef_helper.cpp`：从 CEF SDK 构建运行时辅助程序时使用的小型 CEF 辅助进程源代码。

CEF 脚本只操作复制后的暂存文件，不得修改下载或源 CEF SDK 树。

<a id="development-tests"></a>
## 开发测试

- `test-run-scene-edit-script-inspect.mjs`：在 `temp/` 中创建忽略的 Godot 冒烟测试项目，并针对已构建的编辑器 GDExtension 验证导入 `PackedScene` 检查、只读上下文防护、源缺失失败和不保存行为。

<a id="documentation-localization"></a>
## 文档本地化

- `sync-doc-navigation.mjs`：添加源哈希、稳定锚点和精简的同页语言选择器，不翻译正文。
- `check-doc-i18n.mjs`：验证完整 locale 覆盖、源新鲜度、导航、锚点、Markdown 结构、受保护代码、URL 和链接。
- `doc-i18n-lib.mjs`：负责共享 locale 清单、源规范化、导航渲染和结构辅助逻辑。

运行：

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Locale 和文档集合在 `docs/i18n/languages.json` 中声明。英文仍是规范源。翻译正文必须根据英文源直接编写，不能由这些脚本生成。

常规同步会更新导航和稳定锚点，但保留现有源哈希。直接更新某个已更改
英文页面的全部九种翻译后，请有意只刷新该规范源：

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

可以为多个已审阅的源重复使用此选项。不要确认翻译正文尚未更新的源。
CI 会在运行完整翻译验证器前运行 `sync-doc-navigation.mjs --check`。

<a id="ui-sync"></a>
## UI 同步

- `sync-chat-ui.mjs`：把 `ui/chat/` 复制到 `godot_demo/addons/fennara/dist/`。

`godot_demo/addons/fennara/dist/` 有意提交，因为发布插件 ZIP 必须包含已构建聊天 WebView。请在 `ui/chat/` 中更改，运行同步脚本，然后一并提交源文件和生成的插件资源。

<a id="runtime-sync"></a>
## 运行时同步

- `sync-runtime.mjs`：把 `runtime/` 复制到 `godot_demo/addons/fennara/runtime/`。

`godot_demo/addons/fennara/runtime/` 有意提交，因为发布插件 ZIP 必须包含 Godot 端运行时辅助脚本。请在 `runtime/` 中更改，运行同步脚本，然后一并提交源文件和生成的插件资源。

<a id="guidance-sync"></a>
## 指南同步

- `sync-guidance.mjs`：把 `local/templates/` 中的精简指南和按需知识页面复制到 `godot_demo/addons/fennara/ai/`，与 `fennara install` 和 `fennara update` 写入用户项目的文件一致。

`godot_demo/addons/fennara/ai/` 有意提交，因为演示插件需要映射已安装插件布局。请在 `local/templates/` 中更改，运行同步脚本，然后一并提交源文件和生成的插件指南。

<a id="boundaries"></a>
## 边界

- 脚本可以创建 `.package-preview/` 和根目录 `dist/` 输出。
- 只有明确职责如此的脚本可以更新已提交的生成负载，例如 `sync-chat-ui.mjs`、`sync-runtime.mjs`、`sync-guidance.mjs` 或 `set-version.mjs`。
- 脚本不得把 Godot 编辑器缓存、本地应用数据安装、下载的发布资源或 VM 测试输出写进跟踪的源目录。
