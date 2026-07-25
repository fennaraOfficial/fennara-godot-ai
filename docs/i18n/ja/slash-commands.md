<!-- fennara-i18n: locale=ja source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# 内蔵チャットのスラッシュコマンド

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · **日本語** · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

スラッシュコマンドは Godot 内の Fennara チャットドックで使うショートカットです。UI コマンドであり、MCP ツールでもモデルへ送るプロンプトでもありません。

入力欄で `/` を入力するとコマンドパレットが開きます。

| コマンド | 開く画面 | 用途 |
| --- | --- | --- |
| `/provider` | Provider picker | クラウドプロバイダーの接続、ローカルプロバイダー URL の構成、プロバイダーの切り替え |
| `/model` | Model picker | 現在または接続済みのプロバイダーからモデルを選択 |

<a id="how-they-behave"></a>
## 操作

- 矢印キーで候補間を移動します。
- Enter で選択したコマンドを実行します。
- Escape でコマンドパレットを閉じます。
- チャットメッセージを送信する前に、スラッシュコマンド文字列は入力欄から削除されます。

<a id="common-flow"></a>
## 一般的な流れ

内蔵チャットドックで次を入力します。

```text
/provider
```

OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、ローカル Ollama、LM Studio のいずれかを接続します。

次に:

```text
/model
```

ドックで使うモデルを選びます。

外部 MCP アプリでは、このスラッシュコマンドを使いません。`fennara mcp-setup` でアプリを構成し、そのアプリに Fennara MCP ツールを使うよう依頼してください。
