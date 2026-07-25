<!-- fennara-i18n: locale=tr source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# Katkıda Bulunma

<!-- fennara-doc-nav:start -->
[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Fennara Godot AI'ı geliştirmeye yardımcı olduğunuz için teşekkür ederiz.

<a id="good-contributions"></a>
## İyi Katkılar

- Belge düzeltmeleri
- Yeniden üretilebilir hata düzeltmeleri
- Platform uyumluluğu düzeltmeleri
- Derleme ve paketleme iyileştirmeleri
- Kurulumun anlaşılırlığına yönelik küçük iyileştirmeler

<a id="design-discussion-required"></a>
## Tasarım Tartışması Gerektirenler

Şunlara başlamadan önce bir issue veya discussion açın:

- yeni MCP araçları
- araç şeması değişiklikleri
- sürüm iş akışı değişiklikleri
- büyük mimari değişiklikler
- oluşturulan proje yönergelerini etkileyen değişiklikler

<a id="pull-requests"></a>
## Pull Request'ler

- Pull request'leri küçük ve odaklı tutun.
- Neyin, neden değiştiğini açıklayın.
- Değişikliği nasıl doğruladığınızı açıklayın.
- Görünür kullanıcı arayüzü veya belge işleme değişiklikleri için ekran görüntüleri ya da kayıtlar ekleyin.
- İlgisiz biçimlendirme veya temizlik değişiklikleri eklemeyin.
- Issue veya pull request'lere büyük, oluşturulmuş açıklamalar yapıştırmayın.

<a id="commit-and-pr-titles"></a>
## Commit ve PR Başlıkları

Conventional Commit biçimini kullanın:

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

Yaygın türler:

- `feat`: kullanıcıya yönelik özellik
- `fix`: hata düzeltmesi
- `docs`: belgeler
- `ci`: GitHub Actions ve otomasyon
- `build`: derleme veya paketleme
- `refactor`: davranışı koruyan kod yeniden yapılandırması
- `test`: testler
- `chore`: bakım

<a id="project-boundaries"></a>
## Proje Sınırları

Fennara oyundan bağımsız kalmalıdır. Bir oyunun kontrolleri, hedefleri, ekonomisi, envanteri, çatışması, yol bulması, görevleri veya kullanıcı arayüzü akışı hakkında varsayım yapan API'lerden ya da yönergelerden kaçının.

Ajanlar bir Godot projesinin gerçek sahnelerini, betiklerini, kaynaklarını, ayarlarını, çalışma zamanı durumunu, tanılamalarını ve ekran görüntülerini incelemeli, ardından o proje için genel Fennara araçlarını bir araya getirmelidir.

<a id="documentation-translations"></a>
## Belge Çevirileri

İngilizce, temel alınan kaynaktır. Önce İngilizceyi düzeltin, ardından etkilenen her dili güncelleyin. Çevrilen küme ve dil meta verileri `docs/i18n/languages.json` dosyasındadır.

- İngilizce sayfanın tamamını okuyun ve çeviriyi doğrudan yazın. Toplu makine çevirisi hizmetlerini veya düzyazı oluşturan betikleri kullanmayın.
- Kod bloklarını, satır içi kodu, komutları, yolları, yapılandırma anahtarlarını, URL'leri ve ürün adlarını olduğu gibi koruyun.
- Belge betiklerinin yönettiği kaynak işaretini ve açık İngilizce bağlantı kimliği diğer adlarını koruyun.
- Akıcı bir inceleyici denetlemedikçe çeviriyi ana dilde incelenmiş olarak işaretlemeyin.
- Hukuki metinleri, dahili ajan istemlerini, oluşturulan proje yönergelerini, satıcı dosyalarını veya test sabitlerini bağımsız kaynaklar olarak çevirmeyin.

Temel alınan veya çevrilmiş belgeleri değiştirdikten sonra şunları çalıştırın:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Bu komutlar gezinme meta verilerini korur ve yapıyı doğrular. Çevrilmiş düzyazı yazmazlar.

Normal gezinme eşitlemesi mevcut kaynak karmalarının tümünü korur. İngilizce bir
kaynağı değiştirdikten sonra o sayfayı doğrudan dokuz çevrilmiş dilin tamamında
güncelleyin, ardından yalnızca o temel kaynağı bilinçli olarak kabul edin:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Çevirileri incelenip güncellenen her İngilizce sayfa için
`--accept-source <path>` seçeneğini yineleyin. Dokuz çevirinin tamamı yeni anlamı
içermeden bir kaynak karmasını asla kabul etmeyin.
