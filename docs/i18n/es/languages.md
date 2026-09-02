<!-- fennara-i18n: locale=es source=docs/languages.md sha256=476e4afb5a17f76474a25864d08280c73ba0e4fdd1ebcfbbfd814d010dfac932 -->
<a id="languages-and-translation-status"></a>
# Idiomas y estado de las traducciones

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · **Español** · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../languages.md)
<!-- fennara-doc-nav:end -->

El inglés es la fuente canónica de la documentación. Fennara también ofrece
traducciones completas, escritas por IA, en nueve idiomas. Cada página traducida
incluye un enlace a su fuente actual en inglés e invita a hablantes nativos a
revisarla.

| Idioma | Documentación | Cobertura | Estado de revisión |
| --- | --- | --- | --- |
| Inglés | [Documentación en inglés](../../README.md) | 31/31 | Canónica |
| 简体中文 | [简体中文文档](../zh-CN/README.md) | 31/31 | Se solicita revisión nativa |
| Español | [Documentación en español](README.md) | 31/31 | Se solicita revisión nativa |
| Português do Brasil | [Documentação em português](../pt-BR/README.md) | 31/31 | Se solicita revisión nativa |
| 日本語 | [日本語ドキュメント](../ja/README.md) | 31/31 | Se solicita revisión nativa |
| 한국어 | [한국어 문서](../ko/README.md) | 31/31 | Se solicita revisión nativa |
| Русский | [Документация на русском](../ru/README.md) | 31/31 | Se solicita revisión nativa |
| Français | [Documentation en français](../fr/README.md) | 31/31 | Se solicita revisión nativa |
| Deutsch | [Deutsche Dokumentation](../de/README.md) | 31/31 | Se solicita revisión nativa |
| Türkçe | [Türkçe belgeler](../tr/README.md) | 31/31 | Se solicita revisión nativa |

<a id="what-is-translated"></a>
## Contenido traducido

El conjunto traducido contiene el README principal, todas las páginas que se
encuentran directamente bajo `docs/`, `CONTRIBUTING.md`, `CONTEXT.md`,
`SECURITY.md` y los seis README de subsistemas destinados a colaboradores.

Los textos legales, avisos de terceros, plantillas de incidencias, instrucciones
internas de agentes, instrucciones de proyecto generadas, datos de prueba y
documentación de terceros permanecen en su forma autoritativa. Los archivos
generados o que definen comportamiento no son fuentes de traducción independientes.

<a id="freshness-and-validation"></a>
## Actualidad y validación

Cada página traducida registra la ruta y el hash de su fuente canónica. La
navegación se genera a partir de un único manifiesto de idiomas, y los alias
estables de anclas inglesas permiten que los enlaces profundos sigan funcionando
aunque se traduzcan los encabezados.

Ejecuta:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Estas herramientas no traducen prosa. Solo mantienen los metadatos de navegación
y comprueban la cobertura, actualidad, estructura Markdown, comandos, enlaces,
anclas, bloques de código, tablas y URL. Las correcciones de hablantes nativos
son bienvenidas mediante solicitudes de incorporación de cambios normales.

La sincronización normal conserva los hashes de fuente existentes, por lo que
un cambio en la prosa inglesa deja sus traducciones obsoletas hasta que se
actualizan directamente. Después de revisar las nueve traducciones de una
página inglesa modificada, confirma únicamente esa fuente:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI ejecuta la sincronización de navegación en modo de comprobación antes de la
validación estructural, que también verifica que cada ancla inglesa estable
permanezca unida al encabezado traducido correspondiente.
