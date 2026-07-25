<!-- fennara-i18n: locale=tr source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Elle Kurulum

<!-- fennara-doc-nav:start -->
[Belgeler](README.md)

🌐 **Diller**

[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Bu sayfayı yalnızca Fennara'yı Godot kurulum akışı veya `fennara install`
olmadan bir araya getirmeniz gerektiğinde kullanın.

> [!TIP]
> Windows ve Linux'ta çoğu kullanıcı projeye `addons/fennara` eklemeli,
> Fennara dock'unu açmalı ve **Set Up Fennara** düğmesine basmalıdır. macOS'ta CLI'ı kullanın.
> [Kurulum](setup.md) sayfasına bakın.

> [!IMPORTANT]
> Eklenti ZIP'inin elle kurulması macOS'ta önerilmez. Eklenti, şu anda Apple
> tarafından noter onayı verilmemiş yerel bir kitaplık içerir; tarayıcıdan
> indirme ve Finder ile çıkarma, macOS'un `libfennara.macos.editor` dosyasının
> kötü amaçlı yazılım içermediğini doğrulayamadığını bildirmesine neden olabilir.
> Bu bildirimi önlemek için
> [CLI kurulumunu](setup.md#install-from-the-terminal-recommended-on-macos)
> kullanın. Bildirim zaten görünüyorsa Godot'u kapatın, elle kopyalanmış
> `addons/fennara/` klasörünü kaldırın ve `fennara install` çalıştırın.

Elle kurulum dört bölümden oluşur: CLI, proje eklentisi, paylaşılan yerel
çalışma zamanı paketi ve isteğe bağlı MCP uygulaması yapılandırması.

<a id="1-download-release-files"></a>
## 1. Sürüm Dosyalarını İndirin

En son GitHub sürümünü açın:

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Sürüm manifestini, platformunuza ait dosyaları ve paylaşılan eklenti zip'ini indirin.

| Amaç | Yapı |
| --- | --- |
| Sürüm planı ve SHA-256 değerleri | `fennara-release-manifest-v<version>.json` |
| Windows x86_64 CLI | `fennara-cli-windows-x86_64-v<version>.zip` |
| Windows x86_64 yerel çalışma zamanı | `fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 CLI | `fennara-cli-linux-x86_64-v<version>.zip` |
| Linux x86_64 yerel çalışma zamanı | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Linux x86_64 gömülü webview | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 CLI | `fennara-cli-macos-arm64-v<version>.zip` |
| macOS arm64 yerel çalışma zamanı | `fennara-release-local-macos-arm64-v<version>.zip` |
| Sürümlü tüm platformlar eklentisi | `fennara-release-addon-v<version>.zip` |

Sürüm ayrıca belgeler ve elle indirmeler için kararlı adlı şu eklenti diğer
adını içerir:

```text
fennara-addon-latest.zip
```

Manifest, yerel çalışma zamanı, eklenti ve paylaşılan çalışma zamanı yapıları
için beklenen SHA-256 değerlerini kaydeder. Elle indirilen dosyaları denetlerken
bunu doğruluk kaynağı olarak kullanın.

<a id="2-install-the-cli"></a>
## 2. CLI'ı Kurun

`fennara-cli` zip'ini çıkarın.

`bin` dizinini PATH'e ekleyin veya `fennara` ikili dosyasını mevcut PATH klasörlerinizden birine kopyalayın.

Denetleyin:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Godot Eklentisini Kurun

`fennara-addon` zip'ini çıkarın.

Şunu kopyalayın:

```text
addons/fennara
```

Godot projenize kopyaladığınızda proje şunu içermelidir:

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Yerel Çalışma Zamanı Paketini Kurun

CLI normalde bunu sizin için yönetir. Elle çalışma zamanı kurulumu yalnızca `fennara install` komutunu kullanmıyorsanız gereklidir.

Varsayılan Fennara veri klasörleri:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

Beklenen yerleşim:

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

Windows'ta ikili dosyalar `.exe` uzantısını kullanır.

`current.json`, başlatıcı ikili dosyalarını etkin çalışma zamanı sürümüne yönlendirir. Normal `fennara install` ve `fennara update` komutları bu dosyayı otomatik olarak oluşturur.

Linux gömülü sohbeti paylaşılan `webview/cef/linux-x64/<cef-version>/` çalışma
zamanı konumunu kullanır. Normal `fennara install` / `fennara update`
çalıştırmaları, sürüm tarafından yönetilen CEF çalışma zamanını sürüm manifesti
ve yapısından otomatik olarak kurar. Her şeyi elle kuruyorsanız
`fennara-webview-cef-linux-x64-<cef-version>.zip` dosyasını bu paylaşılan
çalışma zamanı konumuna çıkarın ve eşleşen
`webview/cef/linux-x64/current.json` işaretçisini yazın. Bu yükü Godot proje
eklentisinin dışında tutun; `addons/fennara`, `libcef.so` veya başka CEF
çalışma zamanı dosyaları içermemelidir.

Bu CEF yükü yalnızca gömülü Linux sohbeti içindir. Kullanıcılar, aynı yerleşik
sohbeti gömülü Godot webview'ı yerine yerel daemon aracılığıyla sistem
tarayıcılarında görüntülemek için Chat Settings içinde **Open chat in my
system browser next time** seçeneğini seçebilir.

Son Linux CEF yerleşimi şöyle görünmelidir:

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

`webview/cef/linux-x64/current.json` şu olmalıdır:

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json`, CEF yapısına
ait eşleşen sürüm manifesti olmalıdır, örneğin:

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

Yazılabilir tarayıcı durumunu CEF sürüm dizininin içine koymayın. Normal
kullanım, düzenleyici başına profilleri ve günlükleri Fennara uygulama verisi
önbellek/günlük kökleri altına yazarken çalışma zamanı yükü paylaşılan ve salt
okunur olarak kalır.

<a id="5-configure-your-mcp-app"></a>
## 5. MCP Uygulamanızı Yapılandırın

Yerel çalışma zamanı paketi kurulduktan sonra MCP uygulamanızı yapılandırın:

```bash
fennara mcp-setup --claude
```

Diğer hedefler:

```bash
fennara mcp-setup --help
```

Kurulumdan sonra MCP uygulamasını yeniden başlatın.

Uygulamanız listede yoksa veya bu kurulumun parçası olarak MCP
yapılandırmasını elle düzenliyorsanız kararlı başlatıcı yolu ve JSON/TOML
örnekleri için [MCP Kurulumu](mcp-setup.md) sayfasına bakın.

Bu yalnızca harici MCP uygulamasını Fennara'nın Godot araçlarına bağlar.
Yerleşik Fennara sohbet dock'unun model sağlayıcısını yapılandırmaz. Yerleşik
sohbet istiyorsanız dock'u Godot içinde yapılandırın veya
[MCP Uygulamaları ve Yerleşik Sohbet](chat-vs-mcp.md) sayfasına bakın.

<a id="6-verify"></a>
## 6. Doğrulayın

Godot projesini açın, ardından MCP uygulamanıza şunu sorun:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Yol doğruysa elle kurulum çalışıyor demektir.

<a id="recommended-shortcut"></a>
## Önerilen Kısayol

CLI'ı elle kursanız bile eklentiyi ve yerel çalışma zamanı paketini onun kurmasına izin verebilirsiniz:

```bash
cd path/to/your-godot-project
fennara install
```

CLI ayrıca AI kodlama aracıları için proje rehberi yazar:

```text
AGENTS.md
addons/fennara/ai/
```

AI dizini kısa, her zaman okunan yönergeleri, bir dizini ve yalnızca ilgili
olduğunda yüklenen özel sayfaları içerir. Elle kopyalanmış bir eklenti ZIP'i bu
paketlenmiş dizini içerebilir, ancak proje kökü `AGENTS.md` dosyasını oluşturmaz
veya yenilemez. Fennara'nın eksiksiz proje rehberini yönetmesi ve yenilemesi
gerektiğinde `fennara install` ve `fennara update` kullanın.
