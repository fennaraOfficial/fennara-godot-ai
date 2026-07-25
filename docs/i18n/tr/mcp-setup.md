<!-- fennara-i18n: locale=tr source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# MCP Kurulumu

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Harici bir AI uygulamasını Fennara'nın Godot araçlarına bağlayın. Uygulama kendi
model hesabını, aboneliğini veya API kurulumunu kullanmaya devam eder.

> [!NOTE]
> Bu, yerleşik Fennara sohbetini yapılandırmaz. Hangi yola ihtiyacınız olduğundan
> emin değilseniz [MCP Uygulamaları ve Yerleşik Sohbet](chat-vs-mcp.md)
> sayfasına bakın.

<a id="quick-setup"></a>
## Hızlı Kurulum

1. Godot dock'unda **Set Up Fennara** işlemini tamamlayın.
2. **Chat Settings > MCP Apps** bölümünü açın.
3. Uygulamanızı bulun ve **Set Up** düğmesine basın.
4. Uygulamayı yeniden başlatın.

Fennara, bir uygulamanın MCP yapılandırmasını değiştirmeden önce yedek
oluşturur. Birleşik **Claude** seçeneği Claude Code ve Claude Desktop'ı
yapılandırır. **Gemini & Antigravity** ise iki paylaşılan hedefi de yapılandırır.

<a id="terminal-alternative"></a>
### Terminal Seçeneği

Önce Godot projesinin içinde `fennara install` çalıştırın, ardından bir hedef seçin:

| Uygulama | Komut |
| --- | --- |
| Claude Code ve Claude Desktop | `fennara mcp-setup --claude` |
| Yalnızca Claude Code | `fennara mcp-setup --claude-code` |
| Yalnızca Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini ve Antigravity | `fennara mcp-setup --gemini` veya `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Kurulu CLI'ınızın desteklediği hedef listesi için
`fennara mcp-setup --help` çalıştırın.

<a id="manual-setup"></a>
## Elle Kurulum

Elle kurulumu yalnızca uygulamanız listede yoksa, kurulum komutu uygulamanın
yapılandırma dosyasını bulamıyorsa veya MCP yapılandırmasını bilinçli olarak
elle düzenlemek istiyorsanız kullanın.

Düzenlemeden önce yapılandırma dosyasının yedeğini alın. Ardından kararlı
Fennara MCP başlatıcısını gösteren `fennara` adlı yerel bir stdio MCP sunucusu
ekleyin.

Varsayılan başlatıcı yolları:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Makinenizdeki gerçek mutlak yolu kullanın. MCP uygulamalarını
`versions/<version>/fennara-mcp-runtime` konumuna yönlendirmeyin; `bin/`
içindeki kararlı başlatıcı, uygulama yapılandırmalarının Fennara güncellemeleri
boyunca çalışmasını sağlar.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

Birçok MCP uygulaması üst düzey bir `mcpServers` nesnesi kullanır:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Bazı uygulamalar aynı `mcpServers` anahtarını kullanır, ancak yalnızca
`command` gerektirir. Mevcut yapılandırmada zaten başka sunucular varsa bu
girdileri koruyun ve yalnızca `fennara` sunucusunu ekleyin.

Cline biçimli yapılandırmalar saniye cinsinden daha uzun bir araç zaman aşımı da içerebilir:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### VS Code Biçimli JSON `servers`

VS Code kullanıcı veya proje MCP yapılandırması dahil bazı istemciler üst düzey
bir `servers` nesnesi kullanır ve `type: "stdio"` gerektirir:

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### OpenCode Biçimli JSON `mcp`

OpenCode biçimli JSON yapılandırması üst düzey bir `mcp` nesnesi kullanır.
Zaman aşımı milisaniye cinsindendir:

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### Codex Biçimli TOML

Codex TOML kullanır:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

JSON'ı bir TOML dosyasına veya TOML'ı bir JSON dosyasına yapıştırmayın.
Uygulamanın zaten kullandığı biçimle eşleştirin.

<a id="common-config-locations"></a>
## Yaygın Yapılandırma Konumları

Bunlar Fennara'nın kurulum yardımcısının ve güncel MCP istemcilerinin kullandığı
yaygın konumlardır. Uygulamalar yapılandırma yollarını değiştirebilir ve bazıları
hem genel hem de projeye özel yapılandırmaları destekler. Bir uygulamada
**Open MCP Config** gibi bir komut varsa tahmin etmek yerine bunu kullanın.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

<a id="timeout-guidance"></a>
## Zaman Aşımı Rehberi

Bazı Fennara araçlarının, Godot'tan sahneleri doğrulamasını, çalışma zamanı
durumunu incelemesini, ekran görüntüsü yakalamasını veya tanılama çalıştırmasını
isteyebildikleri için küçük bir varsayılan MCP zaman aşımından daha uzun sürmesi
mümkündür.

İstemci desteklediğinde araç başına daha uzun bir zaman aşımı kullanın:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Bir istemci sunucu başına zaman aşımlarını desteklemiyorsa o istemcinin
belgelenmiş genel MCP zaman aşımı ayarını kullanın.

<a id="verify-the-connection"></a>
## Bağlantıyı Doğrulayın

Godot projesini açın, ardından MCP uygulamanıza şunu sorun:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Birden fazla Godot projesi açıksa harici MCP araç çağrılarını hangi projenin
alacağını seçmek için Fennara dock'unun **MCP target** denetimini kullanın.

<a id="troubleshooting"></a>
## Sorun Giderme

Fennara MCP uygulamasında görünmüyorsa:

- başlatıcı yolunun mutlak olduğunu ve mevcut olduğunu doğrulayın
- yapılandırma söz diziminin uygulamanın gerektirdiği biçimde geçerli JSON, JSON5 veya TOML olduğunu doğrulayın
- sunucunun `fennara` olarak adlandırıldığını doğrulayın
- uygulamanın düzenlediğiniz yapılandırma dosyasını okuduğunu doğrulayın
- MCP uygulamasından tamamen çıkıp yeniden açın
- Godot projesinde Fennara eklentisinin kurulu olduğunu doğrulayın
- amaçlanan Godot projesinin MCP hedefi olarak seçildiğini doğrulayın

<a id="unsupported-mcp-apps"></a>
## Desteklenmeyen MCP Uygulamaları

MCP uygulamanız listede yoksa önce o uygulamanın resmi MCP yapılandırma konumunu
ve biçimini bulun. Ardından bir LLM'den en küçük güvenli düzenlemeyi isteyin:

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

Kaydetmeden önce sonucu inceleyin, ardından MCP uygulamasını yeniden başlatın.
