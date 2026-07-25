<!-- fennara-i18n: locale=tr source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# Depo Haritası

<!-- fennara-doc-nav:start -->
[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Bu, bu depoda çalışan katkıda bulunanlar ve kodlama aracıları için hızlı haritadır.

<a id="find-the-right-area"></a>
## Doğru Alanı Bulun

| Değişiklik | Birincil Konum |
| --- | --- |
| Kullanıcı kurulumu veya CLI davranışı | `local/crates/fennara-cli/` |
| Harici MCP protokolü veya şemaları | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Yerleşik sohbet veya daemon davranışı | `local/crates/fennara-daemon/` |
| Godot düzenleyici entegrasyonu | `fennara-cpp/` |
| Sohbet arayüzü | `ui/chat/` |
| Çalışma zamanı yardımcı betikleri | `runtime/` |
| Paketleme veya sürümler | `scripts/`, `.github/workflows/` |
| Kullanıcı belgeleri | `README.md`, `docs/` |

<a id="top-level"></a>
## Üst Düzey

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `.github/` | Pull request şablonu, issue şablonları ve GitHub Actions iş akışları. |
| `docs/` | Proje belgeleri, kurulum rehberleri, mimari notları, örnekler, demolar ve sürüm notları. |
| `docs/i18n/` | Yerel manifesti ve eksiksiz çevrilmiş belge ağaçları. |
| `fennara-cpp/` | C++ Godot GDExtension kaynağı ve SCons derleme giriş noktası. |
| `godot_demo/addons/fennara/` | Kullanıcı projelerine kopyalanan kurulabilir Godot eklenti yükü. |
| `local/` | Rust CLI, MCP sunucusu, daemon, şemalar ve yerel çalışma zamanı kodu. |
| `media/` | Belgelerin kullandığı görseller ve genel medya. |
| `runtime/` | `runtime_session` ve `runtime_script` tarafından kullanılan Godot çalışma zamanı yardımcı betiklerinin kaynağı. |
| `scripts/` | Sürümleme, paketleme ve sürüm yardımcı betikleri. |
| `ui/chat/` | İsteğe bağlı düzenleyici içi web sohbeti arayüzünün kaynağı. |
| `local/templates/` | `fennara install` tarafından Godot projelerine yazılan ve `fennara update` tarafından yenilenen kısa proje yönergeleri ile isteğe bağlı AI bilgi sayfaları. |
| `local/webview-runtimes/` | Linux CEF yükü gibi paylaşılan Fennara uygulama verilerine kurulan harici webview çalışma zamanları için manifest/yapılandırma dosyaları. |
| `install.ps1` / `install.sh` | Fennara CLI'ını GitHub sürümlerinden kuran önyükleme betikleri. |
| `VERSION` | Sürüm doğruluk kaynağı. |
| `README.md` | İnsanlara dönük kısa genel bakış ve hızlı başlangıç. |
| `docs/README.md` | Görev odaklı belge dizini. |
| `docs/setup.md` | Kullanıcıya dönük, önce eklenti kurulumu; sohbet ön koşulları; MCP bağlantısı; güncelleme akışı ve sorun giderme. |
| `docs/cli.md` | Terminal komutu başvurusu, CLI'a ait kurulum/güncelleme davranışı, kurtarma, tanılama, uygulama verisi yerleşimi ve otomasyon rehberi. |
| `docs/telemetry.md` | Anonim etkinlik yükü, uygulama verisi durumu, teslim davranışı, aylık etkin tanımı ve vazgeçme denetimleri. |
| `CONTRIBUTING.md` | Katkı kuralları. |
| `SECURITY.md` | Güvenlik bildirimi ilkesi. |
| `LICENSE.md` | Proje lisansı. |

<a id="local-rust-packages"></a>
## Yerel Rust Paketleri

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `local/crates/fennara-cli/` | `fennara` komutu: kurulum, güncelleme, CLI kendini güncelleme, doctor, işlem tanılamaları, webview ön koşulu denetimleri, C# desteği, MCP uygulaması kurulumu ve oluşturulan proje rehberi. |
| `local/crates/fennara-cli/src/operation.rs` | Genel kurulum/güncelleme işlem koordinatörü, aşamalar ve CLI devretme giriş noktaları. |
| `local/crates/fennara-cli/src/operation/` | Odaklı işlem günlüğü, kalıcı depolama, tanılama sansürleme ve test modülleri. |
| `local/crates/fennara-cli/src/project_addon.rs` | Mevcut proje eklentisi sürümü ve geçerli platform GDExtension kitaplığı doğrulaması. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Godot başlamadan önce yalnızca Fennara'nın kalıcı çalışma zamanı autoload'unu kaldıran eklentisiz CI dışa aktarma hazırlığı. |
| `local/crates/fennara-cli/src/release_identity.rs` | Kararlı/hazırlık eklentisi kimliği, tam sürüm seçicileri, pull request kanalı doğrulaması ve eski kararlı uyumluluğu. |
| `local/crates/fennara-cli/src/release_channel.rs` | Kanal başına hazırlık işaretçisi doğrulaması ve tam sürümlü bir sürüme çözümleme. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Sürüm manifesti ayrıştırma, yapı özeti doğrulaması, kimlik bağlama ve platform paketi seçimi. |
| `local/crates/fennara-cli/src/release_version.rs` | Manifestler ve sürüm seçimi tarafından kullanılan paylaşılan CLI SemVer ayrıştırması ve önceliği. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Proje eklentisi dosyalarını değiştirmeden mevcut eksiksiz eklentinin tam sürümle benimsenmesi. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Kurulum ve doctor tarafından kullanılan paylaşılan daemon sağlık denetimi, tam sürüm hazırlığı ve başlatma. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Süreç düzeyi hata, kalıcı tanılamalar, sansürleme ve kapalı biçimde başarısız olan işlem günlüğü testleri. |
| `local/crates/fennara-cli/src/diagnostics.rs` | En son veya adlandırılmış, temizlenmiş işlem raporuna kullanıcıya dönük erişim. |
| `local/crates/fennara-mcp/` | Yerel stdio MCP sunucusu ve araç şeması iletimi. |
| `local/crates/fennara-daemon/` | Çalışma zamanı oturumları ve Godot köprü çalışması için kullanılan yerel daemon. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Anonim günlük etkin zamanlayıcı, sınırlı kuyruk, HTTP teslimi ve daemon yaşam döngüsü entegrasyonu. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Rastgele kurulum kimliği doğrulaması, atomik uygulama verisi kalıcılığı, günlük makbuz durumu ve vazgeçme temizliği. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Yerleşik sohbet onay modları, araç riski sınıflandırması, izin kararları ve bekleyen onay isteği türleri. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Daemon'a ait yerleşik sohbet `exec_command` uygulaması: kabuk algılama, cwd doğrulama, süreç başlatma, zaman aşımı/ağaç sonlandırma, çıktı yakalama, sonuç yapısı günlüğü ve sonuç biçimlendirme. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Yerleşik sohbet bağlam sıkıştırma planlayıcısı: tam kuyruk koruması, OpenCode biçimli eski araç sonucu baskısı budaması, özet parçası seçimi/depolama/yeniden oynatma, özet istemi serileştirmesi, belirteç bütçeleri ve yer tutucu işleme. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | Yerleşik sohbet PromptBuilder'ı ve oluşturulmuş çalışma zamanı ortam bağlamı. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Yalnızca yerel yerleşik sohbet iz kaydedicisi, SQLite olay satırları, saklama ve hata ayıklama sorgusu yardımcıları. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, özel uç noktalar, Ollama/local ve LM Studio için yerleşik sohbet sağlayıcısı çalışma zamanı temelleri, katalog/çözümleme, bağlam ön denetimi kancaları, normalleştirilmiş akış/hata türleri ve OpenAI uyumlu veya Anthropic uyumlu bağdaştırıcılar. |
| `local/schemas/tools/` | Paylaşılan araç JSON şemaları. Harici MCP sunucusu ve yerleşik sohbet kendi izin verilen alt kümelerini gömer. |
| `local/webview-runtimes/linux-cef.json` | Sürüm manifesti oluşturma, doctor çıktısı ve eski yedek için kullanılan Linux CEF çalışma zamanı yer tutucu/oluşturulmuş manifesti. CEF'i eklenti zip'ine yerleştirmeden paylaşılan uygulama verisi yerleşimini ve arşiv meta verilerini kaydeder. |
| `local/Cargo.toml` | Rust çalışma alanı yapılandırması. |
| `local/Cargo.lock` | Kilitlenmiş Rust bağımlılık grafiği. |

<a id="gdextension-source"></a>
## GDExtension Kaynağı

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `fennara-cpp/SConstruct` | GDExtension derleme giriş noktası. |
| `fennara-cpp/include/` | Genel C++ başlıkları. |
| `fennara-cpp/src/` | C++ uygulaması. |
| `fennara-cpp/src/setup/` | Yerel ilk çalıştırma kurulum durumu, sürüm manifesti CLI önyüklemesi, özet doğrulama, CLI başlatma ve kalıcı işlem ilerlemesi okuyucusu. |
| `fennara-cpp/src/release/version.cpp` | Sürüm/güncelleme keşfi tarafından kullanılan yerel SemVer doğrulaması ve önceliği. |
| `fennara-cpp/src/release/identity.cpp` | Paketlenmiş kararlı/hazırlık kimliği doğrulaması ve eski kararlı uyumluluğu. |
| `fennara-cpp/src/release/discovery.cpp` | GitHub Latest ve yalıtılmış hazırlık kanalı güncelleme keşfi. |
| `fennara-cpp/src/update/` | Tam hedef güncelleme koordinasyonu, kalıcı makbuz keşfi, kapatma/kurulum devri ve kurtarma arayüzü durumu. |
| `fennara-cpp/src/ui/setup_panel.cpp` | İlerleme, yeniden deneme, günlükler ve temizlenmiş rapor eylemlerine sahip, webview'dan bağımsız ilk çalıştırma kurulum paneli. |
| `fennara-cpp/vendor/cef/` | Linux OSR köprüsünün kullandığı resmi CEF 139 başlık anlık görüntüsü. Çalışma zamanı ikili dosyaları eklentinin dışında kalır. |
| `fennara-cpp/src/ui/webview_host*` | Yerel düzenleyici içi sohbet webview ana bilgisayarı ve platform arka uçları. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Yalnızca Linux'a yönelik paylaşılan CEF çalışma zamanı keşfi, işaretçi doğrulaması ve dinamik `libcef.so` yükleyici temeli. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Dahili sohbet webview'ı için yalnızca Linux'a yönelik CEF ekran dışı işleme yüzeyi, Godot girdi iletimi, köprü ABI yüklemesi ve Godot doku güncellemeleri. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Sabitlenmiş resmi CEF 139 `libcef_dll_wrapper` kaynağından ve Fennara'nın CEF OSR bağdaştırıcısından derlenen, yalnızca Linux'a yönelik küçük köprü kitaplığı. Ana GDExtension, harici `libcef.so` çalışma zamanı yüklendikten sonra bunu dlopen ile açar. |
| `fennara-cpp/src/tools/` | Godot'a dönük araç uygulamaları. |
| `fennara-cpp/src/lsp/` | Betik tanılamaları ve dil sunucusu yardımcıları. |
| `fennara-cpp/src/csharp/` | Yalnızca derleme C# proje seçimi, arka plan hazırlığı, yalıtılmış tanılamalar ve çalışma zamanı ön denetimi. |
| `fennara-cpp/src/runtime/` | Çalışma zamanı sahnesi ön denetimi, betik tanılamaları ve hata ayıklayıcı anlık görüntüleri dahil araçların kullandığı yerel çalışma zamanı desteği. |
| `fennara-cpp/godot-cpp/` | Godot C++ bağlamaları alt modülü. |

<a id="addon-payload"></a>
## Eklenti Yükü

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Godot GDExtension kayıt dosyası. |
| `godot_demo/addons/fennara/VERSION` | Eklenti paketi sürümü. |
| `godot_demo/addons/fennara/release.json` | Tam sürüm, sürüm etiketi, kanal ve hazırlık kaynak commit'i dahil paketlenmiş kararlı veya hazırlık kimliği. |
| `godot_demo/addons/fennara/bin/` | Derlenmiş platform kitaplıkları. |
| `godot_demo/addons/fennara/dist/` | Düzenleyici içi sohbet webview'ının kullandığı paketlenmiş web arayüzü yapıları. |
| `godot_demo/addons/fennara/runtime/` | Eklenti içinde dağıtılan `runtime/` dizininin eşitlenmiş paketli kopyası. |
| `godot_demo/tests/first_run_setup_test.gd` | Headless yerel ilk çalıştırma kurulum durumu ve deterministik hata testi. |
| `godot_demo/tests/export_plugin_test.gd` | Dışa aktarma dışlaması ve autoload geri yüklemesi için headless yerel regresyon testi. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Headless yerel ekran görüntüsü bağımsız değişken sözleşmesi regresyon testi. |
| `godot_demo/tests/image_sheet_test.gd` | Headless paylaşılan ekran görüntüsü/çalışma zamanı sayfası oluşturma regresyon testi. |
| `godot_demo/tests/runtime_image_context_test.gd` | Headless çalışma zamanı ham karesi, sayfa ve rastgele Image çıktısı regresyon testi. |

<a id="runtime-helper-source"></a>
## Çalışma Zamanı Yardımcı Kaynağı

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `runtime/game_capture_helper.gd` | Sahne oturumları ve çalışma zamanı denetimleri için GDExtension tarafından yüklenen çalışma zamanı yardımcı giriş noktası. |
| `runtime/image_label.gd` | Yakalamadan sonra oluşturulan Image hücrelerine basılan kısa, deterministik etiketler. |
| `runtime/image_sheet.gd` | Ekran görüntüsü ve çalışma zamanı betiği bağlamlarında kullanılan paylaşılan saf Image sayfası oluşturma. |
| `runtime/screenshot_script_context.gd` | Yerel yakalama bağlamına paylaşılan Image oluşturmayı ekleyen genel ekran görüntüsü betiği cephesi. |
| `runtime/runtime_script_context.gd` | Ham kareler, Image oluşturma/çıktı, beklemeler, girdi, anlık görüntüler, koşullar, ışın izlemeleri ve tıklamalar dahil `runtime_script` aracına sunulan genel `ctx` yardımcı yüzeyi. |
| `runtime/runtime_input_driver.gd` | Tuşlar, fare düğmeleri, mutlak fare hareketi, göreli fare hareketi, değiştiriciler ve girdi temizliği için düşük düzey çalışma zamanı girdi olay sürücüsü. |
| `runtime/runtime_node_snapshot.gd` | Çalışma zamanı düğüm arama, varlık denetimleri, eski başvuru güvenli anlık görüntüler, özellik okumaları ve çocuk özetleri. |
| `runtime/runtime_physics_query.gd` | Kısa isabet makbuzlarına sahip çalışma zamanı 2D/3D tam ışın izleme ve tarama yardımcıları. |
| `runtime/runtime_query_utils.gd` | Vektör dönüştürme, güvenli düğüm/yol çözümleme, nesne kimliği ve genel hedef eşleştirme için paylaşılan çalışma zamanı sorgu yardımcıları. |
| `runtime/runtime_capture_store.gd` | Çalışma zamanı oturumları, betikler ve ortam denetimleri tarafından kullanılan çalışma zamanı yakalama/durum yapısı yazıcısı. |
| `runtime/runtime_check_runner.gd` | Etkileşimsiz sahne yürütme belirtimleri için çalışma zamanı denetimi çalıştırıcısı. |

<a id="scripts-and-workflows"></a>
## Betikler ve İş Akışları

| Yol | Sahip Olduğu Alan |
| --- | --- |
| `scripts/set-version.mjs` | Depodaki sürümlü dosyaları günceller. |
| `scripts/check-version.mjs` | Sürüm eşitlemesini denetler. |
| `scripts/release-identity.mjs` | SemVer sürüm kimliği ve PR başına hazırlık işaretçileri için paylaşılan Node doğrulama ve oluşturma. |
| `scripts/release-policy.mjs` | Kararlı ve hazırlık sürümü manifestleri için yayımlanmış en düşük uyumlu CLI ilkesi. |
| `scripts/staging-candidate.mjs` | Güvenilir hazırlık adayı kimliği oluşturma ve PR başına monoton işaretçi kararları. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Odaklı hazırlık eklentisi, arşiv, manifest, paylaşılan dosya sistemi ve yayımlama paketi doğrulaması. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Güvenilmeyen derleme çıktıları ve güvenilir yayımlama paketi için katı doğrulama giriş noktaları. |
| `scripts/check-staging-channel-advance.mjs` | Hazırlık kanalı işaretçisi ilerlemeden önce monotonluk ve köken denetimleri uygular. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | İşaretçi yükseltmeden önce yayımlanmış yapı baytlarını ve genel indirme davranışını doğrular. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Yok sayılan geçici bir Godot projesi derler ve düzenleyici GDExtension'ına karşı salt okunur içe aktarılmış `PackedScene` incelemesinin duman testini yapar. |
| `scripts/release-targets.mjs` | Desteklenen platform sürüm hedeflerini ve paketlenmiş yapı adlarını tanımlar. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Sabitlenmiş aday kimliğini ve küçük kanal işaretçisini yazar. |
| `scripts/sync-chat-ui.mjs` | Derleme gerektirmeyen sohbet arayüzü kaynağını eklenti yüküne kopyalar. |
| `scripts/sync-runtime.mjs` | Depo kökü çalışma zamanı yardımcı kaynağını eklenti yüküne kopyalar. |
| `scripts/sync-doc-navigation.mjs` | Metni çevirmeden belge gezintisi, kaynak özetleri ve kararlı bağlantılar ekler. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Çeviri kapsamını, güncelliğini, Markdown yapısını, URL'leri ve bağlantıları doğrular. |
| `scripts/package-preview.mjs` | Platform derlemelerinden sonra eklenti, CLI ve yerel çalışma zamanı önizleme/sürüm zip'lerini oluşturur. |
| `scripts/prepare-linux-cef-runtime.mjs` | Ayrı Linux x64 CEF çalışma zamanı zip'ini hazırlar, hazırlanmış ELF ikili dosyalarını ayıklar, gerekli dosyaları doğrular ve oluşturulan sürüm manifestini yazabilir. |
| `scripts/prepare-linux-cef-sdk.mjs` | `libcef_dll/` sarmalayıcı kaynağına ihtiyaç duyan CI derlemeleri için sabitlenmiş resmi CEF 139 Linux minimal SDK'sını indirir ve çıkarır. |
| `scripts/check-linux-cef-runtime-release.mjs` | Linux CEF çalışma zamanı sürüm yapısını oluşturulmuş `local/webview-runtimes/linux-cef.json` manifestine göre doğrular. |
| `scripts/write-release-manifest.mjs` | Yerel paket, eklenti ve paylaşılan çalışma zamanı özetleri dahil sürüm yapılarından `fennara-release-manifest-v<version>.json` dosyasını yazar ve doğrular. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Ayrı CEF çalışma zamanı zip'inde paketlenen minimal Linux CEF alt süreç yardımcı kaynağı. |
| `.github/workflows/version-check.yml` | Sürüm tutarlılığı denetimi. |
| `.github/workflows/gdextension-build.yml` | Platformlar arası GDExtension derleme denetimi ve Windows headless yerel ilk çalıştırma kurulum durumu testi. |
| `.github/workflows/local-build.yml` | Rust yerel paket derleme denetimi. |
| `.github/workflows/package-preview.yml` | Linux sohbet duman testleri için yalnızca teste yönelik bir Linux CEF çalışma zamanı yapısı dahil elle çalıştırılan paket önizleme yapıları. |
| `.github/workflows/release.yml` | Oluşturulmuş Linux CEF çalışma zamanı paketlemesi, sürüm manifesti oluşturma ve son yapı doğrulaması dahil elle GitHub sürümü yayımlama. |
| `.github/workflows/staging-release.yml` | Elle tam SHA hazırlık derlemesi, yalnızca doğrulama denemesi, tam ön sürüm yayımlama ve PR başına işaretçi ilerletme. |

<a id="where-to-change-things"></a>
## Neyi Nerede Değiştirmeli

| Görev | Buradan başlayın |
| --- | --- |
| Bir Godot aracı eklemek veya değiştirmek | `fennara-cpp/src/tools/` ve `local/schemas/tools/` |
| MCP şema metnini değiştirmek | `local/schemas/tools/` |
| `fennara install` veya `fennara update` değiştirmek | `local/crates/fennara-cli/src/`; yerel hazırlık ve ayrık uygulama/geri alma `release_update.rs`, `update_stage.rs`, `update_stage/` ve `update_apply/` tarafından yönetilir |
| CLI komutlarını veya terminal davranışını değiştirmek | `local/crates/fennara-cli/src/` ve `docs/cli.md` |
| Yerel güncelleme ilerlemesini, kapatma onayını, etkinleştirme el sıkışmasını veya kurtarmayı değiştirmek | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` ve `ui/chat/` |
| Yerel ilk çalıştırma kurulumunu veya CLI önyüklemesini değiştirmek | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` ve `fennara-cpp/src/ui/dock.cpp` |
| Dışa aktarma sırasında eklenti dışlamasını değiştirmek | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` ve `godot_demo/tests/export_plugin_test.gd` |
| Kurulum/güncelleme işlem günlüklerini, aşamalarını, hata kodlarını veya tanılama raporlarını değiştirmek | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` ve `local/crates/fennara-cli/src/diagnostics.rs` |
| Webview ön koşulu denetimlerini değiştirmek | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` ve `fennara-cpp/src/ui/webview_host*` |
| Oluşturulan proje rehberini değiştirmek | `local/templates/` ve `local/crates/fennara-cli/src/project_guidance.rs` |
| Oluşturulan demo eklentisi rehberini eşitlemek | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` ve `godot_demo/addons/fennara/ai/` |
| MCP uygulaması kurulumunu değiştirmek | `local/crates/fennara-cli/src/mcp_setup.rs` ve `docs/mcp-setup.md` |
| Çalışma zamanı oturumu süreç/günlük davranışını değiştirmek | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` ve `fennara-cpp/src/tool_results/` |
| `runtime_script` ctx yardımcılarını, girdiyi, anlık görüntüleri, beklemeleri, ışın izlemeleri, yakalamaları veya temizliği değiştirmek | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` ve `docs/tools.md` |
| Düzenleyici içi sohbet arayüzünü, eğik çizgi komutlarını veya model/sağlayıcı seçiciyi değiştirmek | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` ve `fennara-cpp/src/ui/webview_host*` |
| Yerleşik sohbet sağlayıcılarını değiştirmek | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` ve `ui/chat/` |
| Anonim telemetri alanlarını, zamanlamayı veya gizlilik denetimlerini değiştirmek | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` ve `docs/telemetry.md` |
| Satıcıdan alınmış sohbet arayüzü kitaplıklarını değiştirmek | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` ve `THIRD_PARTY_NOTICES.md` |
| C# desteğini değiştirmek | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/` ve C# araç şemaları ile rehberi |
| Sürüm paketlerini, en düşük CLI ilkesini veya CLI kendini güncellemeyi değiştirmek | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` ve `.github/workflows/release.yml` |
| Sürümü yükseltmek | `node scripts/set-version.mjs <version>` |
| Sohbet ve MCP, sağlayıcılar veya eğik çizgi komutları için kurulum/belgeleri güncellemek | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` ve `llms.txt` |
| Belge çevirilerini güncellemek | Kanonik İngilizce sayfa, `docs/i18n/languages.json`, eşleşen yerel sayfaları, `scripts/sync-doc-navigation.mjs` ve `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Notlar

- Büyük kaynak alanları eklerken veya taşırken bu dosyayı güncel tutun.
- Sürüm adımlarını [release.md](release.md) içinde tutun.
- Kurulum adımlarını [setup.md](setup.md) içinde tutun.
- Terminal komutu davranışını [cli.md](cli.md) içinde tutun.
