<!-- fennara-i18n: locale=tr source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[Belgeler](docs/i18n/tr/README.md)

🌐 **Diller**

[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demolar](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/demos.md)
[![Lisans](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

[Somni Game Studios](https://somnigamestudios.com/) dahil Godot geliştiricileri ve ekipleri tarafından kullanılmaktadır.

Fennara, yapay zeka asistanlarına Godot ile canlı bir bağlantı sağlar. Codex, Claude, Cursor, Gemini ve Antigravity gibi MCP destekli uygulamalardan ya da isteğe bağlı editör içi sohbet panelinden kullanabilirsiniz.

Ajanlar yalnızca proje dosyalarından tahminde bulunmak yerine editörün içinden sahneleri inceleyebilir, betikleri denetleyebilir, ekran görüntüleri yakalayabilir, çalışma zamanı hatalarını okuyabilir ve değişiklikleri doğrulayabilir.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Fennara ile diğer Godot MCP'lerinin karşılaştırması" width="100%" />
      </a>
    </td>
    <td>
      <strong>Öne çıkan demoyu izleyin</strong><br />
      Fennara ile diğer Godot MCP'lerinin karşılaştırması.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Bu videoyu oynatın</a><br />
      <a href="docs/demos.md">Tüm demo videolarına göz atın</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## Ne Yapar

- harici yapay zeka uygulamalarına MCP üzerinden Godot bilgisine sahip araçlar sunar
- Godot editörünün içine isteğe bağlı yerel bir sohbet paneli ekler
- gerçek Godot geri bildirimi döndürür: sahne ağaçları, tanılamalar, ekran görüntüleri, çalışma zamanı günlükleri ve doğrulama sonuçları
- ajanı yalnızca dosya sistemine değil, açık editöre karşı da sorumlu tutar

Harici MCP uygulamaları ile yerleşik sohbet ayrı model ayarları kullanır. Bkz. [MCP Uygulamaları ve Yerleşik Sohbet](docs/chat-vs-mcp.md) ve [Yerleşik Sohbet Sağlayıcıları](docs/providers.md).

<a id="requirements"></a>
## Gereksinimler

- Godot 4.5 veya daha yeni bir sürüm.
- Desteklenen bir masaüstü işletim sistemi: Windows x86_64, Linux x86_64 veya macOS arm64.
- Fennara'yı Claude, Codex, Cursor, Gemini, Antigravity ya da başka bir harici yapay zeka uygulamasından kullanmak istiyorsanız MCP destekli bir kodlama uygulaması.
- Yerleşik Fennara sohbet panelini kullanmak istiyorsanız bir sohbet sağlayıcısı. Bu, bir bulut sağlayıcısı anahtarı veya Ollama / LM Studio gibi yerel bir sağlayıcı olabilir.

Kurulumun tamamı için [Kurulum](docs/setup.md) bölümüne bakın.

<a id="what-setup-adds"></a>
## Kurulum Neler Ekler

- `res://addons/fennara/` altında tutulan Fennara eklentisi
- Fennara uygulama verilerine kurulan küçük bir `fennara` CLI
- yapay zeka kodlama uygulamalarının kullandığı yerel bir MCP sunucusu
- MCP/sohbet isteklerini açık Godot editörüne bağlayan yerel bir daemon
- yapay zeka ajanları için oluşturulan proje yönergeleri

Yerleşik sohbet paneli platformun web görünümünü kullanır: Windows'ta Microsoft Edge WebView2, macOS'ta WKWebView/WebKit ve Linux'ta Fennara tarafından yönetilen paylaşımlı bir CEF çalışma zamanı. İsteğe bağlı sohbet paneli başlatılamasa bile MCP araçları çalışmaya devam eder.

<a id="install"></a>
## Kurulum

Windows ve Linux'ta eklenti veya CLI kurulumundan birini seçin. Eklenti ZIP dosyasını elle indirip açtıktan sonra görülebilen macOS güvenlik bildiriminden kaçınmak istiyorsanız macOS'ta aşağıdaki CLI kurulumunu kullanın.

<a id="add-the-addon-to-your-project"></a>
### Eklentiyi Projenize Ekleyin

- [En Son Sürüm](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest) sayfasını açın, `fennara-addon-latest.zip` dosyasını indirin ve içindeki `addons/fennara/` klasörünü projenize çıkarın.

Projeyi açın, Fennara panelini seçin ve **Set Up Fennara** düğmesine basın.

Fennara bir editör bağımlılığıdır, oyun çalışma zamanı bağımlılığı değildir.
Dışa aktarma sırasında editör eklentisi, çalışma zamanı autoload'unu dışa
aktarılan projeden kaldırır ve `res://addons/fennara/` ile `res://.fennara/`
altındaki dosyaları atlar. Dışa aktarma tamamlandığında editör projesi geri
yüklenir. Bir CI checkout'u eklentiyi `.gitignore` ile dışlıyorsa Godot'yu
başlatmadan önce `fennara prepare-export --project path/to/project` komutunu
çalıştırın veya eklentiyi bu checkout'a kurun. Godot, dışa aktarma eklentileri
çalışmadan önce autoload yollarını doğrular. Bu nedenle hazırlığın önce yapılması
gerekir.

> **macOS:** Sürüm eklentisi şu anda Apple tarafından noter onayı verilmemiş yerel bir kitaplık içerir. Eklenti ZIP dosyasını tarayıcı üzerinden indirip elle açarsanız macOS, `libfennara.macos.editor` dosyasının kötü amaçlı yazılım içermediğini doğrulayamadığını bildirebilir. Bu bildirimi önlemek için aşağıdaki CLI kurulumunu kullanın. Bildirimi zaten görüyorsanız Godot'yu kapatın, elle kopyalanmış `addons/fennara/` klasörünü kaldırın ve ardından Fennara'yı CLI ile kurun.

<a id="install-with-the-cli-recommended-on-macos"></a>
### CLI ile Kurun (macOS'ta Önerilir)

CLI aynı Fennara eklentisini kurar. Yukarıda açıklanan bildirime yol açan tarayıcı ve Finder karantina yolunu önlediği için macOS'ta önerilen kurulum yöntemidir.

CLI'yi Windows'a kurun:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Veya macOS ve Linux'a kurun:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Ardından Fennara'yı Godot projenizden çalıştırın:

```bash
cd path/to/your-godot-project
fennara install
```

Sorun giderme için [Kurulum](docs/setup.md), eksiksiz komut başvurusu için [Fennara CLI](docs/cli.md) bölümüne bakın.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Bir Sağlayıcı Kurun veya MCP Uygulaması Bağlayın

<a id="built-in-chat"></a>
### Yerleşik Sohbet

**Chat Settings > Chat** bölümünü açın, **Open providers** seçeneğini seçin ve bir sağlayıcı bağlayın. Fennara, bulut sağlayıcılarında kendi anahtarınızı kullanır (BYOK). Yerel bir Ollama veya LM Studio sunucusu da kullanabilirsiniz. [Desteklenen sağlayıcılar listesine](docs/providers.md) bakın.

<a id="mcp-apps"></a>
### MCP Uygulamaları

**Chat Settings > MCP Apps** bölümünü açın, uygulamanızı bulun ve **Set Up** düğmesine basın.

Bir uygulamayı terminalden de bağlayabilirsiniz:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

MCP uygulamanız Chat Settings içinde listelenmiyorsa eksiksiz uygulama listesi ve elle yapılandırma talimatları için [MCP Kurulumu](docs/mcp-setup.md) bölümüne bakın.

<a id="update"></a>
## Güncelleme

Fennara panelinde **Update** görüntülendiğinde düğmeye basın ve yönergeleri izleyin.

> **Fennara v0.3.8 veya daha eski bir sürümden yükseltme:** `fennara update` komutunu çalıştırmadan önce yukarıdaki platform kurulum komutuyla CLI'yi bir kez yeniden kurun. Bu CLI sürümleri kullanımdan kaldırılmış bir sürüm etiketini çözümler ve güncel sürümleri keşfedemez. CLI'yi yeniden kurmak gelecekteki güncellemeleri GitHub'ın Latest Release uç noktasına geçirir ve mevcut proje eklentinizi ya da ayarlarınızı kaldırmaz.

> **Fennara v0.3.11'den yükselten macOS kullanıcıları:** Güncellemeden önce yukarıdaki macOS kurulum komutuyla CLI'yi bir kez yeniden kurun. v0.3.11 CLI, kendini güncelleyemeden önce mevcut macOS framework paketini reddeder. Yeniden kurulum yalnızca CLI'yi değiştirir; proje eklentinizi veya ayarlarınızı kaldırmaz.

Terminalden güncellemek için Godot'yu kapatın ve şunu çalıştırın:

```bash
cd path/to/your-godot-project
fennara update
```

Kurtarma ve tanılamalar için [Fennara'yı Güncelleme](docs/setup.md#update-fennara) bölümüne bakın.

<a id="tools"></a>
## Araçlar

Fennara, Godot bilgisine sahip küçük bir araç kümesi sunar:

- proje dosyalarını yazma veya güncelleme ve tanılamaları döndürme
- tek seferlik sahne düzenleme betikleri çalıştırma
- sahne ağaçlarını, düğümleri, kaynakları ve Godot sınıflarını inceleme
- sahneleri doğrulama
- ekran görüntüleri yakalama
- çalışma zamanı oturumlarını başlatma ve çalışma zamanı günlüklerini okuma
- canlı bir sahne üzerinde küçük çalışma zamanı betikleri çalıştırma

Amaç, ajanın normal dosya araçlarının yerini almak değildir. Fennara eksik olan Godot geri bildirim döngüsünü sağlar.

<a id="privacy"></a>
## Gizlilik

Fennara, Godot bağlandıktan sonra UTC günü başına en fazla bir anonim etkin kurulum olayı gönderir. Bu olay rastgele bir kurulum UUID'si, Fennara ve Godot sürümleri, işletim sistemi ve CPU mimarisi içerir. Proje verileri, yolları, istemleri, araç etkinliğini, günlükleri, ekran görüntülerini veya hesap bilgilerini içermez.

Telemetri **Chat Settings > Chat > Anonymous telemetry** seçeneğinden, `FENNARA_DISABLE_TELEMETRY=true` ile veya `DO_NOT_TRACK=1` ile devre dışı bırakılabilir. Eksiksiz veri yükü, depolama, aktarım ve vazgeçme sözleşmesi için [Anonim Telemetri](docs/telemetry.md) bölümüne bakın.

<a id="demos"></a>
## Demolar

Uygulamalı bir Fennara tanıtımı izleyin:

[![Bu Godot Eklentisi Yapay Zeka ile Oyun Geliştirmede Sonsuza Dek Devrim Yaratıyor](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Daha fazla video:

- [Codex'e Bir Yapay Zeka Oyun Görseli Verdim ve Godot'ta Bunu Oluşturdu](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP, Katamari Tarzı Bir Godot Oyunu Oluşturuyor](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [Bu Godot Eklentisi Yapay Zeka ile Oyun Geliştirmeyi Sonsuza Dek Dönüştürüyor](https://www.youtube.com/watch?v=wKln8248y2M)

Fennara kanalındaki diğer videolar için [Demolar](docs/demos.md) bölümüne bakın.

<a id="star-history"></a>
## Yıldız Geçmişi
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Yıldız Geçmişi Grafiği" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## Belgeler

| Şununla başlayın... | Şuna ihtiyacınız olduğunda... |
| --- | --- |
| [Belge ana sayfası](docs/README.md) | Tüm kılavuz ve başvuru sayfaları |
| [Kurulum](docs/setup.md) | Kurulum, güncellemeler ve sorun giderme |
| [Sohbet sağlayıcıları](docs/providers.md) | Yerleşik sohbet modelleri ve anahtarları |
| [MCP kurulumu](docs/mcp-setup.md) | Codex, Claude, Cursor ve diğer MCP uygulamaları |
| [Araçlar](docs/tools.md) | Ajanların kullanabildiği Godot geri bildirimi |
| [Anonim telemetri](docs/telemetry.md) | Toplanan veriler, teslim davranışı ve vazgeçme denetimleri |
| [Katkıda bulunma](CONTRIBUTING.md) | Geliştirme ve pull request yönergeleri |

<a id="community"></a>
## Topluluk

Sorular, kurulum yardımı ve erken geri bildirim için Discord'a bekleriz:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Lisans

Bkz. [LICENSE.md](LICENSE.md).
