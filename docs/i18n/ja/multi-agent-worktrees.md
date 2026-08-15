<!-- fennara-i18n: locale=ja source=docs/multi-agent-worktrees.md sha256=037571da5f2479b968f6b74e8994d0272042d4a26616c50af5f62f3d2a9bbcf9 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# 複数エージェントと Godot worktree

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · **日本語** · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

1 台のマシン上で、複数のコーディングエージェントを別々のリポジトリや worktree に割り当て、あるエージェントが選んだターゲットによって別のエージェントの接続先が変わらないようにします。各プロジェクトは専用の Fennara MCP プロセスと接続を持ち、すべてのプロジェクトが同じユーザー単位のデーモンを共有します。

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

編集、検査、範囲を限定したシーン検証、単独のスクリーンショット呼び出しは並行して実行できます。デーモン管理の対話型ゲーム実行は、マシン全体で 1 つの Runtime Slot を通して直列化されます。

<a id="one-mcp-connection-per-project"></a>
## プロジェクトごとに 1 つの MCP 接続

MCP プロセスは起動時に、安定した Project Root を 1 つ選びます。この MCP Project Binding は `project.godot` を含むディレクトリの正規ファイルシステム識別情報であり、プロジェクト名や Godot のプロセス ID ではありません。

リポジトリまたは worktree ごとに、別々の MCP プロセスと接続を使ってください。複数のエージェントが意図的に同じプロジェクトで作業する場合に限り、1 つの接続を共有できます。Fennara ツールは呼び出し単位のプロジェクトセレクターを公開しないため、モデルが誤ってプロセスを別のプロジェクトへ切り替えることはありません。

各プロジェクトには、Fennara を有効にした接続済みの Godot エディターも必要です。エディターが終了し、別のプロセス ID で再接続した場合でも、同じ Project Root が再接続すれば、既存の MCP プロセスはルーティングを再開します。

<a id="how-a-process-chooses-its-project"></a>
## プロセスがプロジェクトを選ぶ方法

MCP ランタイムは起動時の作業ディレクトリを記録し、次の優先順位でバインドを一度だけ選びます。

1. `--project-path <path>` または `--project-path=<path>`。
2. `FENNARA_PROJECT_PATH`。
3. 起動ディレクトリから最も近い、`project.godot` を含む祖先ディレクトリ。
4. 自動検出で Godot プロジェクトが見つからない場合の legacy-unbound
   互換モード。

コマンドラインと環境変数のパスは、明示的な指定として扱われます。空、アクセス不能、存在しない、ディレクトリでない、Godot プロジェクトでない、または未対応のパスを指定すると MCP サーバーは起動せず、別のプロジェクトへフォールバックすることもありません。相対パスは、記録された起動ディレクトリを基準に解決されます。MCP ホストの起動ディレクトリが不明な場合は絶対パスを推奨します。

Fennara はホスト固有のワークスペース変数を暗黙には使用しません。MCP ホストは、独自のワークスペース値を `--project-path` または `FENNARA_PROJECT_PATH` へ割り当てられます。

<a id="configure-a-project-bound-connection"></a>
## プロジェクトへバインドした接続を構成する

`fennara mcp-setup` は、グローバルかつプロジェクトに依存しないままです。プロジェクト内で実行しても、以後のすべての MCP プロセスがそのプロジェクトへバインドされるわけではありません。安定したランチャーパスは維持し、MCP ホストのプロジェクト設定またはワークスペース設定でバインドを追加してください。

JSON 形式の構成では次のようにします。

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

環境変数を使うこともできます。

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Codex 形式の TOML では次のようにします。

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

次のエージェントは、専用のプロジェクト設定またはワークスペース設定で `/absolute/path/to/worktree-b` を指定します。ホストがプロジェクトディレクトリごとに確実に別の MCP プロセスを起動する場合は、祖先ディレクトリの検出によって明示的なパスと同じバインドを得られます。

<a id="mcp-host-boundaries"></a>
## MCP ホストごとの制約

プロジェクトローカルの構成と起動ディレクトリの動作は、ホストによって異なります。

- VS Code の単一フォルダーワークスペースでは、ホストが文書化している子プロセスの作業ディレクトリを利用できますが、それでも明示的なプロジェクトバインドが最も分かりやすい構成です。
- Claude Code、Gemini CLI、Antigravity、Cline、Cursor、OpenCode、Kiro、Codex は、プロジェクトまたはワークスペースの構成を利用できます。分離を保証する必要がある場合は、明示的なバインドか、文書化されたプロジェクト起動ディレクトリを使ってください。
- Claude Desktop と従来の Windsurf/Cascade の構成はグローバルです。既定の Fennara エントリは legacy-unbound のままで、プロジェクトローカルの自動分離を提供できません。上級ユーザーは、異なる明示的なパスを持つ別名のグローバルエントリを作成できますが、正しいエントリを自分で選ぶ必要があります。

プロジェクトローカル構成の自動生成、および新しい Windsurf/Devin Local への対応は、このワークフローの対象外です。

<a id="start-and-verify-the-editors"></a>
## エディターを起動して確認する

各 worktree には、Fennara を有効にした専用の Godot エディターが必要です。ヘッドレスエディターでは、Fennara のデーモンを共有しながら別々の Godot LSP ポートを使用できます。

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

LSP ポートは Godot が使用します。Fennara は引き続き、通常のループバックアドレス上にある 1 つの共有デーモンを使います。

並行作業を始める前に、各エージェントから `fennara_status` を実行してください。次の項目が報告されることを確認します。

- ルーティングモード `bound`
- 想定するバインド元と正規 Project Root
- バインド先エディターの状態 `connected`
- そのエディターのファイルシステム準備状況

自動検出でプロジェクトが見つからない場合、ステータスは `legacy_unbound` と並行利用に関する警告を報告します。この互換モードでは、ドックで選択した MCP Target が最初に使われ、次に、接続中のエディターが 1 つだけの場合はそのエディターが使われます。分離された並行作業に、バインドされていない接続を使わないでください。

<a id="missing-and-duplicate-editors"></a>
## エディターの不在と重複

有効な Project Binding は、対応するエディターが不在でも存続します。ツール呼び出しは、その Project Root が再接続するまで再試行可能な `bound_project_not_connected` を返し、ドックのターゲットへフォールスルーすることはありません。

同じ Project Root として解決されるエディターが 2 つある場合は、`ambiguous_project_binding` が返ります。重複するエディターを閉じるか、別の worktree を割り当ててください。Fennara は、プロセス ID、接続順、プロジェクト名、ドックのターゲットのいずれによっても選択しません。

同じプロジェクトへのシンボリックリンクの別名は、同一のライブファイルシステム識別情報として解決されます。MCP の起動後にシンボリックリンクの参照先を変えてもバインドは変わりません。その MCP プロセスを再起動して、改めてバインドしてください。

<a id="serialized-runtime-sessions"></a>
## 直列化される Runtime Session

デーモン管理のゲーム実行では、すべてのプロジェクトがマシン全体で 1 つの Runtime Slot を共有します。別のプロジェクトがセッションを開始中または実行中の場合、`runtime_session.start` は `availability: "busy"`、`slot_acquired: false`、推奨値 `retry_after_ms` を含む、成功したドメイン結果 `busy` を返します。所有者、セッション ID、プロセス ID、シーン、ログ、キュー内の位置、予想所要時間は公開されません。

FIFO キューはありません。推奨される再試行間隔の付近でジッターを加えてポーリングし、各 `runtime_session.start` を最終的なアトミック要求として扱ってください。空き状態は予告にすぎません。事前確認の後で別のエージェントが競合に勝つ可能性があります。

所有する Project Root だけが、その Runtime Session の検査、更新、スクリプト実行、停止を行えます。所有者によるステータス確認によって、120 秒の無操作期限が延長されます。上限付きの所有者ランタイム操作は、実行中の無操作期限切れを一時停止し、終端スクリプト結果を返した場合にのみ期限を更新します。タイムアウト、セットアップエラー、キャンセルでは更新されません。実行中は、およそ 30 秒間隔でジッターを加えて所有者ステータスをポーリングしてください。

Runtime Lease の既定の絶対期限は 900 秒です。`max_run_seconds` には 86,400 秒以下の正の整数を指定できます。たとえば、1 時間かかる見込みのリグレッションテストでは、安全のために 4,500 秒を指定できます。絶対期限が一時停止されることはありません。自然終了、明示的な停止、起動失敗、無操作期限、絶対期限のいずれかに達すると、ゲームが停止または回収され、Runtime Slot が解放されます。

<a id="safe-multi-agent-checklist"></a>
## 安全なマルチエージェント用チェックリスト

1. プロジェクトごとに別々のリポジトリまたは worktree を作成します。
2. Fennara をインストールし、各 Project Root に対して Godot エディターを 1 つ開きます。
3. プロジェクトごとに、バインド済み MCP プロセスを 1 つ構成します。
4. 各エージェントから `fennara_status` を実行し、正規ルートを確認します。
5. 編集、検査、範囲を限定したシーン検証、単独のスクリーンショットを並行して進めます。
6. プレイテストでは、非エラーの `busy` 結果をポーリングして再試行します。実行中は、取得できたセッションを所有者ステータスによって維持します。
