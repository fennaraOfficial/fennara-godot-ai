<!-- fennara-i18n: locale=ja source=docs/fennara-vs-traditional-godot-mcp.md sha256=e45a741b1db7c20e40b4a311c198af216172dfa024ca9c123db4f9336c9a6e7f -->
<a id="fennara-vs-traditional-godot-mcp"></a>
# Fennara と従来の Godot MCP の比較

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../fennara-vs-traditional-godot-mcp.md) · [简体中文](../zh-CN/fennara-vs-traditional-godot-mcp.md) · [Español](../es/fennara-vs-traditional-godot-mcp.md) · [Português do Brasil](../pt-BR/fennara-vs-traditional-godot-mcp.md) · **日本語** · [한국어](../ko/fennara-vs-traditional-godot-mcp.md) · [Русский](../ru/fennara-vs-traditional-godot-mcp.md) · [Français](../fr/fennara-vs-traditional-godot-mcp.md) · [Deutsch](../de/fennara-vs-traditional-godot-mcp.md) · [Türkçe](../tr/fennara-vs-traditional-godot-mcp.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../fennara-vs-traditional-godot-mcp.md)
<!-- fennara-doc-nav:end -->

| 従来のコマンドブリッジ | Fennara のフィードバックループ |
| --- | --- |
| エディター操作を公開する | Godot を理解した調査、操作、検査を公開する |
| コマンドの成功が処理の終点になり得る | 診断、検証、実行時ログ、スクリーンショットが次の手順に情報を与える |
| 直接的で既知の編集に最適 | エージェントが調査、変更、検証、復旧を行う必要がある場合に最適 |

多くの Godot MCP サーバーは、AI クライアントへエディターコマンドを公開します。

たとえば、次のようなコマンドです。

- ノードを作成する
- プロパティを設定する
- シーンを開く
- シーンを保存する
- ログを読む
- スクリーンショットを撮る
- プロジェクトを実行する
- シグナルを接続する
- input map を編集する
- マテリアルを管理する
- テストを実行する

これは有用です。Godot を API サーフェスへ変換します。

しかし、実際の AI ゲーム開発で難しいのは、AI が `set_property` を呼び出せるかどうかではありません。

難しいのは、プロジェクトが壊れたときに AI がそれを判断できるかどうかです。

<a id="traditional-mcp-pattern"></a>
## 従来の MCP パターン

```text
AI calls editor command.
Editor returns result.
AI guesses next step.
```

この方法は、小さく直接的な編集ではうまく機能します。

例:

```text
Rename Camera3D to MainCamera.
```

しかし、エージェントがアーキテクチャを調査し、スクリプト、リソース、シーンを編集し、失敗を確認して復旧する必要がある、より大きなプロジェクトタスクには弱い方法です。

<a id="fennara-pattern"></a>
## Fennara のパターン

```text
AI changes project.
Godot feedback comes back.
AI patches and reruns until it works.
```

Fennara はフィードバックを重視します。

- GDScript 診断
- シーン検証
- 実行時エラー
- シーンツリーの調査
- ノードプロパティ
- クラスと API の調査
- スクリーンショット
- 生成されたプロジェクトガイダンス
- 修正して再実行するワークフロー

<a id="the-difference"></a>
## 違い

従来の Godot MCP は次のように問いかけます。

```text
What editor commands should we expose?
```

Fennara は次のように問いかけます。

```text
What feedback does the model need to successfully build inside Godot?
```

コマンドは最低条件です。

フィードバックこそが競争優位です。
