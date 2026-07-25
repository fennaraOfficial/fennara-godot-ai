<!-- fennara-i18n: locale=tr source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# Betikler

<!-- fennara-doc-nav:start -->
[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

Bu dizin yerel geliştirme, paket önizleme ve sürüm iş akışlarınca paylaşılan depo otomasyonunu içerir.

Yardım metinleri aksini söylemediği sürece betikler küçük, belirlenimci ve depo kökünden çalıştırılması güvenli olmalıdır. Depo dışında kullanıcıya özel durum yazmamalıdır.

<a id="version-scripts"></a>
## Sürüm Betikleri

- `set-version.mjs`: depo `VERSION`, eklenti `VERSION`, yerel Rust çalışma alanı meta verileri, kilit dosyası paket sürümleri ve C++ eklentisi sürüm sabitini günceller.
- `check-version.mjs`: bu sürümlenmiş dosyaların hâlâ eşitlenmiş olduğunu doğrular.

`check-version.mjs` dosyasını CI'da ve sürüm paketlemesinden önce çalıştırın. `set-version.mjs` dosyasını yalnızca Fennara sürümünü bilerek değiştirirken kullanın.

<a id="packaging-scripts"></a>
## Paketleme Betikleri

- `package-preview.mjs`: commit edilmiş eklenti veri yüklerini eşitler, ardından GDExtension ve yerel Rust ikilileri zaten derlenmişken platform başına önizleme arşivleri oluşturur.
- `package-addon-all.mjs`: platform eklentisi parçalarını son, tüm platformları içeren eklenti arşivinde birleştirir.
- `release-policy.mjs`: her sürüm izi için asgari uyumlu yayımlanmış CLI'yi tanımlar.
- `write-release-manifest.mjs`: sürüm varlıklarından `fennara-release-manifest-v<version>.json` yazar ve başvurulan her SHA-256 değerini doğrular.

Her iki betik geçici hazırlama için `.package-preview/` kullanır ve zip çıktılarını depo kökündeki `dist/` klasörüne yazar. Bu çıktılar yok sayılır ve commit edilmemelidir.

Paketleme betikleri eklenti veri yükünü küçük tutmalıdır. Özellikle `libcef.so` ve `fennara_cef_helper` gibi Linux CEF çalışma zamanı dosyaları `fennara-addon-*` içine paketlenmemelidir; CEF, kullanıcının paylaşımlı Fennara uygulama verileri dizinine bir kez kurulur.

<a id="staging-release-scripts"></a>
## Hazırlama Sürümü Betikleri

- `write-staging-candidate.mjs`: tek bir pull request ve sabitlenmiş kaynak commit'i için kesin ön sürüm kimliğini oluşturur.
- `validate-staging-build.mjs`: yayımlamadan önce eklenti parçalarını, platform arşivlerini, birleştirilmiş eklentiyi, sürüm manifestosunu ve Linux CEF'i denetler.
- `smoke-public-release.mjs`: yayımlanmış her adayı kimlik doğrulamasız tarayıcı URL'sinden indirir ve kanal ilerlemesinden önce güvenilen varlık ve manifesto karmalarını doğrular.
- `write-staging-pointer.mjs`: tam sürüm manifestosunun karmasını aldıktan sonra küçük PR başına göstergeyi yazar.
- `check-staging-channel-advance.mjs`: geriye veya çakışan kanal hareketini reddeder.
- `validate-staging-publish-bundle.mjs`: son eser paketini aday kodunu çalıştırmadan yeniden doğrular.
- `verify-published-assets.mjs`: beklenen ve indirilen GitHub Release varlık adlarıyla SHA-256 değerlerini karşılaştırır.

Bu betikler `.github/workflows/staging-release.yml` dosyasını destekler. Aday derleme işleri sürüm kimlik bilgileri olmadan çalışır. Yalnızca güvenilen son iş yayımlayabilir ve tam sürüm indirilip doğrulandıktan sonra kanal başına Git başvurusunu ilerletir.

<a id="linux-cef-scripts"></a>
## Linux CEF Betikleri

- `prepare-linux-cef-sdk.mjs`: Linux CEF köprüsünü derlemek için kullanılan sabitlenmiş resmi Linux x64 CEF SDK'sını indirir/çıkarır.
- `prepare-linux-cef-runtime.mjs`: ayrı Linux CEF çalışma zamanı zip dosyasını hazırlar, gerekli dosyaları doğrular, Linux'taki hazırlanmış ELF ikililerini soyutlar ve sürüm paketlemesi için oluşturulan `local/webview-runtimes/linux-cef.json` manifestosunu yazabilir.
- `check-linux-cef-runtime-release.mjs`: sürüm varlıklarının etkin manifestoda adlandırılan CEF çalışma zamanı zip dosyasını içerdiğini ve SHA-256 değerinin eşleştiğini doğrular.
- `cef/linux/fennara_cef_helper.cpp`: CEF SDK'dan çalışma zamanı yardımcısı derlenirken kullanılan küçük CEF yardımcı işlem kaynağı.

CEF betikleri yalnızca kopyalanmış hazırlama dosyalarında çalışır. İndirilmiş/kaynak CEF SDK ağacını değiştirmemelidir.

<a id="development-tests"></a>
## Geliştirme Testleri

- `test-run-scene-edit-script-inspect.mjs`: `temp/` altında yok sayılan bir Godot smoke projesi oluşturur ve derlenmiş editör GDExtension'ına karşı içe aktarılmış `PackedScene` incelemesini, salt okunur bağlam korumalarını, eksik kaynak hatasını ve kaydetmeme davranışını doğrular.

<a id="documentation-localization"></a>
## Belge Yerelleştirmesi

- `sync-doc-navigation.mjs`: düzyazıyı çevirmeden kaynak karmaları, sabit bağlantı kimlikleri ve aynı sayfaya yönelik kompakt dil seçici ekler.
- `check-doc-i18n.mjs`: eksiksiz dil kapsamını, kaynak güncelliğini, gezinmeyi, bağlantı kimliklerini, Markdown yapısını, korunan kodu, URL'leri ve bağlantıları doğrular.
- `doc-i18n-lib.mjs`: paylaşımlı dil manifestosunun, kaynak normalleştirmenin, gezinme işlemenin ve yapısal yardımcıların sahibidir.

Şunları çalıştırın:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Dil ve belge kümesi `docs/i18n/languages.json` içinde bildirilir. İngilizce temel kaynak olarak kalır. Çevrilmiş düzyazı bu betikler tarafından oluşturulmamalı, İngilizce kaynaktan yazılmalıdır.

Normal eşitleme gezinmeyi ve sabit bağlantı kimliklerini günceller, ancak mevcut
kaynak karmalarını korur. Değişen bir İngilizce sayfanın dokuz çevirisinin
tamamını doğrudan güncelledikten sonra yalnızca o kaynağı bilinçli olarak
yenileyin:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

Bu seçenek, incelenen birkaç kaynak için yinelenebilir. Çevrilmiş düzyazısı
güncellenmemiş bir kaynağı kabul etmeyin. CI, tam çeviri doğrulayıcısından önce
`sync-doc-navigation.mjs --check` komutunu çalıştırır.

<a id="ui-sync"></a>
## Kullanıcı Arayüzü Eşitlemesi

- `sync-chat-ui.mjs`: `ui/chat/` içeriğini `godot_demo/addons/fennara/dist/` içine kopyalar.

`godot_demo/addons/fennara/dist/` bilerek commit edilir, çünkü yayımlanan eklenti zip dosyaları derlenmiş sohbet web görünümünü içermelidir. Değişiklikleri `ui/chat/` içinde yapın, eşitleme betiğini çalıştırın ve kaynak ile oluşturulan eklenti varlıklarını birlikte commit edin.

<a id="runtime-sync"></a>
## Çalışma Zamanı Eşitlemesi

- `sync-runtime.mjs`: `runtime/` içeriğini `godot_demo/addons/fennara/runtime/` içine kopyalar.

`godot_demo/addons/fennara/runtime/` bilerek commit edilir, çünkü yayımlanan eklenti zip dosyaları Godot tarafı çalışma zamanı yardımcı betiklerini içermelidir. Değişiklikleri `runtime/` içinde yapın, eşitleme betiğini çalıştırın ve kaynak ile oluşturulan eklenti varlıklarını birlikte commit edin.

<a id="guidance-sync"></a>
## Yönerge Eşitlemesi

- `sync-guidance.mjs`: kompakt yönergeleri ve isteğe bağlı bilgi sayfalarını `local/templates/` içinden `godot_demo/addons/fennara/ai/` içine kopyalar, böylece `fennara install` ve `fennara update` komutlarının kullanıcı projelerine yazdığı dosyalarla eşleşir.

`godot_demo/addons/fennara/ai/` bilerek commit edilir, çünkü demo eklentisi kurulu eklenti düzenini yansıtır. Değişiklikleri `local/templates/` içinde yapın, eşitleme betiğini çalıştırın ve kaynak ile oluşturulan eklenti yönergelerini birlikte commit edin.

<a id="boundaries"></a>
## Sınırlar

- Betikler `.package-preview/` ve kök `dist/` çıktılarını oluşturabilir.
- Betikler commit edilmiş oluşturulmuş veri yüklerini yalnızca `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs` veya `set-version.mjs` gibi açık işleri bu olduğunda güncelleyebilir.
- Betikler Godot editörü önbelleğini, yerel uygulama verileri kurulumlarını, indirilmiş sürüm eserlerini veya VM test çıktılarını izlenen kaynak klasörlerine yazmamalıdır.
