<!-- fennara-i18n: locale=zh-CN source=docs/release.md sha256=23e966649d82728c9ab9802dd2957bb64c1b13fd3e7ae44bd513835bc063ff77 -->
<a id="release-process"></a>
# 发布流程

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../release.md) · **简体中文** · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../release.md)
<!-- fennara-doc-nav:end -->

发布需要手动进行。不要从拉取请求工作流中发布。

> [!IMPORTANT]
> 从 `main` 运行发布，保持 `VERSION` 与工作流输入完全一致，并且
> 明确决定该发布是否需要提高最低 CLI 版本。

<a id="release-at-a-glance"></a>
## 发布概览

| 步骤 | 结果 |
| --- | --- |
| 准备并合并版本更改 | 仓库版本规范源保持一致 |
| 运行 Package Preview | 构建具有发布形态的工件，但不发布 |
| 检查预览 | 验证归档、清单、哈希和 Linux CEF 布局 |
| 从 `main` 运行 Release | 发布标签和 GitHub Release |
| 冒烟测试安装和更新 | 验证公开的用户流程 |

<a id="versioning"></a>
## 版本控制

`VERSION` 是规范源。

发布工具接受 SemVer 值。稳定版使用 `X.Y.Z`。预发布
候选使用隔离的拉取请求预发布版本，例如
`1.2.3-pr.101.2`，其中 `pr-101` 是预发布渠道，`2` 是该
渠道的候选编号。

提升仓库版本：

```bash
node scripts/set-version.mjs X.Y.Z
```

该脚本更新：

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- 插件版本常量
- `local/` 下的 Rust 工作区软件包版本
- `local/Cargo.lock`

插件还包含 `addons/fennara/release.json`。正常执行上述命令时会
自动写入稳定版身份。预发布构建工作区使用明确的身份输入：

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

预发布版本、渠道、源提交和精确发布标签必须
一致。没有此身份的预发布插件会被拒绝。在
`release.json` 出现之前的现有稳定版插件会继续默认为稳定版轨道。

检查版本同步：

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. 准备发布提交

1. 运行版本脚本。
2. 检查差异。
3. 运行与更改表面匹配的本地检查。
4. 将发布准备 PR 合并到 `main`。

常见检查：

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

对于 GDExtension 更改，在可能时还应在本地构建插件：

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. 运行 Package Preview

当打包发生更改，或你想进行试运行时，请在发布前使用它。

GitHub：

```text
Actions > Package Preview > Run workflow
```

该工作流构建 Windows、Linux 和 macOS 软件包，并上传临时
工件。它不会创建标签、GitHub Release 或 `latest`。

Package Preview 会充分贴近地复现 Release 中不发布的部分，以便
在合并前运行发布打包：

- 将无构建步骤的聊天 UI 和运行时辅助源码同步到插件载荷中
- 构建 Linux CEF 运行时 zip
- 写入生成的 Linux CEF 运行时清单
- 将该生成清单输入平台软件包构建
- 组装全平台插件归档
- 将本地或插件软件包重命名为由清单管理的发布资产名称
- 根据生成的清单验证 Linux CEF 运行时资产
- 写入 `fennara-release-manifest-v<version>.json`
- 上传一个 `fennara-package-preview-release-assets` 工件，其中包含
  具有发布形态的 zip 和清单

预览工件可用于在发布前检查 zip 内容和清单形态。
它们是 Actions 工件，不是公开发布资产。

<a id="3-run-release"></a>
## 3. 运行 Release

从 `main` 运行手动发布工作流：

```text
Actions > Release > Run workflow
```

输入：

```text
version: X.Y.Z
promote_latest: true
```

`version` 输入必须与 `VERSION` 匹配。

工作流会发布：

- `v<version>`
- 当 `promote_latest` 为 true 时，将 `v<version>` 标记为 GitHub Latest

发布工作流会在平台打包前准备 Linux CEF 运行时。
它下载固定版本的官方 CEF 139 Linux 最小 SDK，组装
独立的 `fennara-webview-cef-linux-x64-<cef-version>.zip`，剥离暂存的 ELF
二进制文件，写入生成且已启用的 `local/webview-runtimes/linux-cef.json`
清单，并将该清单输入 CLI 软件包。发布作业随后
验证发布资产包含生成清单所指定的精确 CEF zip，
并且其 SHA-256 匹配。它还会写入
`fennara-release-manifest-v<version>.json`，验证每个引用的资产和
哈希，并将该清单与发布一起上传。

拉取请求工作流不会发布版本。Package Preview 工作流
会创建具有发布形态的测试工件，包括清单和 Linux CEF
运行时载荷，使维护者可以在合并前对打包进行冒烟测试。Package
Preview 不是面向用户的发布渠道。

<a id="release-assets"></a>
## 发布资产

每个发布都应包含各平台 CLI 或本地运行时软件包，以及一个共享的全平台插件软件包。

| 目标 | 资产 |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| 所有平台 | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

软件包角色：

| 模式 | 角色 |
| --- | --- |
| `fennara-cli-*` | 仅包含单个平台 `fennara` CLI 的安装脚本载荷 |
| `fennara-release-local-*` | 单个平台的 MCP 和守护进程启动器，以及带版本运行时二进制文件 |
| `fennara-release-addon-v*` | 通过发布清单解析的带版本全平台插件 |
| `fennara-addon-latest.zip` | 用于文档和手动下载的稳定名称全平台插件别名 |
| `fennara-webview-cef-linux-x64-*` | 仅限 Linux 的共享 CEF 运行时，在 Fennara 应用数据中安装一次 |
| `fennara-release-manifest-v*` | 包含资产名称、SHA-256 值、安装原语和共享运行时的安装与更新计划 |

macOS 插件 GDExtension 当前尚未经过 Apple 公证。浏览器下载
和手动 Finder 解压可能会传播隔离元数据，并触发
macOS 验证通知。面向用户的安装文档必须
建议在 macOS 上使用 `fennara install`，解释手动 ZIP 的限制，并
告知受影响用户在通过 CLI 重新安装之前移除手动复制的插件。
发布验证不会把仅仅创建 ZIP 视为 macOS
签名或公证。

`fennara-release-local-*` 前缀可防止旧 CLI 静默绕过
由清单管理的软件包路径。

<a id="release-manifest"></a>
## 发布清单

从 0.3.0 开始，只要发布提供清单，`fennara install` 和 `fennara update` 就会优先使用
发布清单。清单记录：

- `schema_version`
- `version`
- `minimum_cli_version`
- 受支持的安装原语
- 各平台 CLI 和本地运行时资产及其 SHA-256 哈希
- 共享插件资产及其 SHA-256
- 平台专用的共享运行时资产，目前是 Linux CEF

`scripts/release-policy.mjs` 是
`minimum_cli_version` 的规范源。清单写入器在验证
发布身份后选择政策，因此 Stable、Package Preview 和 Staging 无法选择
彼此独立的值。正常的软件包布局或资产名称更改应由
清单数据处理，而不是更改外层 CLI。当某个发布需要更新的更新器移交、
清单模式、安装原语、自更新行为或
其他旧版已发布 CLI 无法安全执行的 CLI 能力时，请提高政策。

当 CLI 太旧时，`fennara update` 应使用清单的
按平台 `assets.cli` 条目先更新已安装的 CLI，然后使用
`--no-self-update` 恢复软件包更新。如果该发布或安装位置
不支持自更新，它应在安装软件包前失败，并
清楚指示重新运行 `install.sh` 或 `install.ps1`。

添加到清单模式 1 的可选发布身份不要求
提高最低 CLI 版本。旧的模式 1 客户端会忽略未知字段，而
支持预发布的客户端会在身份存在时验证它。未来
依赖了解渠道的激活或更新器移交的发布，必须
在发布前重新评估最低 CLI。

<a id="staging-identity-and-discovery-contract"></a>
## 预发布身份和发现契约

预发布渠道按拉取请求隔离：

| 值 | PR 101 示例 |
| --- | --- |
| 渠道 | `pr-101` |
| 候选版本 | `1.2.3-pr.101.2` |
| 精确发布 | `v1.2.3-pr.101.2` |
| 渠道引用 | `fennara-staging/pr-101` |
| 指针文件 | `fennara-staging-channel-pr-101.json` |

每个渠道的 Git 引用只包含一个指向精确
带版本发布的小型指针文件。发布二进制文件绝不会位于移动的渠道引用下。
CLI 可以使用内部版本请求
`channel:pr-101` 解析此指针，然后继续只使用精确版本。

因此 PR 101 和 PR 125 使用不同的发布标签和指针资产。
更新一个渠道无法重定向另一渠道上的测试者。发布
一个渠道绝不会更改稳定版 GitHub Latest 指定或另一个拉取
请求的渠道。

<a id="staging-candidate-workflow"></a>
## 预发布候选工作流

手动 **Staging Release** 工作流会根据打开拉取请求的当前
头部构建候选版本。从 `main` 运行它，并提供：

| 输入 | 含义 |
| --- | --- |
| `pull_request` | 要构建的打开拉取请求 |
| `base_version` | `X.Y.Z` 形式的计划稳定版本 |
| `candidate` | 此拉取请求递增的候选编号 |
| `source_commit` | 可选的完整 SHA，它必须仍然是拉取请求头部 |
| `publish` | 关闭表示仅工件验证，打开表示发布候选版本 |

工作流会在任何平台构建之前冻结拉取请求头部 SHA。
Windows、Linux 和 macOS 作业使用只读
权限检出该精确提交，不保留 Git 凭据，没有发布凭据，也
不能保存共享依赖缓存。它们可以恢复由可信默认分支工作流写入的兼容
SCons/godot-cpp 和 Cargo 缓存。
预发布使用仅恢复缓存操作，因此候选代码可以使用
可信构建输出，但无法为后续运行替换或污染缓存。
候选代码可以生成构建工件，但无法发布 GitHub
Release。

可信仓库脚本随后验证候选身份、精确归档
清单、插件内容、平台软件包布局、发布清单和每个
SHA-256 值。除非明确选择 `publish`，否则发布保持禁用。

启用发布时，可信最终作业会：

1. 将候选工件作为数据重新验证。
2. 创建草稿，上传每个资产，并将其发布为精确的
   `v<exact-version>` 预发布版本，而不更改 GitHub Latest。
3. 下载已发布资产，并比较它们的名称和哈希。
4. 拒绝向后或冲突的渠道更改。
5. 最后通过有条件的 GitHub Contents API 写入，更新小型 `fennara-staging/pr-<number>` 指针引用。
6. 下载活动指针并验证其精确内容。

同一个拉取请求的运行会被串行化。不同的拉取请求使用独立的
并发组、发布标签和指针引用。重试同一个
候选版本会验证现有的精确发布，而不是把文件混入其中。
该工作流绝不会创建、上传到或提升稳定版 GitHub Latest。

稳定版发布不使用字面上的 `latest` 标签或发布。Release
工作流将精确的 `v<version>` 发布创建为草稿，逐字节验证上传的
资产，将其发布为可变发布，并在 `promote_latest` 为 true 时将该精确
发布标记为 GitHub Latest。安装程序和稳定版 CLI
发现会解析 GitHub 的 Latest Release API 端点。

当仓库发布不可变性处于禁用状态时，稳定版和预发布版都是可变的。
两个工作流都会在完成发布或推进预发布渠道之前，验证发布元数据和下载的资产字节。
资产发布使用具有 contents 写入权限的作业范围 `GITHUB_TOKEN`。

发布政策目前要求稳定版清单使用 CLI `0.3.12`，
预发布版清单使用 CLI `0.3.8`。稳定版发现不再解析
已停用的 `latest` 标签。稳定版 `0.3.12` 要求修正后的更新验证、
版本切换预检和 Windows 操作日志处理。诸如 `0.3.12-pr.123.1` 的预发布
候选版本在 SemVer 下低于稳定版 `0.3.12`，因此其最低版本必须保持低于
候选版本，首次运行设置才能安装候选 CLI。不要
仅根据清单模式兼容性更改任一最低版本。

共享插件 zip 包含 `godot_demo/addons/fennara/fennara.gdextension` 引用的每个已构建 GDExtension 二进制文件。Godot 会加载与用户操作系统匹配的库，并忽略其他库。

Linux CEF webview 运行时载荷与插件归档分离。发布
打包会生成已启用的运行时清单，并将该数据嵌入
`fennara-release-manifest-v<version>.json`。CLI 会将匹配的 CEF
载荷安装一次到用户的 Fennara 应用数据目录下：

```text
webview/cef/linux-x64/<cef-version>/
```

不要将 `libcef.so`、CEF 辅助可执行文件、CEF 资源或语言区域包
放入 `fennara-addon-*`。Package Preview 会构建一个用于
测试的独立 CEF 工件，并写入与 Release 所用相同类型的生成运行时清单，
但发布仍是唯一面向用户的发布资产来源。

Linux GDExtension 构建还需要官方 CEF SDK 包装器源码，但不需要
插件中的 CEF 运行时文件。CI 运行：

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

并将提取的目录作为 `FENNARA_CEF_ROOT` 传给 SCons。SCons 使用
`FENNARA_CEF_ROOT/libcef_dll/`，根据固定版本的 CEF 139 C++
包装器构建小型 `libfennara_linux_cef_bridge.so` 插件库。SDK 下载会经过版本和哈希检查，
因为生成的包装器源码必须匹配运行时 CEF ABI。该桥接与
插件一起打包，`libcef.so`、资源、语言区域包和 `fennara_cef_helper` 则保留在
独立的共享 CEF 运行时中。

如果在插件归档中发现 CEF 运行时文件，软件包脚本会失败。
运行时资产名称必须是：

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

zip 解压后，必需文件必须位于其根目录：

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

当选定的 CEF 发行版中存在可选 CEF 运行时文件时，也应包含它们，例如
`chrome-sandbox`、`libEGL.so`、
`libGLESv2.so`、`libvk_swiftshader.so`、`libvulkan.so.1`、
`vk_swiftshader_icd.json`、`snapshot_blob.bin` 和其他 `locales/*.pak`。

要从维护者选定的 CEF 二进制树手动组装运行时 zip：

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

在 Linux 上，该脚本根据
`fennara-cpp/vendor/cef/` 中的官方 CEF 头文件，从
`scripts/cef/linux/fennara_cef_helper.cpp` 构建 `fennara_cef_helper`。在其他操作系统上，请先在 Linux 上构建该辅助程序，
然后传入 `--helper /path/to/fennara_cef_helper`。使用 `--dry-run` 可在写入 zip 前检查
选定文件。

脚本输出 SHA-256 后，更新
`local/webview-runtimes/linux-cef.json`：

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

对于正常发布，工作流会使用 `--write-manifest` 自动写入 Linux CEF 运行时清单，
然后 `scripts/write-release-manifest.mjs`
将运行时字段复制到 `fennara-release-manifest-v<version>.json`。不要
手动启用已检入的占位清单，除非有意
调试手动运行时资产路径或旧版回退行为。如果生成的
清单数据指向缺失的资产，或其 SHA-256 不
匹配，Release 工作流和 Linux `fennara install` / `fennara update` 会
清楚地失败。

CLI 必须原子发布 Linux CEF 运行时更新：在暂存目录中解压并验证，
只在必需文件存在后写入运行时标记，然后发布版本目录，
并使用临时文件重命名更新 `current.json`。已安装的 `fennara-cef-runtime.json` 标记必须使用 `"runtime": "cef"` 标识
原生加载器契约。安装和更新会修复只包含 `"kind": "cef"` 的匹配旧版标记，
而无需再次下载 CEF 载荷。正在运行的编辑器继续使用它们已经
加载的运行时。

CLI 会嵌入 `local/templates/` 中生成的项目指导模板。
发布打包构建 CLI 时，这些模板会与其余 CLI 代码一起编译进二进制文件。

<a id="what-latest-means"></a>
## `latest` 的含义

GitHub 的 Latest Release 指针选择正常
安装和更新流程使用的带版本发布。Fennara 不会创建或移动字面上的 `latest` 标签。

- `install.ps1` 和 `install.sh` 默认获取最新 CLI 资产。
- `fennara update` 默认通过 GitHub 的 Latest Release 端点获取发布清单，在需要时自更新已安装的 CLI，然后从中解析本地或插件或共享运行时资产。
- 编辑器内更新会在关闭前暂存经过验证的资产，在替换前重新检查完整的暂存插件摘要，保留先前的插件、启动器和运行时清单，直到激活验证成功，并要求重新打开的 GDExtension 握手后才删除回滚数据。
- `fennara install` 默认通过 GitHub 的 Latest Release 端点获取发布清单，然后从中解析本地或插件或共享运行时资产。
- Godot 插件更新检查会与 GitHub 的最新发布进行比较。

只有在发布不应成为默认用户安装的版本时，才使用 `promote_latest: false`。

安装程序和发布下载应输出发布元数据、资产下载、
解压、安装和验证步骤。网络获取应使用有界
超时，使 GitHub 或 CDN 卡住时以诊断失败，而不是看似冻结。
在 Windows 上，`install.ps1` 必须在
输出成功前检查 CLI 验证退出代码。退出代码 `-1073741515`（`0xC0000135`）表示 CLI 可执行文件
已写入，但 Windows 因缺少所需 DLL 而无法启动它。
请告知用户安装 Microsoft Visual C++ Redistributable 2015-2022 x64，然后
重新运行 `fennara --version`、`fennara doctor` 和 `fennara install`。
下载 URL：`https://aka.ms/vs/17/release/vc_redist.x64.exe`。

<a id="smoke-test-after-release"></a>
## 发布后冒烟测试

在 Windows 上：

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

在 Godot 项目中：

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

检查项目是否收到：

```text
AGENTS.md
addons/fennara/ai/
```

在 Godot 中打开项目，然后向 MCP 应用询问：

```text
使用 Fennara MCP 运行 fennara_status，并告诉我连接的是哪个 Godot 项目。
```

更新测试：

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## 规则

- Release 工作流只能从 `main` 运行。
- 发布版本输入必须与 `VERSION` 匹配。
- 拉取请求工作流可以构建和上传测试工件，但不得发布版本。
- 保持预期的普通用户发布被指定为 GitHub Latest。
- 除非维护者有意决定替换损坏的发布，否则不要重写已发布的发布标签。
