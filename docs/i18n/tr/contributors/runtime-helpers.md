<!-- fennara-i18n: locale=tr source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Çalışma Zamanı Yardımcıları

<!-- fennara-doc-nav:start -->
[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Bu klasör, `runtime_session` ve `runtime_script` tarafından kullanılan Godot tarafı çalışma zamanı yardımcı betiklerinin kaynağıdır.

Paketlenmiş eklenti kopyası şu konumdadır:

```text
godot_demo/addons/fennara/runtime/
```

Buradaki dosyaları düzenledikten sonra şunu çalıştırın:

```bash
node scripts/sync-runtime.mjs
```

Çalışma zamanı betikleri bu yardımcıları kurulu bir Godot projesindeki `res://addons/fennara/runtime/` konumundan yüklemeye devam eder. Yardımcıları ilkel ve projeden bağımsız tutun: girdi, bekleme, düğüm anlık görüntüleri, yakalamalar, fizik sorguları ve sahne yaşam döngüsü desteği uygundur; oyuna özel hareket, çatışma, görev, envanter veya kullanıcı arayüzü akışı varsayımları uygun değildir.

`image_sheet.gd`, ekran görüntüsü betiği cephesi tarafından da kullanılır. Bileşimini belirlenimci, sahne, animasyon veya oynanış durumundan bağımsız tutun.
