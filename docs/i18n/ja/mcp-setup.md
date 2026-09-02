<!-- fennara-i18n: locale=ja source=docs/mcp-setup.md sha256=86c9fe3fc7a69c2ade417dd01a0ccabb05ddaa91cf417fa8559c28d4b01811bd -->
<a id="mcp-setup"></a>
# MCP セットアップ

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · **日本語** · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

外部 AI アプリを Fennara の Godot ツールへ接続します。そのアプリでは、引き続き独自のモデルアカウント、サブスクリプション、または API 設定が使用されます。

> [!NOTE]
> これは Fennara 内蔵チャットを構成するものではありません。どちらの経路が必要か分からない場合は、[MCP アプリと内蔵チャット](chat-vs-mcp.md)を参照してください。

<a id="quick-setup"></a>
## クイックセットアップ

1. Godot ドックで **Set Up Fennara** を完了します。
2. **Chat Settings > MCP Apps** を開きます。
3. 使用するアプリを見つけて **Set Up** を押します。
4. アプリを再起動します。

Fennara はアプリの MCP 構成を変更する前にバックアップを作成します。統合された **Claude** オプションは、Claude Code と Claude Desktop を構成します。**Gemini & Antigravity** は、共有される両方の対象を構成します。

<a id="terminal-alternative"></a>
### ターミナルを使う方法

まず Godot プロジェクト内で `fennara install` を実行し、次に対象を選択します。

| アプリ | コマンド |
| --- | --- |
| Claude Code と Claude Desktop | `fennara mcp-setup --claude` |
| Claude Code のみ | `fennara mcp-setup --claude-code` |
| Claude Desktop のみ | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini と Antigravity | `fennara mcp-setup --gemini` または `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

インストール済み CLI が対応する対象の一覧を確認するには、`fennara mcp-setup --help` を実行します。

セットアップでは、グローバルでプロジェクトに依存しないランチャーエントリが書き込まれます。プロジェクト内で `fennara mcp-setup` を実行しても、以後のすべての接続がそのプロジェクトへバインドされるわけではありません。

<a id="bind-a-connection-to-one-project"></a>
## 接続を 1 つのプロジェクトへバインドする

同じマシン上に複数のリポジトリや worktree がある場合は、プロジェクトごとに 1 つの MCP プロセスと接続を実行します。そのプロセスを MCP ホストのプロジェクト設定またはワークスペース設定で、次のいずれかを使って構成します。

```text
--project-path /absolute/path/to/godot-project
```

または:

```text
FENNARA_PROJECT_PATH=/absolute/path/to/godot-project
```

ランタイムは起動時に一度だけ、次の優先順位で Project Binding を選択します。

1. `--project-path`
2. `FENNARA_PROJECT_PATH`
3. 起動ディレクトリから最も近い、`project.godot` を含む祖先ディレクトリ
4. 検出でプロジェクトが見つからない場合の legacy-unbound 互換モード

明示的なパスが無効な場合、MCP サーバーは起動しません。ドックのターゲットや別のエディターへフォールスルーすることはありません。有効なバインドは、対応するエディターが一時的に不在でも存続し、その Project Root が再接続すると復旧します。モデル向けのツール呼び出し単位でプロジェクトを上書きする機能はありません。

構成例、ホストごとの対応範囲、ステータスの確認方法、重複エディターの動作、直列化されたプレイテストについては、[複数エージェントと worktree](multi-agent-worktrees.md) を参照してください。

<a id="manual-setup"></a>
## 手動セットアップ

手動セットアップは、使用するアプリが一覧にない場合、セットアップコマンドがアプリの構成ファイルを見つけられない場合、または意図的に MCP 構成を手で編集する場合にだけ使用してください。

編集する前に、構成ファイルのバックアップを作成します。次に、安定した Fennara MCP ランチャーを指す `fennara` という名前のローカル stdio MCP サーバーを追加します。

既定のランチャーパスは次のとおりです。

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

使用するマシンの実際の絶対パスを指定してください。MCP アプリに `versions/<version>/fennara-mcp-runtime` を直接指定しないでください。`bin/` にある安定したランチャーを使うことで、Fennara の更新後もアプリの構成が機能し続けます。

<a id="json-mcpservers"></a>
### JSON の `mcpServers`

多くの MCP アプリでは、トップレベルの `mcpServers` オブジェクトを使用します。

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

同じ `mcpServers` キーを使いながら `command` だけを必要とするアプリもあります。既存の構成に他のサーバーがすでに含まれている場合は、それらのエントリを維持し、`fennara` サーバーだけを追加してください。

分離したままにする必要があるプロジェクトローカルのエントリでは、バインドを `args` に追加します。

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/godot-project"],
      "env": {}
    }
  }
}
```

Cline 形式の構成では、秒単位のより長いツールタイムアウトを含めることもできます。

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### VS Code 形式の JSON `servers`

VS Code のユーザーまたはプロジェクト MCP 構成を含む一部のクライアントでは、トップレベルの `servers` オブジェクトを使用し、`type: "stdio"` を指定する必要があります。

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### OpenCode 形式の JSON `mcp`

OpenCode 形式の JSON 構成では、トップレベルの `mcp` オブジェクトを使用します。タイムアウトの単位はミリ秒です。

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### Codex 形式の TOML

Codex は TOML を使用します。

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

TOML ファイルへ JSON を貼り付けたり、JSON ファイルへ TOML を貼り付けたりしないでください。アプリですでに使用されている形式に合わせます。

Codex 形式のエントリをバインドするには、安定したランチャーは変えずに引数を追加します。

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/godot-project"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

<a id="common-config-locations"></a>
## 一般的な構成ファイルの場所

以下は、Fennara のセットアップヘルパーと現在の MCP クライアントが使用する一般的な場所です。アプリによって構成パスが変更されることがあり、グローバル構成とプロジェクトローカル構成の両方に対応するアプリもあります。アプリに **Open MCP Config** のようなコマンドがある場合は、推測せずにそのコマンドを使用してください。

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

VS Code の単一フォルダーワークスペースでは、プロジェクトが MCP 子プロセスの起動ディレクトリとして渡されることがあります。Claude Code、Gemini CLI、Antigravity、Cline、Cursor、OpenCode、Kiro、Codex はプロジェクトまたはワークスペース単位の構成を利用できます。分離を保証する必要がある場合は、明示的なバインドか、文書化されたプロジェクト起動ディレクトリを使ってください。

Claude Desktop と従来の Windsurf/Cascade は、このワークフローではグローバル構成を使用します。既定のセットアップは legacy-unbound のままです。上級ユーザーは、異なる明示的なプロジェクトパスを設定した、別名のグローバルエントリを作成できますが、これらのアプリはプロジェクトローカルの自動分離を提供しません。

<a id="timeout-guidance"></a>
## タイムアウトの指針

Fennara の一部のツールは、Godot にシーン検証、実行時状態の調査、スクリーンショットの取得、診断の実行を依頼する場合があるため、小さな既定の MCP タイムアウトよりも長い時間がかかる可能性があります。

クライアントが対応している場合は、ツールごとに長めのタイムアウトを使用してください。

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

クライアントがサーバーごとのタイムアウトに対応していない場合は、そのクライアントで文書化されているグローバル MCP タイムアウト設定を使用します。

<a id="verify-the-connection"></a>
## 接続を確認

Godot プロジェクトを開き、MCP アプリへ次のように依頼します。

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

分離した作業では、ステータスがルーティングモード `bound`、想定するバインド元と正規 Project Root、バインド先エディターの状態 `connected`、およびそのエディターのファイルシステム準備状況を報告することを確認してください。

ステータスが `legacy_unbound` を報告する場合、その接続では Project Root を自動検出できていません。ドックの **MCP target** 互換ルートを使用し、このモードが分離された並行作業には安全でないことを警告します。

<a id="troubleshooting"></a>
## トラブルシューティング

Fennara が MCP アプリに表示されない場合:

- ランチャーパスが絶対パスであり、実際に存在することを確認する
- アプリの要件に応じて、構成の構文が有効な JSON、JSON5、または TOML であることを確認する
- サーバー名が `fennara` であることを確認する
- 編集した構成ファイルをアプリが読み込んでいることを確認する
- MCP アプリを完全に終了してから、もう一度開く
- Godot プロジェクトに Fennara アドオンがインストールされていることを確認する
- バインド済み接続では、明示的なパスまたは起動ディレクトリが想定する Godot Project Root であることを確認する
- ステータスが `bound_project_not_connected` を報告する場合は、そのプロジェクトを Godot で開き、アドオンが接続するまで待つ
- ステータスが `ambiguous_project_binding` を報告する場合は、重複しているエディターを閉じるか、別の worktree から開く
- legacy-unbound 接続では、想定するプロジェクトがドックの MCP target として選択されていることを確認する

<a id="unsupported-mcp-apps"></a>
## 未対応の MCP アプリ

MCP アプリが一覧にない場合は、まずそのアプリの公式な MCP 構成の場所と形式を調べます。次に、LLM へ最小限で安全な編集を依頼します。

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

保存する前に結果を確認し、その後 MCP アプリを再起動します。
