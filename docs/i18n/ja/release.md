<!-- fennara-i18n: locale=ja source=docs/release.md sha256=23e966649d82728c9ab9802dd2957bb64c1b13fd3e7ae44bd513835bc063ff77 -->
<a id="release-process"></a>
# リリース手順

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · **日本語** · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../release.md)
<!-- fennara-doc-nav:end -->

リリースは手動です。プルリクエストのワークフローから公開しないでください。

> [!IMPORTANT]
> リリースは `main` から実行し、`VERSION` とワークフロー入力を一致させ、リリースで最低 CLI バージョンを引き上げる必要があるかを明示的に判断してください。

<a id="release-at-a-glance"></a>
## リリースの概要

| 手順 | 結果 |
| --- | --- |
| バージョン変更を準備してマージする | リポジトリ内のバージョン情報源が一致します |
| Package Preview を実行する | 公開せずに、リリースと同じ形の成果物がビルドされます |
| プレビューを調査する | アーカイブ、マニフェスト、ハッシュ、Linux CEF の構成が検証されます |
| `main` から Release を実行する | タグと GitHub Release が公開されます |
| インストールと更新をスモークテストする | 公開されたユーザー向け手順が検証されます |

<a id="versioning"></a>
## バージョニング

`VERSION` が信頼できる唯一の情報源です。

リリースツールは SemVer 値を受け付けます。安定版リリースは `X.Y.Z` を使用します。ステージング候補は `1.2.3-pr.101.2` のような、プルリクエストごとに分離されたプレリリースを使用します。この例では `pr-101` がステージングチャンネル、`2` がそのチャンネルの候補番号です。

リポジトリのバージョンを上げるには、次を実行します。

```bash
node scripts/set-version.mjs X.Y.Z
```

このスクリプトは次を更新します。

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- プラグインのバージョン定数
- `local/` の下にある Rust ワークスペースのパッケージバージョン
- `local/Cargo.lock`

アドオンには `addons/fennara/release.json` も含まれます。通常は、上記のコマンドによって安定版の識別情報が自動的に書き込まれます。ステージングビルド用ワークスペースでは、識別情報を明示的に指定します。

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

ステージングのバージョン、チャンネル、ソースコミット、正確なリリースタグは一致する必要があります。この識別情報を持たないプレリリースアドオンは拒否されます。`release.json` が導入される前の既存の安定版アドオンは、引き続き既定で安定版経路を使用します。

バージョンの同期を確認します。

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. リリースコミットを準備する

1. バージョンスクリプトを実行します。
2. 差分を確認します。
3. 変更した範囲に対応するローカルチェックを実行します。
4. リリース準備 PR を `main` へマージします。

一般的なチェック:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

GDExtension を変更した場合は、可能であればアドオンもローカルでビルドします。

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Package Preview を実行する

パッケージ処理を変更した場合、またはドライランを行いたい場合は、公開前にこれを使用します。

GitHub:

```text
Actions > Package Preview > Run workflow
```

このワークフローは Windows、Linux、macOS のパッケージをビルドし、一時的な成果物をアップロードします。タグ、GitHub Release、`latest` は作成しません。

Package Preview は Release の非公開処理部分を十分忠実に再現し、マージ前にリリースパッケージ処理を検証できるようにします。

- ビルド不要のチャット UI とランタイムヘルパーのソースをアドオンペイロードへ同期します
- Linux CEF ランタイム ZIP をビルドします
- 生成された Linux CEF ランタイムマニフェストを書き込みます
- その生成済みマニフェストをプラットフォームパッケージビルドへ渡します
- 全プラットフォーム向けアドオンアーカイブを組み立てます
- ローカルおよびアドオンパッケージを、マニフェストで管理されるリリースアセット名へ変更します
- 生成されたマニフェストに照らして Linux CEF ランタイムアセットを検証します
- `fennara-release-manifest-v<version>.json` を書き込みます
- リリースと同じ形の ZIP とマニフェストを含む、1 つの `fennara-package-preview-release-assets` 成果物をアップロードします

プレビュー成果物は、公開前に ZIP の内容とマニフェストの形を確認するために役立ちます。これらは Actions の成果物であり、公開リリースのアセットではありません。

<a id="3-run-release"></a>
## 3. Release を実行する

手動のリリースワークフローを `main` から実行します。

```text
Actions > Release > Run workflow
```

入力:

```text
version: X.Y.Z
promote_latest: true
```

`version` 入力は `VERSION` と一致する必要があります。

ワークフローは次を公開します。

- `v<version>`
- `promote_latest` が true の場合、`v<version>` を GitHub Latest として指定します

リリースワークフローは、プラットフォーム別パッケージ処理の前に Linux CEF ランタイムを準備します。固定された公式 CEF 139 Linux minimal SDK をダウンロードし、独立した `fennara-webview-cef-linux-x64-<cef-version>.zip` を組み立て、ステージング済み ELF バイナリから不要情報を除去し、有効化された `local/webview-runtimes/linux-cef.json` マニフェストを生成して、CLI パッケージへ渡します。その後、公開ジョブは、生成済みマニフェストが指定する正確な名前の CEF ZIP がリリースアセットに含まれていること、および SHA-256 が一致することを検証します。さらに `fennara-release-manifest-v<version>.json` を書き込み、参照されるすべてのアセットとハッシュを検証し、そのマニフェストをリリースとともにアップロードします。

プルリクエストのワークフローはリリースを公開しません。Package Preview ワークフローは、マニフェストと Linux CEF ランタイムペイロードを含む、リリースと同じ形のテスト成果物を作成します。そのため、メンテナーはマージ前にパッケージ処理をスモークテストできます。Package Preview はユーザー向けリリースチャンネルではありません。

<a id="release-assets"></a>
## リリースアセット

各リリースには、プラットフォーム別の CLI およびローカルランタイムパッケージと、すべてのプラットフォームで共有されるアドオンパッケージを 1 つ含める必要があります。

| 対象 | アセット |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| すべてのプラットフォーム | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

パッケージの役割:

| パターン | 役割 |
| --- | --- |
| `fennara-cli-*` | 1 つのプラットフォーム向け `fennara` CLI だけを含む、インストールスクリプト用ペイロードです |
| `fennara-release-local-*` | 1 つのプラットフォーム向け MCP およびデーモンランチャーと、バージョン管理されたランタイムバイナリです |
| `fennara-release-addon-v*` | リリースマニフェストを通して解決される、バージョン管理された全プラットフォーム向けアドオンです |
| `fennara-addon-latest.zip` | ドキュメントおよび手動ダウンロード用の、安定した名前を持つ全プラットフォーム向けアドオンの別名です |
| `fennara-webview-cef-linux-x64-*` | Fennara アプリデータへ一度だけインストールされる、Linux 専用の共有 CEF ランタイムです |
| `fennara-release-manifest-v*` | アセット名、SHA-256 値、インストールプリミティブ、共有ランタイムを含むインストールおよび更新計画です |

現在、macOS 用アドオンの GDExtension は Apple の公証を受けていません。ブラウザーによるダウンロードと Finder での手動展開によって隔離メタデータが伝播し、macOS の検証通知が表示される場合があります。ユーザー向けのインストールドキュメントでは、macOS で `fennara install` を推奨し、手動 ZIP の制約を説明し、影響を受けたユーザーへ CLI で再インストールする前に手動コピーしたアドオンを削除するよう案内する必要があります。リリース検証では、ZIP を作成しただけで macOS の署名または公証が済んだとは見なしません。

`fennara-release-local-*` 接頭辞により、古い CLI がマニフェスト管理のパッケージ経路を暗黙に迂回することを防ぎます。

<a id="release-manifest"></a>
## リリースマニフェスト

0.3.0 以降、リリースがマニフェストを公開している場合、`fennara install` と `fennara update` はリリースマニフェストを優先します。マニフェストには次が記録されます。

- `schema_version`
- `version`
- `minimum_cli_version`
- 対応するインストールプリミティブ
- SHA-256 ハッシュ付きの、プラットフォーム別 CLI およびローカルランタイムアセット
- SHA-256 付きの共有アドオンアセット
- プラットフォーム固有の共有ランタイムアセット。現在は Linux CEF

`scripts/release-policy.mjs` が `minimum_cli_version` の信頼できる唯一の情報源です。マニフェストライターはリリース識別情報を検証してからポリシーを選択するため、Stable、Package Preview、Staging が異なる値を独自に選ぶことはできません。通常のパッケージ構成またはアセット名の変更は、外側の CLI を変更するのではなく、マニフェストデータで処理する必要があります。新しい更新処理の引き継ぎ、マニフェストスキーマ、インストールプリミティブ、自己更新動作、または公開済みの古い CLI では安全に実行できないほかの CLI 機能がリリースに必要な場合は、ポリシーを引き上げてください。

CLI が古すぎる場合、`fennara update` はマニフェストのプラットフォーム別 `assets.cli` エントリーを使用してインストール済み CLI を先に更新し、その後 `--no-self-update` を指定してパッケージ更新を再開する必要があります。そのリリースまたはインストール先で自己更新を利用できない場合は、パッケージをインストールする前に失敗し、`install.sh` または `install.ps1` の再実行を促す明確な指示を表示する必要があります。

マニフェストスキーマ 1 に追加された任意のリリース識別情報は、最低 CLI バージョンの引き上げを必要としません。古いスキーマ 1 クライアントは未知のフィールドを無視し、ステージング対応クライアントは識別情報が存在する場合に検証します。チャンネル対応のアクティベーションまたは更新処理の引き継ぎに依存する将来のリリースでは、公開前に最低 CLI バージョンを再検討する必要があります。

<a id="staging-identity-and-discovery-contract"></a>
## ステージングの識別情報と検出契約

ステージングチャンネルはプルリクエストごとに分離されます。

| 値 | PR 101 の例 |
| --- | --- |
| チャンネル | `pr-101` |
| 候補バージョン | `1.2.3-pr.101.2` |
| 正確なリリース | `v1.2.3-pr.101.2` |
| チャンネル参照 | `fennara-staging/pr-101` |
| ポインターファイル | `fennara-staging-channel-pr-101.json` |

チャンネルごとの Git ref には、正確なバージョン管理済みリリースを指す小さなポインターファイルだけが含まれます。リリースバイナリが移動するチャンネル ref の下に置かれることはありません。CLI は内部バージョン要求 `channel:pr-101` を使ってこのポインターを解決でき、その後は正確なバージョンだけを使用して処理を続けます。

したがって、PR 101 と PR 125 は異なるリリースタグとポインターアセットを使用します。一方のチャンネルを更新しても、他方のチャンネルのテスターがリダイレクトされることはありません。あるチャンネルを公開しても、安定版の GitHub Latest 指定や、別のプルリクエストのチャンネルが変わることはありません。

<a id="staging-candidate-workflow"></a>
## ステージング候補ワークフロー

手動の **Staging Release** ワークフローは、開いているプルリクエストの現在の head から候補をビルドします。`main` から実行し、次を指定します。

| 入力 | 意味 |
| --- | --- |
| `pull_request` | ビルドする開いているプルリクエスト |
| `base_version` | `X.Y.Z` 形式で計画されている安定版バージョン |
| `candidate` | このプルリクエスト内で増加する候補番号 |
| `source_commit` | 省略可能な完全 SHA。引き続きプルリクエストの head である必要があります |
| `publish` | 成果物だけを検証する場合はオフ、候補を公開する場合はオン |

ワークフローは、プラットフォーム別ビルドを開始する前にプルリクエストの head SHA を固定します。Windows、Linux、macOS の各ジョブは、その正確なコミットを、読み取り専用権限、永続化されない Git 資格情報、リリース用資格情報なし、共有依存関係キャッシュを保存できない状態でチェックアウトします。信頼済みのデフォルトブランチワークフローによって書き込まれた、互換性のある SCons/godot-cpp および Cargo キャッシュは復元できます。ステージングは復元専用のキャッシュアクションを使用するため、候補コードは信頼済みのビルド出力を利用できますが、後続の実行用キャッシュを置き換えたり汚染したりできません。候補コードはビルド成果物を作成できますが、GitHub Release は公開できません。

その後、信頼済みのリポジトリスクリプトが、候補の識別情報、正確なアーカイブ内容一覧、アドオンの内容、プラットフォーム別パッケージ構成、リリースマニフェスト、すべての SHA-256 値を検証します。`publish` を明示的に選択しない限り、公開は無効のままです。

公開が有効な場合、信頼済みの最終ジョブは次を行います。

1. 候補成果物をデータとして再検証します。
2. ドラフトを作成し、すべてのアセットをアップロードし、GitHub Latest を変更せずに、正確な `v<exact-version>` プレリリースとして公開します。
3. 公開されたアセットをダウンロードし、名前とハッシュを比較します。
4. 後退する、または競合するチャンネル変更を拒否します。
5. 条件付き GitHub Contents API 書き込みを使って、小さな `fennara-staging/pr-<number>` ポインター ref を最後に更新します。
6. 有効なポインターをダウンロードし、その正確な内容を検証します。

1 つのプルリクエストに対する実行は直列化されます。異なるプルリクエストは、別々の並行処理グループ、リリースタグ、ポインター ref を使用します。同じ候補を再実行した場合、ファイルを混在させるのではなく、既存の正確なリリースを検証します。このワークフローは安定版 GitHub Latest を作成、アップロード、昇格することはありません。

安定版の公開では、文字どおりの `latest` タグまたはリリースを使用しません。Release ワークフローは正確な `v<version>` リリースをドラフトとして作成し、アップロードされたアセットをバイト単位で検証し、変更可能なリリースとして公開し、`promote_latest` が true の場合はその正確なリリースを GitHub Latest として指定します。インストーラーおよび安定版 CLI の検出は、GitHub の Latest Release API エンドポイントを解決します。

リポジトリでリリース不変性が無効な間、安定版とステージング版のリリースは変更可能です。どちらのワークフローも、公開を完了する前、またはステージングチャンネルを進める前に、リリースメタデータとダウンロードしたアセットのバイト列を検証します。アセットの公開には、contents 書き込み権限を持つジョブ単位の `GITHUB_TOKEN` を使用します。

現在のリリースポリシーでは、安定版マニフェストに CLI `0.3.12`、ステージング版マニフェストに CLI `0.3.8` が必要です。安定版の検出では、廃止された `latest` タグを解決しなくなりました。安定版 `0.3.12` には、修正済みの更新検証、バージョン切り替えの事前確認、Windows の操作ジャーナル処理が必要です。`0.3.12-pr.123.1` のようなステージング候補は SemVer では安定版 `0.3.12` より低く比較されるため、初回セットアップで候補 CLI をインストールできるよう、最低バージョンは候補バージョンより低く保つ必要があります。マニフェストスキーマとの互換性だけを根拠に、どちらの最低バージョンも変更しないでください。

共有アドオン ZIP には、`godot_demo/addons/fennara/fennara.gdextension` が参照するビルド済み GDExtension バイナリがすべて含まれます。Godot はユーザーの OS に一致するライブラリを読み込み、それ以外を無視します。

Linux CEF webview ランタイムのペイロードは、アドオンアーカイブとは別です。リリースパッケージ処理は有効化済みのランタイムマニフェストを生成し、そのデータを `fennara-release-manifest-v<version>.json` へ埋め込みます。CLI は一致する CEF ペイロードを、ユーザーの Fennara アプリデータディレクトリの下へ一度だけインストールします。

```text
webview/cef/linux-x64/<cef-version>/
```

`libcef.so`、CEF ヘルパー実行ファイル、CEF リソース、ロケールパックを `fennara-addon-*` の中へ置かないでください。Package Preview はテスト用に独立した CEF 成果物をビルドし、Release が使用するものと同種の生成済みランタイムマニフェストを書き込みます。ただし、ユーザー向けリリースアセットの唯一の情報源は、引き続きリリース公開です。

Linux GDExtension のビルドでは、公式 CEF SDK のラッパーソースも必要ですが、アドオン内の CEF ランタイムファイルは必要ありません。CI は次を実行します。

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

そして、展開したディレクトリを `FENNARA_CEF_ROOT` として SCons へ渡します。SCons は `FENNARA_CEF_ROOT/libcef_dll/` を使用し、固定された CEF 139 C++ ラッパーに対して、小さな `libfennara_linux_cef_bridge.so` アドオンライブラリをビルドします。生成されたラッパーソースはランタイム CEF ABI と一致する必要があるため、SDK のダウンロードはバージョンおよびハッシュで検証されます。ブリッジはアドオンに同梱されます。`libcef.so`、リソース、ロケールパック、`fennara_cef_helper` は、独立した共有 CEF ランタイムに残ります。

アドオンアーカイブ内で CEF ランタイムファイルが見つかった場合、パッケージスクリプトは失敗します。ランタイムアセット名は次である必要があります。

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

ZIP を展開したとき、ルートに次の必須ファイルが置かれる必要があります。

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

選択した CEF ディストリビューションに `chrome-sandbox`、`libEGL.so`、`libGLESv2.so`、`libvk_swiftshader.so`、`libvulkan.so.1`、`vk_swiftshader_icd.json`、`snapshot_blob.bin`、追加の `locales/*.pak` などの任意の CEF ランタイムファイルが存在する場合、それらも含めることを推奨します。

メンテナーが選択した CEF バイナリツリーからランタイム ZIP を手動で組み立てるには、次を実行します。

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

Linux 上では、このスクリプトは `fennara-cpp/vendor/cef/` の公式 CEF ヘッダーに対して `scripts/cef/linux/fennara_cef_helper.cpp` から `fennara_cef_helper` をビルドします。別の OS では、最初に Linux でそのヘルパーをビルドし、`--helper /path/to/fennara_cef_helper` を渡してください。ZIP を書き込む前に選択されたファイルを調べるには、`--dry-run` を使用してください。

スクリプトが SHA-256 を表示した後、`local/webview-runtimes/linux-cef.json` を更新します。

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

通常のリリースでは、ワークフローが `--write-manifest` を使って Linux CEF ランタイムマニフェストを自動的に書き込み、その後 `scripts/write-release-manifest.mjs` がランタイムフィールドを `fennara-release-manifest-v<version>.json` へコピーします。手動ランタイムアセット経路または従来のフォールバック動作を意図的にデバッグするのでない限り、チェックイン済みのプレースホルダーマニフェストを手動で有効化しないでください。生成されたマニフェストデータが、存在しないアセット、または SHA-256 が一致しないアセットを指している場合、Release ワークフローと Linux の `fennara install` / `fennara update` は明確に失敗します。

CLI は Linux CEF ランタイム更新をアトミックに公開する必要があります。ステージングディレクトリへ展開して検証し、必須ファイルが存在した後にだけランタイムマーカーを書き込み、その後バージョンディレクトリを公開し、一時ファイルの名前変更によって `current.json` を更新します。インストール済みの `fennara-cef-runtime.json` マーカーでは、`"runtime": "cef"` を使ってネイティブローダー契約を識別する必要があります。インストールおよび更新では、`"kind": "cef"` だけを含む一致した従来マーカーを、CEF ペイロードを再ダウンロードせずに修復します。実行中のエディターは、すでに読み込んだランタイムを使い続けます。

CLI は `local/templates/` から生成済みプロジェクトガイダンステンプレートを埋め込みます。リリースパッケージ処理で CLI をビルドするとき、これらのテンプレートはほかの CLI コードとともにバイナリへコンパイルされます。

<a id="what-latest-means"></a>
## `latest` の意味

GitHub の Latest Release ポインターは、通常のインストールおよび更新手順で使用するバージョン管理済みリリースを選択します。Fennara は、文字どおりの `latest` タグを作成も移動もしません。

- `install.ps1` と `install.sh` は、既定で最新の CLI アセットを取得します。
- `fennara update` は、既定で GitHub の Latest Release エンドポイントを通してリリースマニフェストを取得し、必要に応じてインストール済み CLI を自己更新し、その後マニフェストからローカル、アドオン、共有ランタイムの各アセットを解決します。
- エディター内更新は、シャットダウン前に検証済みアセットをステージングし、置換前にステージング済みアドオン全体のダイジェストを再確認し、アクティベーション検証が成功するまで以前のアドオン、ランチャー、ランタイムマニフェストを保持し、ロールバックデータを削除する前に再度開かれた GDExtension のハンドシェイクを要求します。
- `fennara install` は、既定で GitHub の Latest Release エンドポイントを通してリリースマニフェストを取得し、その後マニフェストからローカル、アドオン、共有ランタイムの各アセットを解決します。
- Godot プラグインの更新確認は、GitHub の最新リリースと比較します。

既定のユーザーインストールにしたくないバージョンを公開する場合にだけ、`promote_latest: false` を使用してください。

インストーラーとリリースのダウンロードでは、リリースメタデータ、アセットのダウンロード、展開、インストール、検証の各手順を表示する必要があります。ネットワーク取得には上限付きのタイムアウトを使用し、GitHub または CDN の停止が固まったように見えるのではなく、診断付きで失敗するようにします。Windows では、`install.ps1` は成功を表示する前に CLI 検証の終了コードを確認する必要があります。終了コード `-1073741515`（`0xC0000135`）は、CLI 実行ファイルは書き込まれたものの、必要な DLL がないため Windows が起動できなかったことを意味します。Microsoft Visual C++ Redistributable 2015-2022 x64 をインストールしてから、`fennara --version`、`fennara doctor`、`fennara install` を再実行するようユーザーへ案内してください。
ダウンロード URL: `https://aka.ms/vs/17/release/vc_redist.x64.exe`。

<a id="smoke-test-after-release"></a>
## リリース後のスモークテスト

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

Godot プロジェクト内:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

プロジェクトに次が追加されていることを確認します。

```text
AGENTS.md
addons/fennara/ai/
```

Godot でプロジェクトを開き、MCP アプリへ次のように依頼します。

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

更新テスト:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## ルール

- リリースワークフローは `main` からだけ実行します。
- リリースのバージョン入力は `VERSION` と一致する必要があります。
- プルリクエストのワークフローはテスト成果物をビルドしてアップロードできますが、リリースを公開してはいけません。
- 通常ユーザー向けとして意図したリリースを、GitHub Latest に指定し続けてください。
- メンテナーが壊れたリリースを置き換えると意図的に判断した場合を除き、公開済みリリースタグを書き換えないでください。
