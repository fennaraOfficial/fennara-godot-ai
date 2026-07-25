<!-- fennara-i18n: locale=ja source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# Fennara CLI

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · **日本語** · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../cli.md)
<!-- fennara-doc-nav:end -->

ターミナルを使用したい場合、診断または復旧が必要な場合、あるいは正確なバージョンを指定して自動インストールしたい場合は、CLI を使用してください。

> [!TIP]
> macOS では CLI が推奨されるインストール方法です。ブラウザーでダウンロードしたアドオン ZIP を手動で展開し、そのネイティブライブラリが Finder の隔離属性を引き継いだ場合に発生することがある macOS のセキュリティ通知を回避できます。

<a id="common-flow"></a>
## 一般的な流れ

```bash
cd path/to/your-godot-project
fennara install
```

ローカルインストールの調査または修復が必要な場合は、`fennara doctor` を使用してください。

通常の Godot での利用手順については[セットアップ](setup.md)を参照してください。このページはターミナルコマンドのリファレンスとして使用してください。

<a id="install-the-cli"></a>
## CLI のインストール

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS および Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

手動で展開した macOS アドオンによって、すでに `libfennara.macos.editor` の通知が発生している場合は、Godot を閉じ、手動コピーした `addons/fennara/` フォルダーを削除してから `fennara install` を実行してください。そうしないと、CLI は完全な既存アドオンを保持します。

`fennara` がすぐに利用できない場合は新しいターミナルを開き、インストールを確認してください。

```bash
fennara --version
fennara doctor
```

CLI はユーザー単位でインストールされます。プロジェクトのアドオンは各 Godot プロジェクト内に残ります。共有ランチャー、バージョン管理されたランタイム、操作記録、ログ、Linux CEF は Fennara のアプリデータに置かれます。

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## コマンド一覧

| コマンド | 用途 |
| --- | --- |
| `fennara install` | プロジェクトアドオンと、それに一致するローカルコンポーネントをインストールまたは採用します |
| `fennara update` | プロジェクトとそのローカルコンポーネントを更新します |
| `fennara doctor` | ローカルインストールを調査または修復します |
| `fennara diagnostics` | サニタイズされた操作レポートを表示します |
| `fennara mcp-setup` | 外部 MCP アプリを接続します |
| `fennara prepare-export` | アドオンを含まない CI エクスポートの前に Fennara の autoload を除去します |
| `fennara recover` | 中断されたネイティブ更新を復元します |
| `fennara self-update` | インストール済み CLI だけを更新します |

インストール済みコマンドの概要を表示するには、`fennara --help` を実行してください。対応する MCP アプリの対象を確認するには、`fennara mcp-setup --help` を使用してください。

<a id="install-a-project"></a>
## プロジェクトのインストール

`project.godot` を含むフォルダー内で実行します。

```bash
fennara install
```

または、プロジェクトを明示的に指定します。

```bash
fennara install --project path/to/project
```

`--version` を指定しない場合、CLI は現在のリリースマニフェストを選択します。再現性が重要な場合は、正確なリリースを使用してください。

```bash
fennara install --project path/to/project --version <version>
```

インストールには、次の 2 つの安全な経路があります。

- 完全なアドオンが存在しない場合、CLI は選択したリリースをダウンロードして検証し、`addons/fennara` をインストールし、一致するローカルコンポーネントをインストールし、Fennara のプロジェクトガイダンスを書き込みます。
- 完全なアドオンがすでに存在する場合、CLI はその `VERSION` を読み取り、現在のプラットフォーム用ライブラリを検証し、その正確なバージョンの CLI 管理コンポーネントをインストールします。プロジェクトのアドオンは変更しません。`--version` を明示した場合は、既存アドオンと一致する必要があります。

<a id="prepare-an-addon-free-ci-export"></a>
## アドオンを含まない CI エクスポートの準備

CI のチェックアウトから `addons/fennara/` を除外する場合は、Godot を起動する前に Fennara の永続的なランタイム autoload を除去します。

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

このコマンドが編集するのは、`project.godot` 内の `_fennara_game_capture` エントリーだけです。ほかの autoload と設定は保持され、再実行しても安全です。プロジェクトの起動時には、エディターまたはエクスポートプラグインが実行される前に Godot が autoload パスを検証するため、この手順は Godot より先に実行する必要があります。代わりに、Godot を起動する前に CI で Fennara アドオンをインストールすることもできます。

<a id="update-a-project"></a>
## プロジェクトの更新

通常のターミナル更新では、対象プロジェクトの Godot を閉じてから次を実行します。

```bash
fennara update --project path/to/project
```

`--version` を指定しない場合、CLI はインストール済みアドオンの識別情報を読み取ります。安定版アドオンは GitHub の Latest リリースを解決し、ステージング版アドオンは自身の `pr-<number>` チャンネルだけを解決します。CLI の自己置換をまたぐ場合も含め、セレクターは直ちに 1 つの正確なバージョンへ固定されます。その後、CLI はリリースアセットを検証し、アドオンとバージョン管理されたローカルコンポーネントを更新し、プロジェクトガイダンスを更新し、プラットフォームの webview 前提条件を確認します。正確なリリースを明示的に選択するには、`--version <version>` を使用してください。

`--no-self-update` は、制御された自動処理、または CLI がすでに置換された後の続行を目的としています。リリースが要求する最低 CLI バージョンを回避するために使用しないでください。

> [!IMPORTANT]
> Fennara v0.3.8 以前からアップグレードする場合は、`fennara update` を実行する前に、[セットアップ](setup.md#install-from-the-terminal-recommended-on-macos)にあるプラットフォーム用インストールコマンドを使って CLI を一度再インストールしてください。これらの CLI は廃止されたリリースタグを照会するため、現在のリリースを検出できません。CLI を再インストールしても、プロジェクトのアドオンや設定は削除されません。

> [!IMPORTANT]
> macOS で Fennara v0.3.11 からアップグレードする場合は、最初に CLI を一度再インストールしてください。その CLI は自己更新に到達する前に既存のフレームワークバンドルを拒否します。再インストールで置き換わるのは CLI だけで、プロジェクトのアドオンと設定は保持されます。

<a id="prepare-while-godot-is-open"></a>
### Godot を開いたまま準備する

エディター内の更新ボタンは、次のステージング形式を使用します。

```bash
fennara update --prepare --project path/to/project
```

準備では、アドオンをダウンロードし、検証し、永続的にステージングします。Godot を閉じること、有効なアドオンを置き換えること、有効なランタイムマニフェストを切り替えること、デーモンを再起動することはありません。Godot ドックは操作レシートを監視し、デタッチされたプロセスによる終了、置換、再起動、検証の手順を開始する前にユーザーへ確認します。ドックは自身がすでに検出した正確なバージョンを渡すため、ポインターが移動しても進行中の更新は変わりません。

Fennara は一度に 1 つの有効な共有ランタイムバージョンをサポートします。Fennara が有効な別の Godot エディターが共有デーモンに接続したままの場合、アクティベーションはブロックされます。そのエディターを閉じてから再試行してください。以前のローカルバージョンとランタイムポインターは、ネットワークへ接続しなくても復旧できるように残ります。

`--prepare` は Godot 統合のための低レベルプリミティブです。ターミナル利用者は通常、Godot をすでに閉じた状態で `fennara update` を使用します。

<a id="recover-an-interrupted-update"></a>
## 中断された更新の復旧

更新後のアドオンが復旧パネルを表示できる段階まで読み込めない場合は、Godot を閉じて次を実行します。

```bash
fennara recover --project path/to/project
```

CLI が復元するのは、復旧可能な状態にある操作だけです。以前のアドオン、共有ランチャー、有効なランタイムマニフェストを復元した後、記録されている Godot 実行ファイルを再度開こうとします。サポートから操作 ID が提示された場合は、特定のトランザクションを選択してください。

```bash
fennara recover --project path/to/project --operation <operation-id>
```

完了済み、準備だけ完了済み、すでにロールバック済みの操作は拒否されます。

<a id="inspect-health-and-failures"></a>
## 正常性と失敗の調査

`doctor` は、検出したプラットフォーム、アプリデータ構成、有効なバージョン、ランチャー、ランタイム、デーモン状態、webview 前提条件を報告します。

```bash
fennara doctor
```

実行中のデーモンまたは MCP ランタイムが `current.json` より古いと報告された場合は、Godot または該当する MCP アプリを再起動し、選択されたランタイムを起動させてください。

不足している基本アプリデータディレクトリを再作成するには、`--repair` を使用します。Linux ではさらに、古くなった CEF プロセスプロファイルを削除し、完全な管理対象ランタイムがすでにインストールされている場合に現在のランタイムマーカーを修復します。

```bash
fennara doctor --repair
```

インストール、更新、復旧、自己更新の各操作は、永続化された状態とイベントを書き込みます。サニタイズされた最新レポートは次のコマンドで表示できます。

```bash
fennara diagnostics
```

以前の操作または機械可読の出力を取得するには、次を使用します。

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

レポートには、安定したエラーコード、フェーズ、コンポーネントのバージョン、選択されたアセット名、ハッシュ検証結果が含まれます。プロジェクト、ホーム、Fennara アプリデータの各パス、資格情報、Bearer トークン、URL クエリは秘匿されます。チャットメッセージ、プロバイダーキー、プロジェクトファイルの内容は含まれません。

<a id="configure-an-external-mcp-app"></a>
## 外部 MCP アプリの設定

Godot のチャットドックでは、これらのコマンドを **Chat Settings > MCP Apps** の下に表示します。Set Up ボタンはローカルデーモンへインストール済み CLI の呼び出しを要求します。そのため、ドックとターミナルの手順は同じ設定およびバックアップ実装を使用します。

対応する対象を選ぶには、`fennara mcp-setup --help` を実行してください。設定変更後は MCP アプリを再起動してください。このコマンドは外部アプリを Fennara MCP サーバーへ接続します。Godot の内蔵チャットドックで使用するモデルプロバイダーを選択するものではありません。[MCP のセットアップ](mcp-setup.md)が、対象の一覧、設定の場所、手動設定例の正式な参照先です。

<a id="update-only-the-cli"></a>
## CLI だけを更新する

通常のプロジェクト更新は、CLI の自己更新を自動的に処理します。インストール済み CLI だけを更新するには、次を実行します。

```bash
fennara self-update
fennara self-update --version <version>
```

`--version` を指定しない場合、自己更新は有効なインストール経路を維持します。安定版は GitHub の Latest リリースを使用し、ステージング版は記録されている PR チャンネルだけを使用します。

ステージング版から安定版へ自動的に移行することはありません。意図的にステージング版を離れるには、Godot を閉じて `fennara update --version <stable-version> --project <path>` を実行してください。共有アクティブバージョンが変わる前に、その正確な安定版リリースが検証されます。

サポートから要求された場合、またはプロジェクト更新でインストール済み CLI が古すぎて安全に続行できないと報告された場合に使用してください。

<a id="automation-guidance"></a>
## 自動処理のガイダンス

- 現在のディレクトリに依存するのではなく、`--project` を渡してください。
- ビルドの再現性が必要な場合は、`--version` を固定してください。
- 失敗時に表示された操作 ID とログパスを保存してください。
- 構造化レポートには `fennara diagnostics --operation <id> --json` を使用してください。
- `current.json`、バージョンディレクトリ、更新レシート、ステージング済みアドオンフォルダーを手動で編集しないでください。
- 対象プロジェクトを Godot で開いている間は、通常のアドオン置換更新を実行しないでください。エディター内の更新手順を使用するか、先に Godot を閉じてください。
