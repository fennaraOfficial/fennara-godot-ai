<!-- fennara-i18n: locale=ja source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# よくある質問

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · **日本語** · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../faq.md)
<!-- fennara-doc-nav:end -->

インストールと更新については、まず[セットアップ](setup.md)を参照してください。このページでは、よくある質問への短い回答と詳細なリファレンスへのリンクを掲載しています。

| 質問 | 短い回答 |
| --- | --- |
| プロバイダーキーは必要ですか？ | 内蔵チャットでクラウドプロバイダーを使う場合にのみ必要です |
| 代わりに外部 MCP アプリを使えますか？ | はい。外部アプリは独自のモデルアカウントを使用します |
| Fennara は私のプロジェクトを Fennara サーバーへアップロードしますか？ | いいえ |
| 複数の Godot エディターを開けますか？ | はい。ドックで外部 MCP の対象を選択します |

<a id="is-fennara-only-a-code-generator"></a>
## Fennara はコードジェネレーターにすぎませんか？

いいえ。Fennara は Godot を理解するエージェントワークフローです。プロジェクトファイル、シーン、診断、実行時エラー、スクリーンショット、Godot エディターのコンテキストを扱えます。

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Fennara は単なる Godot MCP コマンドサーバーですか？

いいえ。MCP は、Codex、Claude、Cursor、Gemini、Antigravity などのアプリから Fennara を使用する方法の 1 つです。Fennara には、オプションの内蔵チャットドックもあります。製品の中心的な考え方は、Godot のフィードバックループです。診断、検証、実行時エラー、スクリーンショット、構造化されたツール結果を提供し、エージェントが誤りを修正できるようにします。

<a id="does-fennara-replace-godot-knowledge"></a>
## Fennara は Godot の知識を不要にしますか？

いいえ。Fennara は Godot を不要にしようとしているわけではありません。AI エージェントが実際の Godot エンジンに対して責任を持つように設計されています。

<a id="how-should-i-install-fennara"></a>
## Fennara はどのようにインストールすればよいですか？

Windows と Linux では、アドオンを追加して Fennara ドックを開き、**Set Up Fennara** を押すか、ターミナルからインストールします。macOS では、ブラウザーからダウンロードしたアドオン ZIP を手動で展開したときに表示されることがあるセキュリティ通知を避けるため、CLI 経由でインストールしてください。両方の方法については、[セットアップ](setup.md)を参照してください。

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## macOS が `libfennara.macos.editor` を検証できないと表示するのはなぜですか？

リリースアドオンには、現在 Apple の notarization を受けていないネイティブライブラリが含まれています。アドオン ZIP をブラウザーからダウンロードして手動で展開すると、Finder がそのライブラリへ quarantine メタデータを引き継ぎ、macOS の通知が表示される場合があります。

この通知を避けるには、[CLI インストール](setup.md#install-from-the-terminal-recommended-on-macos)を使用してください。すでに通知が表示されている場合は Godot を閉じ、手動でコピーした `addons/fennara/` フォルダーを削除し、CLI をインストールして、プロジェクトディレクトリから `fennara install` を実行します。CLI はブラウザーと Finder の quarantine 経路を使わずに、同じアドオンをインストールします。

<a id="do-i-need-a-chat-provider-api-key"></a>
## チャットプロバイダーの API キーは必要ですか？

Fennara 内蔵チャットドックでクラウドプロバイダーを使用する場合にのみ必要です。外部 MCP クライアントは独自のモデルとアプリ構成を使用するため、Fennara チャットへプロバイダーキーを入力しなくても Fennara MCP ツールを利用できます。

内蔵チャットでは、クラウド API キーを使わずにローカルの Ollama または LM Studio も利用できます。[内蔵チャットプロバイダー](providers.md)を参照してください。

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## すでに `mcp-setup --claude` を実行したのに、ドックがプロバイダーを求めるのはなぜですか？

`fennara mcp-setup --claude` は、Claude を Fennara の Godot MCP ツールへ接続します。Fennara 内蔵ドックを Claude へ接続するものではなく、Claude のサブスクリプションを Fennara チャットと共有するものでもありません。

外部 MCP の経路には Claude Code または Claude Desktop を使用してください。Godot の Fennara ドック内で会話したい場合にのみ、別のプロバイダーを構成します。[MCP アプリと内蔵チャット](chat-vs-mcp.md)を参照してください。

<a id="what-are-provider-and-model"></a>
## `/provider` と `/model` とは何ですか？

これらは Fennara 内蔵チャットドックのスラッシュコマンドです。`/provider` はプロバイダーピッカーを開きます。`/model` はモデルピッカーを開きます。これらは UI のショートカットであり、外部 MCP ツールでも、モデルへ送信されるテキストでもありません。[内蔵チャットのスラッシュコマンド](slash-commands.md)を参照してください。

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Fennara は私の Godot プロジェクトを Fennara サーバーへ送信しますか？

いいえ。通常の OSS の経路では、MCP クライアント、デーモン、Godot アドオンがローカルで動作します。内蔵チャットがモデルリクエストを送信するのは、OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、またはローカルの Ollama/LM Studio サーバーなど、ユーザーが構成したプロバイダーだけです。

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## 複数の Godot エディターが開いている場合、どのプロジェクトが MCP ツール呼び出しを受け取りますか？

デーモンは、外部 MCP の呼び出しを有効な MCP target へルーティングします。Godot の Fennara ドックにある MCP target コントロールでプロジェクトを選択してください。内蔵チャットのセッションは、そのチャットを開いた Godot エディターに結び付けられたままです。

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Linux が別の CEF ランタイムをインストールするのはなぜですか？

Linux の内蔵チャットは CEF のオフスクリーンレンダリングを使用します。CEF ペイロードは大きいため、Fennara は各 Godot プロジェクトのアドオンへコピーせず、ユーザーの Fennara アプリデータディレクトリへ 1 回だけインストールします。

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## アドオンに `libcef.so` が含まれるのが正しいですか？

いいえ。`libcef.so`、CEF リソース、ロケールパック、CEF ヘルパーは、共有 Linux CEF ランタイムに属します。アドオンには、Godot アドオンファイル、GDExtension バイナリ、チャット UI ファイル、および ripgrep のような小さな同梱ヘルパーバイナリだけを含めるべきです。

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## 内蔵チャットの webview を起動できない場合はどうなりますか？

Fennara MCP ツールは引き続き動作します。プラットフォームの webview が必要なのは、オプションのエディター内チャットドックだけです。Windows では、`fennara doctor` が Microsoft Edge WebView2 Runtime の欠如を報告した場合、それをインストールしてください。macOS では、WKWebView はシステムの WebKit.framework に含まれています。Linux では `fennara update` を実行し、リリースで管理される CEF ランタイムをインストールまたは修復できるようにします。

Chat Settings の **Open chat in my system browser next time** オプションも使用できます。これにより、同じ Fennara 内蔵チャットとプロバイダー設定を保ったまま、Godot の内蔵 webview ではなく、ローカルデーモン経由でシステムブラウザーに UI を開きます。設定を変更した後は Godot を再起動してください。

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## ブラウザーでチャットを開くと Claude または MCP アプリが使われますか？

いいえ。ブラウザー表示は、Fennara 内蔵チャットの UI とランタイムに関する選択にすぎません。引き続き Fennara の Chat Settings で選択されたプロバイダーを使用します。`fennara mcp-setup --claude` などのコマンドは外部 MCP アプリを構成するものであり、内蔵チャットのモデルを構成するものではありません。

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update` は MCP アプリの構成を書き換えますか？

いいえ。`fennara update` は必要に応じて、インストール済み CLI、プロジェクトアドオン、ローカルランタイムパッケージ、生成済みプロジェクトガイダンス、プラットフォーム管理のランタイムアセットを更新します。`fennara mcp-setup` を再実行するのは、MCP アプリの構成をセットアップまたは修復するときだけです。

<a id="where-does-chat-history-live"></a>
## チャット履歴はどこに保存されますか？

チャット履歴はデーモンによってローカルに保存され、現在の Godot プロジェクトに限定されます。プロバイダーキーとローカルプロバイダー URL も、Godot プロジェクトの外にあるデーモンのローカルストレージへ保存されます。

<a id="what-should-agents-use-fennara-tools-for"></a>
## エージェントは Fennara ツールを何に使うべきですか？

Fennara は、シーンツリー、変更されたノードとリソースのプロパティ、診断、検証、ランタイムセッション、スクリーンショット、エディターデバッガーの状態など、Godot を理解したフィードバックに使用します。Fennara 固有のツールが必要な場合を除き、MCP クライアントは引き続き独自の通常のファイル読み取りおよび検索ツールを使用するべきです。
