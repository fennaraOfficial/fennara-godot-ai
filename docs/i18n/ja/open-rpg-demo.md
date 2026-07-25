<!-- fennara-i18n: locale=ja source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Open RPG デモ解説

<!-- fennara-doc-nav:start -->
[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · **日本語** · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

動画:

https://www.youtube.com/watch?v=0Egu3S-9MM0

このデモでは、GDQuest のオープンソース Godot 4 Open RPG プロジェクトで Fennara MCP を試します。

AI が空のプロジェクトを一から作ったことが要点ではありません。AI エージェントが既存の Godot RPG コードベースで作業し、間違い、Godot からフィードバックを受け、実装を修正して作業を続けたことが重要です。

<a id="project"></a>
## プロジェクト

GDQuest Godot 4 Open RPG:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## 作業内容

熊のプレイヤー戦闘キャラクター Baloo が既存の戦闘に勝利した後、Tactical Guard という新しい戦闘能力を習得する成長機能を追加します。

この能力には次の要件がありました。

- 敵 1 体を対象にする
- 適度なダメージを与える
- Baloo の Defense を上げる
- 習得後に Baloo の戦闘アクションメニューへ表示する
- 習得後に `Baloo learned Tactical Guard!` のようなメッセージを表示する

<a id="what-happened"></a>
## 実際に起きたこと

AI コーディングエージェントは Fennara MCP を通じて実行中の Godot プロジェクトへ接続し、プロジェクトのアーキテクチャを調べました。

使用した Fennara ツール:

- シーンツリーの検査
- ノードプロパティの検査
- GDScript 診断
- シーン検証
- 実行時エラーのフィードバック
- プロジェクトとシーンの検査

最初の実装は完全には動きませんでした。そこがこのデモの有用な部分です。

Fennara が Godot からフィードバックを返し、エージェントは壊れたスクリプトを修正し、実装を調整して、ゲーム内で機能するまで作業を続けました。

<a id="why-this-matters"></a>
## なぜ重要なのか

空のデモは簡単です。AI エージェントが壊れやすいのは既存プロジェクトです。

Fennara の主張は、Godot AI エージェントにはエンジンからのフィードバックが必要だということです。

- スクリプトを解析できたか
- シーンは検証に合格したか
- ランタイムがエラーを出したか
- エージェントは実際のプロジェクト構造を調べたか
- 完了したふりをせず、間違いを修正できるか

従来型 MCP は AI にコマンドを与えます。

Fennara は AI に Godot からのフィードバックを与えます。
