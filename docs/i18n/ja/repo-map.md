<!-- fennara-i18n: locale=ja source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# リポジトリマップ

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · **日本語** · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../repo-map.md)
<!-- fennara-doc-nav:end -->

このリポジトリで作業するコントリビューターとコーディングエージェント向けの簡易マップです。

<a id="find-the-right-area"></a>
## 適切な領域を探す

| 変更 | 主な場所 |
| --- | --- |
| ユーザー向けセットアップまたは CLI の動作 | `local/crates/fennara-cli/` |
| 外部 MCP プロトコルまたはスキーマ | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| 内蔵チャットまたはデーモンの動作 | `local/crates/fennara-daemon/` |
| Godot エディター統合 | `fennara-cpp/` |
| チャット UI | `ui/chat/` |
| ランタイムヘルパースクリプト | `runtime/` |
| パッケージ処理またはリリース | `scripts/`, `.github/workflows/` |
| ユーザードキュメント | `README.md`, `docs/` |

<a id="top-level"></a>
## トップレベル

| パス | 所有するもの |
| --- | --- |
| `.github/` | プルリクエストテンプレート、Issue テンプレート、GitHub Actions ワークフローです。 |
| `docs/` | プロジェクトドキュメント、セットアップガイド、アーキテクチャノート、例、デモ、リリースノートです。 |
| `docs/i18n/` | ロケールマニフェストと、完全な翻訳ドキュメントツリーです。 |
| `fennara-cpp/` | C++ Godot GDExtension のソースと SCons ビルドエントリーポイントです。 |
| `godot_demo/addons/fennara/` | ユーザープロジェクトへコピーされる、インストール可能な Godot アドオンペイロードです。 |
| `local/` | Rust CLI、MCP サーバー、デーモン、スキーマ、ローカルランタイムコードです。 |
| `media/` | ドキュメントで使用する画像と公開メディアです。 |
| `runtime/` | `runtime_session` と `runtime_script` で使用する Godot ランタイムヘルパースクリプトのソースです。 |
| `scripts/` | バージョニング、パッケージ処理、リリース用のヘルパースクリプトです。 |
| `ui/chat/` | 任意のエディター内 Web チャット UI のソースです。 |
| `local/templates/` | `fennara install` が Godot プロジェクトへ書き込み、`fennara update` が更新する、簡潔なプロジェクトガイドラインと必要時に参照する AI 知識ページです。 |
| `local/webview-runtimes/` | Linux CEF ペイロードなど、共有 Fennara アプリデータへインストールされる外部 webview ランタイムのマニフェストおよび設定ファイルです。 |
| `install.ps1` / `install.sh` | GitHub リリースから Fennara CLI をインストールするブートストラップスクリプトです。 |
| `VERSION` | バージョンの信頼できる唯一の情報源です。 |
| `README.md` | 人間向けの短い概要とクイックスタートです。 |
| `docs/README.md` | タスク指向のドキュメント索引です。 |
| `docs/setup.md` | ユーザー向けのアドオン優先セットアップ、チャットの前提条件、MCP 接続、更新手順、トラブルシューティングです。 |
| `docs/cli.md` | ターミナルコマンドのリファレンス、CLI が所有するインストールおよび更新動作、復旧、診断、アプリデータ構成、自動処理のガイダンスです。 |
| `docs/telemetry.md` | 匿名アクティビティのペイロード、アプリデータ状態、配信動作、月間アクティブの定義、オプトアウト操作です。 |
| `CONTRIBUTING.md` | コントリビューションの規則です。 |
| `SECURITY.md` | セキュリティ報告ポリシーです。 |
| `LICENSE.md` | プロジェクトライセンスです。 |

<a id="local-rust-packages"></a>
## ローカル Rust パッケージ

| パス | 所有するもの |
| --- | --- |
| `local/crates/fennara-cli/` | `fennara` コマンドです。インストール、更新、CLI の自己更新、doctor、操作診断、webview 前提条件の確認、C# サポート、MCP アプリ設定、生成済みプロジェクトガイダンスを扱います。 |
| `local/crates/fennara-cli/src/operation.rs` | 公開されるインストールおよび更新操作のコーディネーター、フェーズ、CLI の引き継ぎエントリーポイントです。 |
| `local/crates/fennara-cli/src/operation/` | 責務を絞った操作ジャーナル、永続ストレージ、診断の秘匿処理、テストモジュールです。 |
| `local/crates/fennara-cli/src/project_addon.rs` | 既存プロジェクトアドオンのバージョンと、現在のプラットフォーム向け GDExtension ライブラリの検証です。 |
| `local/crates/fennara-cli/src/prepare_export.rs` | Godot の起動前に Fennara の永続的なランタイム autoload だけを除去する、アドオンを含まない CI エクスポートの準備です。 |
| `local/crates/fennara-cli/src/release_identity.rs` | 安定版およびステージング版アドオンの識別情報、正確なリリースセレクター、プルリクエストチャンネルの検証、従来の安定版との互換性です。 |
| `local/crates/fennara-cli/src/release_channel.rs` | チャンネルごとのステージングポインターの検証と、正確なバージョン管理済みリリースへの解決です。 |
| `local/crates/fennara-cli/src/release_manifest.rs` | リリースマニフェストの解析、アセットハッシュの検証、識別情報の紐付け、プラットフォーム別パッケージの選択です。 |
| `local/crates/fennara-cli/src/release_version.rs` | マニフェストおよびリリース選択で共有される、CLI の SemVer 解析と優先順位です。 |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | プロジェクトのアドオンファイルを置き換えずに、完全な既存アドオンを正確なバージョンで採用します。 |
| `local/crates/fennara-cli/src/daemon_setup.rs` | インストールと doctor が使用する、共有デーモンの正常性確認、正確なバージョンの準備状態、起動処理です。 |
| `local/crates/fennara-cli/tests/operation_failures.rs` | プロセス単位の失敗、永続化された診断、秘匿、安全側に失敗する操作ログのテストです。 |
| `local/crates/fennara-cli/src/diagnostics.rs` | 最新または名前付きのサニタイズ済み操作レポートをユーザーへ提示します。 |
| `local/crates/fennara-mcp/` | ローカル stdio MCP サーバーとツールスキーマ転送です。 |
| `local/crates/fennara-daemon/` | ランタイムセッションと Godot ブリッジ処理に使用するローカルデーモンです。 |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | 匿名の日次アクティブスケジューラー、上限付きキュー、HTTP 配信、デーモンのライフサイクル統合です。 |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | ランダムなインストール識別情報の検証、アプリデータへのアトミックな永続化、日次レシート状態、オプトアウト時の削除です。 |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | 内蔵チャットの承認モード、ツールリスクの分類、許可判断、保留中承認要求の型です。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | デーモンが所有する内蔵チャットの `exec_command` 実装です。シェル検出、cwd 検証、プロセス起動、タイムアウトとプロセスツリー終了、出力取得、結果成果物のログ、結果整形を扱います。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | 内蔵チャットのコンテキスト圧縮プランナーです。正確な末尾の保護、OpenCode 形式の古いツール結果に圧力がかかった際の刈り込み、要約チャンクの選択、保存、再生、要約プロンプトの直列化、トークン予算、プレースホルダー描画を扱います。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | 内蔵チャットの PromptBuilder と、生成されるランタイム環境コンテキストです。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | ローカル限定の内蔵チャットトレースレコーダー、SQLite イベント行、保持処理、デバッグクエリヘルパーです。 |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | 内蔵チャットのプロバイダーランタイムプリミティブ、カタログと解決、コンテキスト事前確認フック、正規化されたストリームおよびエラー型と、OpenAI、Anthropic、OpenRouter、NVIDIA、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、カスタムエンドポイント、Ollama/local、LM Studio 用の OpenAI 互換または Anthropic 互換アダプターです。 |
| `local/schemas/tools/` | 共有ツール JSON スキーマです。外部 MCP サーバーと内蔵チャットは、それぞれ許可された部分集合を埋め込みます。 |
| `local/webview-runtimes/linux-cef.json` | リリースマニフェスト生成、doctor 出力、従来のフォールバックに使用する Linux CEF ランタイムのプレースホルダーまたは生成済みマニフェストです。CEF をアドオン ZIP 内へ置かず、共有アプリデータ構成とアーカイブメタデータを記録します。 |
| `local/Cargo.toml` | Rust ワークスペース設定です。 |
| `local/Cargo.lock` | ロックされた Rust 依存関係グラフです。 |

<a id="gdextension-source"></a>
## GDExtension ソース

| パス | 所有するもの |
| --- | --- |
| `fennara-cpp/SConstruct` | GDExtension のビルドエントリーポイントです。 |
| `fennara-cpp/include/` | 公開 C++ ヘッダーです。 |
| `fennara-cpp/src/` | C++ 実装です。 |
| `fennara-cpp/src/setup/` | ネイティブの初回セットアップ状態、リリースマニフェストを使う CLI ブートストラップ、ハッシュ検証、CLI 起動、永続化された操作進捗の読み取りです。 |
| `fennara-cpp/src/release/version.cpp` | リリースおよび更新の検出で使用する、ネイティブの SemVer 検証と優先順位です。 |
| `fennara-cpp/src/release/identity.cpp` | パッケージ済み安定版およびステージング版の識別情報検証と、従来の安定版との互換性です。 |
| `fennara-cpp/src/release/discovery.cpp` | GitHub Latest および分離されたステージングチャンネルの更新検出です。 |
| `fennara-cpp/src/update/` | 正確な対象の更新調整、永続レシート検出、終了とインストールの引き継ぎ、復旧 UI 状態です。 |
| `fennara-cpp/src/ui/setup_panel.cpp` | webview に依存しない初回セットアップパネルです。進捗、再試行、ログ、サニタイズ済みレポートの操作を含みます。 |
| `fennara-cpp/vendor/cef/` | Linux OSR ブリッジで使用する公式 CEF 139 ヘッダーのスナップショットです。ランタイムバイナリはアドオン外に置かれます。 |
| `fennara-cpp/src/ui/webview_host*` | ネイティブのエディター内チャット webview ホストと、プラットフォーム別バックエンドです。 |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Linux 専用の共有 CEF ランタイム検出、マーカー検証、動的 `libcef.so` ローダーの基盤です。 |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | 内部チャット webview 向けの、Linux 専用 CEF オフスクリーン描画面、Godot 入力転送、ブリッジ ABI 読み込み、Godot テクスチャ更新です。 |
| `fennara-cpp/src/ui/linux_cef_bridge/` | 固定された公式 CEF 139 `libcef_dll_wrapper` ソースと Fennara の CEF OSR アダプターからビルドされる、小さな Linux 専用ブリッジライブラリです。メイン GDExtension は外部の `libcef.so` ランタイムを読み込んだ後、これを dlopen します。 |
| `fennara-cpp/src/tools/` | Godot 向けツールの実装です。 |
| `fennara-cpp/src/lsp/` | スクリプト診断と言語サーバーヘルパーです。 |
| `fennara-cpp/src/csharp/` | ビルド専用の C# プロジェクト選択、バックグラウンド準備、独立した診断、ランタイムの事前確認です。 |
| `fennara-cpp/src/runtime/` | ランタイムシーンの事前確認、スクリプト診断、デバッガースナップショットなど、ツールが使用するネイティブランタイムサポートです。 |
| `fennara-cpp/godot-cpp/` | Godot C++ バインディングのサブモジュールです。 |

<a id="addon-payload"></a>
## アドオンペイロード

| パス | 所有するもの |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Godot GDExtension の登録ファイルです。 |
| `godot_demo/addons/fennara/VERSION` | アドオンパッケージのバージョンです。 |
| `godot_demo/addons/fennara/release.json` | 正確なバージョン、リリースタグ、チャンネル、ステージングのソースコミットを含む、パッケージ済み安定版またはステージング版の識別情報です。 |
| `godot_demo/addons/fennara/bin/` | ビルド済みのプラットフォーム別ライブラリです。 |
| `godot_demo/addons/fennara/dist/` | エディター内チャット webview が使用する、パッケージ済み Web UI アセットです。 |
| `godot_demo/addons/fennara/runtime/` | アドオン内に同梱される、`runtime/` の同期済みコピーです。 |
| `godot_demo/tests/first_run_setup_test.gd` | ヘッドレスで動く、ネイティブ初回セットアップ状態と決定的失敗のテストです。 |
| `godot_demo/tests/export_plugin_test.gd` | ヘッドレスで動く、ネイティブのエクスポート除外と autoload 復元の回帰テストです。 |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | ヘッドレスで動く、ネイティブスクリーンショット引数契約の回帰テストです。 |
| `godot_demo/tests/image_sheet_test.gd` | ヘッドレスで動く、共有スクリーンショットおよびランタイムシート合成の回帰テストです。 |
| `godot_demo/tests/runtime_image_context_test.gd` | ヘッドレスで動く、ランタイムの生フレーム、シート、任意 Image 出力の回帰テストです。 |

<a id="runtime-helper-source"></a>
## ランタイムヘルパーソース

| パス | 所有するもの |
| --- | --- |
| `runtime/game_capture_helper.gd` | シーンセッションおよびランタイムチェック用に GDExtension が読み込む、ランタイムヘルパーのエントリーポイントです。 |
| `runtime/image_label.gd` | キャプチャ後、合成された Image セルに押される、簡潔で決定的なラベルです。 |
| `runtime/image_sheet.gd` | スクリーンショットとランタイムスクリプトのコンテキストで使用する、共有の純粋な Image シート合成です。 |
| `runtime/screenshot_script_context.gd` | ネイティブキャプチャコンテキストへ共有 Image 合成を追加する、公開スクリーンショットスクリプトのファサードです。 |
| `runtime/runtime_script_context.gd` | `runtime_script` に公開される `ctx` ヘルパー面です。生フレーム、Image の合成と出力、待機、入力、スナップショット、条件、レイキャスト、クリックを含みます。 |
| `runtime/runtime_input_driver.gd` | キー、マウスボタン、絶対マウス移動、相対マウス移動、修飾キー、入力クリーンアップのための低レベルランタイム入力イベントドライバーです。 |
| `runtime/runtime_node_snapshot.gd` | ランタイムノード検索、存在確認、古くなった参照にも安全なスナップショット、プロパティ読み取り、子の要約です。 |
| `runtime/runtime_physics_query.gd` | 簡潔なヒットレシートを伴う、ランタイム 2D/3D の正確なレイキャストおよびスキャンヘルパーです。 |
| `runtime/runtime_query_utils.gd` | ベクター変換、安全なノードおよびパス解決、オブジェクト識別情報、汎用ターゲット一致のための共有ランタイムクエリユーティリティです。 |
| `runtime/runtime_capture_store.gd` | ランタイムセッション、スクリプト、環境確認が使用する、ランタイムキャプチャおよび状態成果物のライターです。 |
| `runtime/runtime_check_runner.gd` | 非対話シーン実行仕様のためのランタイムチェックランナーです。 |

<a id="scripts-and-workflows"></a>
## スクリプトとワークフロー

| パス | 所有するもの |
| --- | --- |
| `scripts/set-version.mjs` | リポジトリ全体のバージョン管理対象ファイルを更新します。 |
| `scripts/check-version.mjs` | バージョンの同期を確認します。 |
| `scripts/release-identity.mjs` | SemVer リリース識別情報と PR ごとのステージングポインターを検証および生成する共有 Node 実装です。 |
| `scripts/release-policy.mjs` | 安定版およびステージング版リリースマニフェスト向け、公開済み CLI の最低互換性ポリシーです。 |
| `scripts/staging-candidate.mjs` | 信頼済みのステージング候補識別情報生成と、PR ごとの単調なポインター判断です。 |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | 責務を絞った、ステージングアドオン、アーカイブ、マニフェスト、共有ファイルシステム、公開バンドルの検証です。 |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | 信頼されないビルド出力および信頼済み公開バンドル向けの厳格な検証エントリーポイントです。 |
| `scripts/check-staging-channel-advance.mjs` | ステージングチャンネルポインターを進める前に、単調性および来歴の確認を適用します。 |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | ポインターを昇格する前に、公開済みアセットのバイト列と公開ダウンロード動作を検証します。 |
| `scripts/test-run-scene-edit-script-inspect.mjs` | 無視対象の一時 Godot プロジェクトをビルドし、エディター GDExtension に対して読み取り専用のインポート済み `PackedScene` 調査をスモークテストします。 |
| `scripts/release-targets.mjs` | 対応するプラットフォーム別リリース対象と、パッケージ済みアセット名を定義します。 |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | 固定された候補識別情報と、小さなチャンネルポインターを書き込みます。 |
| `scripts/sync-chat-ui.mjs` | ビルド不要のチャット UI ソースをアドオンペイロードへコピーします。 |
| `scripts/sync-runtime.mjs` | リポジトリルートのランタイムヘルパーソースをアドオンペイロードへコピーします。 |
| `scripts/sync-doc-navigation.mjs` | 翻訳文章を変更せずに、ドキュメントナビゲーション、ソースハッシュ、安定したアンカーを追加します。 |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | 翻訳の網羅性、鮮度、Markdown 構造、URL、リンクを検証します。 |
| `scripts/package-preview.mjs` | プラットフォーム別ビルド後に、アドオン、CLI、ローカルランタイムのプレビューまたはリリース ZIP を組み立てます。 |
| `scripts/prepare-linux-cef-runtime.mjs` | 独立した Linux x64 CEF ランタイム ZIP をステージングし、ステージング済み ELF バイナリから不要情報を除去し、必須ファイルを検証し、生成済みリリースマニフェストを任意で書き込みます。 |
| `scripts/prepare-linux-cef-sdk.mjs` | `libcef_dll/` ラッパーソースを必要とする CI ビルド向けに、固定された公式 CEF 139 Linux minimal SDK をダウンロードして展開します。 |
| `scripts/check-linux-cef-runtime-release.mjs` | 生成済みの `local/webview-runtimes/linux-cef.json` マニフェストに照らして、Linux CEF ランタイムのリリースアセットを検証します。 |
| `scripts/write-release-manifest.mjs` | ローカルパッケージ、アドオン、共有ランタイムのハッシュを含む `fennara-release-manifest-v<version>.json` をリリースアセットから書き込み、検証します。 |
| `scripts/cef/linux/fennara_cef_helper.cpp` | 独立した CEF ランタイム ZIP 内に同梱される、最小限の Linux CEF サブプロセスヘルパーソースです。 |
| `.github/workflows/version-check.yml` | バージョン整合性の確認です。 |
| `.github/workflows/gdextension-build.yml` | クロスプラットフォーム GDExtension のビルド確認と、Windows ヘッドレスで動くネイティブ初回セットアップ状態テストです。 |
| `.github/workflows/local-build.yml` | Rust ローカルパッケージのビルド確認です。 |
| `.github/workflows/package-preview.yml` | Linux チャットのスモークテスト用テスト限定 Linux CEF ランタイム成果物を含む、手動のパッケージプレビュー成果物です。 |
| `.github/workflows/release.yml` | 生成された Linux CEF ランタイムのパッケージ処理、リリースマニフェスト生成、最終アセット検証を含む、手動 GitHub リリース公開です。 |
| `.github/workflows/staging-release.yml` | 手動の正確な SHA によるステージングビルド、検証専用ドライラン、正確なプレリリース公開、PR ごとのポインター更新です。 |

<a id="where-to-change-things"></a>
## 変更箇所の選び方

| タスク | 最初に見る場所 |
| --- | --- |
| Godot ツールを追加または変更する | `fennara-cpp/src/tools/` および `local/schemas/tools/` |
| MCP スキーマの文章を変更する | `local/schemas/tools/` |
| `fennara install` または `fennara update` を変更する | `local/crates/fennara-cli/src/`。ネイティブのステージングと、デタッチされた適用およびロールバックは、`release_update.rs`、`update_stage.rs`、`update_stage/`、`update_apply/` が所有します |
| CLI コマンドまたはターミナル動作を変更する | `local/crates/fennara-cli/src/` および `docs/cli.md` |
| ネイティブ更新の進捗、シャットダウン確認、アクティベーションハンドシェイク、復旧を変更する | `fennara-cpp/src/update/`、`fennara-cpp/src/ui/update_panel.cpp`、`fennara-cpp/src/ui/dock.cpp`、`local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs`、`ui/chat/` |
| ネイティブ初回セットアップまたは CLI ブートストラップを変更する | `fennara-cpp/src/setup/`、`fennara-cpp/src/ui/setup_panel.cpp`、`fennara-cpp/src/ui/dock.cpp` |
| エクスポート時のアドオン除外を変更する | `fennara-cpp/src/ui/export_plugin.cpp`、`fennara-cpp/include/fennara/ui/export_plugin.hpp`、`godot_demo/tests/export_plugin_test.gd` |
| インストールおよび更新の操作ログ、フェーズ、エラーコード、診断レポートを変更する | `local/crates/fennara-cli/src/operation.rs`、`local/crates/fennara-cli/src/operation/`、`local/crates/fennara-cli/src/diagnostics.rs` |
| webview 前提条件の確認を変更する | `local/crates/fennara-cli/src/webview_prereq.rs`、`local/crates/fennara-cli/src/webview_runtime.rs`、`fennara-cpp/src/ui/webview_host*` |
| 生成されるプロジェクトガイダンスを変更する | `local/templates/` および `local/crates/fennara-cli/src/project_guidance.rs` |
| 生成済みデモアドオンのガイダンスを同期する | `local/templates/fennara-guidelines.md`、`local/templates/fennara-ai/`、`scripts/sync-guidance.mjs`、`godot_demo/addons/fennara/ai/` |
| MCP アプリ設定を変更する | `local/crates/fennara-cli/src/mcp_setup.rs` および `docs/mcp-setup.md` |
| ランタイムセッションのプロセスまたはログ動作を変更する | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`、`local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`、`fennara-cpp/src/tools/runtime_session/`、`fennara-cpp/src/tool_results/` |
| `runtime_script` の ctx ヘルパー、入力、スナップショット、待機、レイキャスト、キャプチャ、クリーンアップを変更する | `runtime/`、`scripts/sync-runtime.mjs`、`godot_demo/addons/fennara/runtime/`、`local/schemas/tools/runtime_script.json`、`docs/tools.md` |
| エディター内チャット UI、スラッシュコマンド、モデルまたはプロバイダー選択を変更する | `ui/chat/`、`godot_demo/addons/fennara/dist/`、`fennara-cpp/src/ui/dock.cpp`、`fennara-cpp/src/ui/webview_host*` |
| 内蔵チャットのプロバイダーを変更する | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`、`local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`、`local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`、`ui/chat/` |
| 匿名テレメトリーのフィールド、スケジューリング、プライバシー操作を変更する | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`、`local/crates/fennara-daemon/src/runtime_daemon/telemetry/`、`local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`、`ui/chat/`、`docs/telemetry.md` |
| ベンダー提供のチャット UI ライブラリを変更する | `ui/chat/vendor/`、`godot_demo/addons/fennara/dist/vendor/`、`THIRD_PARTY_NOTICES.md` |
| C# サポートを変更する | `fennara-cpp/src/csharp/`、`fennara-cpp/include/fennara/csharp/`、C# ツールスキーマおよびガイダンス |
| リリースパッケージ、最低 CLI ポリシー、CLI 自己更新を変更する | `local/crates/fennara-cli/src/release_manifest.rs`、`local/crates/fennara-cli/src/release_client.rs`、`local/crates/fennara-cli/src/release_package.rs`、`local/crates/fennara-cli/src/self_update.rs`、`scripts/package-preview.mjs`、`scripts/release-policy.mjs`、`scripts/write-release-manifest.mjs`、`.github/workflows/release.yml` |
| バージョンを上げる | `node scripts/set-version.mjs <version>` |
| チャットと MCP、プロバイダー、スラッシュコマンドに関するセットアップまたはドキュメントを更新する | `README.md`、`docs/mcp-setup.md`、`docs/chat-vs-mcp.md`、`docs/providers.md`、`docs/slash-commands.md`、`docs/setup.md`、`docs/faq.md`、`docs/manual-install.md`、`docs/tools.md`、`docs/examples.md`、`llms.txt` |
| ドキュメント翻訳を更新する | 正式な英語ページ、`docs/i18n/languages.json`、対応するロケールページ、`scripts/sync-doc-navigation.mjs`、`scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## 注意事項

- 主要なソース領域を追加または移動したときは、このファイルも最新に保ってください。
- リリース手順は [release.md](release.md) に置いてください。
- セットアップ手順は [setup.md](setup.md) に置いてください。
- ターミナルコマンドの動作は [cli.md](cli.md) に置いてください。
