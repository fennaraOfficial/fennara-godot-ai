<!-- fennara-i18n: locale=es source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Auxiliares de ejecución

<!-- fennara-doc-nav:start -->
[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Esta carpeta contiene la fuente de los scripts auxiliares de Godot utilizados por `runtime_session` y `runtime_script`.

La copia empaquetada está en:

```text
godot_demo/addons/fennara/runtime/
```

Después de editar, ejecuta:

```bash
node scripts/sync-runtime.mjs
```

Los scripts cargan estos auxiliares desde `res://addons/fennara/runtime/`.
Mantenlos primitivos e independientes del proyecto. Entrada, esperas,
instantáneas de nodos, capturas, consultas físicas y ciclo de vida de escenas
son apropiados. No lo son las suposiciones sobre movimiento, combate, misiones,
inventario o flujo de interfaz de un juego.

`image_sheet.gd` también lo utiliza la fachada de capturas. Mantén su composición determinista e independiente de la escena, animación o jugabilidad.
