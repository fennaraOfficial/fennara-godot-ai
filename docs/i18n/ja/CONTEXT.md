<!-- fennara-i18n: locale=ja source=CONTEXT.md sha256=ee0d279d8a4916d5cf894616b1c72658669a36bf0ec958efef5a09ee196c704e -->
<a id="fennara-context"></a>
# Fennara 用語集

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · **日本語** · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

このファイルでは、Fennara のドキュメント、Issue、リリースノート、エージェント向けガイダンスで共通して使う用語を定義します。

<a id="product-terms"></a>
## 製品用語

**Fennara**

このリポジトリにある、Godot を理解するエージェント環境です。Fennara は AI ツールを、診断、シーン検証、実行時エラー、スクリーンショット、プロジェクトガイダンスなど、実際の Godot から得たフィードバックへ接続します。

**Godot Addon**

ユーザーの Godot プロジェクトの `res://addons/fennara/` にコピーされる、インストール可能なプラグインです。ドック UI、Godot 側の検査ツール、ネイティブ GDExtension ライブラリ、パッケージ化されたチャット UI アセット、実行時ヘルパースクリプト、プロジェクト内のアドオンバージョンを所有します。

**Fennara CLI**

ユーザーのマシンにインストールされる `fennara` コマンドです。インストール、更新、CLI 自体の更新、doctor チェック、MCP アプリのセットアップ、webview の前提条件に関する警告、C# セットアップ確認、プロジェクトガイダンス生成を担当します。

**Local Package**

1 つのプラットフォームとアーキテクチャ向けに、MCP サーバー、デーモン、ランタイムバイナリ、ランチャーバイナリなど、ローカルの Fennara 実行ファイルを含むリリース zip です。

**Project Guidance**

Godot プロジェクトへ配置される生成済みガイダンスファイルです。`AGENTS.md` と `addons/fennara/ai/` 以下へルーティングされた参照文書が含まれ、AI コーディングエージェントに Fennara をいつ、どのように使うかを伝えます。

<a id="mcp-terms"></a>
## MCP 用語

**Fennara MCP Server**

Claude Code、Cursor、Cline、Gemini CLI、または別の MCP クライアントなどの AI コーディングアプリが起動する、ローカル stdio MCP サーバーです。外部アプリへ Fennara ツールを公開します。

**MCP App**

`fennara mcp-setup` で構成する外部 AI アプリです。どの外部アプリが Fennara ツールを呼べるかを設定するもので、Fennara 内蔵チャットのモデルは選択しません。

**MCP Target**

現在 Fennara MCP 呼び出しを受け取るよう選択されている Godot プロジェクトです。

**Tool Schema**

引数、制限、ワークフロー上の注意事項を含む、モデル向けの Fennara MCP ツール定義です。

**Tool Result Envelope**

ツール呼び出し後にモデルへ返す簡潔な結果です。不要な生データを大量に出さず、状態、重要な所見、次に役立つ文脈を示します。

<a id="built-in-chat-terms"></a>
## 内蔵チャット用語

**Built-In Chat**

Godot アドオンまたはシステムブラウザーで使う Fennara 独自のチャット画面です。外部 MCP アプリとは別です。Claude Code を MCP に使いながら、内蔵チャットでは別のプロバイダーとモデルを選べます。

**Chat Surface**

内蔵チャットの表示方法です。embedded モードは Godot ドック内のネイティブ webview を使い、browser モードは同じ UI をローカルデーモンから配信してシステムブラウザーで開きます。

**Chat Provider**

OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、ローカル Ollama、LM Studio など、内蔵チャットの応答を生成できるバックエンドです。

**Model Ref**

内蔵チャットで選択するプロバイダー修飾付きモデル識別子です。`/provider` と `/model` でプロバイダー接続と model ref の選択ができます。

**Provider Connection**

API キーやローカル base URL を含む、デーモンが管理するプロバイダーのローカル設定と認証状態です。秘密情報は Godot プロジェクト内ではなく、デーモン管理のローカルストレージに保存します。

**Generation Trace**

アシスタントメッセージ、ツール呼び出し、プロバイダーとモデル、使用量、コストログを、それらを生成した処理に結び付ける内蔵チャットの保存メタデータです。

<a id="runtime-and-webview-terms"></a>
## ランタイムと webview の用語

**Fennara Daemon**

MCP 呼び出しと内蔵チャット要求を Godot アドオンへ接続し、ローカル状態を保存し、`/chat/` などのチャットルートを提供するローカルサービスです。

**Runtime Session**

実行時検査、ログ、検証、スクリーンショット、および将来の実行中シーンのワークフローに使う、デーモン管理の Godot ランタイムセッションです。

**Godot Snapshot**

ファイルを変更する可能性がある Fennara 支援ターンの前に取得する、復元可能なプロジェクト状態です。セットアップに失敗して孤立したプロンプトが残らないよう、ユーザーターンを永続化する前にスナップショット準備を完了します。

**Webview Runtime**

Godot 内またはその近くで内蔵チャットを表示するためのプラットフォーム機能です。Windows は WebView2、macOS は WebKit/WKWebView、Linux は Fennara アプリデータに置く共有 CEF ランタイムを使います。

**Shared Linux CEF Runtime**

Linux チャット webview が使う外部 Linux CEF ランタイムペイロードです。ユーザーの Fennara アプリデータディレクトリへ一度だけインストールし、各 Godot アドオン zip に同梱してはいけません。

<a id="release-terms"></a>
## リリース用語

**Release Manifest**

`fennara-release-manifest-v<version>.json` という JSON アセットです。リリースアセットをプラットフォームへ対応付け、SHA-256 ハッシュ、共有ランタイムアセット、`minimum_cli_version` を記録します。

**Minimum CLI Version**

リリースマニフェストを利用できる最も古い `fennara` CLI バージョンです。新しいインストールまたは更新ロジックが必要なら、`scripts/release-policy.mjs` の該当トラックを更新します。マニフェスト生成処理はリリース識別情報を検証した後でこのポリシーを適用し、workflow が値を選ぶことはありません。

**Latest Release**

正確なバージョン付きリリースを指す GitHub の Latest Release ポインターです。インストーラーと通常の更新は GitHub API を通してこれを解決します。Fennara は `latest` という固定タグやリリースを使いません。公開後にソースを更新してもリリースアセットは変わらず、公開済みマニフェストを変えるにはアセットを明示的に置き換える必要があります。
