<!-- fennara-i18n: locale=ja source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
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

- `GET /health`: daemon health
- `GET /status`: daemon status と接続済み Godot plugin metadata
- `POST /tools/call`: ツール呼び出しを接続済み Godot plugin へ転送し、結果を待つ
- `WS /godot/ws`: ローカル Godot plugin bridge。接続後に plugin が `hello` を送る

開発用バイナリ:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP サーバー

`crates/fennara-mcp` は JSON-RPC を stdio で通信するローカル MCP サーバーです。

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

- `fennara_status`: MCP サーバーがインストールされ到達可能かを確認し、デーモン実行時は daemon と Godot bridge の状態を返します。
- `write_or_update_file`、`run_scene_edit_script`、`get_scene_tree`、`script_diagnostics`、`screenshot_scene` などの Godot プロジェクトツールは、デーモンを経由して接続済み Godot plugin へ転送されます。

将来 Windows にインストールされるユーザーパス:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
