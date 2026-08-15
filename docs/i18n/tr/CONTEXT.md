<!-- fennara-i18n: locale=tr source=CONTEXT.md sha256=7d76acbada75ade69b43dc52fcd543f90d678c04b3e9b50fc11601b8b1853fd4 -->
<a id="fennara-context"></a>
# Fennara Bağlamı

<!-- fennara-doc-nav:start -->
[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

Bu dosya Fennara belgelerinde, issue'larda, sürüm notlarında ve ajanlara yönelik yönergelerde kullanılan ortak terimleri tanımlar.

<a id="product-terms"></a>
## Ürün Terimleri

**Fennara**

Bu depodaki Godot bilgisine sahip ajan ortamı. Fennara, yapay zeka araçlarını tanılamalar, sahne doğrulaması, çalışma zamanı hataları, ekran görüntüleri ve proje yönergeleri gibi gerçek Godot geri bildirimlerine bağlar.

**Godot Eklentisi**

Kullanıcının Godot projesine `res://addons/fennara/` altına kopyalanan kurulabilir eklenti. Panel kullanıcı arayüzünün, Godot tarafındaki inceleme araçlarının, yerel GDExtension kitaplığının, paketlenmiş sohbet kullanıcı arayüzü varlıklarının, çalışma zamanı yardımcı betiklerinin ve projeye özel eklenti sürümünün sahibidir.

**Fennara CLI**

Kullanıcının makinesine kurulan `fennara` komutu. Kurulumu, güncellemeyi, CLI'nin kendini güncellemesini, doctor denetimlerini, MCP uygulaması kurulumunu, web görünümü önkoşul uyarılarını, C# kurulum denetimlerini ve oluşturulan proje yönergelerini yönetir.

**Yerel Paket**

Tek bir platform/mimari için MCP sunucusu, daemon, çalışma zamanı ikilileri ve başlatıcı ikilileri gibi yerel Fennara yürütülebilir dosyalarını içeren sürüm zip dosyası.

**Proje Yönergeleri**

Yapay zeka kodlama ajanlarının Fennara'yı ne zaman ve nasıl kullanacağını bilmesi için `AGENTS.md` ile `addons/fennara/ai/` altındaki yönlendirilmiş başvurular dahil bir Godot projesine yerleştirilen, oluşturulmuş yönerge dosyaları.

<a id="mcp-terms"></a>
## MCP Terimleri

**Fennara MCP Sunucusu**

Claude Code, Cursor, Cline, Gemini CLI veya başka bir MCP istemcisi gibi bir yapay zeka kodlama uygulamasının başlattığı yerel stdio MCP sunucusu. Fennara araçlarını bu harici uygulamaya sunar.

**MCP Uygulaması**

`fennara mcp-setup` tarafından yapılandırılan harici bir yapay zeka uygulaması. MCP uygulaması kurulumu hangi harici uygulamanın Fennara araçlarını çağırabileceğini denetler; Fennara'nın yerleşik sohbetinin kullandığı modeli seçmez.

**MCP Hedefi**

MCP Proje Bağlaması olmayan harici bir MCP bağlantısının kullandığı, panelden
seçilen ve daemon genelinde geçerli uyumluluk hedefi. Bağlı MCP bağlantıları bu
hedefi ne okur ne de değiştirir.

**MCP Proje Bağlaması**

Bir Fennara MCP işlemi başlatıldığında bir kez seçilen kararlı Proje Kökü. Bu
işlemin çağrılarını daemon genelindeki MCP Hedefini kullanmadan eşleşen Godot
Editör Oturumuna yönlendirir.

**Proje Kökü**

Bir Godot projesinin `project.godot` dosyasını içeren kanonik dosya sistemi
dizini. Fennara, depoları ve worktree'leri ayırt etmek için proje adı yerine
dosya sistemi kimliğini kullanır.

**Godot Editör Oturumu**

O anda bağlı olan tek bir Fennara eklentisi ve Godot editörü örneği. Bir proje
yoluna ve Godot işlem kimliğine sahiptir; bir MCP işleminin Proje Bağlamasını
değiştirmeden bağlantısı kesilip yeniden bağlanabilir.

**Araç Şeması**

Bağımsız değişkenler, sınırlar ve iş akışı notları dahil bir Fennara MCP aracının modele yönelik açıklaması.

**Araç Sonucu Zarfı**

Bir araç çağrısından sonra döndürülen, modele yönelik özlü sonuç. Fennara sonuçları gereksiz ham veri dökmeden durumu, önemli bulguları ve sonraki yararlı bağlamı açıklamalıdır.

<a id="built-in-chat-terms"></a>
## Yerleşik Sohbet Terimleri

**Yerleşik Sohbet**

Fennara'nın Godot eklentisi veya sistem tarayıcısı içindeki kendi sohbet yüzeyi. Harici MCP uygulamalarından ayrıdır. Bir kullanıcı MCP için Claude Code'u yapılandırıp yerleşik sohbet için yine de başka bir sağlayıcı/model seçebilir.

**Sohbet Yüzeyi**

Yerleşik sohbetin görüntüleme modu. Gömülü mod yerel Godot paneli web görünümünü kullanır. Tarayıcı modu aynı kullanıcı arayüzünü yerel daemon üzerinden sunar ve sistem tarayıcısında açar.

**Sohbet Sağlayıcısı**

OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, yerel Ollama veya LM Studio gibi yerleşik sohbet yanıtları üretebilen bir arka uç.

**Model Başvurusu**

Yerleşik sohbette seçilen, sağlayıcı nitelemeli model tanımlayıcısı. `/provider` ve `/model` gibi eğik çizgi komutları kullanıcıların sağlayıcıları bağlamasına ve model başvurularını seçmesine yardımcı olur.

**Sağlayıcı Bağlantısı**

API anahtarları veya yerel temel URL'ler dahil bir sohbet sağlayıcısına ait, daemon tarafından yönetilen yerel ayarlar ve kimlik doğrulama durumu. Sağlayıcı sırları Godot projesinin içinde değil, daemon tarafından yönetilen yerel depolamada kalmalıdır.

**Üretim İzi**

Yerleşik sohbet üretimine ait depolanmış meta veriler. Asistan mesajlarını, araç çağrılarını, sağlayıcı/model seçimini, kullanımı ve maliyet günlüklerini bunları üreten üretime bağlar.

<a id="runtime-and-webview-terms"></a>
## Çalışma Zamanı ve Web Görünümü Terimleri

**Fennara Daemon**

MCP çağrılarını ve yerleşik sohbet isteklerini Godot eklentisine bağlayan, yerel çalışma zamanı durumunu depolayan ve `/chat/` gibi daemon tarafından barındırılan sohbet rotalarını sunan yerel hizmet.

**Çalışma Zamanı Oturumu**

Çalışan sahne incelemesi, günlükler ve çalışma zamanı yakalamaları için
kullanılan, daemon tarafından yönetilen etkileşimli bir Godot oyun işlemi.
Sahibi olan kanonik Proje Kökü, o projenin editörü yeniden bağlansa bile
denetimi elinde tutar. Sınırlı sahne doğrulaması ve bağımsız ekran görüntüsü
çağrıları ayrı yollar kullanır ve Çalışma Zamanı Yuvasını işgal etmez.

**Çalışma Zamanı Yuvası**

Bağlı tüm projeler genelinde daemon tarafından yönetilen en fazla bir Çalışma
Zamanı Oturumunun başlamasına veya çalışmasına izin veren, makine genelindeki
kabul durumu.

**Çalışma Zamanı Kirası**

Sahip Proje Kökünün Çalışma Zamanı Yuvasını kullanmak için sahip olduğu,
yenilenebilir ve süreyle sınırlı hak. Sahip etkinliği hareketsizlik son tarihini
yenilerken mutlak son tarih her zaman uygulanmaya devam eder.

**Godot Anlık Görüntüsü**

Dosyaları değiştirebilecek, Fennara destekli bir turdan önce alınan geri döndürülebilir proje durumu anlık görüntüsü. Başarısız kurulumun sahipsiz istemler bırakmaması için anlık görüntü kurulumu kullanıcı turu kalıcılaştırılmadan önce tamamlanmalıdır.

**Web Görünümü Çalışma Zamanı**

Yerleşik sohbeti Godot'nun içinde veya yakınında göstermek için gereken platform desteği. Windows WebView2, macOS WebKit/WKWebView, Linux ise Fennara uygulama verilerinin altına kurulan paylaşımlı bir CEF çalışma zamanı kullanır.

**Paylaşımlı Linux CEF Çalışma Zamanı**

Linux sohbet web görünümünün kullandığı harici Linux CEF çalışma zamanı veri yükü. Kullanıcının Fennara uygulama verileri dizini altına bir kez kurulur ve her Godot eklentisi zip dosyasına paketlenmemelidir.

<a id="release-terms"></a>
## Sürüm Terimleri

**Sürüm Manifestosu**

`fennara-release-manifest-v<version>.json` adlı JSON varlığı. Sürüm varlıklarını platformlarla eşler, SHA-256 karmalarını kaydeder, paylaşımlı çalışma zamanı varlıklarını bildirir ve `minimum_cli_version` değerini belirler.

**Asgari CLI Sürümü**

Bir sürüm manifestosunu kullanmasına izin verilen en eski `fennara` CLI sürümü. Bir sürüm daha yeni kurulum/güncelleme mantığı gerektiriyorsa `scripts/release-policy.mjs` içindeki izini güncelleyin. Manifesto yazıcı, sürüm kimliğini doğruladıktan sonra bu politikayı uygular; iş akışları değeri seçmez.

**En Son Sürüm**

GitHub'ın tam sürümlenmiş bir sürüme işaret eden Latest Release göstergesi. Kurucular ve varsayılan güncellemeler bu göstergeyi GitHub API'si üzerinden çözümler. Fennara, gerçek anlamda `latest` adlı bir etiket veya sürüm kullanmaz. Kaynak dosyaları yayımlamadan sonra güncellemek sürüm varlıklarını değiştirmez; daha önce yayımlanmış manifesto varlıkları açıkça değiştirilmelidir.
