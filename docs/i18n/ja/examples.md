<!-- fennara-i18n: locale=ja source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# 使用例

<!-- fennara-doc-nav:start -->
[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · **日本語** · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../examples.md)
<!-- fennara-doc-nav:end -->

プロンプトをコピーしてプロジェクトの詳細を置き換え、MCP アプリまたは Fennara 内蔵チャットから送信してください。

| 目的 | 例 |
| --- | --- |
| 接続中のエディターを確認する | [接続を確認](#check-connection) |
| 既存プロジェクトを理解する | [編集前に調査](#inspect-a-project-before-editing) |
| 対象を絞った変更を行う | [アーキテクチャを考慮した変更](#make-a-small-architecture-aware-change) |
| 実行中のプロジェクトを診断する | [実行時エラー](#debug-a-runtime-error) |
| レンダリング結果を調査する | [視覚的フィードバック](#visual-feedback) |

<a id="check-connection"></a>
## 接続を確認

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## 編集前にプロジェクトを調査

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## アーキテクチャを考慮した小さな変更を行う

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## 実行時エラーをデバッグ

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## 視覚的フィードバック

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## 内蔵チャットのプロバイダー設定

Godot 内の Fennara ドックで、次を入力します。

```text
/provider
```

クラウドプロバイダーまたはローカルプロバイダーを接続します。

続いて、次を入力します。

```text
/model
```

ドックで使用するモデルを選択します。

<a id="existing-project-demo-prompt"></a>
## 既存プロジェクト向けデモプロンプト

これは Open RPG デモで使用された種類のプロンプトです。

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
