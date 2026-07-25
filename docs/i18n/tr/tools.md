<!-- fennara-i18n: locale=tr source=docs/tools.md sha256=4cf72381fada4fec347f29da5995d9768b39235f71b437dd698088ac0acb3518 -->
<a id="tools"></a>
# Araçlar

<!-- fennara-doc-nav:start -->
[English](../../tools.md) · [简体中文](../zh-CN/tools.md) · [Español](../es/tools.md) · [Português do Brasil](../pt-BR/tools.md) · [日本語](../ja/tools.md) · [한국어](../ko/tools.md) · [Русский](../ru/tools.md) · [Français](../fr/tools.md) · [Deutsch](../de/tools.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../tools.md)
<!-- fennara-doc-nav:end -->

Fennara, kodlama aracılarına Godot farkındalığı olan inceleme, düzenleme,
doğrulama, ekran görüntüleri ve çalışma zamanı geri bildirimi verir. Normal depo
ve kabuk araçlarının yerini almak yerine onları tamamlar.

Bu sayfa her aracın neler yapabildiğini, başarılı bir çağrının ne anlama
geldiğini ve önemli sınırlamaları veya hata durumlarını açıklar. Canlı araç
şemaları tam bağımsız değişkenler, sonuç alanları, sınırlar ve aracı talimatları
için doğruluk kaynağı olarak kalır. Kurulu projeler ayrıca
`addons/fennara/ai/` konumunda kısa yönergeler ve isteğe bağlı bilgi alır.

<a id="tool-surfaces"></a>
## Araç Yüzeyleri

Codex, Claude Code, Cursor ve Gemini dahil harici MCP istemcileri yerel
`fennara-mcp` süreci üzerinden bağlanır. Fennara'nın yanında kendi model
hesaplarını ve normal dosya, arama, fark ve kabuk araçlarını kullanırlar.

Yerleşik Fennara sohbeti aynı daemon ve Godot köprüsünü kullanır. Aynı Godot
araçlarını çağırabilir ve ayrıca proje kapsamlı `read_file` ve `exec_command`
araçlarını sağlar. Sağlayıcı ve model kurulumu MCP sunucusuna değil, yerleşik
sohbete aittir.

`fennara_status` harici MCP istemcileri tarafından kullanılabilir. Yerleşik
sohbet bağlantı ve etkin proje durumunu zaten daemon'dan alır.

<a id="typical-workflow"></a>
## Tipik İş Akışı

1. Harici bir MCP istemcisi kullanırken bağlı projeyi doğrulayın.
2. İlgili sahneyi, kaynağı, sınıfı, içe aktarma durumunu veya proje ayarını inceleyin.
3. Yararlı olan en küçük düzenlemeyi yapın.
4. Tanılama veya sahne doğrulaması çalıştırın.
5. Görsel ya da davranışsal kanıt önemli olduğunda ekran görüntülerini veya çalışma zamanı araçlarını kullanın.

Düzenleyici dosya sistemi tarama veya içe aktarma sırasında geçici olarak meşgul
olabilir. Yapı araçları hazır olduğunu bildirdikten sonra kullanılmalıdır.

<a id="connection"></a>
## Bağlantı

<a id="fennarastatus"></a>
### `fennara_status`

MCP sunucusunu, daemon'ı, etkin Godot projesini, bağlı düzenleyici oturumlarını,
bileşen sürümlerini, işleme bağlamını, duyurulan araçları ve düzenleyici dosya
sistemi hazırlığını bildirir.

Çalışma davranışı:

- Tek bir düz metin durum bloğu döndürür.
- Hazır bir düzenleyici dosya sistemini tarama veya içe aktarma yapan bir sistemden ayırır.
- Yapılara dönük araçların o anda hazır olup olmadığını bildirir.
- Eşleşmeyen kurulumların tanılanabilmesi için sürüm farklılıklarını gösterir.

Önemli sınırlar ve hatalar:

- Belirli tek bir yapı yolunun hazırlığını değil, proje düzeyi hazırlığı bildirir.
- Bağlantısı kesilmiş bir daemon, eksik etkin proje veya bağlantısı kesilmiş Godot eklentisi hazır proje olarak ele alınmak yerine doğrudan bildirilir.
- Godot dosyaları yeniden içe aktarırken hazırlık kısa süreliğine değişebilir.

<a id="inspection"></a>
## İnceleme

<a id="getscenetree"></a>
### `get_scene_tree`

Bir sahneyi Godot üzerinden yükler ve düğüm hiyerarşisini, düğüm sınıflarını,
bağlı betikleri ve örneklenmiş alt sahneleri döndürür. Döndürülen yollar diğer
sahne araçları tarafından kullanılabilir.

Çalışma davranışı:

- Yazılmış sahneleri yeniden yazmadan okur.
- Düzenlemeden önce düğüm ve örnek yapısını görünür kılar.
- Her kaynağı genişletmek yerine sonucu hiyerarşiye odaklı tutar.

Önemli sınırlar ve hatalar:

- Eksiksiz bir 3D yapı, mesh, malzeme, iskelet veya animasyon raporu değildir.
- Godot'un yükleyemediği bir sahne, tahmini bir ağaç yerine hata döndürür.
- Büyük kaynak ayrıntıları hedefli özellik veya betik incelemesine aittir.

<a id="getnodeproperties"></a>
### `get_node_properties`

Seçili düğümlerde sınıf varsayılanlarından farklı olan özellikleri gösterir ve
gömülü kaynakların yararlı özetlerini genişletir.

Çalışma davranışı:

- Tek çağrıda en fazla beş düğüm hedefini destekler.
- Dışa aktarılan GDScript özelliklerini ve kullanılabilir C# betik meta verilerini okur.
- Animasyonlar, temalar, döşeme verileri, mesh kitaplıkları, sprite kareleri ve animasyon grafikleri gibi kaynakları opak değerler dökmek yerine özetler.

Önemli sınırlar ve hatalar:

- Tam sahne kaynak envanteri değil, düğüm hedefli bir araçtır.
- İçe aktarılmış kaynak yapıları, yazılmış `.tscn` düğümlerinden daha az bilgi sunabilir. Oluşturulan içe aktarılmış kaynağın doğrudan incelenmesi gerektiğinde `run_asset_import_script` kullanın.
- Geçersiz düğüm yolları sessizce yok sayılmak yerine bildirilir.

<a id="getclassinfo"></a>
### `get_class_info`

Kalıtım, özellikler, yöntemler, sinyaller, enum'lar, sabitler ve kullanılabilir
belgeler dahil bir Godot sınıfının gerçek API yüzeyini döndürür.

Çalışma davranışı:

- Çalışma zamanı ClassDB bilgileri bağlı Godot düzenleyicisinden gelir.
- Yerleşik sınıflar, açık bir `master` yedeğiyle bağlı ana ve küçük sürümle eşleşen resmi Godot XML belgelerini kullanır.
- GDExtension ve yerel eklenti sınıfları, resmi Godot belgeleri varmış gibi davranmadan kullanılabilir çalışma zamanı sınıfı ve özellik bilgilerini döndürür.

Önemli sınırlar ve hatalar:

- Eşleşen üst kaynak sınıf XML'i kullanılamıyorsa veya yanıt eksiksiz alınamıyorsa belge araması eksik olabilir.
- Yalnızca çalışma zamanındaki davranış yine de küçük bir düzenleyici tarafı betik araştırması gerektirebilir.
- Mevcut olmayan bir sınıf adı eksik olarak bildirilir.

<a id="editing"></a>
## Düzenleme

<a id="writeorupdatefile"></a>
### `write_or_update_file`

Bir proje metin dosyası oluşturur, yeniden yazar veya içinde tam değiştirme yapar.

Çalışma davranışı:

- `write`, eksiksiz içerikten dosya oluşturur veya mevcut dosyayı değiştirir.
- `update`, benzersiz tek bir tam metin bloğunu değiştirir.
- GDScript ve shader düzenlemeleri otomatik olarak Godot tanılamalarını döndürür.
- Shader düzenlemeleri ayrıca gömülü malzeme verilerinin eski kalmaması için başvuran sahneleri ve kaynakları Godot üzerinden yeniden serileştirmeye çalışır.
- Tek bir proje tanılama derlemesi istenmeden önce C# yazımlarının çok dosyalı bir düzenleme oluşturmasına izin verilir.

Önemli sınırlar ve hatalar:

- Belirsiz veya eksik güncelleme metni, rastgele bir eşleşmeyi değiştirmek yerine başarısız olur.
- Korunan Fennara, Git, Godot önbelleği, eklenti manifesti ve proje ayarı yolları bu araçla düzenlenemez.
- Ham `.tscn`, `.tres` veya `.res` üzerinde işlem yapmak için tasarlanmamıştır.
- C# doğrulaması her tekil yazımdan sonra çalıştırılmaz. İlgili C# düzenlemeleri tamamlandıktan sonra proje tanılama taraması kullanın.
- Güvenli biçimde yeniden serileştirilemeyen başvuran shader sahipleri atlandı veya uyarıldı olarak bildirilir.

<a id="runsceneeditscript"></a>
### `run_scene_edit_script`

Yazılmış tek bir sahne veya Godot kaynak grafiği üzerinde düzenleyici zamanlı
tek bir GDScript çalışanı yürütür. Bu, sahneleri Godot'un nesne modeli ve
serileştiricisi üzerinden incelemenin veya düzenlemenin yapılandırılmış yoludur.

Çalışma davranışı:

- İnceleme modu ayrık, salt okunur bir sahne grafiği yükler ve hiçbir zaman kaydetmez.
- Düzenleme modu düğümleri ekleyebilir, kaldırabilir, yeniden adlandırabilir veya yeniden üst öğeye bağlayabilir; kaynaklar atayabilir; özellikleri değiştirebilir; sahneler oluşturabilir ve Godot serileştirmesiyle kaydedebilir.
- Mevcut sahneler yalnızca çalışan bağlamı değiştirilmiş olarak işaretlediğinde kaydedilir.
- Yeni düğümler ve PackedScene örnekleri, Godot'un amaçlanan yapıyı serileştirmesi için açık sahiplik yardımcıları kullanır.
- Betik tanılamaları yürütmeden önce çalışır ve kaydedilen sahneler takip doğrulaması alır.
- Godot istenen geçersiz kılmaları güvenli biçimde serileştirebildiğinde kalıtılmış sahne kökleri korunur.
- Her çağrı etkili geçici çalışan yolunu döndürür, böylece başarısız bir çalışan sıfırdan yeniden oluşturulmadan düzeltilebilir.

Önemli sınırlar ve hatalar:

- Yüklenen grafik Run Scene düğmesine basmakla aynı değildir. SceneTree bağımlı oyun API'leri, zamanlayıcılar, kare işleme ve genel dönüşümler ayrık düğümlerde kullanıldığında farklı davranabilir veya başarısız olabilir.
- İnceleme modu Fennara bağlam değişikliği yardımcılarını engeller, ancak rastgele GDScript yine de doğrudan dosya sistemi, düzenleyici, işletim sistemi ve kaynak kaydetme yan etkilerinden kaçınmalıdır.
- `.glb` ve `.gltf` gibi içe aktarılmış kaynak dosyaları bu araç tarafından kaydedilmez. İçe aktarma ayarları `run_asset_import_script` aracına aittir.
- PackedScene iç yapısının yanlış sahipliği, örnek içeriğini düzleştirebildiği veya çoğaltabildiği için reddedilir.
- Kaydetme kalıtılmış bir kökü düzleştirecekse Fennara özgün dosyayı geri yükler ve hata bildirir.
- Tanılama veya çalışma zamanı hataları düzenlemeyi durdurur. Başarısız bir sonuç hedef sahneyi oluşturmaz veya güncellemez, ancak geçici çalışan betiği yeniden deneme için kalabilir.

<a id="runassetimportscript"></a>
### `run_asset_import_script`

İçe aktarılmış bir kaynak yapı ve Godot içe aktarma yapılandırması üzerinde,
sınırlı düzenleyici zamanlı tek bir GDScript çalışanı yürütür. Zaten eşleşen
bir `.import` yan dosyasına sahip modelleri, dokuları, sesleri, yazı tiplerini
ve diğer biçimleri destekler.

İnceleme modundaki çalışma davranışı:

- İçe aktarıcıyı, oluşturulan kaynak sınıfını, içe aktarma geçerliliğini, türlenmiş geçerli seçenekleri, oluşturulan dosyaları ve üst kaynak bağımlılıklarını bildirir.
- Oluşturulan kaynağı eski iç içe önbellek girdilerini yeniden kullanmadan yükler.
- İçe aktarılmış bir PackedScene'i sınırlı inceleme için canlı düzenleyici SceneTree içinde geçici olarak örnekleyebilir, ardından kaydetmeden kaldırır.
- Oluşturulan alt kaynaklar için sınırlı özetler sağlar.
- İnceleme modunda içe aktarma seçeneği değişikliklerini hiçbir zaman kalıcılaştırmaz.

Düzenleme modundaki çalışma davranışı:

- Desteklenen mevcut içe aktarma seçeneklerini yerel Godot Variant türlerini koruyarak hazırlar.
- Canlı düzenleyicinin `EditorFileSystem` üzerinden yeniden içe aktarmayı gerçekleştirmesine izin verir.
- Yalnızca kanonik içe aktarma ayarları, oluşturulan çıktılar, düzenleyici dosya sistemi durumu ve yeni bir derin kaynak yüklemesi doğrulandıktan sonra başarı bildirir.
- Doğrulama başarısız olduğunda önceki yapılandırmayı geri yükleyip yeniden içe aktarmaya çalışır ve bu kurtarmanın başarılı olup olmadığını bildirir.

Önemli sınırlar ve hatalar:

- Kaynak dosya zaten içe aktarılmış ve geçerli bir `.import` yan dosyasına sahip olmalıdır.
- Birinci sürüm düzenlemeleri yalnızca desteklenen yerleşik doku ve sahne içe aktarıcıları için güvenli oluşturulmuş önbellek değişiklikleri olarak sınıflandırılan seçenekleri düzenler.
- İçe aktarıcı kimliği, içe aktarma betikleri, `_subresources`, harici çıkarma yolları ve etkileri bilinmeyen seçenekler yalnızca inceleme olarak kalır.
- Bilinmeyen seçenekler, desteklenmeyen seçenekler ve yanlış Variant türüne sahip değerler dönüştürülmek yerine başarısız olur.
- Doğrudan `.import` dosyası değişikliği algılanır, geri yüklenir ve hata olarak bildirilir. Yan dosya kalıcılığının sahibi Fennara'dır.
- Kök betikle yapılandırılan içe aktarılmış sahneler inceleme yardımcısı tarafından geçici olarak örneklenmez.
- Bağımlılıklar seçilen yapıyı içe aktarmak için gerekli dosyaları tanımlar. Bir modeli kullanan sahneler, bir dokuyu kullanan malzemeler, ses çalan betikler veya yazı tipi kullanan temalar gibi aşağı akış proje tüketicilerini tanımlamaz.
- Betik tanılamaları, çalışma zamanı hataları, yeniden içe aktarma hataları, eksik oluşturulan dosyalar, geçersiz dosya sistemi durumu veya yeniden yükleme hataları başarılı sonucu engeller.
- Araç çıktısını korumak için büyük diziler ve kaynak iç yapıları sınırlandırılır veya özetlenir. Sınırlı bir sonuç, her köşenin, anahtarın veya bağımlılığın satır içinde yazdırıldığı vaadi değildir.

<a id="projectsettings"></a>
### `project_settings`

Yapılandırılmış Godot proje ayarlarını, autoload'ları, uygulama meta verilerini,
işleme ve görüntü ayarlarını ve girdi eylemlerini okur ve değiştirir.

Çalışma davranışı:

- Ham `project.godot` metin değiştirmesi yerine Godot farkındalığı olan yapılandırılmış işlemleri kullanır.
- Girdi eylemlerini ölü bölgeler, olay sayıları ve okunabilir olay özetleriyle listeler.
- Denetimler eklerken veya güncellerken yapılandırılmış girdi olaylarını destekler.

Önemli sınırlar ve hatalar:

- Bilinmeyen işlemler veya geçersiz ayar değerleri bildirilir.
- Bu araç sahne veya betik düzenlemesinin yerini almaz.
- Başlangıcı, işlemeyi, girdiyi veya eklenti davranışını etkileyen değişiklikler yine doğrulanmalıdır.

<a id="checks"></a>
## Denetimler

<a id="scriptdiagnostics"></a>
### `script_diagnostics`

Betikler ve shader'lar için Godot farkındalığı olan tanılamalar çalıştırır.

Çalışma davranışı:

- Hedeflenmiş GDScript ve shader çağrıları en fazla beş dosyayı destekler.
- GDScript tanılamaları Godot'un dil sunucusundan gelir.
- Shader tanılamaları Godot'un shader ayrıştırıcısından gelir.
- Hedeflenmiş GDScript denetimleri ayrıca ilgili sahneleri bellekte yükler, böylece sahne ekinden kaynaklanan hatalar betik ve sahneyle ilişkilendirilebilir.
- Proje taramaları GDScript ve shader'ları denetler, ardından bir C# projesi mevcutsa yalıtılmış tek bir artımlı C# derlemesi gerçekleştirir.
- Tanılama C# derlemeleri düzenleyicinin normal çalışma zamanı derlemelerinden ayrı tutulur.

Önemli sınırlar ve hatalar:

- Hedeflenmiş C# dosya tanılamaları desteklenmez. C# bir proje taraması kullanır.
- Proje genelindeki taramalar sahne başına örneklemeyi atlar ve yalnızca bir betik belirli bir sahne üzerinden yüklendiğinde görünen sorunları kaçırabilir.
- Dil sunucusu, ayrıştırıcı veya derleme hataları tanılama hataları olarak döndürülür, temiz sonuçlar olarak ele alınmaz.
- Tanılamalar, denetlenen kodun sınanan bağlamda ayrıştırılabildiğini veya derlenebildiğini kanıtlar. Oyun davranışının doğru olduğunu kanıtlamaz.

<a id="validatescene"></a>
### `validate_scene`

Bir veya daha fazla sahneyi yapısal sorunlar için denetler ve desteklendiği yerde
kısa bir headless başlangıç geçişi çalıştırır.

Çalışma davranışı:

- En fazla on sahne yolunu kabul eder.
- Yapısal denetimler eksik betikleri ve kaynakları, geçersiz düğüm yollarını, yinelenen kardeş adlarını, döngüsel sahne bağımlılıklarını ve ilgili dışa aktarılmış başvuruları kapsar.
- İsteğe bağlı veya çalışma zamanında atanan dışa aktarılmış başvurular koşulsuz hata yerine not olarak bildirilir.
- Temiz yapısal sonuçlara sahip yazılmış sahneler, günlükler ve yapılar tutularak üç saniyelik bir headless başlangıç geçişi alır.
- Büyük sahnelerin sonucu taşırmaması için yinelenen bulgular gruplanır.

Önemli sınırlar ve hatalar:

- İçe aktarılmış kaynak sahneleri, yazılmış proje sahneleri olarak doğrudan başlatılamadıkları için yalnızca yapısal doğrulama alır.
- Fennara doğrulama penceresinden sonra süreci bilinçli olarak durdurur. Bu durdurma kodu tek başına sahne hatası olarak ele alınmaz.
- Kısa bir başlangıç geçişi tüm oyun yollarını, görselleri, performansı, animasyon kalitesini veya kullanıcı etkileşimini doğrulayamaz.
- Yapısal hatalar ilgili sahnenin çalışma zamanı geçişini engeller.

<a id="visual-and-runtime-feedback"></a>
## Görsel ve Çalışma Zamanı Geri Bildirimi

<a id="screenshotscene"></a>
### `screenshot_scene`

Yazılmış sahnelerden ve desteklenen içe aktarılmış 3D yapılardan görsel kanıt yakalar.

Çalışma davranışı:

- Her sahne yalıtılmış bir SubViewport içinde örneklenir. Ekran görüntüsü yakalama yazılmış sahneyi açmaz veya değiştirmez.
- Yapıda ortam veya ışık yoksa otomatik 3D kadrajlama nötr önizleme ışığı ekleyebilir.
- `scene_path` gerekli tek girdidir. Hem `code` hem de `script_path` belirtilmediğinde Fennara ayrık kökü otomatik kadrajla yakalar.
- GDScript, sıradan Godot koduyla tek bir düğüm veya düğüm dizisi seçebilir, konuları serbestçe gruplayabilir, sahne parçalarını gösterebilir ya da gizleyebilir, ayrık sahneyi geçici olarak değiştirebilir ve `ctx.capture(...)` ile yakalamalar isteyebilir. Bu geçici değişiklikler işlenir, ancak yazılmış sahneye hiçbir zaman kaydedilmez.
- `await ctx.capture(...)` sahne durumunu tam o noktada işler ve sıradan bir Godot `Image` döndürür. Çalışan, seçili sonuçları `ctx.output(image, description)` ile yayımlamadan önce yakalanan görüntüleri inceleyebilir, karşılaştırabilir, yeniden boyutlandırabilir, atabilir veya birleştirebilir.
- En fazla sekiz seçili konu için, betikli bir 3D yakalama `view` ve `camera` değerlerini atladığında Fennara 17 deterministik bakış noktasını denetler ve seçili düğüm görünürlüğünü, okunabilir boyutu, kenar açıklığını ve düşük örtüşmeyi destekleyen birini seçer. Yararlı yön zaten biliniyorsa açık bir görünüm veya kamera kullanın; uzak konular tek karede çok küçük olacaksa birden fazla yakalama kullanın.
- Bir ekran görüntüsü çalışanı yalnızca `ctx.root`, `await ctx.capture(...)`, `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)` ve `ctx.error(...)` alır. `ctx.sheet(...)`, durum seçmeden veya yayımlamadan çağıranın sıraladığı Images değerlerini deterministik, isteğe bağlı etiketli sayfalarda birleştirir. Tam yazılmış kadraja ihtiyaç duyduğunda yakalama seçeneklerinde `ctx.root` altındaki geçici bir Camera2D veya Camera3D aktarabilir.
- Kamera yolları, hedef yolları, görünüm dikdörtgenleri ve üst düzey kadrajlama parametreleri kabul edilmez. Tüm seçim ve kadrajlama çalışan betiğinde bulunur.
- Yayımlanan her görüntü kaydedilir ve listelenir. Görüntü özellikli MCP istemcileri ve yerleşik sohbet modelleri, yayımlanan ilk altı çıktıyı çağrı sırasına göre ayrı görüntü bağlamı olarak alır. Sonraki çıktılar kaydedilen yol üzerinden kullanılabilir kalır ve makbuzda açık bir atlanan görüntü sayısı bulunur.
- Seyrek yakalamalar görüntüyü gizlemek yerine kadrajlama ölçümleri ve kısmi durumla döndürülür.

Önemli sınırlar ve hatalar:

- Otomatik kadrajlama büyük bir iç mekân, oda, seviye veya alışılmadık iskeletli yapı için sanatsal açıdan yararlı bakış noktasını her zaman çıkaramaz.
- İçerik doğrulaması kadrajın seyrek veya belirsiz olduğunu bildirse bile döndürülen görüntü geçerli olabilir.
- Yalnızca metin modelleri makbuzu ve kaydedilen yolları alır, ancak ekli görüntü piksellerini doğrudan göremez.
- Yükleme, işleme, yakalama sahipliği veya dosya kaydetme hataları bildirilir.
- Bilinmeyen eski ekran görüntüsü bağımsız değişkenleri geçiş hatasıyla reddedilir.
- Betik ayrıştırma hataları, çalışma zamanı hataları, eksik yakalama çağrıları, ayrık kökün dışındaki düğümler ve geçersiz geçici kameralar yakalama yapılmadan bildirilir.

<a id="runtimesession"></a>
### `runtime_session`

Daemon tarafından yönetilen pencereli bir Godot sahnesini başlatır, denetler veya durdurur.

Çalışma davranışı:

- Bir sahne süreci başlatılmadan önce başlangıç kapıları çalışır.
- Başarılı bir başlatma bir oturum tanımlayıcısı, süreç durumu, günlük yolları, başlangıç bulguları ve kullanılabilir yakalama bilgileri döndürür.
- Durum, tam oturum günlüğünü atmadan yeni çalışma zamanı çıktısını döndürür.
- Durdurma, son süreç ve günlük bilgilerini döndürür.
- C# projeleri başlatmadan önce Godot'un normal Debug çıktısına gerçek bir çalışma zamanı derlemesi alır, böylece süreç güncel derlemeleri kullanır.
- Çalışma zamanı günlüğü Godot çıktısı, çalışma zamanı hataları, yardımcı işaretçileri, yakalamalar ve durdurma olayları için doğruluk kaynağıdır.

Önemli sınırlar ve hatalar:

- Aynı anda genel olarak yalnızca daemon tarafından yönetilen bir çalışma zamanı oturumu etkindir.
- Başarısız başlangıç kapıları sahnenin açılmasını engeller.
- Bir C# çalışma zamanı derlemesi açık düzenleyicinin normal derleme yeniden yüklemesini tetikleyebilir.
- Başlangıç hazırlığı işaretçileri ilk yanıttan sonra gelebilir ve sonraki durum çağrısında görünebilir.
- Yönetilen oturumlar ayrı Godot süreçleridir, düzenleyicinin içinde elle çalışan sahne değildir.

<a id="runtimescript"></a>
### `runtime_script`

Etkin, yönetilen bir çalışma zamanı oturumu içinde sınırlı bir GDScript
araştırması veya girdi sürücüsü çalıştırır.

Çalışma davranışı:

- Canlı düğümleri inceleyebilir, bulguları günlüğe kaydedebilir, durumu bekleyebilir, eşlenmiş veya düşük düzey girdi gönderebilir, ışın izlemeleri gerçekleştirebilir, temel arayüzle etkileşebilir ve kareleri yakalayabilir.
- `ctx.frame()` ile kaydedilmemiş viewport Images değerlerini toplayabilir, `ctx.sheet()` ile ekran görüntüsü çalışanlarının kullanabildiği, çağıranın denetlediği aynı sayfaları oluşturabilir ve türetilmiş Images değerlerini oyun içinde göstermeden doğrudan `ctx.output()` ile yayımlayabilir.
- Yönetilen sahne başka bir araştırma için açık kalırken betik tamamlanabilir.
- Sonuçlar kullanılabildiğinde tanılamaları, çalışma zamanı bulgularını, yakalama yollarını, günlük yollarını ve oturum durumunu içerir.

Önemli sınırlar ve hatalar:

- Geçerli, etkin bir `runtime_session` tanımlayıcısı gerektirir.
- Çalışma zamanı betikleri düzenleyici `@tool` betikleri değildir ve sahne düzenleme çalışanları olarak kullanılamaz.
- Geçersiz tanılamalar, zaman aşımları, çalışma zamanı hataları, kapalı oturumlar veya kullanılamayan düğümler bildirilir.
- Araştırmalar sınırlı kalmalıdır. Kalıcı bir oyun otomasyonu çatısının yerini almazlar.

<a id="scrapeeditor"></a>
### `scrape_editor`

Kullanıcı Godot düzenleyicisi üzerinden bir sahneyi elle çalıştırdıktan sonra
kısa bir hata ayıklayıcı anlık görüntüsü okur.

Çalışma davranışı:

- Yinelenen hata ayıklayıcı sorunlarını gruplar ve gürültülü ayrıntıları sınırlar.
- Yönetilen bir çalışma zamanı oturumuna ait olmayan düzenleyici çalıştırması çıktısını incelemeye yardımcı olur.

Önemli sınırlar ve hatalar:

- Her düzenleyici arayüz öğesini veya günlük satırını okumaktan bilinçli olarak daha dardır.
- `runtime_session` üzerinden başlatılan sahneler için kullanılmamalıdır; yönetilen çalışma zamanı günlüğü daha eksiksizdir.
- Hiçbir şey elle çalıştırılmadığında yararlı hata ayıklayıcı durumu bulunmayabilir.

<a id="built-in-chat-tools-and-controls"></a>
## Yerleşik Sohbet Araçları ve Denetimleri

<a id="readfile"></a>
### `read_file`

Godot yol işlemeyi kullanarak proje kapsamlı metin dosyalarını ve desteklenen
görüntüleri okur. `res://` normalleştirmesi veya görüntü işleme önemli olduğunda
kullanışlıdır. Geniş kaynak gezintisi normal depo araçlarına ait olmaya devam
eder.

<a id="execcommand"></a>
### `exec_command`

Etkin proje kökünü varsayılan çalışma dizini olarak kullanarak etkileşimsiz tek
bir komut çalıştırır.

Çalışma davranışı:

- Standart çıktıyı ve hatayı zaman ve çıktı sınırlarıyla yakalar.
- Etkin proje kökünün dışındaki çalışma dizinlerini reddeder.
- Büyük çıktının model konuşmasında kalması gerekmemesi için daemon tarafında ham bir makbuz saklar.

Önemli sınırlar ve hatalar:

- İşletim sistemi korumalı alanı değil, proje kökü sınırlaması ve onay işlemesidir.
- Etkileşimli terminal, PTY, arka plan oturumu, standart girdi akışı veya rastgele ortam yapılandırması sağlamaz.
- Sıfır olmayan çıkışlar, zaman aşımları ve çıktı kesilmesi bildirilir.

<a id="chat-controls"></a>
### Sohbet Denetimleri

Yerleşik sohbet, projeyi değiştiren ve çalışma zamanı araç çağrıları için onay
modlarını destekler. Salt okunur inceleme hemen çalışabilirken değişiklik veya
yürütme açık onay gerektirebilir. Tam erişim bu istemleri kaldırır, ancak katı
güvenlik denetimlerini atlamaz.

Godot'un betik düzenleyicisinde seçilen kod **Add to Chat** ile eklenebilir.
Oluşturucu göndermeden önce eki gösterir. `/provider` sağlayıcı kurulumunu,
`/model` ise model seçimini açar; bunlar MCP araçları değil, sohbet komutlarıdır.

<a id="what-fennara-does-not-replace"></a>
## Fennara Nelerin Yerini Almaz

Şunlar için normal geliştirme araçlarını kullanın:

- geniş depo araması ve gezintisi
- sıradan metin dosyası okuma
- farklar ve sürüm denetimi
- Godot geri bildirimi gerektirmeyen düzenlemeler
- genel kabuk çalışması

Yanıt projenin Godot tarafından anlaşılmasına, içe aktarılmasına,
serileştirilmesine, işlenmesine, doğrulanmasına veya çalıştırılmasına bağlı
olduğunda Fennara'yı kullanın.
