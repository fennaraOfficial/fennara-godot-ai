<!-- fennara-i18n: locale=tr source=docs/providers.md sha256=d5f056754b227e0b3fe57ed00c86e9d16b9dd39cef2250d43e4417912ae5e07c -->
<a id="built-in-chat-providers"></a>
# Yerleşik Sohbet Sağlayıcıları

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../providers.md)
<!-- fennara-doc-nav:end -->

Godot içindeki Fennara sohbet paneline bir model sağlayıcısı bağlayın.

> [!NOTE]
> Harici MCP uygulamaları kendi model kurulumlarını kullanır. Fennara'yı Codex, Claude, Cursor veya başka bir MCP uygulamasından kullanmak için buraya bir sağlayıcı bağlamanız gerekmez. Bkz. [MCP Uygulamaları ve Yerleşik Sohbet](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Hızlı Kurulum

1. Fennara panelinde **Chat Settings > Chat** bölümünü açın.
2. **Open providers** seçeneğini seçin.
3. Bir bulut sağlayıcısı seçip kendi anahtarınızı girin veya yerel model için Ollama ya da LM Studio seçin.
4. Bir model seçin.

Düzenleyiciye `/provider` ve `/model` de yazabilirsiniz.

<a id="provider-reference"></a>
## Sağlayıcı Başvurusu

| Sağlayıcı | Bağlanma Yöntemi | Model Kimliği Biçimi | Notlar |
| --- | --- | --- | --- |
| OpenAI | [OpenAI API keys](https://platform.openai.com/api-keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `OPENAI_API_KEY`. | `openai/<model>` | OpenAI'ın resmi API'sini kullanır. |
| Anthropic | [Claude Console API keys](https://console.anthropic.com/settings/keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Anthropic'in resmi Messages API'sini kullanır. |
| OpenRouter | [OpenRouter Keys](https://openrouter.ai/settings/keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | OpenRouter API'sini kullanır. |
| Ollama Cloud | [Ollama API keys](https://ollama.com/settings/keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Yerel Ollama sunucusunu değil, Ollama'nın barındırılan API'sini kullanır. |
| DeepSeek | [DeepSeek API keys](https://platform.deepseek.com/api_keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | DeepSeek'in OpenAI uyumlu API'sini kullanır. |
| Z.AI | [Z.AI API keys](https://z.ai/manage-apikey/apikey-list) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `ZHIPU_API_KEY`. | `zai/<model>` | Z.AI'ın OpenAI uyumlu API'sini kullanır. |
| Moonshot AI | [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Moonshot'ın OpenAI uyumlu API'sini kullanır. |
| Moonshot AI (China) | [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Moonshot China'nın OpenAI uyumlu API'sini kullanır. |
| Kimi For Coding | [Kimi Code Console](https://www.kimi.com/code/console) içinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Kimi'nin Anthropic uyumlu Messages API'sini kullanır. Kimi Code erişimi gerektirir. |
| MiniMax | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) içinde **API Keys > Create new secret key** yoluyla kullandıkça ödemeli bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `MINIMAX_API_KEY`. | `minimax/<model>` | MiniMax'in `minimax.io` adresindeki Anthropic uyumlu Messages API'sini kullanır. |
| MiniMax Token Plan | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) içinde **Billing > Token Plan** bölümündeki Subscription Key'i kullanın. Fennara anahtarı/ortam değişkeni: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Token Plan Subscription Key'leri kullandıkça ödemeli API anahtarlarından ayrıdır. |
| MiniMax (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) API anahtarı sayfasından kullandıkça ödemeli bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | MiniMax China'nın `minimaxi.com` adresindeki Anthropic uyumlu Messages API'sini kullanır. |
| MiniMax Token Plan (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) Token Plan sayfasındaki Subscription Key'i kullanın. Fennara anahtarı/ortam değişkeni: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | China Token Plan Subscription Key'leri kullandıkça ödemeli API anahtarlarından ayrıdır. |
| NVIDIA | [build.nvidia.com](https://build.nvidia.com/) adresinde bir anahtar oluşturun. Fennara anahtarı/ortam değişkeni: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | NVIDIA'nın OpenAI uyumlu, barındırılan NIM API'sini kullanır. |
| Ollama | Yerel bir Ollama sunucusu çalıştırın. Bulut API anahtarı gerekmez. | `ollama/<local-model>` | Varsayılan değer `http://127.0.0.1:11434` şeklindedir. |
| LM Studio | LM Studio'nun yerel sunucusunu başlatın. Varsayılan olarak anahtar gerekmez. | `lmstudio/<local-model>` | Varsayılan değer `http://127.0.0.1:1234/v1` şeklindedir. LM Studio sunucunuz kimlik doğrulaması gerektiriyorsa daemon ortamında `LMSTUDIO_API_KEY` ayarlayın. |

Bulut sağlayıcıları kendi API anahtarınızı veya abonelik anahtarınızı gerektirir. Yerel sağlayıcılar, kullanılabilir bir modelle yerel sunucunun çalışmasını gerektirir.

OpenRouter seçimleri her zaman açık `openrouter/<provider>/<model>` biçimini kullanır. Daha eski kaydedilmiş `<provider>/<model>` OpenRouter seçimleri ayarlar yüklenirken bir kez geçirilir, ancak bu eski biçim yeni yönlendirmede kullanılmaz.

Fennara paneldeki sağlayıcı seçiciden gelen anahtarları depolayabilir. Chat Settings, aynı seçiciyi bulmak için **Open providers** düğmesini içerir. Ortam değişkenlerini tercih ederseniz yukarıdaki anahtar/ortam adları Fennara'nın anladığı adlarla aynıdır. Depolanan anahtarlar, Godot projesinin dışında daemon'un yerel uygulama verilerinde tutulur.

<a id="custom-openai-compatible-providers"></a>
## Özel OpenAI Uyumlu Sağlayıcılar

Yerel yönlendirici veya dahili API ağ geçidi gibi OpenAI uyumlu bir uç nokta eklemek için sağlayıcı seçicinin altındaki **Custom** seçeneğini seçin. Şunları girin:

- benzersiz, küçük harfli bir sağlayıcı kimliği
- Fennara'da gösterilen görünen ad
- API sürümünde biten temel URL, örneğin `http://localhost:20128/v1`
- isteğe bağlı API anahtarı
- bir veya daha fazla model kimliği, görünen ad, bağlam uzunluğu ve azami çıktı token sınırı
- isteğe bağlı istek başlıkları

Model kimlikleri uç noktanın beklediğiyle eşleşmelidir. Fennara bunları model seçicide `<provider-id>/<model-id>` olarak sunarken sağlayıcıya yalnızca `<model-id>` gönderir. Uç nokta OpenAI uyumlu `/chat/completions` istek ve akış yanıtı biçimini uygulamalıdır.

API anahtarları ve özel başlık değerleri Fennara'nın korumalı daemon kimlik doğrulama deposunu kullanır. Sağlayıcı tanımları Godot projesinin dışında, daemon tarafından yönetilen yerel uygulama verilerinde kalır. Doğru model sınırları Fennara'nın bir istek modelin bağlam penceresini aşmadan önce konuşma geçmişini sıkıştırmasını ve oluşturulan özetleri modelin çıktı sınırı içinde tutmasını sağlar. Bu alanlar kullanılabilir olmadan önce kaydedilmiş özel modeller, 64.000 bağlam token'ı ve 4.096 çıktı token'ından oluşan uyumluluk varsayılanlarıyla yüklenir.

Kaydettikten sonra özel sağlayıcı model sayısıyla sağlayıcı seçicide görünür. Formu yeniden açıp model eklemek veya yeniden adlandırmak için bu sağlayıcıyı seçin. API anahtarı boş bırakılırsa kaydedilmiş anahtar korunur, yeni girilen başlıklar ise kaydedilmiş başlıklarla ada göre birleştirilir.

<a id="where-settings-live"></a>
## Ayarların Konumu

Fennara yerleşik sohbet ayarlarını daemon aracılığıyla, Godot projesinin dışında yerel olarak depolar:

- sağlayıcı API anahtarları
- özel sağlayıcı başlık değerleri
- özel OpenAI uyumlu sağlayıcı tanımları
- yerel sağlayıcı temel URL'leri
- Ollama ve LM Studio için ayrı maksimum çıkış token değerleri
- seçili model
- akıl yürütme yoğunluğu
- sağlayıcı yanıt zaman aşımı
- Godot içine gömülü veya sistem tarayıcısında açılan sohbet görüntüleme modu
- sohbet geçmişi

Bu ayarlar `res://addons/fennara/` içine yazılmaz ve Claude, Codex, Cursor, Gemini ya da diğer harici MCP uygulamalarıyla paylaşılmaz.

<a id="provider-response-timeout"></a>
## Sağlayıcı yanıt zaman aşımı

**Provider response timeout** ayarı, yerleşik sohbetin her model isteğinin tamamlanmasını ne kadar süre bekleyeceğini denetler. Varsayılan değer 120 saniyedir ve 30 ile 3600 saniye arasındaki değerleri kabul eder. Değeri artırmak, daha yavaş yerel modellerin veya çok sayıda araç kullanan uzun turların tamamlanmasına yardımcı olabilir. Daemon, seçilen zaman aşımını sağlayıcı isteğine uygular ve sınıra ulaşıldığında isteği iptal eder.

<a id="chat-display-setting"></a>
## Sohbet Görüntüleme Ayarı

Chat Settings iletişim kutusu **Open chat in my system browser next time** seçeneğini içerir.

Bu seçenek kapalıyken Fennara yerleşik sohbeti Godot panelinin içinde işlemeye çalışır. Açıkken panel bir **Open chat** düğmesi gösterir ve aynı yerleşik sohbeti yerel daemon üzerinden `127.0.0.1` adresinde başlatır. Bu, Godot editörünün GPU ve bellek kullanımını azaltabilir ve yerel web görünümü başlatılamazsa yedek yoldur.

Bu ayarın değiştirilmesi Godot bir sonraki kez başlatıldığında etkili olur. Yalnızca yerleşik sohbet kullanıcı arayüzünün nerede görüntülendiğini değiştirir; seçili sağlayıcıyı, modeli, API anahtarlarını, sohbet geçmişini, MCP uygulaması kurulumunu veya Claude/Codex/Cursor'ın harici olarak hangi modeli kullandığını değiştirmez.

<a id="picker-shortcuts"></a>
## Seçici Kısayolları

Chat Settings, panel denetimleri ve `/provider` aynı sağlayıcı seçiciyi açar. Model seçiciyi açmak için `/model` veya paneldeki model denetimini kullanın.

Komut paleti davranışı için [Yerleşik Sohbet Eğik Çizgi Komutları](slash-commands.md) bölümüne bakın.

<a id="local-providers"></a>
## Yerel Sağlayıcılar

Ollama için:

```bash
ollama serve
ollama pull llama3.1:8b
```

Ardından şunu seçin:

```text
ollama/llama3.1:8b
```

Daha eski `local/<model>` seçimleri Ollama uyumluluk diğer adları olarak hâlâ kabul edilir. Yeni ayarlarda açık `ollama/<model>` biçimini tercih edin.

Fennara, Ollama'nın çağrı başına üst sınırını OpenAI uyumlu `max_tokens`
alanında gönderir. Ollama bu alanı yerel `num_predict` seçeneğiyle eşler.

LM Studio için yerel sunucuyu LM Studio'dan başlatın ve şu biçimde bir model kimliği seçin:

```text
lmstudio/<loaded-model-id>
```

Ollama ve LM Studio sağlayıcı kurulum formları, her sağlayıcı için ayrı ayrı
saklanan çağrı başına maksimum çıkış ayarlarına aynı varsayılan değeri ve bağlam
sınırlama politikasını uygular. Her ayar varsayılan olarak 8.192 tokendir. Yerel
bir sunucu yüklenen bağlam uzunluğunu bildirdiğinde Fennara, girdi için yer
bırakmak amacıyla o sağlayıcının ayarını bağlamın yarısıyla sınırlar. Fennara bu
etkili sınırı `max_tokens` olarak gönderir ve sohbet geçmişini ne zaman
sıkıştıracağına karar verirken aynı değeri ayırır.

<a id="model-catalog"></a>
## Model Kataloğu

Daemon bulut sağlayıcıları için yerel bir model kataloğu tutar ve yerel sunuculardan şu anda kullanılabilir modellerini ister. Godot açıkken bir katalog veya yerel sunucu değişirse model seçiciyi yenileyin ya da sağlayıcı/model seçiciyi yeniden açın.

Fennara istek göndermeden önce temel model yeteneklerini denetler:

- metin çıktısı gereklidir
- Fennara araç kullanımı için araç çağırma gereklidir
- görüntü ekleri görüntü bağlamı olarak gönderilmeden önce görüntü girdisi gereklidir

Ollama görüntü girdisi Fennara sohbette henüz etkin değildir.
