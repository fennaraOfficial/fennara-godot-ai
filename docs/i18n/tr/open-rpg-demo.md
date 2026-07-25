<!-- fennara-i18n: locale=tr source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Open RPG Demo Dökümü

<!-- fennara-doc-nav:start -->
[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

Video:

https://www.youtube.com/watch?v=0Egu3S-9MM0

Bu demo Fennara MCP'yi GDQuest'in açık kaynaklı Godot 4 Open RPG projesinde sınar.

Demonun amacı bir yapay zekanın sıfırdan boş bir proje oluşturması değildir. Amaç, bir yapay zeka ajanının mevcut bir Godot RPG kod tabanında çalışması, hatalar yapması, Godot'dan geri bildirim alması, uygulamayı düzeltmesi ve devam etmesidir.

<a id="project"></a>
## Proje

GDQuest Godot 4 Open RPG:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## Görev

Ayı oyuncu savaşçı Baloo'nun mevcut bir karşılaşmayı kazandıktan sonra Tactical Guard adlı yeni bir savaş yeteneğinin kilidini açtığı bir ilerleme özelliği eklemek.

Yeteneğin şunları yapması gerekiyordu:

- bir düşmanı hedefleme
- makul düzeyde hasar verme
- Baloo'nun Defense değerini yükseltme
- kilit açıldıktan sonra Baloo'nun savaş eylemi menüsünde görünme
- kilit açıldıktan sonra `Baloo learned Tactical Guard!` gibi bir mesaj gösterme

<a id="what-happened"></a>
## Neler Oldu

Bir yapay zeka kodlama ajanı, Fennara MCP üzerinden canlı Godot projesine bağlandı ve proje mimarisini inceledi.

Fennara araçlarını şunlar için kullandı:

- sahne ağacı incelemesi
- düğüm özelliği incelemesi
- GDScript tanılamaları
- sahne doğrulaması
- çalışma zamanı hatası geri bildirimi
- proje ve sahne incelemesi

İlk uygulama kusursuz çalışmadı. Yararlı olan kısım buydu.

Fennara Godot'dan geri bildirim döndürdü, ajan bozuk betiği düzeltti, uygulamayı ayarladı ve özellik oyun içinde çalışana kadar devam etti.

<a id="why-this-matters"></a>
## Bu Neden Önemli

Boş demolar kolaydır. Yapay zeka ajanlarının genellikle bozulduğu yer mevcut projelerdir.

Fennara'nın tezi, Godot yapay zeka ajanlarının motor geri bildirimine ihtiyaç duyduğudur:

- Betik ayrıştırıldı mı?
- Sahne doğrulandı mı?
- Çalışma zamanı hata verdi mi?
- Ajan gerçek proje yapısını inceledi mi?
- Ajan görev bitmiş gibi davranmak yerine hatayı düzeltebilir mi?

Geleneksel MCP, yapay zekaya komutlar verir.

Fennara, yapay zekaya Godot'dan geri bildirim verir.
