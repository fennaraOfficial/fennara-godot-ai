<!-- fennara-i18n: locale=zh-CN source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# 匿名遥测

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../telemetry.md) · **简体中文** · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara 每个 UTC 日最多发送一次小型匿名活动事件。只有兼容的 Godot 编辑器连接到本地守护进程后才会发送。它帮助维护者统计活跃安装数、受支持平台的使用情况和版本采用情况。

遥测默认启用。打开 **Chat Settings > Chat > Anonymous telemetry** 可将其禁用。无头和自动化环境也可以设置：

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

环境变量优先于保存的 UI 首选项。关闭遥测会停止后续事件，并删除本地遥测身份和最后发送状态。重新启用后，Godot 下次连接时会创建新的随机身份。

<a id="event-contents"></a>
## 事件内容

`fennara_active_installation` 事件只包含：

| 字段 | 用途 |
| --- | --- |
| `schema_version` | 小型遥测负载约定的版本 |
| `event` | 固定事件名称 |
| `installation_id` | 本地生成的随机 UUID，不从硬件或账户派生 |
| `fennara_version` | 正在运行的守护进程版本 |
| `godot_version` | 数字形式的 Godot 版本，例如 `4.6.3` |
| `platform` | `windows`、`macos` 或 `linux` |
| `architecture` | `x86_64` 或 `aarch64` |

Fennara 不发送项目名称、项目路径、账户信息、提示词、聊天消息、提供商密钥、模型名称、工具名称、工具参数、工具结果、日志、截图、场景内容、文件名或错误文本。

<a id="storage-and-transport"></a>
## 存储与传输

守护进程将随机身份和上一次成功发送的 UTC 日期存储在共享 Fennara 应用数据目录下：

```text
Fennara/
  telemetry/
    state.json
```

守护进程通过 HTTPS 将事件发送到 `https://fennara.io/api/telemetry`。接收端会验证严格的字段允许列表，并以服务器端 HMAC 替换原始安装 UUID，再将事件转发给 PostHog。该事件禁用了 PostHog 用户档案和 IP 地理定位。

Vercel 接收端在处理 HTTPS 请求时必然会看到常规网络元数据，但这些元数据不会复制到 PostHog 事件负载中。

<a id="delivery-behavior"></a>
## 发送行为

遥测在 Godot 工具调用路径之外运行：

- 有界队列无需等待即可接收活动信号。
- 一个后台工作进程复用单个 HTTP 客户端。
- 请求使用很短的超时。
- 队列已满、文件系统问题、网络故障或服务器拒绝都会被静默容忍，绝不会导致 Fennara 工具失败。
- 只有服务器接受事件后才记录 UTC 日期，因此后续 Godot 连接可以重试失败的发送。
- 关闭时只短暂等待，然后取消遥测工作进程，不会拖延守护进程。

一个安装对应一个持久化的随机 UUID。在两台电脑上使用 Fennara 会计为两个安装。清除 Fennara 应用数据，或禁用后重新启用遥测，都会创建新身份。

月活跃安装数按自然月内至少发送过一次 `fennara_active_installation` 事件的不同匿名安装身份计算。
