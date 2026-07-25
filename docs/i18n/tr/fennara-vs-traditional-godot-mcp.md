<!-- fennara-i18n: locale=tr source=docs/fennara-vs-traditional-godot-mcp.md sha256=e45a741b1db7c20e40b4a311c198af216172dfa024ca9c123db4f9336c9a6e7f -->
<a id="fennara-vs-traditional-godot-mcp"></a>
# Fennara ile Geleneksel Godot MCP Karşılaştırması

<!-- fennara-doc-nav:start -->
[English](../../fennara-vs-traditional-godot-mcp.md) · [简体中文](../zh-CN/fennara-vs-traditional-godot-mcp.md) · [Español](../es/fennara-vs-traditional-godot-mcp.md) · [Português do Brasil](../pt-BR/fennara-vs-traditional-godot-mcp.md) · [日本語](../ja/fennara-vs-traditional-godot-mcp.md) · [한국어](../ko/fennara-vs-traditional-godot-mcp.md) · [Русский](../ru/fennara-vs-traditional-godot-mcp.md) · [Français](../fr/fennara-vs-traditional-godot-mcp.md) · [Deutsch](../de/fennara-vs-traditional-godot-mcp.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../fennara-vs-traditional-godot-mcp.md)
<!-- fennara-doc-nav:end -->

| Geleneksel komut köprüsü | Fennara geri bildirim döngüsü |
| --- | --- |
| Editör eylemlerini sunar | Godot bilgisine sahip inceleme, eylem ve denetimleri sunar |
| Başarılı bir komut akışın sonu olabilir | Tanılamalar, doğrulama, çalışma zamanı günlükleri ve ekran görüntüleri sonraki adımı yönlendirir |
| Doğrudan ve bilinen düzenlemeler için en iyisidir | Bir ajanın incelemesi, değiştirmesi, doğrulaması ve kurtarması gerektiğinde en iyisidir |

Çoğu Godot MCP sunucusu, editör komutlarını yapay zeka istemcilerine sunar.

Örnekler:

- düğüm oluşturma
- özellik ayarlama
- sahne açma
- sahne kaydetme
- günlükleri okuma
- ekran görüntüsü alma
- proje çalıştırma
- sinyal bağlama
- girdi haritasını düzenleme
- malzemeleri yönetme
- testleri çalıştırma

Bu yararlıdır. Godot'yu bir API yüzeyine dönüştürür.

Ancak gerçek yapay zeka oyun geliştirmesinde zor olan, bir yapay zekanın `set_property` çağrısı yapıp yapamaması değildir.

Zor olan, yapay zekanın projenin bozuk olduğunu anlayıp anlayamamasıdır.

<a id="traditional-mcp-pattern"></a>
## Geleneksel MCP Kalıbı

```text
Yapay zeka editör komutunu çağırır.
Editör sonucu döndürür.
Yapay zeka sonraki adımı tahmin eder.
```

Bu, küçük ve doğrudan düzenlemelerde iyi çalışır.

Örnek:

```text
Camera3D adını MainCamera olarak değiştir.
```

Ancak ajanın mimariyi incelemesi, betikleri/kaynakları/sahneleri düzenlemesi, hataları görmesi ve kurtarması gereken daha büyük proje görevlerinde daha zayıftır.

<a id="fennara-pattern"></a>
## Fennara Kalıbı

```text
Yapay zeka projeyi değiştirir.
Godot geri bildirimi gelir.
Yapay zeka çalışana kadar düzeltip yeniden çalıştırır.
```

Fennara geri bildirime odaklanır:

- GDScript tanılamaları
- sahne doğrulaması
- çalışma zamanı hataları
- sahne ağacı incelemesi
- düğüm özellikleri
- sınıf/API incelemesi
- ekran görüntüleri
- oluşturulan proje yönergeleri
- düzeltip yeniden çalıştırma iş akışları

<a id="the-difference"></a>
## Fark

Geleneksel Godot MCP şunu sorar:

```text
Hangi editör komutlarını sunmalıyız?
```

Fennara şunu sorar:

```text
Modelin Godot içinde başarılı biçimde geliştirme yapabilmesi için hangi geri bildirime ihtiyacı var?
```

Komutlar temel gereksinimdir.

Savunulabilir üstünlük geri bildirimdedir.
