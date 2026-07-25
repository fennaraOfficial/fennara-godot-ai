<!-- fennara-i18n: locale=tr source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Godot Veri Yükü

<!-- fennara-doc-nav:start -->
[Belgeler](../README.md)

🌐 **Diller**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Bu dizin, kullanıcı projelerine kopyalanan ve sürüm arşivlerinde paketlenen, Godot tarafındaki eklenti veri yükünün kaynak ağacıdır.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` normal bir Godot eklentisi dizini olarak kurulabilir kalmalıdır. Buraya commit edilen her şey, bir kullanıcı projesinin doğrudan `res://addons/fennara/` altında alabileceği bir şey olmalıdır.

<a id="what-belongs-here"></a>
## Buraya Ait Olanlar

- Godot'nun yüklediği `addons/fennara/fennara.gdextension` ve `.uid` dosyaları.
- Platform derlemelerinin ürettiği `addons/fennara/bin/` editör GDExtension ikilileri.
- Yerel sohbet web görünümünün kullandığı, oluşturulmuş `addons/fennara/dist/` web sohbeti varlıkları.
- `runtime/` kaynağından eşitlenmiş `addons/fennara/runtime/` Godot tarafı çalışma zamanı yardımcı betikleri.
- Paketleme sırasında depo `VERSION` dosyasıyla eşleşen `addons/fennara/VERSION`.

<a id="what-does-not-belong-here"></a>
## Buraya Ait Olmayanlar

- `.godot/`, `.import/`, günlükler, geçici dosyalar veya editör önbellekleri gibi yerel Godot kullanıcı durumu.
- İş akışlarından gelen kök paket çıktıları. Bunlar `dist/` veya `.package-preview/` gibi yok sayılan derleme klasörlerinde bulunur.
- Fennara daemon/MCP yürütülebilir dosyaları veya Linux CEF çalışma zamanı gibi paylaşımlı yerel çalışma zamanı veri yükleri. Bunlar her Godot proje eklentisine kopyalanmak yerine CLI tarafından kullanıcının Fennara uygulama verileri dizinine kurulur.

<a id="generated-files"></a>
## Oluşturulan Dosyalar

Sohbet kullanıcı arayüzü kaynağı `ui/chat/` altındadır. Değiştirdikten sonra şunu çalıştırın:

```powershell
node scripts\sync-chat-ui.mjs
```

Bu, oluşturulmuş web görünümü dosyalarını `godot_demo/addons/fennara/dist/` içine eşitler. Bu dizin bilerek commit edilir, çünkü eklenti kullanıcıları Node.js veya ön uç derleme adımına ihtiyaç duymamalıdır.

Çalışma zamanı yardımcısı kaynağı `runtime/` altındadır. Değiştirdikten sonra şunu çalıştırın:

```powershell
node scripts\sync-runtime.mjs
```

Bu, Godot tarafındaki çalışma zamanı yardımcılarını `godot_demo/addons/fennara/runtime/` içine eşitler. Bu dizin bilerek commit edilir, çünkü eklenti kullanıcıları bu betikleri sürüm zip dosyasıyla almalıdır.
