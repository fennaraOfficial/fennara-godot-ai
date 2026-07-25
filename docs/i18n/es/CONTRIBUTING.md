<!-- fennara-i18n: locale=es source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# Contribuir

<!-- fennara-doc-nav:start -->
[Documentación](README.md)

🌐 **Idiomas**

[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · **Español** · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Gracias por ayudar a mejorar Fennara Godot AI.

<a id="good-contributions"></a>
## Buenas contribuciones

- Correcciones de documentación
- Correcciones de errores reproducibles
- Correcciones de compatibilidad entre plataformas
- Mejoras de compilación y empaquetado
- Pequeñas mejoras en la claridad de la configuración

<a id="design-discussion-required"></a>
## Cambios que requieren debate previo

Abre una incidencia o un debate antes de empezar a trabajar en:

- nuevas herramientas MCP
- cambios en los esquemas de las herramientas
- cambios en el flujo de publicación
- grandes cambios de arquitectura
- cambios que afecten a las instrucciones de proyecto generadas

<a id="pull-requests"></a>
## Solicitudes de incorporación de cambios

- Mantén las solicitudes pequeñas y centradas.
- Explica qué cambió y por qué.
- Explica cómo verificaste el cambio.
- Incluye capturas de pantalla o grabaciones para cambios visibles de interfaz o renderizado de documentación.
- No incluyas cambios de formato ni limpieza que no estén relacionados.
- No pegues descripciones extensas generadas en incidencias o solicitudes de incorporación de cambios.

<a id="commit-and-pr-titles"></a>
## Títulos de commits y solicitudes

Utiliza el estilo Conventional Commits:

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

Tipos habituales:

- `feat`: funcionalidad visible para el usuario
- `fix`: corrección de un error
- `docs`: documentación
- `ci`: GitHub Actions y automatización
- `build`: compilación o empaquetado
- `refactor`: reestructuración del código que conserva el comportamiento
- `test`: pruebas
- `chore`: mantenimiento

<a id="project-boundaries"></a>
## Límites del proyecto

Fennara debe ser independiente del juego. Evita API o instrucciones que presupongan los controles, objetivos, economía, inventario, combate, búsqueda de rutas, misiones o flujo de interfaz de un juego.

Los agentes deben inspeccionar las escenas, scripts, recursos, ajustes, estado de ejecución, diagnósticos y capturas de pantalla reales de un proyecto de Godot, y después combinar las herramientas genéricas de Fennara para ese proyecto.

<a id="documentation-translations"></a>
## Traducciones de la documentación

El inglés es la fuente canónica. Corrige primero el inglés y después actualiza
todos los idiomas afectados. El conjunto traducido y sus metadatos se encuentran
en `docs/i18n/languages.json`.

- Lee la página completa en inglés y escribe directamente la traducción. No utilices servicios masivos de traducción automática ni scripts que generen prosa.
- Conserva exactamente bloques de código, código en línea, comandos, rutas, claves de configuración, URL y nombres de productos.
- Conserva el marcador de origen y los alias explícitos de anclas inglesas que administran los scripts.
- No marques una traducción como revisada por un hablante nativo salvo que la haya comprobado una persona con dominio del idioma.
- No traduzcas de forma independiente textos legales, prompts internos de agentes, instrucciones de proyecto generadas, archivos de terceros ni datos de prueba.

Después de cambiar documentación canónica o traducida, ejecuta:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Estos comandos mantienen los metadatos de navegación y validan la estructura. No
escriben prosa traducida.

La sincronización normal de la navegación conserva todos los hashes de fuente
existentes. Después de cambiar una fuente en inglés, actualiza directamente esa
página en los nueve idiomas traducidos y, a continuación, confirma de forma
deliberada solo esa fuente canónica:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Repite `--accept-source <path>` para cada página en inglés cuyas traducciones se
hayan revisado y actualizado. Nunca aceptes el hash de una fuente antes de que
las nueve traducciones contengan el nuevo significado.
