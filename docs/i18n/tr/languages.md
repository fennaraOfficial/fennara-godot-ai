<!-- fennara-i18n: locale=tr source=docs/languages.md sha256=29ca1071b436e0ff29fa5d18d9e2b09cbe64749513ea7f4e1e6471569fcb6456 -->
<a id="languages-and-translation-status"></a>
# Diller ve Çeviri Durumu

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../languages.md)
<!-- fennara-doc-nav:end -->

İngilizce, belgelerin temel alınan kaynağıdır. Fennara ayrıca dokuz dilde yapay zeka tarafından yazılmış eksiksiz çeviriler sunar. Çevrilen her sayfa güncel İngilizce kaynağına bağlantı verir ve ana dil konuşurlarını incelemeye davet eder.

| Dil | Belgeler | Kapsam | İnceleme durumu |
| --- | --- | --- | --- |
| English | [İngilizce belgeler](../../README.md) | 30/30 | Temel kaynak |
| 简体中文 | [Basitleştirilmiş Çince belgeler](../zh-CN/README.md) | 30/30 | Ana dil incelemesi istendi |
| Español | [İspanyolca belgeler](../es/README.md) | 30/30 | Ana dil incelemesi istendi |
| Português do Brasil | [Brezilya Portekizcesi belgeler](../pt-BR/README.md) | 30/30 | Ana dil incelemesi istendi |
| 日本語 | [Japonca belgeler](../ja/README.md) | 30/30 | Ana dil incelemesi istendi |
| 한국어 | [Korece belgeler](../ko/README.md) | 30/30 | Ana dil incelemesi istendi |
| Русский | [Rusça belgeler](../ru/README.md) | 30/30 | Ana dil incelemesi istendi |
| Français | [Fransızca belgeler](../fr/README.md) | 30/30 | Ana dil incelemesi istendi |
| Deutsch | [Almanca belgeler](../de/README.md) | 30/30 | Ana dil incelemesi istendi |
| Türkçe | [Türkçe belgeler](README.md) | 30/30 | Ana dil incelemesi istendi |

<a id="what-is-translated"></a>
## Neler Çevrilir

Çeviri kümesi ana README'yi, doğrudan `docs/` altındaki tüm sayfaları, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md` dosyalarını ve katkıda bulunanlara yönelik altı alt sistem README'sini içerir.

Hukuki metinler, üçüncü taraf bildirimleri, issue şablonları, dahili ajan talimatları, oluşturulan proje yönergeleri, test sabitleri ve satıcı belgeleri yetkili biçimlerinde kalır. Oluşturulan veya davranış içeren dosyalar bağımsız çeviri kaynakları değildir.

<a id="freshness-and-validation"></a>
## Güncellik ve Doğrulama

Çevrilen her sayfa temel alınan kaynak yolunu ve kaynak karmasını kaydeder. Gezinme tek bir dil manifestosundan oluşturulur ve sabit İngilizce bağlantı kimliği diğer adları, başlıklar çevrildiğinde derin bağlantıların çalışmasını sürdürür.

Şunları çalıştırın:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Bu araçlar düzyazıyı çevirmez. Yalnızca gezinme meta verilerini korur ve kapsamı, güncelliği, Markdown yapısını, komutları, bağlantıları, bağlantı kimliklerini, kod bloklarını, tabloları ve URL'leri denetler. Ana dil konuşurlarının düzeltmeleri normal pull request'ler üzerinden memnuniyetle karşılanır.

Normal eşitleme mevcut kaynak karmalarını korur. Bu nedenle İngilizce
düzyazıdaki bir değişiklik, çeviriler doğrudan güncellenene kadar onları güncel
olmayan durumda bırakır. Değişen bir İngilizce sayfanın dokuz çevirisinin
tamamını inceledikten sonra yalnızca o kaynağı kabul edin:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI, yapısal doğrulamadan önce gezinme eşitlemesini denetim kipinde çalıştırır.
Bu işlem ayrıca her sabit İngilizce bağlantı kimliğinin karşılık gelen çevrilmiş
başlığa bağlı kalmasını doğrular.
