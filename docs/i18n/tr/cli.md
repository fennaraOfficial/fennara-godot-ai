<!-- fennara-i18n: locale=tr source=docs/cli.md sha256=16441a0d18c69d735854b2f54a905e9d7f5277a8eae9a9c89eced18cfcaca06a -->
<a id="fennara-cli"></a>
# Fennara CLI

<!-- fennara-doc-nav:start -->
[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../cli.md)
<!-- fennara-doc-nav:end -->

Terminali tercih ettiğinizde, tanılama veya kurtarmaya ihtiyaç duyduğunuzda ya da tam bir sürümle otomatik kurulum istediğinizde CLI'yi kullanın.

> [!TIP]
> CLI, macOS'ta önerilen kurulum yöntemidir. Tarayıcıdan indirilen bir eklenti ZIP dosyası elle açıldığında ve yerel kitaplığı Finder karantinasını devraldığında oluşabilen macOS güvenlik bildirimini önler.

<a id="common-flow"></a>
## Yaygın Akış

```bash
cd path/to/your-godot-project
fennara install
```

Yerel kurulumu incelemeniz veya onarmanız gerektiğinde `fennara doctor` kullanın.

Normal Godot yolculuğu için [Kurulum](setup.md) bölümünü kullanın. Bu sayfayı terminal komutları başvurusu olarak saklayın.

<a id="install-the-cli"></a>
## CLI'yi Kurma

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS ve Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Elle çıkarılmış bir macOS eklentisi `libfennara.macos.editor` için zaten bildirim gösteriyorsa `fennara install` komutunu çalıştırmadan önce Godot'yu kapatın ve elle kopyalanmış `addons/fennara/` klasörünü kaldırın. CLI bunun dışındaki durumlarda mevcut eksiksiz eklentiyi korur.

`fennara` hemen kullanılamıyorsa yeni bir terminal açın, ardından kurulumu denetleyin:

```bash
fennara --version
fennara doctor
```

CLI kullanıcı başına kurulur. Proje eklentileri kendi Godot projelerinde kalır; paylaşımlı başlatıcılar, sürümlenmiş çalışma zamanları, işlem kayıtları, günlükler ve Linux CEF, Fennara uygulama verilerinde tutulur:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## Komut Özeti

| Komut | Amaç |
| --- | --- |
| `fennara install` | Bir proje eklentisini ve eşleşen yerel bileşenlerini kurma veya benimseme |
| `fennara update` | Bir projeyi ve yerel bileşenlerini güncelleme |
| `fennara doctor` | Yerel kurulumu inceleme veya onarma |
| `fennara diagnostics` | Arındırılmış bir işlem raporu gösterme |
| `fennara mcp-setup` | Harici bir MCP uygulaması bağlama |
| `fennara prepare-export` | Eklentisiz bir CI dışa aktarmasından önce Fennara autoload'unu kaldırma |
| `fennara recover` | Kesintiye uğramış yerel bir güncellemeyi geri yükleme |
| `fennara self-update` | Yalnızca kurulu CLI'yi güncelleme |

Kurulu komutların özeti için `fennara --help` çalıştırın. Desteklenen MCP uygulaması hedefleri için `fennara mcp-setup --help` kullanın.

<a id="install-a-project"></a>
## Bir Proje Kurma

`project.godot` içeren bir klasörde çalıştırın:

```bash
fennara install
```

Ya da projeyi açıkça belirtin:

```bash
fennara install --project path/to/project
```

`--version` olmadan CLI güncel sürüm manifestosunu seçer. Yeniden üretilebilirlik önemliyse tam bir sürüm kullanın:

```bash
fennara install --project path/to/project --version <version>
```

Kurulumun iki güvenli yolu vardır:

- Eksiksiz bir eklenti yoksa CLI seçili sürümü indirip doğrular, `addons/fennara` dizinini kurar, eşleşen yerel bileşenleri kurar ve Fennara proje yönergelerini yazar.
- Eksiksiz bir eklenti zaten varsa CLI onun `VERSION` değerini okur, mevcut platform kitaplığını doğrular ve tam olarak o sürüme ait CLI tarafından yönetilen bileşenleri kurar. Proje eklentisini değiştirmeden tutar. Açıkça verilen `--version` mevcut eklentiyle eşleşmelidir.

Sürüm tabanlı kurulumlarda CLI önce isteği tek bir tam sürüme çözümler, bu sürüm daha yeni bir CLI sağlıyorsa kurulu Fennara CLI'yi günceller ve ardından kuruluma yeni CLI ile devam eder. Yerel `--source` kurulumları sürüm hizmetine bağlanmaz ve kendi kendini güncellemez.

<a id="prepare-an-addon-free-ci-export"></a>
## Eklentisiz Bir CI Dışa Aktarması Hazırlama

`addons/fennara/` bir CI checkout'undan dışlanıyorsa Godot başlamadan önce
Fennara'nın kalıcı çalışma zamanı autoload'unu kaldırın:

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

Komut yalnızca `project.godot` içindeki `_fennara_game_capture` girdisini
düzenler. Diğer autoload'ları ve ayarları korur, ayrıca yeniden çalıştırılması
güvenlidir. Proje başlangıcı, editör veya dışa aktarma eklentileri çalışmadan
önce autoload yollarını doğruladığı için bu adım Godot'dan önce
gerçekleştirilmelidir. Alternatif olarak CI, Godot'yu başlatmadan önce Fennara
eklentisini kurabilir.

<a id="update-a-project"></a>
## Bir Projeyi Güncelleme

Normal bir terminal güncellemesi için o projeye ait Godot'yu kapatın ve şunu çalıştırın:

```bash
fennara update --project path/to/project
```

`--version` olmadan CLI kurulu eklenti kimliğini okur. Kararlı eklentiler GitHub'ın Latest sürümünü, hazırlama eklentileri ise yalnızca kendi `pr-<number>` kanalını çözümler. Seçici, CLI'nin kendini değiştirmesi dahil, hemen tek bir kesin sürüme sabitlenir. CLI ardından sürüm varlıklarını doğrular, eklentiyi ve sürümlenmiş yerel bileşenleri yeniler, proje yönergelerini günceller ve platform web görünümü önkoşulunu denetler. Kesin bir sürümü açıkça seçmek için `--version <version>` kullanın.

`--no-self-update`, denetimli otomasyon veya CLI zaten değiştirildikten sonraki devam işlemi içindir. Bir sürümün asgari CLI gereksinimini atlatmak için kullanmayın.

> [!IMPORTANT]
> Fennara v0.3.8 veya daha eski bir sürümden yükseltiyorsanız `fennara update` komutundan önce [Kurulum](setup.md#install-from-the-terminal-recommended-on-macos) bölümündeki platform kurulum komutuyla CLI'yi bir kez yeniden kurun. Bu CLI'ler kullanımdan kaldırılmış bir sürüm etiketini sorgular ve güncel sürümleri bulamaz. CLI'yi yeniden kurmak proje eklentinizi veya ayarlarınızı kaldırmaz.

> [!IMPORTANT]
> macOS'ta Fennara v0.3.11'den yükseltmeden önce CLI'yi bir kez yeniden kurun. Bu CLI, kendini güncelleme aşamasına ulaşmadan mevcut framework paketini reddeder. Yeniden kurulum yalnızca CLI'yi değiştirir ve proje eklentisiyle ayarları korur.

<a id="prepare-while-godot-is-open"></a>
### Godot Açıkken Hazırlama

Editör içi güncelleme düğmesi hazırlama biçimini kullanır:

```bash
fennara update --prepare --project path/to/project
```

Hazırlama eklentiyi indirir, doğrular ve kalıcı biçimde hazırlar. Godot'yu kapatmaz, canlı eklentiyi değiştirmez, etkin çalışma zamanı manifestosunu değiştirmez veya daemon'u yeniden başlatmaz. Godot paneli işlem makbuzunu izler ve ayrık kapatma, değiştirme, yeniden açma ve doğrulama adımına başlamadan önce kullanıcıya sorar. Panel keşfettiği tam sürümü ilettiği için göstergenin hareket etmesi sürmekte olan güncellemeyi değiştiremez.

Fennara aynı anda tek bir etkin paylaşımlı çalışma zamanı sürümünü destekler. Fennara etkin başka bir Godot editörü paylaşımlı daemon'a bağlı kalırsa etkinleştirme engellenir. Diğer editörü kapatın ve yeniden deneyin. Önceki yerel sürüm ve çalışma zamanı göstergesi ağ erişimi olmadan kurtarma için kullanılabilir kalır.

`--prepare`, Godot tümleştirmesine yönelik alt düzey bir ilkeldir. Terminal kullanıcıları normalde Godot zaten kapalıyken `fennara update` kullanır.

<a id="recover-an-interrupted-update"></a>
## Kesintiye Uğramış Güncellemeyi Kurtarma

Güncellenen eklenti kurtarma panelini gösterecek kadar yüklenemezse Godot'yu kapatın ve şunu çalıştırın:

```bash
fennara recover --project path/to/project
```

CLI yalnızca kurtarılabilir durumdaki işlemleri geri yükler. Önceki eklentiyi, paylaşımlı başlatıcıları ve etkin çalışma zamanı manifestosunu geri yükler, ardından kaydedilmiş Godot yürütülebilir dosyasını yeniden açmayı dener. Destek ekibi işlem kimliğini verdiğinde belirli bir işlemi seçin:

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Tamamlanmış, yalnızca hazırlanmış ve zaten geri alınmış işlemler reddedilir.

<a id="inspect-health-and-failures"></a>
## Durumu ve Hataları İnceleme

`doctor` algılanan platformu, uygulama verileri düzenini, etkin sürümü, başlatıcıları, çalışma zamanlarını, daemon durumunu ve web görünümü önkoşulunu bildirir:

```bash
fennara doctor
```

Çalışan daemon veya MCP çalışma zamanının `current.json` dosyasından daha eski olduğunu bildirirse seçili çalışma zamanını başlatması için Godot'yu ya da etkilenen MCP uygulamasını yeniden başlatın.

Eksik temel uygulama verileri dizinlerini yeniden oluşturmak için `--repair` kullanın. Linux'ta ayrıca eski CEF işlem profillerini temizler ve eksiksiz bir yönetilen çalışma zamanı zaten kuruluysa geçerli çalışma zamanı işaretçisini onarır:

```bash
fennara doctor --repair
```

Kurulum, güncelleme, kurtarma ve kendini güncelleme işlemleri kalıcı durum ve olaylar yazar. En yeni arındırılmış raporu şu komutla gösterin:

```bash
fennara diagnostics
```

Daha eski bir işlem veya makine tarafından okunabilen çıktı için:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Raporlar kararlı hata kodlarını, aşamaları, bileşen sürümlerini, seçili varlık adlarını ve karma doğrulama sonuçlarını içerir. Proje, ana dizin ve Fennara uygulama verileri yollarını, kimlik bilgilerini, taşıyıcı token'larını ve URL sorgularını gizler. Sohbet mesajlarını, sağlayıcı anahtarlarını veya proje dosyası içeriklerini içermez.

<a id="configure-an-external-mcp-app"></a>
## Harici MCP Uygulamasını Yapılandırma

Godot sohbet paneli bu komutları **Chat Settings > MCP Apps** altında sunar. Set Up düğmesi yerel daemon'dan kurulu CLI'yi çağırmasını ister, böylece panel ve terminal iş akışları aynı yapılandırma ve yedekleme uygulamasını kullanır.

Desteklenen hedefi seçmek için `fennara mcp-setup --help` çalıştırın. Yapılandırmasını değiştirdikten sonra MCP uygulamasını yeniden başlatın. Bu komut harici uygulamayı Fennara MCP sunucusuna bağlar; yerleşik Godot sohbet panelinin kullandığı model sağlayıcısını seçmez. Hedef listesi, yapılandırma konumları ve elle yapılandırma örneklerinin kaynağı [MCP Kurulumu](mcp-setup.md) sayfasıdır.

<a id="update-only-the-cli"></a>
## Yalnızca CLI'yi Güncelleme

Normal proje güncellemeleri CLI'nin kendini güncellemesini otomatik olarak yönetir. Yalnızca kurulu CLI'yi güncellemek için:

```bash
fennara self-update
fennara self-update --version <version>
```

`--version` olmadan kendini güncelleme etkin kurulum izini korur: kararlı iz GitHub'ın Latest sürümünü, hazırlama izi ise yalnızca kaydedilmiş PR kanalını kullanır.

Hazırlama izi hiçbir zaman otomatik olarak kararlı ize geçmez. Hazırlama izinden bilinçli olarak çıkmak için Godot'yu kapatın ve `fennara update --version <stable-version> --project <path>` çalıştırın. Paylaşımlı etkin sürüm değişmeden önce tam olarak bu kararlı sürüm doğrulanır.

Bunu destek ekibi istediğinde veya bir proje güncellemesi kurulu CLI'nin güvenli biçimde devam etmek için çok eski olduğunu bildirdiğinde kullanın.

<a id="automation-guidance"></a>
## Otomasyon Yönergeleri

- Geçerli dizine güvenmek yerine `--project` iletin.
- Bir derlemenin yeniden üretilebilir olması gerektiğinde `--version` sabitleyin.
- Hata durumunda yazdırılan işlem kimliğini ve günlük yolunu saklayın.
- Yapılandırılmış raporlama için `fennara diagnostics --operation <id> --json` kullanın.
- `current.json`, sürüm dizinleri, güncelleme makbuzları veya hazırlanmış eklenti klasörlerini elle düzenlemeyin.
- Proje Godot'da açıkken eklentiyi değiştiren normal bir güncelleme çalıştırmayın. Editör içi güncelleme akışını kullanın veya önce Godot'yu kapatın.
