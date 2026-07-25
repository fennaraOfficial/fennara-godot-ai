<!-- fennara-i18n: locale=ja source=docs/languages.md sha256=29ca1071b436e0ff29fa5d18d9e2b09cbe64749513ea7f4e1e6471569fcb6456 -->
<a id="languages-and-translation-status"></a>
# 言語と翻訳状況

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · **日本語** · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../languages.md)
<!-- fennara-doc-nav:end -->

英語がドキュメントの正規ソースです。Fennara ではさらに、AI が執筆した完全な翻訳を 9 言語で提供しています。翻訳された各ページには現在の英語ソースへのリンクがあり、ネイティブスピーカーによるレビューをお願いしています。

| 言語 | ドキュメント | 対応範囲 | レビュー状況 |
| --- | --- | --- | --- |
| English | [英語ドキュメント](../../README.md) | 30/30 | 正規版 |
| 简体中文 | [簡体中文文档](../zh-CN/README.md) | 30/30 | ネイティブレビュー募集中 |
| Español | [Documentación en español](../es/README.md) | 30/30 | ネイティブレビュー募集中 |
| Português do Brasil | [Documentação em português](../pt-BR/README.md) | 30/30 | ネイティブレビュー募集中 |
| 日本語 | [日本語ドキュメント](README.md) | 30/30 | ネイティブレビュー募集中 |
| 한국어 | [한국어 문서](../ko/README.md) | 30/30 | ネイティブレビュー募集中 |
| Русский | [Документация на русском](../ru/README.md) | 30/30 | ネイティブレビュー募集中 |
| Français | [Documentation en français](../fr/README.md) | 30/30 | ネイティブレビュー募集中 |
| Deutsch | [Deutsche Dokumentation](../de/README.md) | 30/30 | ネイティブレビュー募集中 |
| Türkçe | [Türkçe belgeler](../tr/README.md) | 30/30 | ネイティブレビュー募集中 |

<a id="what-is-translated"></a>
## 翻訳対象

翻訳対象には、メインの README、`docs/` 直下の全ページ、`CONTRIBUTING.md`、`CONTEXT.md`、`SECURITY.md`、およびコントリビューター向けの 6 つのサブシステム README が含まれます。

法的文書、サードパーティ通知、Issue テンプレート、エージェント向け内部指示、生成されたプロジェクトガイダンス、テストフィクスチャ、ベンダードキュメントは、それぞれの権威ある形式のまま維持されます。生成ファイルや動作を担うファイルは、独立した翻訳ソースにはなりません。

<a id="freshness-and-validation"></a>
## 鮮度と検証

翻訳された各ページには、正規ソースのパスとソースハッシュが記録されます。ナビゲーションは 1 つのロケールマニフェストから生成され、安定した英語アンカーのエイリアスによって、見出しが翻訳されてもディープリンクが機能し続けます。

次のコマンドを実行してください。

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

これらのツールは文章を翻訳しません。ナビゲーションメタデータを保守し、対応範囲、鮮度、Markdown 構造、コマンド、リンク、アンカー、コードブロック、表、URL を検査するだけです。ネイティブスピーカーによる修正は、通常の Pull Request で歓迎します。

通常の同期では既存のソースハッシュが保持されるため、英語の文章を変更すると、直接更新されるまで翻訳は古い状態になります。変更した 1 つの英語ページについて 9 つの翻訳をすべて確認した後、そのソースだけを承認します。

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI は構造検証の前にナビゲーション同期をチェックモードで実行します。この処理では、安定した各英語アンカーが、対応する翻訳済み見出しに引き続き付いていることも検証します。
