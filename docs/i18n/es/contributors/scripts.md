<!-- fennara-i18n: locale=es source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# Scripts

<!-- fennara-doc-nav:start -->
[Documentación](../README.md)

🌐 **Idiomas**

[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

Este directorio contiene automatización del repositorio compartida por el desarrollo local, la vista previa de paquetes y los flujos de publicación.

Los scripts deben ser pequeños, deterministas y seguros de ejecutar desde la raíz del repositorio, a menos que su texto de ayuda indique lo contrario. No deben escribir estado específico del usuario fuera del repositorio.

<a id="version-scripts"></a>
## Scripts de versión

- `set-version.mjs`: actualiza el `VERSION` del repositorio, el `VERSION` del addon, los metadatos del workspace local de Rust, las versiones de paquetes del lockfile y la constante de versión del plugin de C++.
- `check-version.mjs`: verifica que esos archivos con versión sigan sincronizados.

Ejecuta `check-version.mjs` en CI y antes de empaquetar una publicación. Usa `set-version.mjs` solo cuando cambies deliberadamente la versión de Fennara.

<a id="packaging-scripts"></a>
## Scripts de empaquetado

- `package-preview.mjs`: sincroniza las cargas útiles confirmadas del addon y después ensambla archivos de vista previa por plataforma, una vez que la GDExtension y los binarios locales de Rust ya se han compilado.
- `package-addon-all.mjs`: combina las partes del addon por plataforma en el archivo final del addon para todas las plataformas.
- `release-policy.mjs`: define la CLI publicada compatible mínima para cada canal de publicación.
- `write-release-manifest.mjs`: escribe `fennara-release-manifest-v<version>.json` a partir de los recursos de publicación y valida cada SHA-256 al que se hace referencia.

Ambos scripts usan `.package-preview/` como preparación temporal y escriben los ZIP resultantes en la carpeta `dist/` de la raíz del repositorio. Esos resultados se ignoran y no deben confirmarse.

Los scripts de empaquetado deben mantener pequeña la carga útil del addon. En particular, los archivos del runtime CEF de Linux, como `libcef.so` y `fennara_cef_helper`, no deben incluirse dentro de `fennara-addon-*`; CEF se instala una vez en el directorio compartido de datos de aplicación de Fennara del usuario.

<a id="staging-release-scripts"></a>
## Scripts de publicaciones de staging

- `write-staging-candidate.mjs`: crea la identidad exacta de prelanzamiento para una solicitud de incorporación de cambios y un commit de origen congelado.
- `validate-staging-build.mjs`: comprueba las partes del addon, los archivos por plataforma, el addon ensamblado, el manifiesto de publicación y CEF para Linux antes de publicar.
- `smoke-public-release.mjs`: descarga cada candidato publicado mediante su URL de navegador sin autenticación y verifica los hashes de los recursos de confianza y del manifiesto antes de avanzar el canal.
- `write-staging-pointer.mjs`: escribe el pequeño puntero por solicitud de incorporación de cambios después de calcular el hash del manifiesto de publicación exacto.
- `check-staging-channel-advance.mjs`: rechaza el movimiento del canal hacia atrás o que entre en conflicto.
- `validate-staging-publish-bundle.mjs`: vuelve a validar el paquete final de artefactos sin ejecutar el código candidato.
- `verify-published-assets.mjs`: compara los nombres y valores SHA-256 esperados con los de los recursos descargados de GitHub Release.

Estos scripts respaldan `.github/workflows/staging-release.yml`. Los trabajos de compilación de candidatos se ejecutan sin credenciales de publicación. Solo el trabajo final de confianza puede publicar, y avanza la referencia de Git por canal después de que se haya descargado y verificado la publicación exacta.

<a id="linux-cef-scripts"></a>
## Scripts CEF para Linux

- `prepare-linux-cef-sdk.mjs`: descarga y extrae el SDK oficial y fijado de CEF para Linux x64 que se usa para compilar el puente CEF de Linux.
- `prepare-linux-cef-runtime.mjs`: prepara el ZIP independiente del runtime CEF de Linux, valida los archivos requeridos, elimina símbolos de los binarios ELF preparados en Linux y puede escribir el manifiesto generado `local/webview-runtimes/linux-cef.json` para el empaquetado de la publicación.
- `check-linux-cef-runtime-release.mjs`: valida que los recursos de publicación contengan el ZIP del runtime CEF nombrado por el manifiesto habilitado y que coincida su SHA-256.
- `cef/linux/fennara_cef_helper.cpp`: código fuente del pequeño proceso auxiliar CEF que se usa al compilar el auxiliar del runtime desde el SDK de CEF.

Los scripts de CEF operan únicamente sobre archivos copiados para preparación. No deben modificar el árbol descargado o de origen del SDK de CEF.

<a id="development-tests"></a>
## Pruebas de desarrollo

- `test-run-scene-edit-script-inspect.mjs`: crea un proyecto de prueba de Godot ignorado bajo `temp/` y verifica la inspección de un `PackedScene` importado, las protecciones de contexto de solo lectura, el fallo por una fuente ausente y el comportamiento sin guardado frente a una GDExtension de editor compilada.

<a id="documentation-localization"></a>
## Localización de documentación

- `sync-doc-navigation.mjs`: añade hashes de fuente, anclas estables y el selector compacto de idioma de la misma página sin traducir la prosa.
- `check-doc-i18n.mjs`: valida la cobertura completa de idiomas, la vigencia de la fuente, la navegación, las anclas, la estructura de Markdown, el código protegido, las URL y los enlaces.
- `doc-i18n-lib.mjs`: contiene el manifiesto de idiomas compartido, la normalización de la fuente, la representación de la navegación y los auxiliares estructurales.

Ejecuta:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

El conjunto de idiomas y documentos se declara en `docs/i18n/languages.json`. El inglés
sigue siendo canónico. La prosa traducida debe escribirse a partir de la fuente en inglés,
no debe ser generada por estos scripts.

La sincronización normal actualiza la navegación y las anclas estables, pero
conserva los hashes de fuente existentes. Después de actualizar directamente
las nueve traducciones de una página inglesa modificada, actualiza de forma
deliberada solo esa fuente:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

La opción puede repetirse para varias fuentes revisadas. No confirmes una fuente
cuya prosa traducida no se haya actualizado. CI ejecuta
`sync-doc-navigation.mjs --check` antes del validador completo de traducciones.

<a id="ui-sync"></a>
## Sincronización de la interfaz

- `sync-chat-ui.mjs`: copia `ui/chat/` en `godot_demo/addons/fennara/dist/`.

`godot_demo/addons/fennara/dist/` se confirma intencionalmente porque los ZIP publicados del addon deben contener la interfaz web de chat compilada. Haz los cambios en `ui/chat/`, ejecuta el script de sincronización y confirma juntos tanto la fuente como los recursos generados del addon.

<a id="runtime-sync"></a>
## Sincronización del runtime

- `sync-runtime.mjs`: copia `runtime/` en `godot_demo/addons/fennara/runtime/`.

`godot_demo/addons/fennara/runtime/` se confirma intencionalmente porque los ZIP publicados del addon deben contener los scripts auxiliares del runtime del lado de Godot. Haz los cambios en `runtime/`, ejecuta el script de sincronización y confirma juntos tanto la fuente como los recursos generados del addon.

<a id="guidance-sync"></a>
## Sincronización de instrucciones

- `sync-guidance.mjs`: copia las instrucciones compactas y las páginas de conocimiento bajo demanda de `local/templates/` en `godot_demo/addons/fennara/ai/`, de forma que coincidan con los archivos que `fennara install` y `fennara update` escriben en los proyectos de los usuarios.

`godot_demo/addons/fennara/ai/` se confirma intencionalmente porque el addon de demostración refleja la disposición de un addon instalado. Haz los cambios en `local/templates/`, ejecuta el script de sincronización y confirma juntos tanto la fuente como las instrucciones generadas del addon.

<a id="boundaries"></a>
## Límites

- Los scripts pueden crear resultados en `.package-preview/` y en el `dist/` de la raíz.
- Los scripts pueden actualizar cargas útiles generadas y confirmadas solo cuando esa sea su función explícita, como `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs` o `set-version.mjs`.
- Los scripts no deben escribir la caché del editor de Godot, instalaciones locales en datos de aplicación, artefactos de publicación descargados ni resultados de pruebas de VM en carpetas de fuente con seguimiento.
