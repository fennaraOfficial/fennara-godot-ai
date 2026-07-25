<!-- fennara-i18n: locale=ja source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# ランタイムヘルパー

<!-- fennara-doc-nav:start -->
[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · **日本語** · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

このフォルダーは `runtime_session` と `runtime_script` が使う Godot 側ランタイムヘルパースクリプトのソースです。

パッケージされたコピー:

```text
godot_demo/addons/fennara/runtime/
```

変更後に実行します。

```bash
node scripts/sync-runtime.mjs
```

インストール済み Godot プロジェクト内では、ランタイムスクリプトが `res://addons/fennara/runtime/` からヘルパーを読み込みます。ヘルパーはプリミティブでプロジェクト非依存に保ってください。入力、待機、ノードスナップショット、キャプチャ、物理クエリ、シーンライフサイクルは適切です。ゲーム固有の移動、戦闘、クエスト、インベントリ、UI フローの前提は適切ではありません。

`image_sheet.gd` は screenshot script facade からも使われます。合成結果を決定的にし、シーン、アニメーション、ゲームプレイ状態に依存させないでください。
