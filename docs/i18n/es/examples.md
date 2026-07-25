<!-- fennara-i18n: locale=es source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# Ejemplos

<!-- fennara-doc-nav:start -->
[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · **Español** · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../examples.md)
<!-- fennara-doc-nav:end -->

Copia un prompt, sustituye los datos del proyecto y envíalo desde una aplicación
MCP o desde el chat integrado de Fennara.

| Objetivo | Ejemplo |
| --- | --- |
| Confirmar el editor conectado | [Comprobar la conexión](#comprobar-la-conexión) |
| Comprender un proyecto existente | [Inspeccionar antes de editar](#inspeccionar-un-proyecto-antes-de-editar) |
| Realizar un cambio específico | [Cambio consciente de la arquitectura](#realizar-un-pequeño-cambio-consciente-de-la-arquitectura) |
| Diagnosticar un proyecto en ejecución | [Error de ejecución](#depurar-un-error-de-ejecución) |
| Inspeccionar el resultado renderizado | [Información visual](#información-visual) |

<a id="check-connection"></a>
## Comprobar la conexión

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## Inspeccionar un proyecto antes de editar

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## Realizar un pequeño cambio consciente de la arquitectura

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## Depurar un error de ejecución

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## Información visual

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## Configurar el proveedor del chat integrado

En el panel de Fennara dentro de Godot:

```text
/provider
```

Conecta un proveedor en la nube o local.

Después:

```text
/model
```

Elige el modelo que debe utilizar el panel.

<a id="existing-project-demo-prompt"></a>
## Prompt de demostración para un proyecto existente

Este es el tipo de prompt utilizado para la demostración de Open RPG:

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
