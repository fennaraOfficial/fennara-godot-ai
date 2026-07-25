<!-- fennara-i18n: locale=ja source=docs/providers.md sha256=17c492522ed54ff1ffb4942e27572fa7e15709cec2853ac38be5b5f0dc91db0d -->
<a id="built-in-chat-providers"></a>
# 内蔵チャットプロバイダー

<!-- fennara-doc-nav:start -->
[ドキュメント](README.md)

🌐 **言語**

[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · **日本語** · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ 英語の原文を基に AI が執筆した翻訳です。ネイティブスピーカーによるレビューを歓迎します。 [英語の原文](../../providers.md)
<!-- fennara-doc-nav:end -->

Godot 内の Fennara チャットドックへモデルプロバイダーを接続します。

> [!NOTE]
> 外部 MCP アプリは、独自のモデル設定を使用します。Codex、Claude、Cursor、または別の MCP アプリから Fennara を使用するために、ここでプロバイダーを接続する必要はありません。[MCP アプリと内蔵チャット](chat-vs-mcp.md)を参照してください。

<a id="quick-setup"></a>
## クイックセットアップ

1. Fennara ドックで **Chat Settings > Chat** を開きます。
2. **Open providers** を選択します。
3. クラウドプロバイダーを選択して自分のキーを入力するか、ローカルモデル用に Ollama または LM Studio を選択します。
4. モデルを選択します。

composer に `/provider` と `/model` を入力することもできます。

<a id="provider-reference"></a>
## プロバイダーリファレンス

| プロバイダー | 接続方法 | モデル ID の形式 | 備考 |
| --- | --- | --- | --- |
| OpenAI | [OpenAI API keys](https://platform.openai.com/api-keys) でキーを作成します。Fennara のキーまたは環境変数: `OPENAI_API_KEY`。 | `openai/<model>` | OpenAI の公式 API を使用します。 |
| Anthropic | [Claude Console API keys](https://console.anthropic.com/settings/keys) でキーを作成します。Fennara のキーまたは環境変数: `ANTHROPIC_API_KEY`。 | `anthropic/<model>` | Anthropic の公式 Messages API を使用します。 |
| OpenRouter | [OpenRouter Keys](https://openrouter.ai/settings/keys) でキーを作成します。Fennara のキーまたは環境変数: `OPENROUTER_API_KEY`。 | `openrouter/<provider>/<model>` | OpenRouter の API を使用します。 |
| Ollama Cloud | [Ollama API keys](https://ollama.com/settings/keys) でキーを作成します。Fennara のキーまたは環境変数: `OLLAMA_API_KEY`。 | `ollama-cloud/<model>` | ローカルの Ollama サーバーではなく、Ollama のホスト型 API を使用します。 |
| DeepSeek | [DeepSeek API keys](https://platform.deepseek.com/api_keys) でキーを作成します。Fennara のキーまたは環境変数: `DEEPSEEK_API_KEY`。 | `deepseek/<model>` | DeepSeek の OpenAI 互換 API を使用します。 |
| Z.AI | [Z.AI API keys](https://z.ai/manage-apikey/apikey-list) でキーを作成します。Fennara のキーまたは環境変数: `ZHIPU_API_KEY`。 | `zai/<model>` | Z.AI の OpenAI 互換 API を使用します。 |
| Moonshot AI | [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys) でキーを作成します。Fennara のキーまたは環境変数: `MOONSHOT_API_KEY`。 | `moonshotai/<model>` | Moonshot の OpenAI 互換 API を使用します。 |
| Moonshot AI (China) | [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys) でキーを作成します。Fennara のキーまたは環境変数: `MOONSHOT_API_KEY`。 | `moonshotai-cn/<model>` | Moonshot China の OpenAI 互換 API を使用します。 |
| Kimi For Coding | [Kimi Code Console](https://www.kimi.com/code/console) でキーを作成します。Fennara のキーまたは環境変数: `KIMI_API_KEY`。 | `kimi-for-coding/<model>` | Kimi の Anthropic 互換 Messages API を使用します。Kimi Code へのアクセスが必要です。 |
| MiniMax | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) の **API Keys > Create new secret key** で従量課金キーを作成します。Fennara のキーまたは環境変数: `MINIMAX_API_KEY`。 | `minimax/<model>` | `minimax.io` にある MiniMax の Anthropic 互換 Messages API を使用します。 |
| MiniMax Token Plan | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) の **Billing > Token Plan** にある Subscription Key を使用します。Fennara のキーまたは環境変数: `MINIMAX_API_KEY`。 | `minimax-coding-plan/<model>` | Token Plan の Subscription Key は、従量課金 API キーとは別です。 |
| MiniMax (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) の API キーページで従量課金キーを作成します。Fennara のキーまたは環境変数: `MINIMAX_API_KEY`。 | `minimax-cn/<model>` | `minimaxi.com` にある MiniMax China の Anthropic 互換 Messages API を使用します。 |
| MiniMax Token Plan (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) の Token Plan ページにある Subscription Key を使用します。Fennara のキーまたは環境変数: `MINIMAX_API_KEY`。 | `minimax-cn-coding-plan/<model>` | China Token Plan の Subscription Key は、従量課金 API キーとは別です。 |
| NVIDIA | [build.nvidia.com](https://build.nvidia.com/) でキーを作成します。Fennara のキーまたは環境変数: `NVIDIA_API_KEY`。 | `nvidia/<publisher>/<model>` | NVIDIA の OpenAI 互換ホスト型 NIM API を使用します。 |
| Ollama | ローカル Ollama サーバーを実行します。クラウド API キーは必要ありません。 | `ollama/<local-model>` | 既定値は `http://127.0.0.1:11434` です。 |
| LM Studio | LM Studio のローカルサーバーを起動します。既定ではキーは必要ありません。 | `lmstudio/<local-model>` | 既定値は `http://127.0.0.1:1234/v1` です。LM Studio サーバーで認証が必要な場合は、デーモンの環境に `LMSTUDIO_API_KEY` を設定します。 |

クラウドプロバイダーには、ユーザー自身の API キーまたは Subscription Key が必要です。ローカルプロバイダーでは、利用可能なモデルを備えたローカルサーバーを実行する必要があります。

OpenRouter の選択では、常に明示的な `openrouter/<provider>/<model>` 形式を使用します。以前保存された `<provider>/<model>` 形式の OpenRouter 選択は、設定の読み込み時に一度だけ移行されますが、その古い形式は新しいルーティングには使用されません。

Fennara は、ドックのプロバイダーピッカーからキーを保存できます。Chat Settings には、同じピッカーを見つけるための **Open providers** ボタンがあります。環境変数を使用したい場合、上記のキーまたは環境変数名は Fennara が認識するものと同じです。保存されたキーは Godot プロジェクトの外にある、デーモンのローカルアプリデータに置かれます。

<a id="custom-openai-compatible-providers"></a>
## カスタム OpenAI 互換プロバイダー

ローカルルーターや内部 API ゲートウェイなど、OpenAI 互換エンドポイントを追加するには、プロバイダーピッカーの下部にある **Custom** を選択します。次の情報を入力します。

- 一意の小文字プロバイダー ID
- Fennara に表示される名前
- API バージョンで終わるベース URL、たとえば `http://localhost:20128/v1`
- オプションの API キー
- 1 つ以上のモデル ID、表示名、コンテキスト長、最大出力トークン制限
- オプションのリクエストヘッダー

モデル ID は、エンドポイントが期待する値と一致しなければなりません。Fennara はモデルピッカーで `<provider-id>/<model-id>` として公開しますが、プロバイダーへ送信するのは `<model-id>` だけです。エンドポイントは OpenAI 互換の `/chat/completions` リクエストとストリーミング応答の形式を実装する必要があります。

API キーとカスタムヘッダーの値には、Fennara の保護されたデーモン認証ストアが使用されます。プロバイダー定義は Godot プロジェクトの外にある、デーモン管理のローカルアプリデータに置かれます。正確なモデル制限を設定すると、リクエストがモデルのコンテキストウィンドウを超える前に Fennara が会話履歴を圧縮し、生成される要約をモデルの出力制限内に保てます。これらのフィールドが利用可能になる前に保存された既存のカスタムモデルは、コンテキスト 64,000 トークン、出力 4,096 トークンの互換性既定値で読み込まれます。

保存後、カスタムプロバイダーはモデル数とともにプロバイダーピッカーへ表示されます。そのプロバイダーを選択するとフォームが再び開き、モデルを追加または名前変更できます。API キーを空のままにすると保存済みのキーが維持され、新たに入力したヘッダーは名前ごとに保存済みのヘッダーと統合されます。

<a id="where-settings-live"></a>
## 設定の保存場所

Fennara は内蔵チャットの設定を、Godot プロジェクトの外にあるデーモン経由でローカルに保存します。

- プロバイダーの API キー
- カスタムプロバイダーのヘッダー値
- カスタム OpenAI 互換プロバイダーの定義
- ローカルプロバイダーのベース URL
- 選択したモデル
- reasoning effort
- Godot 内への埋め込み、またはシステムブラウザーで開くというチャット表示モード
- チャット履歴

これらの設定は `res://addons/fennara/` に書き込まれず、Claude、Codex、Cursor、Gemini、その他の外部 MCP アプリと共有されることもありません。

<a id="chat-display-setting"></a>
## チャット表示設定

Chat Settings ダイアログには、**Open chat in my system browser next time** があります。

この設定がオフの場合、Fennara は内蔵チャットを Godot ドック内にレンダリングしようとします。オンの場合、ドックには **Open chat** ボタンが表示され、`127.0.0.1` のローカルデーモン経由で同じ内蔵チャットが起動します。これにより Godot エディターの GPU とメモリの使用量を減らせる場合があり、ネイティブ webview を起動できない場合のフォールバック経路にもなります。

この設定の変更は、次に Godot を起動したときに有効になります。変わるのは内蔵チャット UI の表示場所だけです。選択したプロバイダー、モデル、API キー、チャット履歴、MCP アプリのセットアップ、Claude、Codex、Cursor が外部で使用するモデルは変わりません。

<a id="picker-shortcuts"></a>
## ピッカーのショートカット

Chat Settings、ドックコントロール、`/provider` は、同じプロバイダーピッカーを開きます。モデルピッカーを開くには、`/model` またはドックのモデルコントロールを使用します。

コマンドパレットの動作については、[内蔵チャットのスラッシュコマンド](slash-commands.md)を参照してください。

<a id="local-providers"></a>
## ローカルプロバイダー

Ollama の場合:

```bash
ollama serve
ollama pull llama3.1:8b
```

次を選択します。

```text
ollama/llama3.1:8b
```

以前の `local/<model>` 形式の選択も、Ollama 互換エイリアスとして引き続き受け入れられます。新しい設定には、明示的な `ollama/<model>` 形式を使用してください。

LM Studio の場合は、LM Studio からローカルサーバーを起動し、次の形式のモデル ID を選択します。

```text
lmstudio/<loaded-model-id>
```

<a id="model-catalog"></a>
## モデルカタログ

デーモンはクラウドプロバイダー用のローカルモデルカタログを保持し、ローカルサーバーには現在利用可能なモデルを問い合わせます。Godot を開いている間にカタログまたはローカルサーバーが変わった場合は、モデルピッカーを更新するか、プロバイダーまたはモデルピッカーを開き直してください。

Fennara はリクエストを送信する前に、モデルの基本機能を確認します。

- テキスト出力が必要
- Fennara ツールを使うには tool calling が必要
- 画像添付を画像コンテキストとして送信する前に image input が必要

Fennara チャットでは、Ollama の image input はまだ有効になっていません。
