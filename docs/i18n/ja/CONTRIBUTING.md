<!-- fennara-i18n: locale=ja source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# コントリビューション

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · **日本語** · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Fennara Godot AI の改善にご協力いただき、ありがとうございます。

<a id="good-contributions"></a>
## 歓迎するコントリビューション

- ドキュメントの修正
- 再現可能なバグ修正
- プラットフォーム互換性の修正
- ビルドとパッケージングの改善
- セットアップ手順を分かりやすくする小さな改善

<a id="design-discussion-required"></a>
## 事前に設計の相談が必要な変更

次の作業を始める前に、Issue または Discussion を作成してください。

- 新しい MCP ツール
- ツールスキーマの変更
- リリースワークフローの変更
- 大規模なアーキテクチャ変更
- 生成されるプロジェクトガイダンスに影響する変更

<a id="pull-requests"></a>
## Pull Request

- Pull Request は小さく、焦点を絞ってください。
- 何を、なぜ変更したかを説明してください。
- 変更をどのように検証したかを説明してください。
- UI またはドキュメント表示に関する変更には、スクリーンショットか録画を添付してください。
- 無関係な整形やクリーンアップを含めないでください。
- Issue や Pull Request に大量の自動生成文を貼り付けないでください。

<a id="commit-and-pr-titles"></a>
## Commit と PR のタイトル

Conventional Commits 形式を使います。

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

よく使う種別:

- `feat`: ユーザー向け機能
- `fix`: バグ修正
- `docs`: ドキュメント
- `ci`: GitHub Actions と自動化
- `build`: ビルドまたはパッケージング
- `refactor`: 振る舞いを変えないコード再構成
- `test`: テスト
- `chore`: メンテナンス

<a id="project-boundaries"></a>
## プロジェクトの境界

Fennara はゲームの種類に依存しない状態を保つ必要があります。特定のゲームの操作、目標、経済、インベントリ、戦闘、経路探索、クエスト、UI フローを前提とする API やガイダンスは避けてください。

エージェントは Godot プロジェクトの実際のシーン、スクリプト、リソース、設定、実行時状態、診断、スクリーンショットを調べ、そのプロジェクトに合わせて汎用的な Fennara ツールを組み合わせるべきです。

<a id="documentation-translations"></a>
## ドキュメント翻訳

英語が canonical source です。先に英語を正し、その後、影響を受けるすべての locale を更新します。翻訳対象と locale metadata は `docs/i18n/languages.json` にあります。

- 英語ページ全体を読み、翻訳を直接書きます。bulk machine-translation service や prose-generation script は使いません。
- code block、inline code、command、path、configuration key、URL、product name は正確に保持します。
- documentation script が管理する source marker と明示的な English anchor alias を保持します。
- 流暢な reviewer が確認していない翻訳を native-reviewed と記載しません。
- legal text、internal agent prompt、generated project guidance、vendor file、test fixture は独立した翻訳元にしません。

canonical または翻訳済みドキュメントを変更した後に実行します。

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

これらは navigation metadata を整備して構造を検証します。翻訳本文は書きません。

通常のナビゲーション同期では、既存の各ソースハッシュが保持されます。英語ソースを変更したら、そのページを 9 つの翻訳済みロケールすべてで直接更新してから、その正規ソースだけを明示的に承認します。

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

翻訳を確認して更新した英語ページごとに `--accept-source <path>` を繰り返します。9 つの翻訳すべてに新しい意味が反映される前に、ソースハッシュを承認してはいけません。
