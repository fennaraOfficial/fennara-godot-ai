<!-- fennara-i18n: locale=tr source=docs/chat-vs-mcp.md sha256=03cb522aed8f8e305feaca0c2ed51f7ba29b2657a721df4196b15bc6ccf12c9c -->
<a id="mcp-apps-or-built-in-chat"></a>
# MCP Uygulamaları mı, Yerleşik Sohbet mi?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara ikisini de destekler. Konuşmanın nerede gerçekleşmesini istediğinizi seçin.

| | Harici MCP uygulaması | Yerleşik Fennara sohbeti |
| --- | --- | --- |
| Sohbet ettiğiniz yer | Codex, Claude, Cursor, Gemini veya başka bir MCP uygulaması | Fennara paneli veya sistem tarayıcınız |
| Model hesabı | Harici uygulamanın hesabı veya aboneliği | Fennara Chat Settings içinde bağlanan bir sağlayıcı |
| Fennara'nın ekledikleri | Godot bilgisine sahip MCP araçları | Sohbet kullanıcı arayüzü, aynı temel Godot araçları ve yalnızca sohbete özel dosya ve kabuk araçları |
| Kurulum | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> İki yolu da kullanabilirsiniz. Model ayarları ayrı kalır.

<a id="external-mcp-apps"></a>
## Harici MCP Uygulamaları

Bir MCP uygulamasını bağlamak, o uygulamanın yerel Fennara MCP sunucusunu başlatmasına ve Godot araçlarını çağırmasına olanak tanır. Uygulamanın aboneliğini veya oturum açma bilgilerini yerleşik sohbetle paylaşmaz.

Bir uygulamayı **Chat Settings > MCP Apps** üzerinden kurun veya CLI'yi kullanın:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Fennara sohbet sağlayıcısı anahtarı gerekmez. Kurulumdan sonra harici uygulamayı yeniden başlatın. Tüm hedefler ve elle yapılandırma için [MCP Kurulumu](mcp-setup.md) bölümüne bakın.

<a id="built-in-chat"></a>
## Yerleşik Sohbet

Yerleşik sohbet için Fennara Chat Settings içinde bir sağlayıcının bağlanması gerekir. Bulut sağlayıcısı için kendi anahtarınızı kullanın ya da yerel Ollama veya LM Studio sunucusu bağlayın.

Aynı sohbet Godot panelinin içinde veya sistem tarayıcınızda görünebilir. Bu görüntüleme seçimi sağlayıcıyı, modeli, geçmişi veya projeyi değiştirmez.

Kod eklemek için Godot'nun betik editöründe kodu seçin, bağlam menüsünü açın ve **Add to Chat** seçeneğini seçin. Sağlayıcı ve model kurulumu için [Yerleşik Sohbet Sağlayıcıları](providers.md) bölümüne bakın.

<a id="project-routing"></a>
## Proje Yönlendirmesi

Her iki yol da Godot geri bildirimi için yerel Fennara daemon'unu kullanır.

- Harici MCP çağrıları, paneldeki **MCP target** denetimiyle seçilen projeye gider.
- Yerleşik sohbet, sohbeti açan Godot editörüne bağlı kalır.

Harici bir MCP bağlantısını doğrulamak için şunu sorun:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```
