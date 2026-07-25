<!-- fennara-i18n: locale=es source=docs/release.md sha256=23e966649d82728c9ab9802dd2957bb64c1b13fd3e7ae44bd513835bc063ff77 -->
<a id="release-process"></a>
# Proceso de publicación

<!-- fennara-doc-nav:start -->
[Documentación](README.md)

🌐 **Idiomas**

[English](../../release.md) · [简体中文](../zh-CN/release.md) · **Español** · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../release.md)
<!-- fennara-doc-nav:end -->

Las publicaciones son manuales. No publiques desde flujos de solicitudes de incorporación de cambios.

> [!IMPORTANT]
> Ejecuta las publicaciones desde `main`, mantén idénticos `VERSION` y la entrada
> del flujo, y decide explícitamente si la versión requiere aumentar la versión
> mínima de la CLI.

<a id="release-at-a-glance"></a>
## Resumen de una publicación

| Paso | Resultado |
| --- | --- |
| Preparar e integrar el cambio de versión | Las fuentes de versión coinciden |
| Ejecutar Package Preview | Se crean recursos con forma de publicación sin publicar |
| Inspeccionar la prueba | Se verifican archivos, manifiesto, hashes y disposición de CEF |
| Ejecutar Release desde `main` | Se publican la etiqueta y la versión de GitHub |
| Probar instalación y actualización | Se verifica el flujo público del usuario |

<a id="versioning"></a>
## Versionado

`VERSION` es la fuente canónica.

Las herramientas aceptan SemVer. Las versiones estables usan `X.Y.Z`. Los
candidatos de staging usan una versión previa aislada por PR, como
`1.2.3-pr.101.2`, donde `pr-101` es el canal y `2` su número de candidato.

Para aumentar la versión:

```bash
node scripts/set-version.mjs X.Y.Z
```

El script actualiza:

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- constantes de versión del plugin
- versión del workspace Rust bajo `local/`
- `local/Cargo.lock`

El addon también contiene `addons/fennara/release.json`. El comando normal
escribe automáticamente la identidad estable. Un workspace de staging usa:

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

La versión, canal, commit y etiqueta exacta deben coincidir. Se rechaza un addon
de versión previa sin esta identidad. Los addons estables anteriores a
`release.json` siguen usando el canal estable.

Comprueba la sincronización:

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. Preparar el commit de publicación

1. Ejecuta el script de versión.
2. Revisa el diff.
3. Ejecuta comprobaciones locales acordes con los cambios.
4. Integra la PR de preparación en `main`.

Comprobaciones habituales:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

Para cambios de GDExtension, compila también el addon localmente cuando sea posible:

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Ejecutar Package Preview

Utilízalo antes de publicar cuando cambie el empaquetado o necesites una prueba.

GitHub:

```text
Actions > Package Preview > Run workflow
```

El flujo compila paquetes de Windows, Linux y macOS y sube recursos temporales.
No crea etiquetas, versiones de GitHub ni `latest`.

Package Preview reproduce las partes no publicadoras de Release con la fidelidad
suficiente para ejercitar el empaquetado de la versión antes de integrarla:

- sincroniza la interfaz de chat sin compilación y el código fuente de los auxiliares del runtime con el paquete del addon
- crea el ZIP del runtime CEF de Linux
- escribe el manifiesto generado del runtime CEF de Linux
- entrega ese manifiesto generado a las compilaciones de paquetes por plataforma
- ensambla el archivo del addon para todas las plataformas
- cambia el nombre de los paquetes locales y del addon a los nombres de recursos de publicación administrados por el manifiesto
- valida el recurso del runtime CEF de Linux con el manifiesto generado
- escribe `fennara-release-manifest-v<version>.json`
- sube un artefacto `fennara-package-preview-release-assets` que contiene los ZIP y el manifiesto con la misma forma que una publicación

Los artefactos de vista previa son útiles para comprobar el contenido de los ZIP
y la forma del manifiesto antes de publicar. Son artefactos de Actions, no
recursos públicos de una versión.

<a id="3-run-release"></a>
## 3. Ejecutar Release

Ejecuta el flujo manual desde `main`:

```text
Actions > Release > Run workflow
```

Entradas:

```text
version: X.Y.Z
promote_latest: true
```

`version` debe coincidir con `VERSION`.

El flujo publica:

- `v<version>`
- marca `v<version>` como GitHub Latest si `promote_latest` es true

El flujo de publicación prepara el runtime CEF de Linux antes del empaquetado
por plataforma. Descarga el SDK mínimo oficial CEF 139 para Linux, monta el ZIP
independiente
`fennara-webview-cef-linux-x64-<cef-version>.zip`, elimina símbolos de los ELF
preparados, escribe un manifiesto `local/webview-runtimes/linux-cef.json`
generado y habilitado, y entrega ese manifiesto a los paquetes de la CLI. El
trabajo de publicación valida después que los recursos de la versión incluyan
el ZIP de CEF exacto nombrado por el manifiesto generado y que coincida su
SHA-256. También escribe `fennara-release-manifest-v<version>.json`, valida cada
recurso y hash al que se hace referencia y sube el manifiesto junto con la
versión.

Los flujos de solicitudes de incorporación de cambios no publican versiones.
Package Preview crea artefactos de prueba con forma de publicación, incluidos
el manifiesto y la carga útil del runtime CEF de Linux, para que los responsables
puedan probar el empaquetado antes de integrar. Package Preview no es el canal de
publicación dirigido a los usuarios.

<a id="release-assets"></a>
## Recursos de una versión

Cada versión debe contener paquetes de la CLI y del runtime local por plataforma,
además de un paquete compartido del addon para todas las plataformas.

| Destino | Recursos |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| Todas las plataformas | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Funciones de los paquetes:

| Patrón | Función |
| --- | --- |
| `fennara-cli-*` | Carga útil del script de instalación que contiene únicamente la CLI `fennara` para una plataforma |
| `fennara-release-local-*` | Iniciadores MCP y del daemon, además de los binarios versionados del runtime para una plataforma |
| `fennara-release-addon-v*` | Addon versionado para todas las plataformas que se resuelve mediante el manifiesto de publicación |
| `fennara-addon-latest.zip` | Alias de nombre estable del addon para todas las plataformas, destinado a la documentación y las descargas manuales |
| `fennara-webview-cef-linux-x64-*` | Runtime CEF compartido exclusivo de Linux, que se instala una vez en los datos de aplicación de Fennara |
| `fennara-release-manifest-v*` | Plan de instalación y actualización que contiene nombres de recursos, valores SHA-256, primitivas de instalación y runtimes compartidos |

La GDExtension del addon para macOS no está notarizada actualmente por Apple.
Las descargas mediante navegador y la extracción manual con Finder pueden
propagar metadatos de cuarentena y activar la notificación de verificación de
macOS. La documentación de instalación dirigida al usuario debe recomendar
`fennara install` en macOS, explicar la limitación del ZIP manual y decir a los
usuarios afectados que eliminen el addon copiado manualmente antes de volver a
instalarlo mediante la CLI. La validación de la versión no trata la mera creación
del ZIP como firma o notarización de macOS.

El prefijo `fennara-release-local-*` impide que CLI antiguas eludan
silenciosamente el camino administrado por manifiesto.

<a id="release-manifest"></a>
## Manifiesto de la versión

Desde 0.3.0, `fennara install` y `fennara update` prefieren el manifiesto de
publicación siempre que la versión publique uno. El manifiesto registra:

- `schema_version`
- `version`
- `minimum_cli_version`
- primitivas de instalación compatibles
- recursos de CLI y runtime por plataforma con SHA-256
- addon compartido con SHA-256
- runtimes compartidos por plataforma, actualmente CEF

`scripts/release-policy.mjs` es la fuente canónica de `minimum_cli_version`.
El escritor elige la política tras validar la identidad, por lo que Stable,
Package Preview y Staging no pueden elegir valores independientes. Los cambios
normales de disposición o nombres deben expresarse en datos del manifiesto, no
cambiando la CLI exterior. Aumenta la política cuando una versión necesite un
traspaso, esquema, primitiva, actualización propia u otra capacidad que una CLI
publicada antigua no pueda ejecutar con seguridad.

Si la CLI es demasiado antigua, `fennara update` debe usar la entrada
`assets.cli` por plataforma del manifiesto para actualizar primero la CLI
instalada y después reanudar la actualización del paquete con
`--no-self-update`. Si la actualización automática no está disponible para esa
versión o ubicación de instalación, debe fallar antes de instalar paquetes e
imprimir una instrucción clara para volver a ejecutar `install.sh` o
`install.ps1`.

La identidad opcional de publicación añadida al esquema 1 del manifiesto no
requiere aumentar la CLI mínima. Los clientes antiguos del esquema 1 ignoran
los campos desconocidos, mientras que los clientes que comprenden staging
validan la identidad cuando está presente. Una versión futura que dependa de una
activación que comprenda canales o de un traspaso del actualizador debe volver a
evaluar la CLI mínima antes de publicarse.

<a id="staging-identity-and-discovery-contract"></a>
## Contrato de identidad y descubrimiento de staging

Los canales están aislados por PR:

| Valor | Ejemplo para PR 101 |
| --- | --- |
| Canal | `pr-101` |
| Versión candidata | `1.2.3-pr.101.2` |
| Versión exacta | `v1.2.3-pr.101.2` |
| Referencia del canal | `fennara-staging/pr-101` |
| Archivo puntero | `fennara-staging-channel-pr-101.json` |

La referencia de Git por canal solo contiene un pequeño archivo puntero a una
versión exacta. Los binarios de la versión nunca residen bajo la referencia
móvil del canal. La CLI puede resolver este puntero con la solicitud interna de
versión `channel:pr-101` y después continúa usando únicamente la versión exacta.

Por tanto, las solicitudes de incorporación de cambios 101 y 125 usan etiquetas
de publicación y recursos puntero distintos. Actualizar un canal no puede
redirigir a los probadores del otro canal. Publicar un canal nunca cambia la
designación estable de GitHub Latest ni el canal de otra solicitud de
incorporación de cambios.

<a id="staging-candidate-workflow"></a>
## Flujo de candidatos de staging

El flujo manual **Staging Release** crea un candidato desde la cabeza actual de
una PR abierta. Ejecútalo desde `main`:

| Entrada | Significado |
| --- | --- |
| `pull_request` | Solicitud de incorporación de cambios abierta que se compilará |
| `base_version` | Versión estable prevista en formato `X.Y.Z` |
| `candidate` | Número creciente del candidato para esta solicitud de incorporación de cambios |
| `source_commit` | SHA completo opcional que todavía debe ser la cabeza de la solicitud de incorporación de cambios |
| `publish` | Desactivado para validar únicamente artefactos, activado para publicar el candidato |

El flujo congela el SHA de la cabeza de la solicitud antes de cualquier
compilación de plataforma. Los trabajos de Windows, Linux y macOS extraen ese
commit exacto con permisos de solo lectura, sin credenciales de Git conservadas,
sin credenciales de publicación y sin capacidad para guardar cachés compartidas
de dependencias. Pueden restaurar cachés compatibles de SCons/godot-cpp y Cargo
escritas por flujos de confianza de la rama predeterminada. Staging usa la acción
de caché de solo restauración, de modo que el código candidato puede consumir
resultados de compilación de confianza, pero no puede sustituir ni envenenar
cachés para ejecuciones posteriores. El código candidato puede producir
artefactos de compilación, pero no puede publicar una versión de GitHub.

Los scripts de confianza del repositorio validan después la identidad del
candidato, el inventario exacto de archivos, el contenido del addon, la
disposición de los paquetes de plataforma, el manifiesto de publicación y cada
valor SHA-256. La publicación permanece desactivada a menos que se seleccione
explícitamente `publish`.

Cuando está activada, el trabajo final fiable:

1. Vuelve a validar los recursos como datos.
2. Crea un borrador, sube todos los recursos y lo publica como la versión previa exacta `v<exact-version>` sin cambiar GitHub Latest.
3. Descarga los recursos publicados y compara nombres y hashes.
4. Rechaza un cambio de canal hacia atrás o conflictivo.
5. Actualiza al final la pequeña referencia puntero `fennara-staging/pr-<number>` mediante una escritura condicional de GitHub Contents API.
6. Descarga el puntero activo y verifica su contenido exacto.

Las ejecuciones de una solicitud de incorporación de cambios se serializan.
Solicitudes distintas usan grupos de concurrencia, etiquetas de publicación y
referencias puntero separados. Volver a intentar el mismo candidato verifica la
versión exacta existente, en lugar de mezclar archivos con ella. El flujo nunca
crea, sube recursos a ni promueve la versión estable de GitHub Latest.

La publicación estable no usa una etiqueta o versión literal `latest`. El flujo
Release crea la versión exacta `v<version>` como borrador, verifica byte por byte
los recursos subidos, la publica como versión mutable y marca esa versión exacta
como GitHub Latest cuando `promote_latest` es true. Los instaladores y el
descubrimiento estable de la CLI resuelven el punto final Latest Release de la
API de GitHub.

Las versiones estables y de staging son mutables mientras esté desactivada la
inmutabilidad de versiones del repositorio. Ambos flujos verifican los metadatos
de la versión y los bytes de los recursos descargados antes de terminar la
publicación o avanzar un canal de staging. La publicación de recursos usa el
`GITHUB_TOKEN` limitado al trabajo con acceso de escritura sobre el contenido.

La política requiere actualmente CLI `0.3.12` para estable y `0.3.8` para
staging. El descubrimiento estable ya no resuelve la etiqueta retirada `latest`.
Stable `0.3.12` requiere la validación corregida de actualizaciones, la
comprobación previa del cambio de versión y el manejo del diario de operaciones
de Windows. Un candidato de staging como
`0.3.12-pr.123.1` es menor que `0.3.12` según SemVer, por lo que su mínimo debe
permanecer por debajo de la versión candidata para que la primera configuración
pueda instalar la CLI candidata. No cambies ninguno de los dos mínimos basándote
solo en la compatibilidad con el esquema del manifiesto.

El ZIP del addon contiene todos los binarios mencionados por
`godot_demo/addons/fennara/fennara.gdextension`. Godot carga el correspondiente
a su sistema e ignora los demás.

Las cargas útiles del runtime de la vista web CEF de Linux están separadas del
archivo del addon. El empaquetado de la versión genera el manifiesto habilitado
del runtime e incorpora esos datos a
`fennara-release-manifest-v<version>.json`. La CLI instala una vez la carga útil
de CEF correspondiente bajo el directorio de datos de aplicación de Fennara del
usuario:

```text
webview/cef/linux-x64/<cef-version>/
```

No coloques `libcef.so`, ejecutables auxiliares de CEF, recursos de CEF ni
paquetes de idioma dentro de `fennara-addon-*`. Package Preview crea un artefacto
CEF independiente para realizar pruebas y escribe el mismo tipo de manifiesto
generado del runtime que usa Release, pero la publicación de versiones sigue
siendo la única fuente dirigida al usuario de los recursos de publicación.

Las compilaciones de la GDExtension para Linux también necesitan el código
fuente del wrapper del SDK oficial de CEF, pero no los archivos del runtime CEF
en el addon. CI ejecuta:

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

y pasa el directorio extraído como `FENNARA_CEF_ROOT` a SCons. SCons usa
`FENNARA_CEF_ROOT/libcef_dll/` para compilar
la pequeña biblioteca del addon `libfennara_linux_cef_bridge.so` con el wrapper
C++ fijado de CEF 139. La descarga del SDK se comprueba por versión y hash porque
el código fuente generado del wrapper debe coincidir con el ABI del runtime CEF.
El puente se empaqueta con el addon; `libcef.so`, los recursos, los paquetes de
idioma y `fennara_cef_helper` permanecen en el runtime CEF compartido e
independiente.

Los scripts fallan si encuentran CEF dentro del addon. El recurso debe llamarse:

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

El ZIP debe extraer en su raíz:

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

Los archivos opcionales del runtime CEF, como `chrome-sandbox`, `libEGL.so`,
`libGLESv2.so`, `libvk_swiftshader.so`, `libvulkan.so.1`,
`vk_swiftshader_icd.json`, `snapshot_blob.bin` y otros `locales/*.pak`, deben
incluirse cuando estén presentes en la distribución CEF seleccionada.

Para montar manualmente el ZIP del runtime desde un árbol binario de CEF elegido
por un responsable:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

En Linux, el script compila `fennara_cef_helper` desde
`scripts/cef/linux/fennara_cef_helper.cpp` con los encabezados de
CEF oficiales de `fennara-cpp/vendor/cef/`. En otro sistema operativo, compila
primero ese auxiliar en Linux y pasa
`--helper /path/to/fennara_cef_helper`. Usa `--dry-run` para inspeccionar los
archivos seleccionados antes de escribir el ZIP.

Después de que el script imprima el SHA-256, actualiza
`local/webview-runtimes/linux-cef.json`:

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

En las versiones normales, el flujo escribe automáticamente el manifiesto del
runtime CEF de Linux con `--write-manifest`, y después
`scripts/write-release-manifest.mjs` copia los campos del runtime en
`fennara-release-manifest-v<version>.json`. No habilites manualmente el
manifiesto marcador confirmado a menos que estés depurando deliberadamente una
ruta manual de recursos del runtime o un comportamiento de alternativa
heredado. Si los datos del manifiesto generado apuntan a un recurso ausente o
cuyo SHA-256 no coincide, el flujo Release y `fennara install` o
`fennara update` en Linux fallan con claridad.

La CLI debe publicar las actualizaciones del runtime CEF de Linux de forma
atómica: extrae y valida en un directorio de staging, escribe el marcador del
runtime solo después de que estén presentes los archivos requeridos, después
publica el directorio de la versión y actualiza `current.json` mediante el
cambio de nombre de un archivo temporal. El marcador instalado
`fennara-cef-runtime.json` debe identificar el contrato del cargador nativo con
`"runtime": "cef"`. La instalación y la actualización reparan un marcador
heredado correspondiente que solo contiene `"kind": "cef"` sin volver a
descargar la carga útil de CEF. Los editores en ejecución siguen usando el
runtime que ya cargaron.

La CLI incorpora las plantillas generadas de instrucciones del proyecto desde
`local/templates/`. Cuando el empaquetado de la versión compila la CLI, esas
plantillas se compilan dentro del binario junto con el resto del código de la
CLI.

<a id="what-latest-means"></a>
## Significado de `latest`

El puntero GitHub Latest Release selecciona la versión versionada que usan los
flujos normales de instalación y actualización. Fennara no crea ni mueve una
etiqueta literal `latest`.

- `install.ps1` y `install.sh` obtienen de forma predeterminada el recurso de la CLI más reciente.
- `fennara update` obtiene de forma predeterminada el manifiesto de publicación mediante el punto final Latest Release de GitHub, actualiza automáticamente la CLI instalada cuando es necesario y después resuelve desde él los recursos locales, del addon y de runtimes compartidos.
- Las actualizaciones dentro del editor preparan recursos verificados antes de cerrar, vuelven a comprobar el resumen completo del addon preparado antes de sustituirlo, conservan el addon, los iniciadores y el manifiesto del runtime anteriores hasta que termina la validación de activación y exigen el saludo de la GDExtension reabierta antes de eliminar los datos de reversión.
- `fennara install` obtiene de forma predeterminada el manifiesto de publicación mediante el punto final Latest Release de GitHub y después resuelve desde él los recursos locales, del addon y de runtimes compartidos.
- La comprobación de actualizaciones del plugin de Godot compara con la última versión de GitHub.

Usa `promote_latest: false` solo al publicar una versión que no deba convertirse
en la instalación predeterminada de los usuarios.

Los instaladores y las descargas de versiones deben mostrar los metadatos de la
versión y los pasos de descarga, extracción, instalación y verificación de los
recursos. Las solicitudes de red deben usar tiempos de espera limitados para que
un bloqueo de GitHub o de la CDN falle con un diagnóstico, en lugar de parecer
congelado. En Windows, `install.ps1` debe comprobar el código de salida de la
verificación de la CLI antes de anunciar el éxito. El código de salida
`-1073741515` (`0xC0000135`) significa que se escribió el ejecutable de la CLI,
pero Windows no pudo iniciarlo porque falta una DLL necesaria; indica al usuario
que instale Microsoft Visual C++ Redistributable 2015-2022 x64 y después vuelva
a ejecutar `fennara --version`, `fennara doctor` y `fennara install`.
URL de descarga: `https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## Prueba posterior a la publicación

En Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

En un proyecto de Godot:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Comprueba que el proyecto recibió:

```text
AGENTS.md
addons/fennara/ai/
```

Abre Godot y pregunta a la aplicación MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Prueba de actualización:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## Reglas

- El flujo Release solo se ejecuta desde `main`.
- La entrada de versión debe coincidir con `VERSION`.
- Los flujos de solicitudes de incorporación de cambios pueden compilar y subir artefactos de prueba, pero no deben publicar versiones.
- Mantén la versión para usuarios normales designada como GitHub Latest.
- No reescribas etiquetas de versiones publicadas a menos que los responsables decidan intencionalmente sustituir una versión rota.
