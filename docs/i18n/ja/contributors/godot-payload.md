<!-- fennara-i18n: locale=ja source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Godot ペイロード

<!-- fennara-doc-nav:start -->
[ドキュメント](../README.md)

🌐 **言語**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · **日本語** · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

このディレクトリは、ユーザープロジェクトへコピーされ、リリースアーカイブへパッケージされる Godot 側アドオンペイロードのソースツリーです。

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` は通常の Godot アドオンディレクトリとしてインストール可能な状態を保つ必要があります。ここへ commit するものは、ユーザープロジェクトの `res://addons/fennara/` へそのまま配置できるものに限ります。

<a id="what-belongs-here"></a>
## ここに置くもの

- Godot が読み込む `addons/fennara/fennara.gdextension` と `.uid` ファイル
- プラットフォームビルドが生成する `addons/fennara/bin/` の GDExtension editor バイナリ
- ネイティブチャット webview が使う `addons/fennara/dist/` の生成済み Web アセット
- `runtime/` から同期する `addons/fennara/runtime/` の Godot 側ランタイムヘルパー
- パッケージング時にリポジトリの `VERSION` と一致する `addons/fennara/VERSION`

<a id="what-does-not-belong-here"></a>
## ここに置かないもの

- `.godot/`、`.import/`、ログ、一時ファイル、editor cache などのローカル Godot ユーザー状態
- workflow が作るルートパッケージ出力。これらは無視対象の `dist/` または `.package-preview/` に置きます。
- Fennara daemon/MCP 実行ファイルや Linux CEF ランタイムなどの共有ローカルランタイム。CLI がユーザーの Fennara アプリデータへインストールし、各 Godot アドオンへコピーしません。

<a id="generated-files"></a>
## 生成ファイル

チャット UI のソースは `ui/chat/` にあります。変更後に実行します。

```powershell
node scripts\sync-chat-ui.mjs
```

生成済み webview ファイルは `godot_demo/addons/fennara/dist/` へ同期されます。このコピーは意図的に commit します。アドオン利用者に Node.js やフロントエンドビルドを要求しないためです。

ランタイムヘルパーのソースは `runtime/` にあります。変更後に実行します。

```powershell
node scripts\sync-runtime.mjs
```

Godot 側ヘルパーは `godot_demo/addons/fennara/runtime/` へ同期されます。リリース zip に含めるため、このコピーも意図的に commit します。
