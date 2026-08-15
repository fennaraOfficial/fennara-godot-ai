<!-- fennara-i18n: locale=ja source=local/README.md sha256=b63443a8ff7310d53044b20261e7f206a949bad07299656235ba39fe69f92d9d -->
<a id="fennara-local-tools"></a>
# Fennara ローカルツール

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · **日本語** · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

このフォルダーには Fennara のローカルネイティブコンポーネントがあります。

<a id="daemon"></a>
## デーモン

`crates/fennara-daemon` はローカルデーモンを次のアドレスで実行します。

```text
http://127.0.0.1:41287
```

エンドポイント:

- `GET /health`: デーモンのヘルスチェック。
- `GET /status`: デーモンの状態と接続済み Godot プラグインのメタデータ。
- 特権付きのバインド済みステータス: 1 つの MCP プロセスの正規 Project Root を、接続中の Godot エディターセッションと照合します。
- `POST /tools/call`: ツール呼び出しを接続済み Godot プラグインへ転送し、ツール結果を待ちます。
- `WS /godot/ws`: ローカル Godot プラグインブリッジ。プラグインは接続後に `hello` メッセージを送ります。

現在のユーザーが使用する、Fennara を有効にしたすべてのエディターと外部 MCP プロセスは、1 つのデーモンを共有します。バインド済みの外部要求は正規 Project Root でルーティングされ、内蔵チャットの内部要求は Godot Editor Session に結び付けられたままになり、legacy-unbound MCP 要求はドックで選択した互換ターゲットを使います。

デーモンは、マシン全体で 1 つの Runtime Slot も所有します。エディターが再接続しても制御権が移らないよう、Runtime Session の所有権と更新可能なリース状態は Project Root に関連付けられます。

開発用バイナリ:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP サーバー

`crates/fennara-mcp` はローカル MCP サーバーです。JSON-RPC を stdio で通信するため、MCP クライアントはローカルプロセスとして起動できます。

各 MCP プロセスは起動時に、任意の Project Binding を 1 つ固定します。選択順は `--project-path`、`FENNARA_PROJECT_PATH`、起動ディレクトリから最も近い `project.godot` の祖先ディレクトリです。プロジェクトが見つからない場合は自動的に legacy-unbound 互換モードへ入り、明示的なパスが無効な場合は起動に失敗します。プロジェクトをまたいで分離するには、プロジェクトごとに 1 つの MCP プロセスと接続を使ってください。

`crates/fennara-project-identity` は MCP ランタイムとデーモンが共有します。Project Root の検出、検証、正規化、損失のないプロトコル変換、稼働中のファイルシステム上の同一性判定を担当します。

`fennara-mcp` は起動時に `local/schemas/tools/` から選択した MCP 向けスキーマを埋め込み、呼び出しをローカルデーモンへ転送します。実行時に外部スキーマサービスは不要です。内蔵チャットは同じスキーマディレクトリから、関連する別のツール集合を選びます。

`fennara install` は `local/templates/` からプロジェクトガイダンスも生成します。

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

ビルド:

```powershell
cd local
cargo build
```

Windows の terminal で Rust PATH がまだ更新されていない場合:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

開発用バイナリ:

```text
local/target/debug/fennara-mcp.exe
```

現在のツール:

- `fennara_status`: MCP サーバーがインストール済みで到達可能かを確認し、デーモンの実行中はルーティングモード、バインド元とルート、選択されたエディターの状態、Godot ブリッジの準備状況を報告します。
- `write_or_update_file`、`run_scene_edit_script`、`get_scene_tree`、`script_diagnostics`、`screenshot_scene` などの Godot プロジェクトツールは、デーモンを経由して接続済み Godot plugin へ転送されます。

将来 Windows にインストールされるユーザーパス:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
