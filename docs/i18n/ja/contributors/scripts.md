<!-- fennara-i18n: locale=ja source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# スクリプト

<!-- fennara-doc-nav:start -->
[ドキュメント](../README.md)

🌐 **言語**

[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · **日本語** · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

このディレクトリには、ローカル開発、Package Preview、リリースワークフローで共有されるリポジトリ自動化が含まれています。

スクリプトは小さく、決定的で、ヘルプテキストに別の記載がない限りリポジトリルートから安全に実行できるものにしてください。リポジトリ外へユーザー固有の状態を書き込んではいけません。

<a id="version-scripts"></a>
## バージョンスクリプト

- `set-version.mjs`: リポジトリの `VERSION`、アドオンの `VERSION`、ローカル Rust ワークスペースのメタデータ、lockfile のパッケージバージョン、C++ プラグインのバージョン定数を更新します。
- `check-version.mjs`: これらのバージョン管理対象ファイルが同期したままであることを確認します。

CI とリリースパッケージングの前に `check-version.mjs` を実行してください。`set-version.mjs` は、Fennara のバージョンを意図的に変更するときだけ使用します。

<a id="packaging-scripts"></a>
## パッケージングスクリプト

- `package-preview.mjs`: コミット済みのアドオンペイロードを同期し、GDExtension とローカル Rust バイナリがすでにビルドされた後に、プラットフォームごとのプレビューアーカイブを組み立てます。
- `package-addon-all.mjs`: プラットフォーム別のアドオンパーツを、最終的な全プラットフォーム対応アドオンアーカイブへ統合します。
- `release-policy.mjs`: リリーストラックごとに、互換性のある公開済み CLI の最小バージョンを定義します。
- `write-release-manifest.mjs`: リリースアセットから `fennara-release-manifest-v<version>.json` を書き出し、参照されるすべての SHA-256 を検証します。

どちらのスクリプトも `.package-preview/` を一時ステージングに使用し、zip 出力をリポジトリルートの `dist/` フォルダーへ書き込みます。これらの出力は無視されるため、コミットしてはいけません。

パッケージングスクリプトは、アドオンペイロードを小さく保つ必要があります。特に、`libcef.so` や `fennara_cef_helper` などの Linux CEF ランタイムファイルを `fennara-addon-*` 内へ同梱してはいけません。CEF はユーザーの共有 Fennara アプリデータディレクトリへ 1 回だけインストールされます。

<a id="staging-release-scripts"></a>
## ステージングリリーススクリプト

- `write-staging-candidate.mjs`: 1 つの Pull Request と固定されたソースコミットについて、正確なプレリリース識別情報を作成します。
- `validate-staging-build.mjs`: 公開前に、アドオンパーツ、プラットフォームアーカイブ、組み立て済みアドオン、リリースマニフェスト、Linux CEF を検査します。
- `smoke-public-release.mjs`: 認証不要のブラウザー URL を通じて公開済みの各候補をダウンロードし、チャンネルを進める前に信頼済みアセットとマニフェストのハッシュを検証します。
- `write-staging-pointer.mjs`: 正確なリリースマニフェストをハッシュした後、Pull Request ごとの小さなポインターを書き出します。
- `check-staging-channel-advance.mjs`: 後退する、または競合するチャンネル移動を拒否します。
- `validate-staging-publish-bundle.mjs`: 候補のコードを実行せずに、最終アーティファクトバンドルを再検証します。
- `verify-published-assets.mjs`: 期待される GitHub Release アセットの名前と SHA-256 を、ダウンロード済みのものと比較します。

これらのスクリプトは `.github/workflows/staging-release.yml` を支えます。候補のビルドジョブはリリース資格情報を持たずに実行されます。信頼された最終ジョブだけが公開でき、正確なリリースをダウンロードして検証した後に、チャンネルごとの Git ref を進めます。

<a id="linux-cef-scripts"></a>
## Linux CEF スクリプト

- `prepare-linux-cef-sdk.mjs`: Linux CEF ブリッジのビルドに使用する、バージョン固定された公式 Linux x64 CEF SDK をダウンロードして展開します。
- `prepare-linux-cef-runtime.mjs`: 別個の Linux CEF ランタイム zip をステージし、必要なファイルを検証し、Linux 上でステージ済み ELF バイナリからシンボルを除去し、リリースパッケージング用に生成される `local/webview-runtimes/linux-cef.json` マニフェストを書き出すこともできます。
- `check-linux-cef-runtime-release.mjs`: 有効なマニフェストで指定された名前の CEF ランタイム zip がリリースアセットに含まれ、その SHA-256 が一致することを検証します。
- `cef/linux/fennara_cef_helper.cpp`: CEF SDK からランタイムヘルパーをビルドするときに使用する、小さな CEF ヘルパープロセスのソースです。

CEF スクリプトは、コピーされたステージングファイルだけを操作します。ダウンロード済みまたはソースの CEF SDK ツリーを変更してはいけません。

<a id="development-tests"></a>
## 開発テスト

- `test-run-scene-edit-script-inspect.mjs`: `temp/` の下に無視対象の Godot スモークプロジェクトを作成し、ビルド済みのエディター GDExtension に対して、インポート済み `PackedScene` の調査、読み取り専用コンテキストのガード、ソース欠如時の失敗、保存しない動作を検証します。

<a id="documentation-localization"></a>
## ドキュメントのローカライズ

- `sync-doc-navigation.mjs`: 文章を翻訳せず、ソースハッシュ、安定したアンカー、同じページを指すコンパクトな言語セレクターを追加します。
- `check-doc-i18n.mjs`: ロケール対応範囲の完全性、ソースの鮮度、ナビゲーション、アンカー、Markdown 構造、保護対象コード、URL、リンクを検証します。
- `doc-i18n-lib.mjs`: 共有ロケールマニフェスト、ソースの正規化、ナビゲーションのレンダリング、構造ヘルパーを所有します。

次を実行します。

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

ロケールとドキュメントの集合は `docs/i18n/languages.json` で宣言されます。英語が正規版のままです。翻訳された文章は英語ソースから執筆する必要があり、これらのスクリプトによって生成されるものではありません。

通常の同期ではナビゲーションと安定したアンカーが更新されますが、既存のソースハッシュは保持されます。変更した英語ページについて 9 つの翻訳をすべて直接更新した後、そのソースだけを明示的に更新します。

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

このオプションは、確認済みの複数のソースに対して繰り返し指定できます。翻訳された文章を更新していないソースを承認してはいけません。CI は翻訳全体のバリデーターより前に `sync-doc-navigation.mjs --check` を実行します。

<a id="ui-sync"></a>
## UI の同期

- `sync-chat-ui.mjs`: `ui/chat/` を `godot_demo/addons/fennara/dist/` へコピーします。

`godot_demo/addons/fennara/dist/` は、リリースされるアドオン zip にビルド済みチャット webview を含める必要があるため、意図的にコミットされます。`ui/chat/` で変更を行い、同期スクリプトを実行し、ソースと生成されたアドオンアセットをまとめてコミットしてください。

<a id="runtime-sync"></a>
## ランタイムの同期

- `sync-runtime.mjs`: `runtime/` を `godot_demo/addons/fennara/runtime/` へコピーします。

`godot_demo/addons/fennara/runtime/` は、リリースされるアドオン zip に Godot 側のランタイムヘルパースクリプトを含める必要があるため、意図的にコミットされます。`runtime/` で変更を行い、同期スクリプトを実行し、ソースと生成されたアドオンアセットをまとめてコミットしてください。

<a id="guidance-sync"></a>
## ガイダンスの同期

- `sync-guidance.mjs`: `local/templates/` にある簡潔なガイドラインと必要時に読み込むナレッジページを `godot_demo/addons/fennara/ai/` へコピーし、`fennara install` と `fennara update` がユーザーのプロジェクトへ書き込むファイルに合わせます。

`godot_demo/addons/fennara/ai/` は、デモアドオンがインストール済みアドオンのレイアウトを再現するため、意図的にコミットされます。`local/templates/` で変更を行い、同期スクリプトを実行し、ソースと生成されたアドオンガイダンスをまとめてコミットしてください。

<a id="boundaries"></a>
## 境界

- スクリプトは `.package-preview/` とルートの `dist/` に出力を作成できます。
- スクリプトがコミット済みの生成ペイロードを更新できるのは、`sync-chat-ui.mjs`、`sync-runtime.mjs`、`sync-guidance.mjs`、`set-version.mjs` のように、それが明示された役割である場合だけです。
- スクリプトは、Godot エディターのキャッシュ、ローカルアプリデータへのインストール、ダウンロード済みリリースアーティファクト、VM テスト出力を、追跡対象のソースフォルダーへ書き込んではいけません。
