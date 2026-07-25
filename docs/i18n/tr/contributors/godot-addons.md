<!-- fennara-i18n: locale=tr source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Godot Eklentileri

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Bu dizin Godot'nun bir projede beklediği biçimi yansıtır:

```text
res://addons/
  fennara/
```

Depo veri yükünü `godot_demo/addons/` altında tutmak, paketleme ve yerel test betiklerinin yolları yeniden biçimlendirmeden eklentiyi bir projeye kopyalamasını sağlar.

<a id="current-addon"></a>
## Geçerli Eklenti

`fennara/`, kurulabilir Fennara Godot AI eklentisidir. Şunları içerir:

- Yerel uzantının Godot giriş noktası olan `fennara.gdextension`.
- `fennara-cpp/` kaynağından oluşturulan platform editörü ikililerini içeren `bin/`.
- `ui/chat/` kaynağından eşitlenen, oluşturulmuş yerel sohbet web görünümü varlıklarını içeren `dist/`.
- Depo kökündeki `runtime/` kaynağından eşitlenen Godot tarafı yardımcı betiklerini içeren `runtime/`.
- Hata ayıklayıcıya yönelik eklenti varlıklarını içeren `debugger/`.
- Paketlenmiş eklenti sürümü işaretçisi olan `VERSION`.

<a id="rules"></a>
## Kurallar

- Eklentiye göre yolları kararlı tutun. Kullanıcı projeleri bu klasörü `res://addons/fennara/` olarak alır.
- Paket önizleme zip dosyalarını, sürüm zip dosyalarını, indirilmiş CEF arşivlerini, günlükleri veya yerel test çıktılarını buraya koymayın.
- Oluşturulmuş `fennara/dist/` web görünümü dosyalarını, oluşturulmuş çıktıyı bilerek düzeltip kaynak değişikliğini de eşitlemediğiniz sürece elle düzenlemeyin.
- Eşitlenmiş `fennara/runtime/` çalışma zamanı yardımcısı dosyalarını `runtime/` kaynağını da güncellemeden ve `node scripts/sync-runtime.mjs` çalıştırmadan elle düzenlemeyin.
- Yeni eklenti veri yüklerini buraya yalnızca Godot projelerine kopyalanmaları amaçlanıyorsa ekleyin.
