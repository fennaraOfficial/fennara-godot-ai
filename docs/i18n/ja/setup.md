<!-- fennara-i18n: locale=ja source=docs/setup.md sha256=d470da8cda3e69aacb89ee5f06f1d7831df5ab7f50c94a599bfab5bfe22157e3 -->
<a id="setup"></a>
# セットアップ

<!-- fennara-doc-nav:start -->
[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · **日本語** · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../setup.md)
<!-- fennara-doc-nav:end -->

Fennara をインストールし、会話を行う場所を選び、Godot プロジェクトを接続します。

> [!TIP]
> ほとんどのユーザーは、アドオンを追加し、Fennara ドックを開いて **Set Up Fennara** を押すだけです。macOS では、手動でダウンロードしたアドオン ZIP によって発生し得るセキュリティ通知を避けるため、以下の CLI インストールを使用してください。

<a id="before-you-start"></a>
## 始める前に

| 要件 | 必要になる場合 |
| --- | --- |
| Godot 4.5 以降 | 常に必要 |
| Windows x86_64、Linux x86_64、または macOS arm64 | 常に必要 |
| MCP 対応 AI アプリ | 外部 MCP を使用する場合のみ |
| クラウド API キー、Ollama、または LM Studio | 内蔵チャットを使用する場合のみ |
| `dotnet` として利用可能な .NET SDK | C# の診断とランタイム事前確認を行う場合のみ |

<a id="install-from-godot"></a>
## Godot からインストール

> [!IMPORTANT]
> macOS では、リリースアドオンに、現在 Apple の notarization を受けていないネイティブライブラリが含まれています。アドオン ZIP をブラウザーからダウンロードして手動で展開すると、`libfennara.macos.editor` がマルウェアを含まないことを検証できないと macOS が報告する場合があります。この通知を避けるには、[ターミナルからインストール](#install-from-the-terminal-recommended-on-macos)を使用してください。

1. [最新リリース](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)から `fennara-addon-latest.zip` をダウンロードし、`addons/fennara/` をプロジェクトへコピーします。
2. プロジェクトを開き、Fennara ドックを選択します。
3. **Set Up Fennara** を押します。

Fennara は対応するローカルコンポーネントをインストールし、開いているプロジェクトを接続します。以前の共有デーモンがアイドル状態なら、セットアップは対応するバージョンを有効にする前にそのデーモンを停止します。バージョンを切り替えるには、接続中のプロジェクトが 0 件である必要があります。セットアップ中のプロジェクトは通常、バージョンが異なる間は切断されたままです。セットアップが接続中のプロジェクトを報告する場合は、Fennara が有効な他のすべてのエディターを閉じてから再試行してください。現在のプロジェクトに古い接続が残っている場合は、このエディターを閉じて開き直し、再試行します。
セットアップに失敗した場合、ドックに **Retry**、**Copy Report**、**Open Logs** が表示されます。コピーされるレポートはサニタイズされ、API キー、チャット内容、プロジェクトファイルは含まれません。

> [!NOTE]
> アドオンはプロジェクト内に残ります。CLI、デーモン、MCP サーバー、ログ、共有ブラウザーランタイムは、プロジェクト外の Fennara アプリデータに置かれます。

<a id="install-from-the-terminal-recommended-on-macos"></a>
## ターミナルからインストール (macOS で推奨)

CLI は同じアドオンをインストールし、macOS で推奨されるインストール方法です。前述のネイティブライブラリ通知の原因になる、ブラウザーと Finder の quarantine 経路を回避できます。

Windows に CLI をインストールします。

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS と Linux では、次を実行します。

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

次に、プロジェクト内で Fennara を実行します。

```bash
cd path/to/your-godot-project
fennara install
```

macOS ですでにアドオンを手動展開して通知が表示されている場合は、`fennara install` を実行する前に Godot を閉じ、手動でコピーした `addons/fennara/` フォルダーを削除してください。CLI は既存の完全なアドオンを置き換えずに維持するため、この手順が重要です。

プロジェクトに完全な Fennara アドオンがすでに含まれている場合、CLI はそれを維持し、対応するローカルコンポーネントをインストールします。そうでない場合は、現在のリリースアドオンもインストールします。バージョン固定と自動化については、[CLI インストールのリファレンス](cli.md#install-a-project)を参照してください。

<a id="choose-how-you-use-fennara"></a>
## Fennara の使用方法を選択

| 経路 | モデルアカウント | セットアップ |
| --- | --- | --- |
| 内蔵チャット | Fennara Chat Settings で接続したプロバイダー | [プロバイダーを接続](#connect-the-built-in-chat) |
| 外部 MCP アプリ | アプリ独自のモデルアカウントまたはサブスクリプション | [MCP アプリを接続](#connect-an-mcp-app) |
| 両方 | それぞれの経路が独自のモデル設定を維持 | 両方のセクションを完了 |

<a id="connect-the-built-in-chat"></a>
### 内蔵チャットを接続

1. **Chat Settings > Chat** を開きます。
2. **Open providers** を選択します。
3. 自分のキーでクラウドプロバイダーを接続するか、ローカルの Ollama または LM Studio サーバーを接続します。
4. モデルを選択します。

対応プロバイダー、キー、ローカルサーバー URL、モデル ID については、[内蔵チャットプロバイダー](providers.md)を参照してください。composer から同じ操作を行うには、`/provider` と `/model` を使用します。

内蔵チャットは、各プラットフォームの webview を使用します。

| プラットフォーム | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | システムの WKWebView/WebKit |
| Linux | Fennara 管理の共有 CEF ランタイム |

`fennara install`、`fennara update`、`fennara doctor` は、これらの前提条件を確認します。オプションの内蔵チャットを起動できなくても、MCP ツールは引き続き動作します。

代わりにシステムブラウザーを使用するには、Chat Settings で **Open chat in my system browser next time** を有効にし、Godot を再起動します。変わるのは内蔵チャットの表示場所だけです。プロバイダー、履歴、プロジェクト接続は同じままです。

次の内蔵チャットメッセージへコードを添付するには、Godot のスクリプトエディターでコードを選択し、コンテキストメニューを開いて **Add to Chat** を選びます。

<a id="connect-an-mcp-app"></a>
### MCP アプリを接続

**Chat Settings > MCP Apps** を開き、使用するアプリを見つけて **Set Up** を押します。Fennara を読み込めるように、アプリを再起動してください。

ターミナルからアプリを接続することもできます。

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

使用するアプリが一覧にない場合は、対応するすべての対象と手動構成形式について [MCP セットアップ](mcp-setup.md)を参照してください。

セットアップで作成されるエントリはグローバルで、プロジェクトには依存しません。複数のエージェントが別々のリポジトリや worktree を使用する場合は、セットアップ後に Project Root ごとのプロジェクトバインド済み MCP 接続を 1 つずつ構成してください。詳しくは [複数エージェントと worktree](multi-agent-worktrees.md) を参照してください。

外部 MCP アプリは独自のモデルアカウントを使用します。内蔵チャットは Fennara の Chat Settings で選択したプロバイダーを使用します。違いについては、[MCP アプリと内蔵チャット](chat-vs-mcp.md)を参照してください。

<a id="verify-the-connection"></a>
## 接続を確認

Godot プロジェクトを開き、MCP アプリへ次のように依頼します。

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

分離した作業では、ルーティングモード `bound`、想定する正規 Project Root、バインド先エディターの状態 `connected`、そのエディターのファイルシステム準備状況が報告されることを確認してください。一致するエディターが開いていないバインド済み接続は存続し、エディターが再接続するまで再試行可能な `bound_project_not_connected` を報告します。

ステータスが `legacy_unbound` を報告する場合、ドックの MCP target が互換ルートです。単一クライアントで使う場合は想定するターゲットを選択し、並行作業を始める場合は明示的な Project Binding を構成してください。

<a id="update-fennara"></a>
## Fennara を更新

ドックに **Update** が表示されたら押し、指示に従ってください。Fennara は Godot を閉じるよう求める前に、更新をダウンロードして検証します。インストール後に同じプロジェクトを開き直し、更新の検証が完了するまでは以前の動作するバージョンを維持します。

ターミナルから更新するには Godot を閉じ、次を実行します。

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Fennara v0.3.8 以前からアップグレードする場合は、`fennara update` を実行する前に、上記のプラットフォーム別インストールコマンドで CLI を一度再インストールしてください。これらの CLI は廃止されたリリースタグを照会するため、現在のリリースを見つけられません。CLI を再インストールすると、プロジェクトアドオンや設定を削除せずに、以降の更新が GitHub の Latest Release エンドポイントを使用するようになります。

> [!IMPORTANT]
> macOS で Fennara v0.3.11 からアップグレードする場合は、CLI を一度再インストールしてください。その CLI は自己更新へ進む前に既存の framework bundle を拒否します。再インストールで置き換えられるのは CLI だけで、プロジェクトアドオンと設定は維持されます。

検証に失敗した場合は、ドックの **Restore Previous Version**、**Open Logs**、または **Copy Report** を使用してください。正確なバージョン、準備、更新中断からの復旧については、[CLI 更新のリファレンス](cli.md#update-a-project)を参照してください。

<a id="troubleshooting"></a>
## トラブルシューティング

<a id="an-install-or-update-failed"></a>
### インストールまたは更新に失敗した

ドックからサニタイズ済みレポートをコピーするか、ターミナルで最新のレポートを表示します。

```bash
fennara diagnostics
```

操作 ID、JSON 出力、記録されるフィールド、秘匿化の保証については、[CLI 診断](cli.md#inspect-health-and-failures)を参照してください。

<a id="fennara-is-not-found"></a>
### `fennara` が見つからない

新しいターミナルを開き、次を実行します。

```bash
fennara doctor
```

それでもコマンドを使用できない場合は、Fennara の `bin` ディレクトリを PATH へ追加します。プラットフォーム別のパスは、[CLI インストールページ](cli.md#install-the-cli)に記載されています。

<a id="windows-binaries-fail-before-starting"></a>
### Windows バイナリが起動前に失敗する

Fennara バイナリが `VCRUNTIME` または `MSVCP` DLL の欠如、終了コード `-1073741515`、または `0xc0000135` を報告する場合は、Microsoft Visual C++ Redistributable 2015-2022 x64 をインストールします。

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

これは、それらの Microsoft ランタイム DLL がない Windows マシンでのみ必要です。

<a id="a-release-requires-a-newer-cli"></a>
### リリースに新しい CLI が必要

CLI の自己更新で必要なバージョンをインストールできない場合は、[CLI をインストール](cli.md#install-the-cli)のインストールスクリプトを再実行してから、コマンドをもう一度実行します。

<a id="the-addon-is-not-visible-in-godot"></a>
### Godot にアドオンが表示されない

次のファイルが存在することを確認し、プロジェクトを開き直します。

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` が誤ったプロジェクトを表示する

最初に、報告されたルーティングモードを確認します。`bound` の場合は、想定する `--project-path`、`FENNARA_PROJECT_PATH`、またはプロジェクト起動ディレクトリを指定して MCP プロセスを再起動します。`legacy_unbound` の場合は、想定するプロジェクトを開き、Fennara ドックの MCP target コントロールで選択します。

バインドされたルートが `not_connected` の場合は、そのプロジェクトを開いてアドオンが接続するまで待ちます。`ambiguous` の場合は、重複しているエディターを閉じるか、別の worktree から起動してください。

<a id="c-diagnostics-are-missing"></a>
### C# 診断が表示されない

プロジェクトに明確な `.csproj`、`.sln`、または `.slnx` が 1 つ含まれていることを確認してから、次を実行します。

```bash
dotnet --version
```

ブラウザーランタイムのレイアウト、手動復旧、実装の詳細については、[アーキテクチャ](architecture.md)、[手動インストール](manual-install.md)、[FAQ](faq.md)を参照してください。
