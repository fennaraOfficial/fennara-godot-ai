<!-- fennara-i18n: locale=ja source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Fennara チャット UI

<!-- fennara-doc-nav:start -->
[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · **日本語** · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

このフォルダーはオプションの editor 内チャット画面のソースです。

最初のバージョンは意図的にビルド不要です。プレーンな HTML、CSS、JavaScript を使うことで、OSS リポジトリを調べやすくし、webview host と daemon chat bridge が確定する前にフロントエンド toolchain を追加しないようにしています。

パッケージされたコピーは `godot_demo/addons/fennara/dist/` にあります。

このフォルダーを編集した後に実行します。

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## デザイン上の注意

- Godot editor の画面に合わせ、コンパクトな操作、控えめなコントラスト、小さな角丸、明確な focus state を使います。マーケティングページ風の hero 表現は使いません。
- ローカル Fennara daemon/chat API だけを使い、ホストサービスを必須にしません。
- OpenRouter は、ユーザーが提供し Godot プロジェクト外にローカル保存するキーを使います。
- モデル未接続でも status、settings、transcript、composer state が確認できる UI にします。
