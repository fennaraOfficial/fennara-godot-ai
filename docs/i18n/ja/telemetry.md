<!-- fennara-i18n: locale=ja source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# 匿名テレメトリー

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · **日本語** · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara は UTC で 1 日に最大 1 回、小さな匿名アクティビティイベントを送信します。互換性のある Godot エディターがローカルデーモンへ接続した後にだけ送信されます。メンテナーがアクティブなインストール数、対応プラットフォームの利用状況、バージョン普及状況を把握するためのものです。

テレメトリーは既定で有効です。無効にするには **Chat Settings > Chat > Anonymous telemetry** を開きます。ヘッドレス環境や自動化環境では、次のいずれかを設定できます。

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

環境変数は保存された UI 設定より優先されます。無効にすると以後のイベントを停止し、ローカルのテレメトリー ID と最終送信状態を削除します。再度有効にすると、次に Godot が接続したとき新しいランダム ID が作られます。

<a id="event-contents"></a>
## イベントの内容

`fennara_active_installation` には次の情報だけが含まれます。

| フィールド | 目的 |
| --- | --- |
| `schema_version` | 小さなテレメトリーペイロード契約のバージョン |
| `event` | 固定されたイベント名 |
| `installation_id` | ローカルで生成するランダム UUID。ハードウェアやアカウントからは生成しない |
| `fennara_version` | 実行中デーモンのバージョン |
| `godot_version` | `4.6.3` などの数値 Godot バージョン |
| `platform` | `windows`、`macos`、`linux` |
| `architecture` | `x86_64` または `aarch64` |

Fennara はプロジェクト名、プロジェクトパス、アカウント情報、プロンプト、チャットメッセージ、プロバイダーキー、モデル名、ツール名、ツール引数、ツール結果、ログ、スクリーンショット、シーン内容、ファイル名、エラー文を送信しません。

<a id="storage-and-transport"></a>
## 保存と通信

デーモンはランダム ID と最後に送信成功した UTC 日付を共有 Fennara アプリデータ内に保存します。

```text
Fennara/
  telemetry/
    state.json
```

イベントは HTTPS で `https://fennara.io/api/telemetry` へ送信されます。受信側は許可されたフィールドだけであることを厳密に検証し、生の UUID をサーバー側 HMAC に置き換えてから PostHog へ転送します。このイベントでは PostHog の person profile と IP geolocation を無効にしています。

Vercel の受信処理は HTTPS 要求を扱う際に通常のネットワークメタデータを観測します。その情報を PostHog のイベントペイロードへコピーすることはありません。

<a id="delivery-behavior"></a>
## 送信動作

テレメトリーは Godot ツール呼び出しと別に動作します。

- 制限付きキューが待機せずアクティビティ通知を受け取ります。
- 1 つのバックグラウンド worker が単一の HTTP クライアントを再利用します。
- 要求には短いタイムアウトがあります。
- キュー満杯、ファイルシステム問題、ネットワーク障害、サーバー拒否は黙って許容され、Fennara ツールを失敗させません。
- UTC 日付はサーバーがイベントを受理した後だけ記録するため、失敗後は次の Godot 接続時に再試行できます。
- シャットダウンは短時間だけ待ち、その後デーモンを遅らせず worker をキャンセルします。

1 インストールは、保存された 1 つのランダム UUID です。2 台のコンピューターで使えば 2 インストールとして数えます。Fennara アプリデータの消去、または無効化後の再有効化により新しい ID が作られます。

月間アクティブインストール数は、その暦月に `fennara_active_installation` を 1 回以上送信した匿名インストール ID の重複なし件数です。
