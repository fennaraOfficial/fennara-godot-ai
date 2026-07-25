<!-- fennara-i18n: locale=tr source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Yerleşik Sohbet Eğik Çizgi Komutları

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Eğik çizgi komutları Godot içindeki Fennara sohbet panelinde bulunan kısayollardır. Kullanıcı arayüzü komutlarıdır, MCP araçları veya modele gönderilen istemler değildir.

Komut paletini açmak için düzenleyiciye `/` yazın.

| Komut | Açtığı yer | Kullanım amacı |
| --- | --- | --- |
| `/provider` | Sağlayıcı seçici | Bir bulut sağlayıcısı bağlama, yerel sağlayıcı URL'sini yapılandırma veya sağlayıcı değiştirme. |
| `/model` | Model seçici | Geçerli veya bağlı sağlayıcıdan bir model seçme. |

<a id="how-they-behave"></a>
## Davranış Biçimleri

- Komut önerileri arasında hareket etmek için ok tuşlarını kullanın.
- Seçili komutu çalıştırmak için Enter'a basın.
- Komut paletini kapatmak için Escape'e basın.
- Sohbet mesajı gönderilmeden önce eğik çizgi komutu metni düzenleyiciden kaldırılır.

<a id="common-flow"></a>
## Yaygın Akış

Yerleşik sohbet paneli için:

```text
/provider
```

OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, yerel Ollama veya LM Studio bağlayın.

Ardından:

```text
/model
```

Panelin kullanmasını istediğiniz modeli seçin.

Harici MCP uygulamalarında bu eğik çizgi komutlarını kullanmayın. Uygulamayı `fennara mcp-setup` ile yapılandırın, ardından uygulamadan Fennara MCP araçlarını kullanmasını isteyin.
