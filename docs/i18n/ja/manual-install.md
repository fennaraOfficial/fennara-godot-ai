<!-- fennara-i18n: locale=ja source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# 手動インストール

<!-- fennara-doc-nav:start -->
[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · **日本語** · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Godot のセットアップフローや `fennara install` を使わずに Fennara を構成する必要がある場合にのみ、このページを使用してください。

> [!TIP]
> Windows と Linux では、ほとんどのユーザーは `addons/fennara` をプロジェクトへ追加し、Fennara ドックを開いて **Set Up Fennara** を押すだけです。macOS では CLI を使用してください。[セットアップ](setup.md)を参照してください。

> [!IMPORTANT]
> macOS では、アドオン ZIP の手動インストールは推奨されません。アドオンには、現在 Apple の notarization を受けていないネイティブライブラリが含まれており、ブラウザーからのダウンロードと Finder での展開によって、`libfennara.macos.editor` がマルウェアを含まないことを検証できないと macOS が報告する場合があります。この通知を避けるには、[CLI インストール](setup.md#install-from-the-terminal-recommended-on-macos)を使用してください。すでに通知が表示されている場合は Godot を閉じ、手動でコピーした `addons/fennara/` フォルダーを削除して、`fennara install` を実行します。

手動インストールは、CLI、プロジェクトアドオン、共有ローカルランタイムパッケージ、オプションの MCP アプリ構成という 4 つの部分で構成されます。

<a id="1-download-release-files"></a>
## 1. リリースファイルをダウンロード

GitHub の最新リリースを開きます。

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

リリースマニフェスト、使用するプラットフォーム向けのファイル、共有アドオン zip をダウンロードします。

| 用途 | アセット |
| --- | --- |
| リリース計画と SHA-256 値 | `fennara-release-manifest-v<version>.json` |
| Windows x86_64 CLI | `fennara-cli-windows-x86_64-v<version>.zip` |
| Windows x86_64 ローカルランタイム | `fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 CLI | `fennara-cli-linux-x86_64-v<version>.zip` |
| Linux x86_64 ローカルランタイム | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Linux x86_64 内蔵 webview | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 CLI | `fennara-cli-macos-arm64-v<version>.zip` |
| macOS arm64 ローカルランタイム | `fennara-release-local-macos-arm64-v<version>.zip` |
| バージョン付き全プラットフォーム対応アドオン | `fennara-release-addon-v<version>.zip` |

リリースには、ドキュメントおよび手動ダウンロード向けに、次の安定した名前のアドオンエイリアスも含まれています。

```text
fennara-addon-latest.zip
```

マニフェストには、ローカルランタイム、アドオン、共有ランタイムアセットの期待される SHA-256 が記録されています。手動ダウンロードを確認するときは、これを信頼できる唯一の情報源として使用してください。

<a id="2-install-the-cli"></a>
## 2. CLI をインストール

`fennara-cli` zip を展開します。

その `bin` ディレクトリを PATH に追加するか、`fennara` バイナリを既存の PATH フォルダーのいずれかへコピーします。

動作を確認します。

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Godot アドオンをインストール

`fennara-addon` zip を展開します。

次のフォルダーをコピーします。

```text
addons/fennara
```

Godot プロジェクトへコピーし、プロジェクト内に次のファイルが存在する状態にします。

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. ローカルランタイムパッケージをインストール

通常は CLI がこの処理を管理します。ランタイムを手動でセットアップする必要があるのは、`fennara install` を使わない場合だけです。

Fennara の既定のデータフォルダーは次のとおりです。

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

期待されるレイアウトは次のとおりです。

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

Windows では、バイナリに `.exe` が付きます。

`current.json` は、ランチャーバイナリが使用する有効なランタイムバージョンを指定します。通常の `fennara install` コマンドと `fennara update` コマンドは、このファイルを自動的に作成します。

Linux の内蔵チャットは、共有の `webview/cef/linux-x64/<cef-version>/` ランタイムの場所を使用します。通常の `fennara install` と `fennara update` の実行時には、リリースマニフェストとアセットに基づいて、リリース管理の CEF ランタイムが自動的にインストールされます。すべてを手動でインストールする場合は、`fennara-webview-cef-linux-x64-<cef-version>.zip` をその共有ランタイムの場所へ展開し、対応する `webview/cef/linux-x64/current.json` マーカーを書き込みます。このペイロードは Godot プロジェクトアドオンの外に置いてください。`addons/fennara` に `libcef.so` やその他の CEF ランタイムファイルを含めてはいけません。

この CEF ペイロードは、Linux の内蔵チャット専用です。Chat Settings で **Open chat in my system browser next time** を選択すると、同じ内蔵チャットを Godot の内蔵 webview ではなく、ローカルデーモン経由でシステムブラウザーに表示できます。

最終的な Linux CEF レイアウトは次のようになります。

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` は次の内容でなければなりません。

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` は、CEF アセットに対応するリリースマニフェストでなければなりません。例:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

書き込み可能なブラウザー状態を CEF のバージョンディレクトリ内へ置かないでください。通常の使用では、エディターごとのプロファイルとログは Fennara アプリデータのキャッシュおよびログルートに書き込まれ、ランタイムペイロードは共有された読み取り専用の状態に保たれます。

<a id="5-configure-your-mcp-app"></a>
## 5. MCP アプリを構成

ローカルランタイムパッケージのインストール後、MCP アプリを構成します。

```bash
fennara mcp-setup --claude
```

その他の対象を確認するには、次を実行します。

```bash
fennara mcp-setup --help
```

セットアップ後に MCP アプリを再起動してください。

使用するアプリが一覧にない場合、またはこのインストールの一環として MCP 構成を手動で編集する場合は、安定したランチャーパスと JSON/TOML の例について [MCP セットアップ](mcp-setup.md)を参照してください。

これは外部 MCP アプリを Fennara の Godot ツールへ接続するだけです。Fennara 内蔵チャットドックのモデルプロバイダーを構成するものではありません。内蔵チャットを使う場合は Godot 内でドックを構成するか、[MCP アプリと内蔵チャット](chat-vs-mcp.md)を参照してください。

<a id="6-verify"></a>
## 6. 確認

Godot プロジェクトを開き、MCP アプリへ次のように依頼します。

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

パスが正しければ、手動インストールは機能しています。

<a id="recommended-shortcut"></a>
## 推奨される近道

CLI を手動でインストールした場合でも、アドオンとローカルランタイムパッケージのインストールは CLI に任せられます。

```bash
cd path/to/your-godot-project
fennara install
```

CLI は AI コーディングエージェント向けのプロジェクトガイダンスも書き込みます。

```text
AGENTS.md
addons/fennara/ai/
```

AI ディレクトリには、常に読み込まれる簡潔なガイドライン、インデックス、必要な場合にだけ読み込まれる専門ページが含まれています。手動でコピーしたアドオン ZIP にこのパッケージ済みディレクトリを含めることはできますが、プロジェクトルートの `AGENTS.md` を作成または更新することはありません。Fennara に完全なプロジェクトガイダンスを管理および更新させる場合は、`fennara install` と `fennara update` を使用してください。
