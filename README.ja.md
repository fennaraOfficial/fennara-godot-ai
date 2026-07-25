<!-- fennara-i18n: locale=ja source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · **日本語** · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/ja/demos.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

[Somni Game Studios](https://somnigamestudios.com/) を含む Godot 開発者とチームに利用されています。

Fennara は AI アシスタントを実行中の Godot へ接続します。Codex、Claude、Cursor、Gemini、Antigravity など MCP 対応アプリから利用することも、オプションの editor 内チャットドックから利用することもできます。

エージェントはプロジェクトファイルだけから推測せず、シーンの検査、スクリプト確認、スクリーンショット、実行時エラー、editor 内での変更検証を行えます。

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Fennara と他の Godot MCP の比較" width="100%" />
      </a>
    </td>
    <td>
      <strong>注目のデモを見る</strong><br />
      Fennara と他の Godot MCP を比較します。<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">動画を再生</a><br />
      <a href="docs/i18n/ja/demos.md">すべてのデモ動画を見る</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## 主な機能

- MCP を通じて外部 AI アプリへ Godot 対応ツールを公開
- Godot editor 内にオプションのローカルチャットドックを追加
- シーンツリー、診断、スクリーンショット、実行時ログ、検証結果など、本物の Godot フィードバックを返す
- ファイルシステムだけでなく、開いている editor の結果に対してエージェントへ責任を持たせる

外部 MCP アプリと内蔵チャットはモデル設定を共有しません。[MCP アプリと内蔵チャット](docs/i18n/ja/chat-vs-mcp.md)および[内蔵チャットプロバイダー](docs/i18n/ja/providers.md)を参照してください。

<a id="requirements"></a>
## 必要条件

- Godot 4.5 以降
- Windows x86_64、Linux x86_64、macOS arm64 のいずれか
- Claude、Codex、Cursor、Gemini、Antigravity などから使う場合のみ、MCP 対応コーディングアプリ
- 内蔵チャットを使う場合のみ、クラウドプロバイダーのキー、または Ollama / LM Studio などのローカルプロバイダー

完全な手順は[セットアップ](docs/i18n/ja/setup.md)を参照してください。

<a id="what-setup-adds"></a>
## セットアップで追加されるもの

- `res://addons/fennara/` に置かれる Fennara アドオン
- Fennara アプリデータへインストールされる小さな `fennara` CLI
- AI コーディングアプリが使うローカル MCP サーバー
- MCP とチャット要求を開いている Godot editor へ接続するローカルデーモン
- AI エージェント向けに生成されるプロジェクトガイダンス

内蔵チャットドックはプラットフォームの webview を使います。Windows は Microsoft Edge WebView2、macOS は WKWebView/WebKit、Linux は Fennara 管理の共有 CEF ランタイムです。オプションのチャットドックが起動できなくても MCP ツールは動作します。

<a id="install"></a>
## インストール

Windows と Linux ではアドオンまたは CLI のどちらでもインストールできます。macOS では、アドオン ZIP を手動でダウンロードして展開した後に表示されることがある security notification を避けるため、CLI を使ってください。

<a id="add-the-addon-to-your-project"></a>
### アドオンをプロジェクトへ追加

- [Latest Release](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest) を開き、`fennara-addon-latest.zip` をダウンロードし、その `addons/fennara/` をプロジェクトへ展開します。

プロジェクトを開き、Fennara ドックを選んで **Set Up Fennara** を押します。

Fennara はエディターの依存関係であり、ゲームのランタイム依存関係ではありません。エクスポート時に、エディタープラグインはエクスポートされるプロジェクトからランタイムの autoload を除去し、`res://addons/fennara/` と `res://.fennara/` を除外します。エクスポートが完了すると、エディタープロジェクトは元の状態に戻されます。CI のチェックアウトで `.gitignore` によりアドオンが除外される場合は、Godot を起動する前に `fennara prepare-export --project path/to/project` を実行するか、そのチェックアウトにアドオンをインストールしてください。Godot はエクスポートプラグインが実行される前に autoload パスを検証するため、この準備を先に行う必要があります。

> **macOS:** リリースアドオンの native library は現在 Apple-notarized ではありません。ブラウザーから ZIP を取得して手動展開すると、macOS が `libfennara.macos.editor` を malware ではないと検証できない旨を表示する場合があります。通知を避けるには以下の CLI インストールを使ってください。すでに表示された場合は Godot を閉じ、手動コピーした `addons/fennara/` を削除してから CLI でインストールします。

<a id="install-with-the-cli-recommended-on-macos"></a>
### CLI でインストール (macOS で推奨)

CLI は同じ Fennara アドオンをインストールします。ブラウザーと Finder の quarantine 経路を避けられるため、macOS ではこの方法を推奨します。

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS と Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Godot プロジェクトで実行します。

```bash
cd path/to/your-godot-project
fennara install
```

問題がある場合は[セットアップ](docs/i18n/ja/setup.md)、全コマンドは[Fennara CLI](docs/i18n/ja/cli.md)を参照してください。

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## プロバイダーまたは MCP アプリを接続

<a id="built-in-chat"></a>
### 内蔵チャット

**Chat Settings > Chat** を開き、**Open providers** を選んでプロバイダーを接続します。クラウドプロバイダーには自分のキーを使う BYOK 方式です。ローカル Ollama または LM Studio も利用できます。[対応プロバイダー](docs/i18n/ja/providers.md)を参照してください。

<a id="mcp-apps"></a>
### MCP アプリ

**Chat Settings > MCP Apps** を開き、アプリを探して **Set Up** を押します。

terminal からも接続できます。

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

一覧にない場合は、全アプリと手動構成を説明する[MCP セットアップ](docs/i18n/ja/mcp-setup.md)を参照してください。

<a id="update"></a>
## 更新

Fennara ドックに **Update** が表示されたら押し、案内に従います。

> **Fennara v0.3.8 以前から更新する場合:** `fennara update` の前に、上記のプラットフォーム別コマンドで CLI を一度再インストールします。古い CLI は廃止済み release tag を参照するため現在のリリースを見つけられません。再インストールしても既存のアドオンや設定は削除されません。

> **macOS で Fennara v0.3.11 から更新する場合:** 更新前に macOS 用コマンドで CLI を一度再インストールします。v0.3.11 CLI は自己更新へ到達する前に既存 framework bundle を拒否します。置き換わるのは CLI だけで、アドオンと設定は残ります。

terminal から更新するには Godot を閉じて実行します。

```bash
cd path/to/your-godot-project
fennara update
```

復旧と診断は[セットアップの更新節](docs/i18n/ja/setup.md#update-fennara)を参照してください。

<a id="tools"></a>
## ツール

Fennara は少数の Godot 対応ツールを公開します。

- プロジェクトファイルを作成または更新し、診断を返す
- 一度限りのシーン編集スクリプトを実行
- シーンツリー、ノード、リソース、Godot class を検査
- シーンを検証
- スクリーンショットを取得
- ランタイムセッションを開始してログを読む
- 実行中シーンへ小さな runtime script を実行

通常のファイルツールを置き換えるのではなく、不足している Godot フィードバックループを提供します。

<a id="privacy"></a>
## プライバシー

Godot 接続後、Fennara は UTC で 1 日に最大 1 回、匿名の active-installation event を送信します。ランダムな installation UUID、Fennara と Godot のバージョン、OS、CPU architecture だけを含みます。プロジェクトデータ、パス、プロンプト、ツール操作、ログ、スクリーンショット、アカウント情報は含みません。

**Chat Settings > Chat > Anonymous telemetry**、`FENNARA_DISABLE_TELEMETRY=true`、`DO_NOT_TRACK=1` のいずれかで無効にできます。完全な契約は[匿名テレメトリー](docs/i18n/ja/telemetry.md)にあります。

<a id="demos"></a>
## デモ

Fennara の実践的な解説動画をご覧ください。

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

ほかの動画:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Fennara チャンネルの動画は[デモ](docs/i18n/ja/demos.md)にあります。

<a id="star-history"></a>
## Star History
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## ドキュメント

| 最初に読むページ | 用途 |
| --- | --- |
| [ドキュメントホーム](docs/i18n/ja/README.md) | すべてのガイドとリファレンス |
| [セットアップ](docs/i18n/ja/setup.md) | インストール、更新、トラブルシューティング |
| [チャットプロバイダー](docs/i18n/ja/providers.md) | 内蔵チャットのモデルとキー |
| [MCP セットアップ](docs/i18n/ja/mcp-setup.md) | Codex、Claude、Cursor など |
| [ツール](docs/i18n/ja/tools.md) | エージェントが使える Godot フィードバック |
| [匿名テレメトリー](docs/i18n/ja/telemetry.md) | 収集データ、送信動作、オプトアウト |
| [コントリビューション](docs/i18n/ja/CONTRIBUTING.md) | 開発と Pull Request |

<a id="community"></a>
## コミュニティ

質問、セットアップの支援、早期フィードバックは Discord へ:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## ライセンス

[LICENSE.md](LICENSE.md) を参照してください。
