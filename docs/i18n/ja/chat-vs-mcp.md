<!-- fennara-i18n: locale=ja source=docs/chat-vs-mcp.md sha256=03cb522aed8f8e305feaca0c2ed51f7ba29b2657a721df4196b15bc6ccf12c9c -->
<a id="mcp-apps-or-built-in-chat"></a>
# MCP アプリと内蔵チャットのどちらを使うべきですか？

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · **日本語** · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara は両方に対応しています。会話を行いたい場所に応じて選んでください。

| | 外部 MCP アプリ | Fennara 内蔵チャット |
| --- | --- | --- |
| 会話する場所 | Codex、Claude、Cursor、Gemini、または別の MCP アプリ | Fennara ドックまたはシステムブラウザー |
| モデルアカウント | 外部アプリのアカウントまたはサブスクリプション | Fennara の Chat Settings で接続したプロバイダー |
| Fennara が追加するもの | Godot 対応 MCP ツール | チャット UI、同じ中核 Godot ツール、チャット専用のファイルツールとシェルツール |
| セットアップ | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> 2 つの経路を両方とも利用できます。それぞれのモデル設定は別々に保たれます。

<a id="external-mcp-apps"></a>
## 外部 MCP アプリ

MCP アプリを接続すると、そのアプリがローカルの Fennara MCP サーバーを起動して Godot ツールを呼び出せるようになります。アプリのサブスクリプションやログインが内蔵チャットと共有されるわけではありません。

**Chat Settings > MCP Apps** からアプリをセットアップするか、CLI を使用します。

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Fennara のチャットプロバイダーキーは必要ありません。セットアップ後に外部アプリを再起動してください。すべての対象アプリと手動構成については、[MCP セットアップ](mcp-setup.md)を参照してください。

<a id="built-in-chat"></a>
## 内蔵チャット

内蔵チャットを使うには、Fennara の Chat Settings でプロバイダーを接続する必要があります。クラウドプロバイダーには自分のキーを使用するか、ローカルの Ollama または LM Studio サーバーを接続してください。

同じチャットを Godot ドック内またはシステムブラウザーに表示できます。この表示方法の選択によって、プロバイダー、モデル、履歴、プロジェクトが変わることはありません。

コードを添付するには、Godot のスクリプトエディターでコードを選択し、コンテキストメニューを開いて **Add to Chat** を選びます。プロバイダーとモデルのセットアップについては、[内蔵チャットプロバイダー](providers.md)を参照してください。

<a id="project-routing"></a>
## プロジェクトのルーティング

どちらの経路も、Godot のフィードバックを得るためにローカルの Fennara デーモンを使用します。

- 外部 MCP の呼び出しは、ドックの **MCP target** コントロールで選択されたプロジェクトへ送られます。
- 内蔵チャットは、そのチャットを開いた Godot エディターに結び付けられたままです。

外部 MCP の接続を確認するには、次のように依頼します。

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```
