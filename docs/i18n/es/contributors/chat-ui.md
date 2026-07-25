<!-- fennara-i18n: locale=es source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Interfaz de chat de Fennara

<!-- fennara-doc-nav:start -->
[Documentación](../README.md)

🌐 **Idiomas**

[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

Esta carpeta contiene la fuente de la superficie opcional de chat dentro del editor.

La primera versión no requiere compilación deliberadamente: HTML, CSS y
JavaScript simples. Esto facilita inspeccionar el repositorio y evita una cadena
de herramientas frontend antes de estabilizar el host y el puente del daemon.

La copia empaquetada está en `godot_demo/addons/fennara/dist/`.

Después de editar, ejecuta:

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## Notas de diseño

- Reproducir las superficies del editor: controles compactos, contraste discreto, radios pequeños, foco claro y sin presentación publicitaria.
- Utilizar únicamente API locales del daemon y el chat, sin servicios alojados obligatorios.
- OpenRouter debe usar una clave del usuario guardada localmente fuera del proyecto.
- La interfaz debe seguir siendo útil sin conexión a un modelo: estado, configuración, transcripción y cuadro de redacción deben permanecer visibles.
