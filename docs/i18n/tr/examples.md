<!-- fennara-i18n: locale=tr source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# Örnekler

<!-- fennara-doc-nav:start -->
[Belgeler](README.md)

🌐 **Diller**

[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../examples.md)
<!-- fennara-doc-nav:end -->

Bir istemi kopyalayın, proje ayrıntılarını değiştirin ve bir MCP uygulamasından veya yerleşik Fennara sohbetinden gönderin.

| Hedef | Örnek |
| --- | --- |
| Bağlı editörü doğrulama | [Bağlantıyı Denetleme](#check-connection) |
| Mevcut projeyi anlama | [Düzenlemeden Önce İnceleme](#inspect-a-project-before-editing) |
| Odaklı bir değişiklik yapma | [Mimarinin Farkında Olan Değişiklik](#make-a-small-architecture-aware-change) |
| Çalışan bir projeyi tanılama | [Çalışma Zamanı Hatası](#debug-a-runtime-error) |
| İşlenmiş çıktıyı inceleme | [Görsel Geri Bildirim](#visual-feedback) |

<a id="check-connection"></a>
## Bağlantıyı Denetleme

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## Bir Projeyi Düzenlemeden Önce İnceleme

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## Küçük, Mimarinin Farkında Olan Bir Değişiklik Yapma

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## Çalışma Zamanı Hatasında Hata Ayıklama

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## Görsel Geri Bildirim

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## Yerleşik Sohbet Sağlayıcısı Kurulumu

Godot içindeki Fennara panelinde:

```text
/provider
```

Bir bulut sağlayıcısı veya yerel sağlayıcı bağlayın.

Ardından:

```text
/model
```

Panelin kullanacağı modeli seçin.

<a id="existing-project-demo-prompt"></a>
## Mevcut Proje Demo İstemi

Open RPG demosunda kullanılan istemin türü şöyledir:

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
