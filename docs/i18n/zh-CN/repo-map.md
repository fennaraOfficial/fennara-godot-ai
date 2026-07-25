<!-- fennara-i18n: locale=zh-CN source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# 仓库地图

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../repo-map.md) · **简体中文** · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../repo-map.md)
<!-- fennara-doc-nav:end -->

这是供在此仓库中工作的贡献者和编程智能体使用的快速地图。

<a id="find-the-right-area"></a>
## 找到正确区域

| 更改 | 主要位置 |
| --- | --- |
| 用户设置或 CLI 行为 | `local/crates/fennara-cli/` |
| 外部 MCP 协议或模式 | `local/crates/fennara-mcp/`、`local/schemas/tools/` |
| 内置聊天或守护进程行为 | `local/crates/fennara-daemon/` |
| Godot 编辑器集成 | `fennara-cpp/` |
| 聊天 UI | `ui/chat/` |
| 运行时辅助脚本 | `runtime/` |
| 打包或发布 | `scripts/`、`.github/workflows/` |
| 用户文档 | `README.md`、`docs/` |

<a id="top-level"></a>
## 顶层

| 路径 | 负责内容 |
| --- | --- |
| `.github/` | 拉取请求模板、议题模板和 GitHub Actions 工作流。 |
| `docs/` | 项目文档、设置指南、架构说明、示例、演示和发布说明。 |
| `docs/i18n/` | 语言区域清单和完整的翻译文档树。 |
| `fennara-cpp/` | C++ Godot GDExtension 源码和 SCons 构建入口点。 |
| `godot_demo/addons/fennara/` | 复制到用户项目中的可安装 Godot 插件载荷。 |
| `local/` | Rust CLI、MCP 服务器、守护进程、模式和本地运行时代码。 |
| `media/` | 文档使用的图像和公开媒体。 |
| `runtime/` | `runtime_session` 和 `runtime_script` 使用的 Godot 运行时辅助脚本源码。 |
| `scripts/` | 版本、打包和发布辅助脚本。 |
| `ui/chat/` | 可选编辑器内网页聊天 UI 的源码。 |
| `local/templates/` | 由 `fennara install` 写入 Godot 项目并由 `fennara update` 刷新的精简项目指南和按需 AI 知识页面。 |
| `local/webview-runtimes/` | 安装到共享 Fennara 应用数据中的外部 webview 运行时清单和配置文件，例如 Linux CEF 载荷。 |
| `install.ps1` / `install.sh` | 从 GitHub 发布安装 Fennara CLI 的引导脚本。 |
| `VERSION` | 版本规范源。 |
| `README.md` | 简短的面向用户概述和快速入门。 |
| `docs/README.md` | 面向任务的文档索引。 |
| `docs/setup.md` | 面向用户的插件优先设置、聊天先决条件、MCP 连接、更新流程和故障排除。 |
| `docs/cli.md` | 终端命令参考、CLI 负责的安装或更新行为、恢复、诊断、应用数据布局和自动化指南。 |
| `docs/telemetry.md` | 匿名活动载荷、应用数据状态、传递行为、月活定义和退出控制。 |
| `CONTRIBUTING.md` | 贡献规则。 |
| `SECURITY.md` | 安全问题报告政策。 |
| `LICENSE.md` | 项目许可证。 |

<a id="local-rust-packages"></a>
## 本地 Rust 包

| 路径 | 负责内容 |
| --- | --- |
| `local/crates/fennara-cli/` | `fennara` 命令：安装、更新、CLI 自更新、doctor、操作诊断、webview 先决条件检查、C# 支持、MCP 应用设置和生成项目指导。 |
| `local/crates/fennara-cli/src/operation.rs` | 公开的安装或更新操作协调器、阶段和 CLI 移交入口点。 |
| `local/crates/fennara-cli/src/operation/` | 聚焦的操作日志、持久存储、诊断脱敏和测试模块。 |
| `local/crates/fennara-cli/src/project_addon.rs` | 现有项目插件版本和当前平台 GDExtension 库验证。 |
| `local/crates/fennara-cli/src/prepare_export.rs` | 不含插件的 CI 导出准备，在 Godot 启动前仅移除 Fennara 的持久化运行时自动加载项。 |
| `local/crates/fennara-cli/src/release_identity.rs` | 稳定版或预发布版插件身份、精确发布选择器、拉取请求渠道验证和旧版稳定版兼容。 |
| `local/crates/fennara-cli/src/release_channel.rs` | 按渠道的预发布指针验证，以及解析到精确的带版本发布。 |
| `local/crates/fennara-cli/src/release_manifest.rs` | 发布清单解析、资产哈希验证、身份绑定和平台软件包选择。 |
| `local/crates/fennara-cli/src/release_version.rs` | 清单和发布选择共用的 CLI SemVer 解析及优先级。 |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | 采用现有完整插件的精确版本，而不替换项目插件文件。 |
| `local/crates/fennara-cli/src/daemon_setup.rs` | 安装和 doctor 共用的守护进程健康检查、精确版本就绪检查和启动。 |
| `local/crates/fennara-cli/tests/operation_failures.rs` | 进程级故障、持久诊断、脱敏和故障关闭操作日志测试。 |
| `local/crates/fennara-cli/src/diagnostics.rs` | 面向用户访问最新或指定名称的净化操作报告。 |
| `local/crates/fennara-mcp/` | 本地 stdio MCP 服务器和工具模式转发。 |
| `local/crates/fennara-daemon/` | 用于运行时会话和 Godot 桥接工作的本地守护进程。 |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | 匿名每日活跃调度器、有界队列、HTTP 传递和守护进程生命周期集成。 |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | 随机安装身份验证、原子应用数据持久化、每日回执状态和退出后清理。 |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | 内置聊天批准模式、工具风险分类、权限决定和待处理批准请求类型。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | 由守护进程负责的内置聊天 `exec_command` 实现：shell 检测、cwd 验证、进程生成、超时或进程树终止、输出捕获、结果工件日志和结果格式化。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | 内置聊天上下文压缩规划器：精确尾部保护、OpenCode 风格的旧工具结果压力修剪、摘要分块选择或存储或重放、摘要提示词序列化、token 预算和占位符渲染。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | 内置聊天 PromptBuilder 和生成的运行时环境上下文。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | 仅限本地的内置聊天追踪记录器、SQLite 事件行、保留策略和调试查询辅助功能。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | 内置聊天提供方运行时原语、目录或解析、上下文预检钩子、规范化流或错误类型，以及面向 OpenAI、Anthropic、OpenRouter、NVIDIA、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、自定义端点、Ollama 或本地和 LM Studio 的 OpenAI 兼容或 Anthropic 兼容适配器。 |
| `local/schemas/tools/` | 共享工具 JSON 模式。外部 MCP 服务器和内置聊天各自嵌入其允许的子集。 |
| `local/webview-runtimes/linux-cef.json` | 用于生成发布清单、doctor 输出和旧版回退的 Linux CEF 运行时占位或生成清单。它记录共享应用数据布局和归档元数据，而不把 CEF 放进插件 zip。 |
| `local/Cargo.toml` | Rust 工作区配置。 |
| `local/Cargo.lock` | 锁定的 Rust 依赖图。 |

<a id="gdextension-source"></a>
## GDExtension 源码

| 路径 | 负责内容 |
| --- | --- |
| `fennara-cpp/SConstruct` | GDExtension 构建入口点。 |
| `fennara-cpp/include/` | 公开 C++ 头文件。 |
| `fennara-cpp/src/` | C++ 实现。 |
| `fennara-cpp/src/setup/` | 原生首次运行设置状态、发布清单 CLI 引导、哈希验证、CLI 启动和持久操作进度读取器。 |
| `fennara-cpp/src/release/version.cpp` | 发布或更新发现使用的原生 SemVer 验证和优先级。 |
| `fennara-cpp/src/release/identity.cpp` | 打包的稳定版或预发布版身份验证和旧版稳定版兼容。 |
| `fennara-cpp/src/release/discovery.cpp` | GitHub Latest 和隔离预发布渠道更新发现。 |
| `fennara-cpp/src/update/` | 精确目标更新协调、持久回执发现、关闭或安装移交以及恢复 UI 状态。 |
| `fennara-cpp/src/ui/setup_panel.cpp` | 不依赖 webview 的首次运行设置面板，带进度、重试、日志和净化报告操作。 |
| `fennara-cpp/vendor/cef/` | Linux OSR 桥接使用的官方 CEF 139 头文件快照。运行时二进制文件保留在插件之外。 |
| `fennara-cpp/src/ui/webview_host*` | 原生编辑器内聊天 webview 主机和平台后端。 |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | 仅限 Linux 的共享 CEF 运行时发现、标记验证和动态 `libcef.so` 加载器基础。 |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | 仅限 Linux 的 CEF 屏幕外渲染表面、Godot 输入转发、桥接 ABI 加载和 Godot 纹理更新，供内部聊天 webview 使用。 |
| `fennara-cpp/src/ui/linux_cef_bridge/` | 由固定版本的官方 CEF 139 `libcef_dll_wrapper` 源码和 Fennara CEF OSR 适配器构建的小型 Linux 专用桥接库。加载外部 `libcef.so` 运行时后，主 GDExtension 会对它调用 dlopen。 |
| `fennara-cpp/src/tools/` | 面向 Godot 的工具实现。 |
| `fennara-cpp/src/lsp/` | 脚本诊断和语言服务器辅助功能。 |
| `fennara-cpp/src/csharp/` | 仅构建的 C# 项目选择、后台准备、隔离诊断和运行时预检。 |
| `fennara-cpp/src/runtime/` | 工具使用的原生运行时支持，包括运行时场景预检、脚本诊断和调试器快照。 |
| `fennara-cpp/godot-cpp/` | Godot C++ 绑定子模块。 |

<a id="addon-payload"></a>
## 插件载荷

| 路径 | 负责内容 |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Godot GDExtension 注册文件。 |
| `godot_demo/addons/fennara/VERSION` | 插件包版本。 |
| `godot_demo/addons/fennara/release.json` | 打包的稳定版或预发布版身份，包括精确版本、发布标签、渠道和预发布源提交。 |
| `godot_demo/addons/fennara/bin/` | 构建的平台库。 |
| `godot_demo/addons/fennara/dist/` | 编辑器内聊天 webview 使用的打包网页 UI 资产。 |
| `godot_demo/addons/fennara/runtime/` | 插件内附带的 `runtime/` 同步打包副本。 |
| `godot_demo/tests/first_run_setup_test.gd` | 无头原生首次运行设置状态和确定性故障测试。 |
| `godot_demo/tests/export_plugin_test.gd` | 无头原生导出排除和自动加载项恢复回归测试。 |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | 无头原生截图参数契约回归测试。 |
| `godot_demo/tests/image_sheet_test.gd` | 无头共享截图或运行时图像表组合回归测试。 |
| `godot_demo/tests/runtime_image_context_test.gd` | 无头运行时原始帧、图像表和任意 Image 输出回归测试。 |

<a id="runtime-helper-source"></a>
## 运行时辅助源码

| 路径 | 负责内容 |
| --- | --- |
| `runtime/game_capture_helper.gd` | 由 GDExtension 为场景会话和运行时检查加载的运行时辅助入口点。 |
| `runtime/image_label.gd` | 捕获后标记到合成 Image 单元格上的精简确定性标签。 |
| `runtime/image_sheet.gd` | 截图和运行时脚本上下文使用的共享纯 Image 图像表合成。 |
| `runtime/screenshot_script_context.gd` | 为原生捕获上下文添加共享 Image 合成功能的公开截图脚本外观。 |
| `runtime/runtime_script_context.gd` | 暴露给 `runtime_script` 的公开 `ctx` 辅助表面，包括原始帧、Image 合成或输出、等待、输入、快照、条件、射线检测和点击。 |
| `runtime/runtime_input_driver.gd` | 面向按键、鼠标按钮、绝对鼠标移动、相对鼠标移动、修饰键和输入清理的底层运行时输入事件驱动程序。 |
| `runtime/runtime_node_snapshot.gd` | 运行时节点查找、存在性检查、防陈旧引用快照、属性读取和子项摘要。 |
| `runtime/runtime_physics_query.gd` | 带精简命中回执的运行时 2D 或 3D 精确射线检测和扫描辅助功能。 |
| `runtime/runtime_query_utils.gd` | 用于向量强制转换、安全节点或路径解析、对象身份和通用目标匹配的共享运行时查询工具。 |
| `runtime/runtime_capture_store.gd` | 运行时会话、脚本和环境检查使用的运行时捕获或状态工件写入器。 |
| `runtime/runtime_check_runner.gd` | 用于非交互式场景执行规范的运行时检查运行器。 |

<a id="scripts-and-workflows"></a>
## 脚本和工作流

| 路径 | 负责内容 |
| --- | --- |
| `scripts/set-version.mjs` | 更新整个仓库中带版本的文件。 |
| `scripts/check-version.mjs` | 检查版本同步。 |
| `scripts/release-identity.mjs` | 用于 SemVer 发布身份和按 PR 预发布指针的共享 Node 验证及生成。 |
| `scripts/release-policy.mjs` | 稳定版和预发布版清单所需的最低兼容已发布 CLI 政策。 |
| `scripts/staging-candidate.mjs` | 可信预发布候选身份生成和单调的按 PR 指针决定。 |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | 聚焦的预发布插件、归档、清单、共享文件系统和发布捆绑验证。 |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | 用于不可信构建输出和可信发布捆绑的严格验证入口点。 |
| `scripts/check-staging-channel-advance.mjs` | 在预发布渠道指针推进之前应用单调性和来源检查。 |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | 在指针提升之前验证已发布资产字节和公开下载行为。 |
| `scripts/test-run-scene-edit-script-inspect.mjs` | 构建一个被忽略的临时 Godot 项目，并针对编辑器 GDExtension 冒烟测试只读导入 `PackedScene` 检查。 |
| `scripts/release-targets.mjs` | 定义支持的平台发布目标及其打包资产名称。 |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | 写入冻结的候选身份及其小型渠道指针。 |
| `scripts/sync-chat-ui.mjs` | 将无构建步骤的聊天 UI 源码复制到插件载荷。 |
| `scripts/sync-runtime.mjs` | 将仓库根目录的运行时辅助源码复制到插件载荷。 |
| `scripts/sync-doc-navigation.mjs` | 添加文档导航、源哈希和稳定锚点，不翻译正文。 |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | 验证翻译覆盖度、新鲜度、Markdown 结构、URL 和链接。 |
| `scripts/package-preview.mjs` | 在平台构建后组装插件、CLI 和本地运行时预览或发布 zip。 |
| `scripts/prepare-linux-cef-runtime.mjs` | 暂存单独的 Linux x64 CEF 运行时 zip，剥离暂存的 ELF 二进制文件，验证必需文件，并可写入生成的发布清单。 |
| `scripts/prepare-linux-cef-sdk.mjs` | 为需要 `libcef_dll/` 包装器源码的 CI 构建下载并提取固定的官方 CEF 139 Linux 最小 SDK。 |
| `scripts/check-linux-cef-runtime-release.mjs` | 根据生成的 `local/webview-runtimes/linux-cef.json` 清单验证 Linux CEF 运行时发布资产。 |
| `scripts/write-release-manifest.mjs` | 根据发布资产写入并验证 `fennara-release-manifest-v<version>.json`，包括本地软件包、插件和共享运行时哈希。 |
| `scripts/cef/linux/fennara_cef_helper.cpp` | 打包在单独 CEF 运行时 zip 中的最小 Linux CEF 子进程辅助源码。 |
| `.github/workflows/version-check.yml` | 版本一致性检查。 |
| `.github/workflows/gdextension-build.yml` | 跨平台 GDExtension 构建检查，以及 Windows 无头原生首次运行设置状态测试。 |
| `.github/workflows/local-build.yml` | Rust 本地软件包构建检查。 |
| `.github/workflows/package-preview.yml` | 手动软件包预览工件，包括用于 Linux 聊天冒烟测试的仅限测试 Linux CEF 运行时工件。 |
| `.github/workflows/release.yml` | 手动 GitHub 发布，包括生成的 Linux CEF 运行时打包、发布清单生成和最终资产验证。 |
| `.github/workflows/staging-release.yml` | 手动精确 SHA 预发布构建、仅验证试运行、精确预发布发行和按 PR 指针推进。 |

<a id="where-to-change-things"></a>
## 在哪里更改内容

| 任务 | 从这里开始 |
| --- | --- |
| 添加或更改 Godot 工具 | `fennara-cpp/src/tools/` 和 `local/schemas/tools/` |
| 更改 MCP 模式文本 | `local/schemas/tools/` |
| 更改 `fennara install` 或 `fennara update` | `local/crates/fennara-cli/src/`；原生预发布和分离式应用或回滚由 `release_update.rs`、`update_stage.rs`、`update_stage/` 和 `update_apply/` 负责 |
| 更改 CLI 命令或终端行为 | `local/crates/fennara-cli/src/` 和 `docs/cli.md` |
| 更改原生更新进度、关闭确认、激活握手或恢复 | `fennara-cpp/src/update/`、`fennara-cpp/src/ui/update_panel.cpp`、`fennara-cpp/src/ui/dock.cpp`、`local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` 和 `ui/chat/` |
| 更改原生首次运行设置或 CLI 引导 | `fennara-cpp/src/setup/`、`fennara-cpp/src/ui/setup_panel.cpp` 和 `fennara-cpp/src/ui/dock.cpp` |
| 更改导出时的插件排除行为 | `fennara-cpp/src/ui/export_plugin.cpp`、`fennara-cpp/include/fennara/ui/export_plugin.hpp` 和 `godot_demo/tests/export_plugin_test.gd` |
| 更改安装或更新操作日志、阶段、错误代码或诊断报告 | `local/crates/fennara-cli/src/operation.rs`、`local/crates/fennara-cli/src/operation/` 和 `local/crates/fennara-cli/src/diagnostics.rs` |
| 更改 webview 先决条件检查 | `local/crates/fennara-cli/src/webview_prereq.rs`、`local/crates/fennara-cli/src/webview_runtime.rs` 和 `fennara-cpp/src/ui/webview_host*` |
| 更改生成的项目指导 | `local/templates/` 和 `local/crates/fennara-cli/src/project_guidance.rs` |
| 同步生成的演示插件指导 | `local/templates/fennara-guidelines.md`、`local/templates/fennara-ai/`、`scripts/sync-guidance.mjs` 和 `godot_demo/addons/fennara/ai/` |
| 更改 MCP 应用设置 | `local/crates/fennara-cli/src/mcp_setup.rs` 和 `docs/mcp-setup.md` |
| 更改运行时会话进程或日志行为 | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`、`local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`、`fennara-cpp/src/tools/runtime_session/` 和 `fennara-cpp/src/tool_results/` |
| 更改 `runtime_script` ctx 辅助功能、输入、快照、等待、射线检测、捕获或清理 | `runtime/`、`scripts/sync-runtime.mjs`、`godot_demo/addons/fennara/runtime/`、`local/schemas/tools/runtime_script.json` 和 `docs/tools.md` |
| 更改编辑器内聊天 UI、斜杠命令或模型或提供方选择器 | `ui/chat/`、`godot_demo/addons/fennara/dist/`、`fennara-cpp/src/ui/dock.cpp` 和 `fennara-cpp/src/ui/webview_host*` |
| 更改内置聊天提供方 | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`、`local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`、`local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` 和 `ui/chat/` |
| 更改匿名遥测字段、调度或隐私控制 | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`、`local/crates/fennara-daemon/src/runtime_daemon/telemetry/`、`local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`、`ui/chat/` 和 `docs/telemetry.md` |
| 更改供应商提供的聊天 UI 库 | `ui/chat/vendor/`、`godot_demo/addons/fennara/dist/vendor/` 和 `THIRD_PARTY_NOTICES.md` |
| 更改 C# 支持 | `fennara-cpp/src/csharp/`、`fennara-cpp/include/fennara/csharp/` 以及 C# 工具模式和指导 |
| 更改发布包、最低 CLI 政策或 CLI 自更新 | `local/crates/fennara-cli/src/release_manifest.rs`、`local/crates/fennara-cli/src/release_client.rs`、`local/crates/fennara-cli/src/release_package.rs`、`local/crates/fennara-cli/src/self_update.rs`、`scripts/package-preview.mjs`、`scripts/release-policy.mjs`、`scripts/write-release-manifest.mjs` 和 `.github/workflows/release.yml` |
| 提升版本 | `node scripts/set-version.mjs <version>` |
| 更新聊天与 MCP、提供方或斜杠命令的设置或文档 | `README.md`、`docs/mcp-setup.md`、`docs/chat-vs-mcp.md`、`docs/providers.md`、`docs/slash-commands.md`、`docs/setup.md`、`docs/faq.md`、`docs/manual-install.md`、`docs/tools.md`、`docs/examples.md` 和 `llms.txt` |
| 更新文档翻译 | 规范英语页面、`docs/i18n/languages.json`、匹配的语言区域页面、`scripts/sync-doc-navigation.mjs` 和 `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## 说明

- 添加或移动主要源码区域时，请保持本文件为最新。
- 将发布步骤保留在 [release.md](release.md)。
- 将设置步骤保留在 [setup.md](setup.md)。
- 将终端命令行为保留在 [cli.md](cli.md)。
