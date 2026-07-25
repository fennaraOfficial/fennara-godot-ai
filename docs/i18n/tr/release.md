<!-- fennara-i18n: locale=tr source=docs/release.md sha256=60b8cc51e0fcde9b4e18eadc230aaf1d8cc4fad2fe70cbf5190ab9123bac0073 -->
<a id="release-process"></a>
# Sürüm Süreci

<!-- fennara-doc-nav:start -->
[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../release.md)
<!-- fennara-doc-nav:end -->

Sürümler el ile oluşturulur. Pull request iş akışlarından yayımlama yapmayın.

> [!IMPORTANT]
> Sürümleri `main` üzerinden çalıştırın, `VERSION` ile iş akışı girdisini aynı
> tutun ve sürümün daha yüksek bir en düşük CLI sürümü gerektirip gerektirmediğine
> açıkça karar verin.

<a id="release-at-a-glance"></a>
## Bir Bakışta Sürüm

| Adım | Sonuç |
| --- | --- |
| Sürüm değişikliğini hazırlayıp birleştirin | Depodaki sürüm kaynakları birbiriyle uyuşur |
| Package Preview çalıştırın | Sürüm biçimli yapılar yayımlanmadan oluşturulur |
| Önizlemeyi inceleyin | Arşivler, manifest, özetler ve Linux CEF yerleşimi doğrulanır |
| `main` üzerinden Release çalıştırın | Etiket ve GitHub Release yayımlanır |
| Kurulum ve güncelleme duman testi yapın | Genel kullanıcı akışı doğrulanır |

<a id="versioning"></a>
## Sürümleme

`VERSION` doğruluk kaynağıdır.

Sürüm araçları SemVer değerlerini kabul eder. Kararlı sürümler `X.Y.Z` kullanır.
Hazırlık adayları, `pr-101` hazırlık kanalı ve `2` bu kanalın aday numarası
olacak şekilde `1.2.3-pr.101.2` gibi yalıtılmış bir pull request ön sürümü
kullanır.

Depo sürümünü yükseltmek için:

```bash
node scripts/set-version.mjs X.Y.Z
```

Betik şunları günceller:

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- eklenti sürümü sabitleri
- `local/` altındaki Rust çalışma alanı paket sürümü
- `local/Cargo.lock`

Eklenti ayrıca `addons/fennara/release.json` taşır. Kararlı kimlik yukarıdaki
normal komut tarafından otomatik olarak yazılır. Bir hazırlık derlemesi çalışma
alanı açık kimlik girdilerini kullanır:

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

Hazırlık sürümü, kanal, kaynak commit'i ve tam sürüm etiketi birbiriyle
uyuşmalıdır. Bu kimliğe sahip olmayan bir ön sürüm eklentisi reddedilir.
`release.json` öncesinden kalan mevcut kararlı eklentiler varsayılan olarak
kararlı izi kullanmaya devam eder.

Sürüm eşitlemesini denetleyin:

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. Sürüm Commit'ini Hazırlayın

1. Sürüm betiğini çalıştırın.
2. Farkı inceleyin.
3. Değişen yüzeyle eşleşen yerel denetimleri çalıştırın.
4. Sürüm hazırlığı PR'ını `main` ile birleştirin.

Yaygın denetimler:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

GDExtension değişikliklerinde, mümkün olduğunda eklentiyi yerel olarak da derleyin:

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Package Preview Çalıştırın

Bunu, paketleme değiştiğinde veya yayımlamadan deneme yapmak istediğinizde kullanın.

GitHub:

```text
Actions > Package Preview > Run workflow
```

İş akışı Windows, Linux ve macOS paketlerini derler ve geçici yapılar yükler.
Etiket, GitHub Release veya `latest` oluşturmaz.

Package Preview, birleştirmeden önce sürüm paketlemesini sınayabilmek için
Release'in yayımlama dışındaki bölümlerini yeterince yakından yansıtır:

- derleme gerektirmeyen sohbet arayüzünü ve çalışma zamanı yardımcı kaynağını eklenti yüküne eşitler
- Linux CEF çalışma zamanı zip'ini oluşturur
- oluşturulmuş Linux CEF çalışma zamanı manifestini yazar
- oluşturulan bu manifesti platform paketi derlemelerine aktarır
- tüm platformları içeren eklenti arşivini oluşturur
- yerel/eklenti paketlerini manifest tarafından yönetilen sürüm yapısı adlarına yeniden adlandırır
- Linux CEF çalışma zamanı yapısını oluşturulan manifeste göre doğrular
- `fennara-release-manifest-v<version>.json` dosyasını yazar
- sürüm biçimli zip'leri ve manifesti içeren tek bir `fennara-package-preview-release-assets` yapısı yükler

Önizleme yapıları, yayımlamadan önce zip içeriklerini ve manifest biçimini
denetlemek için kullanışlıdır. Bunlar Actions yapılarıdır, genel sürüm yapıları
değildir.

<a id="3-run-release"></a>
## 3. Release Çalıştırın

Elle çalıştırılan sürüm iş akışını `main` üzerinden çalıştırın:

```text
Actions > Release > Run workflow
```

Girdiler:

```text
version: X.Y.Z
promote_latest: true
```

`version` girdisi `VERSION` ile eşleşmelidir.

İş akışı şunları yayımlar:

- `v<version>`
- `promote_latest` true olduğunda `v<version>` sürümünü GitHub Latest olarak işaretler

Sürüm iş akışı platform paketlemesinden önce Linux CEF çalışma zamanını
hazırlar. Sabitlenmiş resmi CEF 139 Linux minimal SDK'sını indirir, ayrı
`fennara-webview-cef-linux-x64-<cef-version>.zip` dosyasını oluşturur,
hazırlanmış ELF ikili dosyalarını ayıklar, oluşturulmuş etkin bir
`local/webview-runtimes/linux-cef.json` manifesti yazar ve bu manifesti CLI
paketlerine aktarır. Ardından yayımlama işi, sürüm yapılarının oluşturulmuş
manifestin adlandırdığı tam CEF zip'ini içerdiğini ve SHA-256 değerinin
eşleştiğini doğrular. Ayrıca `fennara-release-manifest-v<version>.json`
dosyasını yazar, başvurulan her yapıyı ve özeti doğrular ve bu manifesti
sürümle birlikte yükler.

Pull request iş akışları sürüm yayımlamaz. Package Preview iş akışı, bakım
yapanların birleştirmeden önce paketleme duman testi yapabilmesi için manifest
ve Linux CEF çalışma zamanı yükü dahil sürüm biçimli test yapıları oluşturur.
Package Preview, kullanıcıya dönük sürüm kanalı değildir.

<a id="release-assets"></a>
## Sürüm Yapıları

Her sürüm platform başına CLI/yerel çalışma zamanı paketlerini ve tüm platformlar için tek bir paylaşılan eklenti paketini içermelidir.

| Hedef | Yapılar |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| Tüm platformlar | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Paket rolleri:

| Desen | Rol |
| --- | --- |
| `fennara-cli-*` | Tek platform için yalnızca `fennara` CLI'ını içeren kurulum betiği yükü |
| `fennara-release-local-*` | Tek platform için MCP ve daemon başlatıcıları ile sürümlü çalışma zamanı ikili dosyaları |
| `fennara-release-addon-v*` | Sürüm manifesti üzerinden çözümlenen sürümlü tüm platformlar eklentisi |
| `fennara-addon-latest.zip` | Belgeler ve elle indirmeler için kararlı adlı tüm platformlar eklentisi diğer adı |
| `fennara-webview-cef-linux-x64-*` | Fennara uygulama verilerine bir kez kurulan yalnızca Linux'a yönelik paylaşılan CEF çalışma zamanı |
| `fennara-release-manifest-v*` | Yapı adlarını, SHA-256 değerlerini, kurulum temel işlemlerini ve paylaşılan çalışma zamanlarını içeren kurulum ve güncelleme planı |

macOS eklentisi GDExtension'ına şu anda Apple noter onayı verilmemiştir.
Tarayıcıdan indirme ve Finder ile elle çıkarma karantina meta verilerini
aktarabilir ve macOS doğrulama bildirimini tetikleyebilir. Kullanıcıya dönük
kurulum belgeleri macOS'ta `fennara install` önermeli, elle ZIP sınırlamasını
açıklamalı ve etkilenen kullanıcılara CLI üzerinden yeniden kurmadan önce elle
kopyalanmış eklentiyi kaldırmalarını söylemelidir. Sürüm doğrulaması yalnızca ZIP
oluşturmayı macOS imzalama veya noter onayı olarak kabul etmez.

`fennara-release-local-*` ön eki, eski CLI'ların manifest tarafından yönetilen
paket yolunu sessizce atlamasını önler.

<a id="release-manifest"></a>
## Sürüm Manifesti

0.3.0 sürümünden itibaren `fennara install` ve `fennara update`, sürüm bir
manifest yayımladığında bunu tercih eder. Manifest şunları kaydeder:

- `schema_version`
- `version`
- `minimum_cli_version`
- desteklenen kurulum temel işlemleri
- SHA-256 özetleriyle platform başına CLI ve yerel çalışma zamanı yapıları
- SHA-256 değerine sahip paylaşılan eklenti yapısı
- platforma özel paylaşılan çalışma zamanı yapıları, şu anda Linux CEF

`scripts/release-policy.mjs`, `minimum_cli_version` için doğruluk kaynağıdır.
Manifest yazıcısı, sürüm kimliğini doğruladıktan sonra ilkeyi seçer; böylece
Stable, Package Preview ve Staging birbirinden bağımsız değerler seçemez.
Normal paket yerleşimi veya yapı adı değişiklikleri dış CLI değiştirilerek değil,
manifest verileriyle ele alınmalıdır. Bir sürüm daha yeni bir güncelleyici
devri, manifest şeması, kurulum temel işlemi, kendini güncelleme davranışı veya
daha eski yayımlanmış bir CLI'ın güvenli biçimde gerçekleştiremeyeceği başka bir
CLI özelliği gerektirdiğinde ilkeyi yükseltin.

CLI çok eski olduğunda `fennara update`, önce kurulu CLI'ı güncellemek için
manifestin platform başına `assets.cli` girdisini kullanmalı, ardından
`--no-self-update` ile paket güncellemesine devam etmelidir. Bu sürüm veya
kurulum konumu için kendini güncelleme kullanılamıyorsa paketleri kurmadan önce
başarısız olmalı ve `install.sh` ya da `install.ps1` dosyasını yeniden
çalıştırmaya yönelik açık bir talimat yazdırmalıdır.

Manifest şeması 1'e eklenen isteğe bağlı sürüm kimliği, en düşük CLI artışı
gerektirmez. Eski şema-1 istemcileri bilinmeyen alanları yok sayar; hazırlık
farkındalığı olan istemciler ise mevcut olduğunda kimliği doğrular. Kanal
farkındalığı olan etkinleştirmeye veya güncelleyici devrine dayanan gelecekteki
bir sürüm, yayımlanmadan önce en düşük CLI'ı yeniden değerlendirmelidir.

<a id="staging-identity-and-discovery-contract"></a>
## Hazırlık Kimliği ve Keşif Sözleşmesi

Hazırlık kanalları pull request başına yalıtılır:

| Değer | PR 101 örneği |
| --- | --- |
| Kanal | `pr-101` |
| Aday sürüm | `1.2.3-pr.101.2` |
| Tam sürüm | `v1.2.3-pr.101.2` |
| Kanal başvurusu | `fennara-staging/pr-101` |
| İşaretçi dosyası | `fennara-staging-channel-pr-101.json` |

Kanal başına Git başvurusu yalnızca tam sürümlü bir sürüme giden küçük bir
işaretçi dosyası içerir. Sürüm ikili dosyaları hiçbir zaman hareketli kanal
başvurusu altında bulunmaz. CLI bu işaretçiyi dahili `channel:pr-101` sürüm
isteğiyle çözümleyebilir, ardından yalnızca tam sürümü kullanarak devam eder.

Bu nedenle PR 101 ve PR 125 farklı sürüm etiketleri ve işaretçi yapıları
kullanır. Bir kanalı güncellemek diğer kanaldaki test kullanıcılarını yeniden
yönlendiremez. Bir kanalı yayımlamak kararlı GitHub Latest tanımını veya başka
bir pull request'in kanalını hiçbir zaman değiştirmez.

<a id="staging-candidate-workflow"></a>
## Hazırlık Adayı İş Akışı

Elle çalıştırılan **Staging Release** iş akışı, açık bir pull request'in güncel
head'inden aday oluşturur. Bunu `main` üzerinden çalıştırın ve şunları sağlayın:

| Girdi | Anlam |
| --- | --- |
| `pull_request` | Derlenecek açık pull request |
| `base_version` | `X.Y.Z` biçiminde planlanan kararlı sürüm |
| `candidate` | Bu pull request için artan aday numarası |
| `source_commit` | Hâlâ pull request head'i olması gereken isteğe bağlı tam SHA |
| `publish` | Yalnızca yapı doğrulaması için kapalı, adayı yayımlamak için açık |

İş akışı herhangi bir platform derlemesinden önce pull request head SHA'sını
sabitler. Windows, Linux ve macOS işleri yalnızca okuma izinleriyle, kalıcı Git
kimlik bilgileri olmadan, sürüm kimlik bilgileri olmadan ve paylaşılan bağımlılık
önbelleklerini kaydetme yeteneği olmadan tam bu commit'i kullanıma alır. Güvenilir
varsayılan dal iş akışlarının yazdığı uyumlu SCons/godot-cpp ve Cargo
önbelleklerini geri yükleyebilirler. Hazırlık yalnızca geri yükleme önbellek
eylemini kullanır; böylece aday kodu güvenilir derleme çıktılarını tüketebilir
ancak sonraki çalıştırmaların önbelleklerini değiştiremez veya zehirleyemez.
Aday kodu derleme yapıları üretebilir, ancak GitHub Release yayımlayamaz.

Güvenilir depo betikleri daha sonra aday kimliğini, tam arşiv envanterini,
eklenti içeriklerini, platform paketi yerleşimini, sürüm manifestini ve her
SHA-256 değerini doğrular. `publish` açıkça seçilmedikçe yayımlama devre dışı
kalır.

Yayımlama etkinleştirildiğinde güvenilir son iş:

1. Aday yapılarını veri olarak yeniden doğrular.
2. Bir taslak oluşturur, her yapıyı yükler ve GitHub Latest'ı değiştirmeden tam `v<exact-version>` ön sürümü olarak yayımlar.
3. Yayımlanan yapıları indirir ve adlarını ve özetlerini karşılaştırır.
4. Geriye giden veya çakışan bir kanal değişikliğini reddeder.
5. Küçük `fennara-staging/pr-<number>` işaretçi başvurusunu en son, koşullu bir GitHub Contents API yazımıyla günceller.
6. Etkin işaretçiyi indirir ve tam içeriğini doğrular.

Bir pull request'e ait çalıştırmalar sıraya alınır. Farklı pull request'ler ayrı
eşzamanlılık grupları, sürüm etiketleri ve işaretçi başvuruları kullanır. Aynı
adayı yeniden denemek, dosyaları içine karıştırmak yerine mevcut tam sürümü
doğrular. İş akışı hiçbir zaman kararlı GitHub Latest oluşturmaz, ona yükleme
yapmaz veya onu yükseltmez.

Kararlı yayımlama gerçek bir `latest` etiketi veya sürümü kullanmaz. Release iş
akışı tam `v<version>` sürümünü taslak olarak oluşturur, yüklenen yapıları bayt
bayt doğrular, değiştirilebilir bir sürüm olarak yayımlar ve `promote_latest`
true olduğunda tam bu sürümü GitHub Latest olarak işaretler. Yükleyiciler ve
kararlı CLI keşfi GitHub'ın Latest Release API uç noktasını çözümler.

Depo sürümü değiştirilemezliği devre dışıyken kararlı ve hazırlık sürümleri
değiştirilebilir. Her iki iş akışı da yayımlamayı tamamlamadan veya bir hazırlık
kanalını ilerletmeden önce sürüm meta verilerini ve indirilen yapı baytlarını
doğrular. Yapı yayımlama, içerik yazma erişimine sahip iş kapsamlı
`GITHUB_TOKEN` kullanır.

Sürüm ilkesi şu anda kararlı manifestler için CLI `0.4.1`, hazırlık
manifestleri için CLI `0.3.8` gerektirir. Kararlı keşif artık kullanımdan
kaldırılan `latest` etiketini çözümlemez. Kararlı `0.4.1`, düzeltilmiş
güncelleme doğrulamasını, sürüm geçişi ön denetimini, Windows işlem günlüğü
işlemeyi ve Linux CEF çalışma zamanı işaretçisi onarımını gerektirir. `0.4.1-pr.123.1` gibi bir hazırlık adayı SemVer altında
kararlı `0.4.1` değerinden küçük karşılaştırılır; bu nedenle ilk çalıştırma
kurulumunun aday CLI'ı kurabilmesi için aday sürümün en düşük değeri aday
sürümden küçük kalmalıdır. İki en düşük değerden hiçbirini yalnızca manifest
şeması uyumluluğuna dayanarak değiştirmeyin.

Paylaşılan eklenti zip'i, `godot_demo/addons/fennara/fennara.gdextension`
tarafından başvurulan her derlenmiş GDExtension ikili dosyasını içerir. Godot,
kullanıcının işletim sistemiyle eşleşen kitaplığı yükler ve diğerlerini yok
sayar.

Linux CEF webview çalışma zamanı yükleri eklenti arşivinden ayrıdır. Sürüm
paketleme etkin çalışma zamanı manifestini oluşturur ve bu verileri
`fennara-release-manifest-v<version>.json` içine gömer. CLI eşleşen CEF yükünü
kullanıcının Fennara uygulama verisi dizini altına bir kez kurar:

```text
webview/cef/linux-x64/<cef-version>/
```

`libcef.so`, CEF yardımcı çalıştırılabilir dosyaları, CEF kaynakları veya yerel
dil paketlerini `fennara-addon-*` içine koymayın. Package Preview test için ayrı
bir CEF yapısı oluşturur ve Release tarafından kullanılanla aynı türde
oluşturulmuş çalışma zamanı manifesti yazar, ancak sürüm yayımlama kullanıcıya
dönük sürüm yapılarının tek kaynağı olarak kalır.

Linux GDExtension derlemeleri ayrıca resmi CEF SDK sarmalayıcı kaynağına ihtiyaç
duyar, ancak eklentide CEF çalışma zamanı dosyalarına ihtiyaç duymaz. CI şunu
çalıştırır:

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

ve çıkarılan dizini `FENNARA_CEF_ROOT` olarak SCons'a aktarır. SCons,
sabitlenmiş CEF 139 C++ sarmalayıcısına karşı küçük
`libfennara_linux_cef_bridge.so` eklenti kitaplığını derlemek için
`FENNARA_CEF_ROOT/libcef_dll/` kullanır. Oluşturulan sarmalayıcı kaynağının
çalışma zamanı CEF ABI'siyle eşleşmesi gerektiği için SDK indirmesi sürüm ve
özet bakımından denetlenir. Köprü eklentiyle paketlenir; `libcef.so`, kaynaklar,
yerel dil paketleri ve `fennara_cef_helper` ayrı paylaşılan CEF çalışma zamanında
kalır.

Paket betikleri eklenti arşivinde CEF çalışma zamanı dosyaları bulunursa
başarısız olur. Çalışma zamanı yapısı adı şu olmalıdır:

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

Zip, gerekli dosyalar kökünde olacak şekilde çıkarılmalıdır:

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

`chrome-sandbox`, `libEGL.so`, `libGLESv2.so`, `libvk_swiftshader.so`,
`libvulkan.so.1`, `vk_swiftshader_icd.json`, `snapshot_blob.bin` ve ek
`locales/*.pak` gibi isteğe bağlı CEF çalışma zamanı dosyaları, seçilen CEF
dağıtımında mevcut olduklarında dahil edilmelidir.

Bakım yapanın seçtiği bir CEF ikili ağacından çalışma zamanı zip'ini elle oluşturmak için:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

Linux'ta betik, `fennara-cpp/vendor/cef/` içindeki resmi CEF başlıklarına karşı
`scripts/cef/linux/fennara_cef_helper.cpp` dosyasından
`fennara_cef_helper` derler. Başka bir işletim sisteminde önce bu yardımcıyı
Linux'ta derleyin ve `--helper /path/to/fennara_cef_helper` aktarın. Zip'i
yazmadan önce seçilen dosyaları incelemek için `--dry-run` kullanın.

Betik SHA-256 değerini yazdırdıktan sonra
`local/webview-runtimes/linux-cef.json` dosyasını güncelleyin:

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Normal sürümlerde iş akışı Linux CEF çalışma zamanı manifestini
`--write-manifest` ile otomatik olarak yazar, ardından
`scripts/write-release-manifest.mjs` çalışma zamanı alanlarını
`fennara-release-manifest-v<version>.json` içine kopyalar. Elle bir çalışma
zamanı yapısı yolunda veya eski yedek davranışında bilinçli olarak hata
ayıklamıyorsanız, depoda bulunan yer tutucu manifesti elle etkinleştirmeyin.
Oluşturulmuş manifest verileri eksik olan veya SHA-256 değeri eşleşmeyen bir
yapıyı gösteriyorsa Release iş akışı ve Linux `fennara install` /
`fennara update` açık bir biçimde başarısız olur.

CLI, Linux CEF çalışma zamanı güncellemelerini atomik olarak yayımlamalıdır:
bir hazırlık dizinine çıkarıp doğrulayın, çalışma zamanı işaretçisini yalnızca
gerekli dosyalar mevcut olduktan sonra yazın, ardından sürüm dizinini yayımlayın
ve `current.json` dosyasını geçici dosya yeniden adlandırmasıyla güncelleyin.
Kurulu `fennara-cef-runtime.json` işaretçisi yerel yükleyici sözleşmesini
`"runtime": "cef"` ile tanımlamalıdır. Kurulum ve güncelleme, yalnızca
`"kind": "cef"` içeren eşleşen eski bir işaretçiyi CEF yükünü yeniden indirmeden
onarır. Çalışan düzenleyiciler önceden yükledikleri çalışma zamanını kullanmaya
devam eder.

CLI, `local/templates/` içindeki oluşturulmuş proje rehberi şablonlarını gömer.
Sürüm paketleme CLI'ı derlediğinde bu şablonlar CLI kodunun geri kalanıyla
birlikte ikili dosyada derlenir.

<a id="what-latest-means"></a>
## `latest` Ne Anlama Gelir

GitHub'ın Latest Release işaretçisi, normal kurulum ve güncelleme akışlarının
kullandığı sürümlü sürümü seçer. Fennara gerçek bir `latest` etiketi oluşturmaz
veya taşımaz.

- `install.ps1` ve `install.sh` varsayılan olarak en son CLI yapısını getirir.
- `fennara update` varsayılan olarak GitHub'ın Latest Release uç noktası üzerinden sürüm manifestini getirir, gerektiğinde kurulu CLI'ı kendisi günceller, ardından yerel/eklenti/paylaşılan çalışma zamanı yapılarını buradan çözümler.
- Düzenleyici içi güncellemeler kapanmadan önce doğrulanmış yapıları hazırlar, değiştirmeden önce hazırlanmış eklentinin tam özetini yeniden denetler, etkinleştirme doğrulaması başarılı olana kadar önceki eklentiyi, başlatıcıları ve çalışma zamanı manifestini tutar ve geri alma verilerini silmeden önce yeniden açılan GDExtension el sıkışmasını gerektirir.
- `fennara install` varsayılan olarak GitHub'ın Latest Release uç noktası üzerinden sürüm manifestini getirir, ardından yerel/eklenti/paylaşılan çalışma zamanı yapılarını buradan çözümler.
- Godot eklentisi güncelleme denetimi GitHub'ın en son sürümüyle karşılaştırır.

Yalnızca varsayılan kullanıcı kurulumu olmaması gereken bir sürüm yayımlarken `promote_latest: false` kullanın.

Yükleyiciler ve sürüm indirmeleri sürüm meta verilerini, yapı indirme, çıkarma,
kurma ve doğrulama adımlarını yazdırmalıdır. Ağdan getirmeler sınırlı zaman
aşımları kullanmalıdır; böylece GitHub/CDN takılmaları donmuş gibi görünmek
yerine bir tanılamayla başarısız olur. Windows'ta `install.ps1`, başarı yazmadan
önce CLI doğrulama çıkış kodunu denetlemelidir. Çıkış kodu `-1073741515`
(`0xC0000135`), CLI çalıştırılabilir dosyasının yazıldığı ancak gerekli bir DLL
eksik olduğu için Windows'un onu başlatamadığı anlamına gelir; kullanıcıya
Microsoft Visual C++ Redistributable 2015-2022 x64 kurmasını ve ardından
`fennara --version`, `fennara doctor` ve `fennara install` komutlarını yeniden
çalıştırmasını söyleyin.
İndirme URL'si: `https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## Sürümden Sonra Duman Testi

Windows'ta:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

Bir Godot projesinde:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Projenin şunları aldığını denetleyin:

```text
AGENTS.md
addons/fennara/ai/
```

Projeyi Godot'ta açın, ardından MCP uygulamasına şunu sorun:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Güncelleme testi:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## Kurallar

- Release iş akışı yalnızca `main` üzerinden çalışır.
- Sürüm girdisi `VERSION` ile eşleşmelidir.
- Pull request iş akışları test yapılarını oluşturup yükleyebilir, ancak sürüm yayımlamamalıdır.
- Normal kullanıcılar için amaçlanan sürümü GitHub Latest olarak belirlenmiş tutun.
- Bakım yapanlar bozuk bir sürümü değiştirmeye bilinçli olarak karar vermedikçe yayımlanmış sürüm etiketlerini yeniden yazmayın.
