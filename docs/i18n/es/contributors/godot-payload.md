<!-- fennara-i18n: locale=es source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Paquete de Godot

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Este directorio es el árbol fuente del addon orientado a Godot que se copia en los proyectos y se incluye en los archivos de las versiones.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` debe poder instalarse como un directorio normal de
addon. Todo lo incluido aquí debe poder llegar directamente a
`res://addons/fennara/` en un proyecto de usuario.

<a id="what-belongs-here"></a>
## Qué debe incluirse

- `addons/fennara/fennara.gdextension` y archivos `.uid` que carga Godot.
- Binarios GDExtension de editor en `addons/fennara/bin/`.
- Recursos generados del chat web en `addons/fennara/dist/`.
- Auxiliares de ejecución sincronizados desde `runtime/` en `addons/fennara/runtime/`.
- `addons/fennara/VERSION`, coincidente con `VERSION` al empaquetar.

<a id="what-does-not-belong-here"></a>
## Qué no debe incluirse

- Estado local como `.godot/`, `.import/`, registros, temporales o cachés.
- Resultados de empaquetado. Pertenecen a directorios ignorados como `dist/` o `.package-preview/`.
- Runtimes locales compartidos, como el daemon, MCP o CEF. La CLI los instala en los datos de aplicación, no en cada addon.

<a id="generated-files"></a>
## Archivos generados

Tras cambiar `ui/chat/`, ejecuta:

```powershell
node scripts\sync-chat-ui.mjs
```

Esto sincroniza la vista web con `godot_demo/addons/fennara/dist/`, que se
incluye en Git para que los usuarios no necesiten Node.js ni compilar la interfaz.

Tras cambiar `runtime/`, ejecuta:

```powershell
node scripts\sync-runtime.mjs
```

Esto sincroniza los auxiliares con `godot_demo/addons/fennara/runtime/`, que se
incluyen para que formen parte del ZIP publicado.
