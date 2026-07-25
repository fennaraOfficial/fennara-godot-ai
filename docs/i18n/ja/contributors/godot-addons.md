<!-- fennara-i18n: locale=ja source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Godot アドオン

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · **日本語** · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

このディレクトリは Godot プロジェクト内で必要な構造を再現します。

```text
res://addons/
  fennara/
```

リポジトリのペイロードを `godot_demo/addons/` に置くことで、パッケージングとローカルテスト用スクリプトはパスを組み替えずにアドオンをコピーできます。

<a id="current-addon"></a>
## 現在のアドオン

`fennara/` はインストール可能な Fennara Godot AI アドオンです。

- `fennara.gdextension`: ネイティブ拡張の Godot entry point
- `bin/`: `fennara-cpp/` からビルドするプラットフォーム別 editor バイナリ
- `dist/`: `ui/chat/` から同期する生成済みネイティブチャット webview アセット
- `runtime/`: リポジトリルートの `runtime/` から同期する Godot 側ヘルパー
- `debugger/`: debugger 側のアドオンアセット
- `VERSION`: パッケージされたアドオンのバージョンマーカー

<a id="rules"></a>
## ルール

- アドオン内の相対パスを安定させてください。ユーザープロジェクトでは `res://addons/fennara/` になります。
- package preview zip、release zip、ダウンロードした CEF archive、ログ、ローカルテスト出力をここへ置かないでください。
- `fennara/dist/` の生成ファイルを直接編集しないでください。意図的に生成出力へパッチする場合も、ソースを変更して同期してください。
- `fennara/runtime/` の同期済みヘルパーを直接編集しないでください。`runtime/` も更新し、`node scripts/sync-runtime.mjs` を実行します。
- Godot プロジェクトへコピーすることを意図したものだけを新しいアドオンペイロードとして追加してください。
