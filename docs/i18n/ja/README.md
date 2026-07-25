<!-- fennara-i18n: locale=ja source=docs/README.md sha256=2f8fb6a711c8bb56af570d1657f802c63cbdf2ced6b2c620339c588c9c9211cb -->
<a id="fennara-documentation"></a>
# Fennara ドキュメント

<!-- fennara-doc-nav:start -->
[English](../../README.md) · [简体中文](../zh-CN/README.md) · [Español](../es/README.md) · [Português do Brasil](../pt-BR/README.md) · **日本語** · [한국어](../ko/README.md) · [Русский](../ru/README.md) · [Français](../fr/README.md) · [Deutsch](../de/README.md) · [Türkçe](../tr/README.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../README.md)
<!-- fennara-doc-nav:end -->

実行したい作業から始めてください。各ページは通常の手順を先に示し、高度な詳細を後半にまとめています。

<a id="languages"></a>
## 言語

別の言語で同じページを読むには、ページ上部に追加される language menu を使います。coverage、review status、source-of-truth policy は [言語と翻訳状況](languages.md) を参照してください。

<a id="start-here"></a>
## はじめに

| やりたいこと | 読むページ |
| --- | --- |
| Fennara をインストールする | [セットアップ](setup.md) |
| 内蔵チャットを接続する | [チャットプロバイダー](providers.md) |
| Codex、Claude、Cursor などの MCP アプリを接続する | [MCP セットアップ](mcp-setup.md) |
| Fennara を更新または復旧する | [Fennara の更新](setup.md#update-fennara) |
| セットアップ問題を解決する | [トラブルシューティング](setup.md#troubleshooting) |

<a id="use-fennara"></a>
## Fennara を使う

| ガイド | 内容 |
| --- | --- |
| [MCP アプリと内蔵チャット](chat-vs-mcp.md) | 各経路が使うモデルアカウント |
| [ツール](tools.md) | Godot 対応ツールと使いどころ |
| [例](examples.md) | よくある Godot 作業向けプロンプト |
| [スラッシュコマンド](slash-commands.md) | チャットドックの `/provider` と `/model` |
| [FAQ](faq.md) | よくある質問への短い回答 |
| [デモ](demos.md) | 動画とプロジェクト解説 |
| [匿名テレメトリー](telemetry.md) | 収集データ、送信動作、オプトアウト |

<a id="reference-and-recovery"></a>
## リファレンスと復旧

| リファレンス | 用途 |
| --- | --- |
| [Fennara CLI](cli.md) | ターミナルコマンド、診断、自動化 |
| [手動インストール](manual-install.md) | 通常のインストーラーを使えない場合 |
| [MCP セットアップ](mcp-setup.md) | アプリ固有または手動の構成 |
| [プロバイダー](providers.md) | キー、モデル ID、ローカルサーバーの詳細 |

<a id="for-contributors"></a>
## コントリビューター向け

| 文書 | 目的 |
| --- | --- |
| [コントリビューション](CONTRIBUTING.md) | 貢献と Pull Request の要件 |
| [アーキテクチャ](architecture.md) | システム境界とランタイムフロー |
| [リポジトリマップ](repo-map.md) | コードと生成ファイルの場所 |
| [リリース手順](release.md) | パッケージ、マニフェスト、検証、公開 |
| [プロジェクト用語](CONTEXT.md) | コードと文書で共通する名称 |
| [セキュリティ](SECURITY.md) | 脆弱性の報告方法 |
| [GitHub メタデータ](github-metadata.md) | repository description と topics |
| [Godot ペイロード](contributors/godot-payload.md) | packaged addon source の境界 |
| [Godot アドオン](contributors/godot-addons.md) | addon directory の構造と規則 |
| [ローカルツール](contributors/local-tools.md) | CLI、daemon、MCP server、local runtime |
| [ランタイムヘルパー](contributors/runtime-helpers.md) | Godot 側 runtime helper source |
| [リポジトリスクリプト](contributors/scripts.md) | build、sync、validation、packaging automation |
| [チャット UI](contributors/chat-ui.md) | editor 内 chat source と design rule |

<a id="learn-from-examples"></a>
## 実例から学ぶ

- [Fennara と従来型 Godot MCP の比較](fennara-vs-traditional-godot-mcp.md)
- [Open RPG デモ解説](open-rpg-demo.md)
- [プロンプト例](examples.md)
