<!-- fennara-i18n: locale=tr source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Fennara Sohbet Kullanıcı Arayüzü

<!-- fennara-doc-nav:start -->
[Belgeler](../README.md)

🌐 **Diller**

[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

Bu klasör isteğe bağlı editör içi sohbet yüzeyinin kaynağını içerir.

İlk sürüm bilerek derlemesizdir: düz HTML, CSS ve JavaScript. Bu, OSS deposunu incelemeyi kolaylaştırır ve web görünümü barındırıcısı ile daemon sohbet köprüsü netleşmeden önce bir ön uç araç zinciri eklemekten kaçınır.

Paketlenmiş kopya `godot_demo/addons/fennara/dist/` konumundadır.

Bu klasörü düzenledikten sonra şunu çalıştırın:

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## Tasarım Notları

- Godot editörü yüzeyleriyle eşleşin: kompakt denetimler, sakin karşıtlık, küçük yarıçaplar, belirgin odak durumları ve pazarlama tarzı kahraman alanı kullanılmaması.
- Yalnızca yerel Fennara daemon/sohbet API'lerini kullanın; barındırılan hizmetler gerektirmeyin.
- OpenRouter desteği, Godot projesinin dışında yerel olarak depolanan ve kullanıcı tarafından sağlanan bir anahtar kullanmalıdır.
- Kullanıcı arayüzünü model bağlantısı olmadan da yararlı tutun: durum, ayarlar, konuşma dökümü ve düzenleyici durumları görünür kalmalıdır.
