<!-- fennara-i18n: locale=tr source=docs/setup.md sha256=d470da8cda3e69aacb89ee5f06f1d7831df5ab7f50c94a599bfab5bfe22157e3 -->
<a id="setup"></a>
# Kurulum

<!-- fennara-doc-nav:start -->
[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../setup.md)
<!-- fennara-doc-nav:end -->

Fennara'yı kurun, nerede sohbet etmek istediğinizi seçin ve Godot projenizi bağlayın.

> [!TIP]
> Çoğu kullanıcının yalnızca eklentiyi eklemesi, Fennara dock'unu açması ve
> **Set Up Fennara** düğmesine basması gerekir. macOS'ta elle indirilen bir
> eklenti ZIP'inden sonra görülebilen güvenlik bildirimini önlemek için aşağıdaki
> CLI kurulumunu kullanın.

<a id="before-you-start"></a>
## Başlamadan Önce

| Gereksinim | Ne zaman gerekir |
| --- | --- |
| Godot 4.5 veya daha yenisi | Her zaman |
| Windows x86_64, Linux x86_64 veya macOS arm64 | Her zaman |
| MCP özellikli bir AI uygulaması | Yalnızca harici MCP kullanımı için |
| Bir bulut API anahtarı, Ollama veya LM Studio | Yalnızca yerleşik sohbet için |
| `dotnet` olarak kullanılabilen .NET SDK | Yalnızca C# tanılamaları ve çalışma zamanı ön denetimi için |

<a id="install-from-godot"></a>
## Godot'tan Kurun

> [!IMPORTANT]
> macOS'ta sürüm eklentisi, şu anda Apple tarafından noter onayı verilmemiş
> yerel bir kitaplık içerir. Eklenti ZIP'ini tarayıcı üzerinden indirmek ve elle
> çıkarmak, macOS'un `libfennara.macos.editor` dosyasının kötü amaçlı yazılım
> içermediğini doğrulayamadığını bildirmesine neden olabilir. Bu bildirimi
> önlemek için [Terminalden Kurun](#install-from-the-terminal-recommended-on-macos)
> bölümünü kullanın.

1. [En son sürümden](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   `fennara-addon-latest.zip` dosyasını indirin ve `addons/fennara/` dizinini
   projenize kopyalayın.
2. Projeyi açın ve Fennara dock'unu seçin.
3. **Set Up Fennara** düğmesine basın.

Fennara eşleşen yerel bileşenleri kurar ve açık projeyi bağlar. Daha eski bir
paylaşılan daemon boşta ise kurulum eşleşen sürümü etkinleştirmeden önce onu
durdurur. Sürüm geçişi sıfır bağlı proje gerektirir. Kurulmakta olan proje,
sürümler farklıyken normalde bağlantısız kalır. Kurulum bağlı bir proje
bildirirse Fennara etkin diğer tüm düzenleyicileri kapatıp yeniden deneyin.
Geçerli proje için eski bir bağlantı kalırsa bu düzenleyiciyi kapatıp yeniden
açın, ardından yeniden deneyin.
Kurulum başarısız olursa dock **Retry**, **Copy Report** ve **Open Logs**
seçeneklerini sunar. Kopyalanan raporlar temizlenir ve API anahtarları, sohbet
içeriği veya proje dosyalarını içermez.

> [!NOTE]
> Eklenti projenizde kalır. CLI, daemon, MCP sunucusu, günlükler ve paylaşılan
> tarayıcı çalışma zamanı projenin dışındaki Fennara uygulama verilerinde bulunur.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## Terminalden Kurun (macOS'ta Önerilir)

CLI aynı eklentiyi kurar ve macOS'ta önerilen kurulum yöntemidir. Yukarıda
açıklanan yerel kitaplık bildirimine neden olan tarayıcı ve Finder karantina
yolunu önler.

CLI'ı Windows'a kurun:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Veya macOS ve Linux'a:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Ardından Fennara'yı proje içinde çalıştırın:

```bash
cd path/to/your-godot-project
fennara install
```

Eklentiyi macOS'ta zaten elle çıkardıysanız ve bildirimi görüyorsanız
`fennara install` çalıştırmadan önce Godot'u kapatıp elle kopyalanmış
`addons/fennara/` klasörünü kaldırın. Bu önemlidir çünkü CLI mevcut eksiksiz bir
eklentiyi değiştirmek yerine korur.

Proje zaten eksiksiz bir Fennara eklentisi içeriyorsa CLI bunu tutar ve eşleşen
yerel bileşenleri kurar. Aksi durumda geçerli sürüm eklentisini de kurar. Sürüm
sabitleme ve otomasyon için [CLI kurulum başvurusuna](cli.md#install-a-project)
bakın.

<a id="choose-how-you-use-fennara"></a>
## Fennara'yı Nasıl Kullanacağınızı Seçin

| Yol | Model hesabı | Kurulum |
| --- | --- | --- |
| Yerleşik sohbet | Fennara Chat Settings içinde bağlanan bir sağlayıcı | [Bir sağlayıcı bağlayın](#connect-the-built-in-chat) |
| Harici MCP uygulaması | Uygulamanın kendi model hesabı veya aboneliği | [Bir MCP uygulaması bağlayın](#connect-an-mcp-app) |
| İkisi de | Her yol kendi model ayarlarını tutar | İki bölümü de tamamlayın |

<a id="connect-the-built-in-chat"></a>
### Yerleşik Sohbeti Bağlayın

1. **Chat Settings > Chat** bölümünü açın.
2. **Open providers** seçeneğini belirleyin.
3. Kendi anahtarınızla bir bulut sağlayıcısı bağlayın veya yerel bir Ollama ya da
   LM Studio sunucusu bağlayın.
4. Bir model seçin.

Desteklenen sağlayıcılar, anahtarlar, yerel sunucu URL'leri ve model kimlikleri
için [Yerleşik Sohbet Sağlayıcıları](providers.md) sayfasına bakın. Oluşturucu
içinden aynı eylemler için `/provider` ve `/model` kullanın.

Gömülü sohbet platform webview'ını kullanır:

| Platform | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | Sistem WKWebView/WebKit |
| Linux | Fennara tarafından yönetilen paylaşılan CEF çalışma zamanı |

`fennara install`, `fennara update` ve `fennara doctor` bu ön koşulları
denetler. İsteğe bağlı gömülü sohbet başlatılamasa da MCP araçları çalışmaya
devam eder.

Bunun yerine sistem tarayıcısını kullanmak için Chat Settings içinde **Open chat
in my system browser next time** seçeneğini etkinleştirip Godot'u yeniden
başlatın. Bu yalnızca yerleşik sohbetin göründüğü yeri değiştirir. Aynı
sağlayıcıyı, geçmişi ve proje bağlantısını korur.

Sonraki yerleşik sohbet iletisine kod eklemek için Godot'un betik
düzenleyicisinde kodu seçin, bağlam menüsünü açın ve **Add to Chat** seçeneğini
belirleyin.

<a id="connect-an-mcp-app"></a>
### Bir MCP Uygulaması Bağlayın

**Chat Settings > MCP Apps** bölümünü açın, uygulamanızı bulun ve **Set Up**
düğmesine basın. Fennara'yı yükleyebilmesi için uygulamayı yeniden başlatın.

Bir uygulamayı terminalden de bağlayabilirsiniz:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Uygulamanız listede yoksa desteklenen tüm hedefler ve elle yapılandırma
biçimleri için [MCP Kurulumu](mcp-setup.md) sayfasına bakın.

Kurulum girdisi genel ve projeden bağımsızdır. Birden fazla ajan ayrı depolar
veya worktree'ler kullanıyorsa kurulumdan sonra her Proje Kökü için projeye
bağlı ayrı bir MCP bağlantısı yapılandırın. Bkz.
[Birden Fazla Ajan ve Worktree](multi-agent-worktrees.md).

Harici MCP uygulamaları kendi model hesaplarını kullanır. Yerleşik sohbet
Fennara Chat Settings içinde seçilen sağlayıcıyı kullanır. Ayrım için
[MCP Uygulamaları ve Yerleşik Sohbet](chat-vs-mcp.md) sayfasına bakın.

<a id="verify-the-connection"></a>
## Bağlantıyı Doğrulayın

Godot projesini açın, ardından MCP uygulamanıza şunu sorun:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Yalıtılmış çalışma için `bound` yönlendirme modunu, beklenen kanonik Proje
Kökünü, `connected` bağlı editör durumunu ve o editörün dosya sistemi hazır olma
durumunu bildirdiğini doğrulayın. Eşleşen açık editörü olmayan bağlı bir bağlantı
canlı kalır ve editör yeniden bağlanana kadar yeniden denenebilir
`bound_project_not_connected` durumunu bildirir.

Durum `legacy_unbound` bildiriyorsa panelin MCP hedefi uyumluluk yoludur. Tek
istemcili kullanım için amaçlanan hedefi seçin veya eşzamanlı çalışmadan önce
açık bir Proje Bağlaması yapılandırın.

<a id="update-fennara"></a>
## Fennara'yı Güncelleyin

Dock **Update** gösterdiğinde buna basın ve istemleri izleyin. Fennara, Godot'u
kapatmanızı istemeden önce güncellemeyi indirip doğrular. Kurulumdan sonra aynı
projeyi yeniden açar ve güncelleme doğrulanana kadar önceki çalışan sürümü tutar.

Terminalden güncellemek için Godot'u kapatıp şunu çalıştırın:

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Fennara v0.3.8 veya daha eski bir sürümden yükseltiyorsanız `fennara update`
> çalıştırmadan önce yukarıdaki platform kurulum komutuyla CLI'ı bir kez yeniden
> kurun. Bu CLI'lar kullanımdan kaldırılmış bir sürüm etiketini sorgular ve
> güncel sürümleri keşfedemez. CLI'ı yeniden kurmak, proje eklentinizi veya
> ayarlarınızı kaldırmadan gelecekteki güncellemeleri GitHub'ın Latest Release
> uç noktasına geçirir.

> [!IMPORTANT]
> macOS'ta Fennara v0.3.11 sürümünden yükseltmeden önce CLI'ı bir kez yeniden
> kurun. Bu CLI kendini güncellemeye ulaşmadan önce mevcut framework paketini
> reddeder. Yeniden kurulum yalnızca CLI'ı değiştirir ve proje eklentisiyle
> ayarları korur.

Doğrulama başarısız olursa dock'ta **Restore Previous Version**, **Open Logs**
veya **Copy Report** kullanın. Tam sürümler, hazırlık ve kesintiye uğrayan
güncelleme kurtarması için [CLI güncelleme başvurusuna](cli.md#update-a-project)
bakın.

<a id="troubleshooting"></a>
## Sorun Giderme

<a id="an-install-or-update-failed"></a>
### Bir Kurulum veya Güncelleme Başarısız Oldu

Dock'tan temizlenmiş raporu kopyalayın veya terminalde en son raporu gösterin:

```bash
fennara diagnostics
```

İşlem kimlikleri, JSON çıktısı, kaydedilen alanlar ve sansürleme güvenceleri
için [CLI tanılamalarına](cli.md#inspect-health-and-failures) bakın.

<a id="fennara-is-not-found"></a>
### `fennara` Bulunamadı

Yeni bir terminal açıp şunu çalıştırın:

```bash
fennara doctor
```

Komut hâlâ kullanılamıyorsa Fennara `bin` dizinini PATH'e ekleyin.
[CLI kurulum sayfası](cli.md#install-the-cli) platform yollarını listeler.

<a id="windows-binaries-fail-before-starting"></a>
### Windows İkili Dosyaları Başlamadan Başarısız Oluyor

Bir Fennara ikili dosyası eksik bir `VCRUNTIME` veya `MSVCP` DLL, çıkış kodu
`-1073741515` ya da `0xc0000135` bildirirse Microsoft Visual C++
Redistributable 2015-2022 x64 kurun:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

Bu yalnızca söz konusu Microsoft çalışma zamanı DLL'lerinin eksik olduğu Windows
makinelerinde gereklidir.

<a id="a-release-requires-a-newer-cli"></a>
### Bir Sürüm Daha Yeni Bir CLI Gerektiriyor

CLI kendini güncelleme gerekli sürümü kuramıyorsa
[CLI'ı Kurun](cli.md#install-the-cli) bölümündeki kurulum betiğini yeniden
çalıştırın, ardından komutu yeniden deneyin.

<a id="the-addon-is-not-visible-in-godot"></a>
### Eklenti Godot'ta Görünmüyor

Bu dosyanın mevcut olduğunu doğrulayın, ardından projeyi yeniden açın:

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` Yanlış Projeyi Gösteriyor

Önce bildirilen yönlendirme modunu okuyun. `bound` için MCP işlemini amaçlanan
`--project-path`, `FENNARA_PROJECT_PATH` veya proje başlangıç diziniyle yeniden
başlatın. `legacy_unbound` için amaçlanan projeyi açın ve Fennara panelindeki
MCP hedef denetimiyle seçin.

Bağlı bir kök `not_connected` durumundaysa o projeyi açın ve eklentisinin
bağlanmasını bekleyin. `ambiguous` durumundaysa yinelenen editörü kapatın veya
onu ayrı bir worktree'den başlatın.

<a id="c-diagnostics-are-missing"></a>
### C# Tanılamaları Eksik

Projenin tek ve belirgin bir `.csproj`, `.sln` veya `.slnx` içerdiğini doğrulayın,
ardından şunu çalıştırın:

```bash
dotnet --version
```

Tarayıcı çalışma zamanı yerleşimleri, elle kurtarma ve uygulama ayrıntıları için
[Mimari](architecture.md), [Elle Kurulum](manual-install.md) ve
[SSS](faq.md) sayfalarına bakın.
