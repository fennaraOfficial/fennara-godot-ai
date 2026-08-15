<!-- fennara-i18n: locale=tr source=docs/faq.md sha256=3a644ba5c7ce06e847a4d6eb49ab232f9fa54eaecf1a97952ce69918dba4d677 -->
<a id="faq"></a>
# SSS

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../faq.md)
<!-- fennara-doc-nav:end -->

Kurulum ve güncellemeler için [Kurulum](setup.md) ile başlayın. Kısa yanıtlar ve ayrıntılı başvuru bağlantıları için bu sayfayı kullanın.

| Soru | Kısa yanıt |
| --- | --- |
| Sağlayıcı anahtarına ihtiyacım var mı? | Yalnızca yerleşik sohbetteki bir bulut sağlayıcısı için |
| Bunun yerine harici bir MCP uygulaması kullanabilir miyim? | Evet, kendi model hesabını kullanır |
| Fennara projemi bir Fennara sunucusuna yükler mi? | Hayır |
| Birden fazla Godot editörü açık olabilir mi? | Evet; eşzamanlı her MCP bağlantısını kendi projesine bağlayın |

<a id="is-fennara-only-a-code-generator"></a>
## Fennara Yalnızca Bir Kod Oluşturucu mu?

Hayır. Fennara, Godot bilgisine sahip bir ajan iş akışıdır. Proje dosyaları, sahneler, tanılamalar, çalışma zamanı hataları, ekran görüntüleri ve Godot editörü bağlamıyla çalışabilir.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Fennara Yalnızca Başka Bir Godot MCP Komut Sunucusu mu?

Hayır. MCP, Fennara'yı Codex, Claude, Cursor, Gemini ve Antigravity gibi uygulamalardan kullanmanın yollarından biridir. Fennara'nın isteğe bağlı yerleşik bir sohbet paneli de vardır. Ürünün ana tezi Godot geri bildirim döngüsüdür: ajanların hataları düzeltebilmesi için tanılamalar, doğrulama, çalışma zamanı hataları, ekran görüntüleri ve yapılandırılmış araç sonuçları.

<a id="does-fennara-replace-godot-knowledge"></a>
## Fennara Godot Bilgisinin Yerini Alır mı?

Hayır. Fennara, Godot'yu isteğe bağlı hale getirmeye çalışmaz. Yapay zeka ajanlarını gerçek Godot motoruna karşı sorumlu tutmak üzere tasarlanmıştır.

<a id="how-should-i-install-fennara"></a>
## Fennara'yı Nasıl Kurmalıyım?

Windows ve Linux'ta eklentiyi ekleyin, Fennara panelini açın ve **Set Up Fennara** düğmesine basın ya da terminalden kurun. Tarayıcıdan indirilen eklenti ZIP dosyası elle açıldığında oluşabilen güvenlik bildiriminden kaçınmak için macOS'ta CLI üzerinden kurun. İki yol için de [Kurulum](setup.md) bölümüne bakın.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## macOS Neden `libfennara.macos.editor` Dosyasını Doğrulayamadığını Söylüyor?

Sürüm eklentisi şu anda Apple tarafından noter onayı verilmemiş yerel bir kitaplık içerir. Eklenti ZIP dosyası tarayıcı üzerinden indirilip elle açıldığında Finder, karantina meta verilerini bu kitaplığa aktarabilir ve macOS bildirimine yol açabilir.

Bunu önlemek için [CLI kurulumunu](setup.md#install-from-the-terminal-recommended-on-macos) kullanın. Bildirim zaten görünüyorsa Godot'yu kapatın, elle kopyalanmış `addons/fennara/` klasörünü kaldırın, CLI'yi kurun ve proje dizininden `fennara install` çalıştırın. CLI aynı eklentiyi tarayıcı ve Finder karantina yolu olmadan kurar.

<a id="do-i-need-a-chat-provider-api-key"></a>
## Sohbet Sağlayıcısı API Anahtarına İhtiyacım Var mı?

Yalnızca yerleşik Fennara sohbet panelinde bir bulut sağlayıcısı kullanmak istiyorsanız gerekir. Harici MCP istemcileri kendi model/uygulama yapılandırmalarını kullanır ve Fennara sohbetine sağlayıcı anahtarı girmeden Fennara MCP araçlarını kullanabilir.

Yerleşik sohbet, bulut API anahtarı olmadan yerel Ollama veya LM Studio da kullanabilir. Bkz. [Yerleşik Sohbet Sağlayıcıları](providers.md).

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## `mcp-setup --claude` Çalıştırdığım Halde Panel Neden Sağlayıcı İstiyor?

`fennara mcp-setup --claude`, Claude'u Fennara'nın Godot MCP araçlarına bağlar. Yerleşik Fennara panelini Claude'a bağlamaz ve Claude aboneliğinizi Fennara sohbetiyle paylaşmaz.

Harici MCP akışı için Claude Code veya Claude Desktop kullanın. Yalnızca Godot içindeki Fennara panelinde sohbet etmek istiyorsanız ayrı bir sağlayıcı yapılandırın. Bkz. [MCP Uygulamaları ve Yerleşik Sohbet](chat-vs-mcp.md).

<a id="what-are-provider-and-model"></a>
## `/provider` ve `/model` Nedir?

Bunlar yerleşik Fennara sohbet panelindeki eğik çizgi komutlarıdır. `/provider` sağlayıcı seçiciyi açar. `/model` model seçiciyi açar. Kullanıcı arayüzü kısayollarıdır, harici MCP araçları veya modele gönderilen metin değildir. Bkz. [Yerleşik Sohbet Eğik Çizgi Komutları](slash-commands.md).

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Fennara Godot Projemi Bir Fennara Sunucusuna Gönderir mi?

Hayır. Normal OSS yolunda MCP istemcisi, daemon ve Godot eklentisi yerel olarak çalışır. Yerleşik sohbet, model isteklerini yalnızca yapılandırdığınız OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax veya yerel Ollama/LM Studio sunucusu gibi sağlayıcıya gönderir.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## Birden Fazla Godot Editörü Açıksa MCP Araç Çağrılarını Hangi Proje Alır?

Projeye bağlı bir MCP işlemi, her çağrıyı kendi kanonik Proje Kökünün bağlı
editörüne yönlendirir. Farklı depolarda veya worktree'lerde eşzamanlı çalışan
ajanlar için proje başına bir MCP işlemi ve bağlantısı kullanın.

Bağlı olmayan bir MCP işlemi uyumluluk davranışını korur: daemon panelden
seçilen MCP Hedefini veya bağlı tek editörü kullanır. Eşzamanlı çalışmadan önce
`fennara_status` komutunu çalıştırın ve beklenen kökle birlikte `bound`
yönlendirme modunu şart koşun. Yerleşik sohbet oturumları, sohbeti açan editöre
bağlı kalır. Bkz. [Birden Fazla Ajan ve Worktree](multi-agent-worktrees.md).

<a id="can-several-agents-run-their-games-at-the-same-time"></a>
## Birden Fazla Ajan Oyunlarını Aynı Anda Çalıştırabilir mi?

Daemon tarafından yönetilen Çalışma Zamanı Oturumları üzerinden çalıştıramaz.
Düzenleme, inceleme, sınırlı sahne doğrulaması ve bağımsız ekran görüntüleri
eşzamanlı yapılabilir; ancak tüm projeler etkileşimli, yönetilen oyun
çalıştırmaları için tek bir Çalışma Zamanı Yuvasını paylaşır. Kaybeden başlatma
isteği normal ve anonim bir `busy` sonucu döndürür; böylece ajan geçerli
çalışmayı bozmadan jitter ile sorgulama yapıp yeniden deneyebilir. Sahip proje,
çalışma sürerken durumu sorgulayarak hareketsizlik zaman aşımı için son tarihi
yeniler; kiranın mutlak bitiş tarihi hiçbir zaman değişmez.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Linux Neden Ayrı Bir CEF Çalışma Zamanı Kurar?

Linux'ta gömülü sohbet CEF ekran dışı işleme kullanır. CEF veri yükü büyüktür, bu nedenle Fennara her Godot proje eklentisine kopyalamak yerine kullanıcının Fennara uygulama verileri dizini altına bir kez kurar.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## Eklentinin `libcef.so` İçermesi mi Gerekiyor?

Hayır. `libcef.so`, CEF kaynakları, dil paketleri ve CEF yardımcısı paylaşımlı Linux CEF çalışma zamanında bulunur. Eklenti yalnızca Godot eklenti dosyalarını, GDExtension ikililerini, sohbet kullanıcı arayüzü dosyalarını ve ripgrep gibi küçük, paketlenmiş yardımcı ikilileri içermelidir.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## Yerleşik Sohbet Web Görünümü Başlatılamazsa Ne Olur?

Fennara MCP araçları çalışmaya devam eder. Yalnızca isteğe bağlı editör içi sohbet paneli platform web görünümüne ihtiyaç duyar. `fennara doctor` eksik olduğunu bildirirse Windows'ta Microsoft Edge WebView2 Runtime'ı kurun. macOS'ta WKWebView sistemin WebKit.framework bileşeninden gelir. Linux'ta sürüm tarafından yönetilen CEF çalışma zamanının kurulabilmesi veya onarılabilmesi için `fennara update` çalıştırın.

Chat Settings içinde **Open chat in my system browser next time** seçeneğini de kullanabilirsiniz. Bu seçenek aynı yerleşik Fennara sohbetini ve sağlayıcı ayarlarını korur, ancak kullanıcı arayüzünü gömülü Godot web görünümü yerine sistem tarayıcınızda yerel daemon üzerinden açar. Ayarı değiştirdikten sonra Godot'yu yeniden başlatın.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## Sohbeti Tarayıcımda Açmak Claude veya MCP Uygulamamı mı Kullanır?

Hayır. Tarayıcıda görüntüleme, yerleşik Fennara sohbeti için yalnızca kullanıcı arayüzü/çalışma zamanı seçimidir. Yine Fennara sohbet ayarlarında seçilen sağlayıcıyı kullanır. `fennara mcp-setup --claude` ve benzeri komutlar harici MCP uygulamalarını yapılandırır; yerleşik sohbet modelini yapılandırmaz.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update` MCP Uygulaması Yapılandırmasını Yeniden Yazar mı?

Hayır. `fennara update`, gerektiğinde kurulu CLI'yi, proje eklentisini, yerel çalışma zamanı paketini, oluşturulan proje yönergelerini ve platform tarafından yönetilen çalışma zamanı varlıklarını yeniler. `fennara mcp-setup` komutunu yalnızca bir MCP uygulaması yapılandırmasını kurarken veya onarırken yeniden çalıştırın.

<a id="where-does-chat-history-live"></a>
## Sohbet Geçmişi Nerede Tutulur?

Sohbet geçmişi daemon tarafından yerel olarak depolanır ve geçerli Godot projesiyle sınırlandırılır. Sağlayıcı anahtarları ve yerel sağlayıcı URL'leri de Godot projesinin dışında, daemon tarafından yerel olarak depolanır.

<a id="what-should-agents-use-fennara-tools-for"></a>
## Ajanlar Fennara Araçlarını Ne İçin Kullanmalıdır?

Fennara'yı Godot bilgisine sahip geri bildirim için kullanın: sahne ağaçları, değiştirilmiş düğüm/kaynak özellikleri, tanılamalar, doğrulama, çalışma zamanı oturumları, ekran görüntüleri ve editör hata ayıklayıcısı durumu. MCP istemcileri Fennara'ya özel bir araç gerekmedikçe kendi olağan dosya okuma/arama araçlarını kullanmaya devam etmelidir.
