<!-- fennara-i18n: locale=es source=docs/repo-map.md sha256=48c47152f755d34f1f6526d58c15c3d16205d5389a00442e4f1409efa514e73c -->
<a id="repo-map"></a>
# Mapa del repositorio

<!-- fennara-doc-nav:start -->
[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · **Español** · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Este es el mapa rápido para colaboradores y agentes de programación que trabajan en este repositorio.

<a id="find-the-right-area"></a>
## Encontrar el área correcta

| Cambio | Ubicación principal |
| --- | --- |
| Configuración del usuario o comportamiento de la CLI | `local/crates/fennara-cli/` |
| Protocolo MCP externo o esquemas | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Detección o igualdad de la Raíz del proyecto | `local/crates/fennara-project-identity/` |
| Chat integrado o comportamiento del daemon | `local/crates/fennara-daemon/` |
| Integración con el editor de Godot | `fennara-cpp/` |
| Interfaz de chat | `ui/chat/` |
| Scripts auxiliares de ejecución | `runtime/` |
| Empaquetado o publicaciones | `scripts/`, `.github/workflows/` |
| Documentación del usuario | `README.md`, `docs/` |

<a id="top-level"></a>
## Nivel superior

| Ruta | Responsabilidad |
| --- | --- |
| `.github/` | Plantilla de solicitudes de incorporación de cambios, plantillas de incidencias y flujos de GitHub Actions. |
| `docs/` | Documentación del proyecto, guías de configuración, notas de arquitectura, ejemplos, demostraciones y notas de publicación. |
| `docs/i18n/` | Manifiesto de idiomas y árboles completos de documentación traducida. |
| `fennara-cpp/` | Fuente C++ de la GDExtension y punto de entrada de SCons. |
| `godot_demo/addons/fennara/` | Paquete instalable del addon de Godot que se copia en los proyectos de los usuarios. |
| `local/` | CLI de Rust, servidor MCP, daemon, esquemas y código del runtime local. |
| `media/` | Imágenes y medios públicos de la documentación. |
| `runtime/` | Código fuente de los scripts auxiliares del runtime de Godot usados por `runtime_session` y `runtime_script`. |
| `scripts/` | Scripts auxiliares de versiones, empaquetado y publicación. |
| `ui/chat/` | Fuente de la interfaz web opcional dentro del editor. |
| `local/templates/` | Instrucciones compactas y páginas de conocimiento bajo demanda escritas por `fennara install` y actualizadas por `fennara update`. |
| `local/webview-runtimes/` | Archivos de manifiesto y configuración para runtimes externos de vistas web instalados en los datos compartidos de aplicación de Fennara, como la carga útil CEF de Linux. |
| `install.ps1` / `install.sh` | Scripts de arranque que instalan la CLI de Fennara desde versiones de GitHub. |
| `VERSION` | Fuente canónica de la versión. |
| `README.md` | Resumen humano y primeros pasos. |
| `docs/README.md` | Índice de documentación orientado a tareas. |
| `docs/setup.md` | Configuración dirigida al usuario y centrada en el addon, requisitos del chat, conexión MCP, flujo de actualización y resolución de problemas. |
| `docs/multi-agent-worktrees.md` | Enrutamiento con una conexión MCP por proyecto, límites de los hosts compatibles, verificación y uso serializado de la Ranura de ejecución. |
| `docs/cli.md` | Referencia de comandos de terminal, comportamiento de instalación y actualización propiedad de la CLI, recuperación, diagnósticos, disposición de datos de aplicación y orientación para automatización. |
| `docs/telemetry.md` | Carga útil de actividad anónima, estado en datos de aplicación, comportamiento de entrega, definición de actividad mensual y controles de exclusión. |
| `CONTRIBUTING.md` | Reglas de contribución. |
| `SECURITY.md` | Política de seguridad. |
| `LICENSE.md` | Licencia del proyecto. |

<a id="local-rust-packages"></a>
## Paquetes Rust locales

| Ruta | Responsabilidad |
| --- | --- |
| `local/crates/fennara-cli/` | Comando `fennara`: instalación, actualización, actualización automática de la CLI, doctor, diagnósticos de operaciones, comprobaciones de requisitos de la vista web, compatibilidad con C#, configuración de aplicaciones MCP e instrucciones generadas para el proyecto. |
| `local/crates/fennara-cli/src/operation.rs` | Coordinador público de operaciones de instalación y actualización, fases y puntos de entrada de traspaso de la CLI. |
| `local/crates/fennara-cli/src/operation/` | Diario de operaciones específico, almacenamiento duradero, ocultación de datos en diagnósticos y módulos de pruebas. |
| `local/crates/fennara-cli/src/project_addon.rs` | Validación de la versión del addon existente en el proyecto y de la biblioteca GDExtension del editor para la plataforma actual. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Preparación de exportaciones de CI sin el addon que elimina únicamente el autoload persistente del runtime de Fennara antes de que se inicie Godot. |
| `local/crates/fennara-cli/src/release_identity.rs` | Identidad estable o de staging del addon, selectores de publicación exactos, validación del canal de solicitudes de incorporación de cambios y compatibilidad estable heredada. |
| `local/crates/fennara-cli/src/release_channel.rs` | Validación del puntero de staging por canal y resolución a una versión exacta. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Análisis del manifiesto de publicación, validación de hashes de recursos, vinculación de identidad y selección de paquetes de plataforma. |
| `local/crates/fennara-cli/src/release_version.rs` | Análisis y precedencia compartidos de SemVer de la CLI usados por los manifiestos y la selección de publicaciones. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Adopción con versión exacta de un addon completo existente sin sustituir los archivos del addon del proyecto. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Comprobación compartida del estado del daemon, disponibilidad de una versión exacta e inicio usados por install y doctor. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Pruebas a nivel de proceso de fallos, diagnósticos duraderos, ocultación de datos y registros de operaciones que fallan de forma segura. |
| `local/crates/fennara-cli/src/diagnostics.rs` | Acceso dirigido al usuario al informe saneado más reciente o a uno con nombre. |
| `local/crates/fennara-project-identity/` | Detección, validación y canonización compartidas de la Raíz del proyecto, conversión sin pérdidas para el protocolo e igualdad activa del sistema de archivos. |
| `local/crates/fennara-mcp/` | Servidor MCP local mediante stdio, detección de la Vinculación de proyecto limitada al proceso, presentación del estado y reenvío de esquemas de herramientas. |
| `local/crates/fennara-daemon/` | Daemon local compartido utilizado para el enrutamiento por Raíz del proyecto, la propiedad y las concesiones de la Ranura de ejecución, las sesiones de ejecución y el trabajo del puente de Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs` | Selección mutuamente excluyente de sesión del editor, proyecto vinculado y destino MCP heredado, además de correlación de solicitudes y respuestas de Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/runtime_slot.rs` | Admisión atómica de la Ranura de ejecución de todo el equipo, identidad del propietario y estado de la concesión. |
| `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs` | Ciclo de vida del proceso de juego administrado, autorización del propietario, registros, limpieza al vencer y recibos de sesión. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Programador anónimo de actividad diaria, cola limitada, entrega HTTP e integración con el ciclo de vida del daemon. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Validación de la identidad aleatoria de instalación, persistencia atómica en datos de aplicación, estado del recibo diario y limpieza de la exclusión. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Modos de aprobación del chat integrado, clasificación de riesgos de herramientas, decisiones de permisos y tipos de solicitudes de aprobación pendientes. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Implementación de `exec_command` del chat integrado propiedad del daemon: detección del shell, validación del cwd, inicio del proceso, tiempo de espera y cierre del árbol, captura de resultados, registro del artefacto de resultado y formato del resultado. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Planificador de compactación del contexto del chat integrado: protección exacta de la cola, poda de resultados antiguos de herramientas por presión al estilo de OpenCode, selección, almacenamiento y reproducción de fragmentos de resumen, serialización del prompt de resumen, presupuestos de tokens y representación de marcadores. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | PromptBuilder del chat integrado y contexto generado del entorno del runtime. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Registrador de trazas exclusivamente local del chat integrado, filas de eventos de SQLite, retención y auxiliares de consultas de depuración. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Primitivas del runtime de proveedores del chat integrado, catálogo y resolución, hooks de comprobación previa del contexto, tipos normalizados de streaming y errores, y adaptadores compatibles con OpenAI o Anthropic para OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, puntos finales personalizados, Ollama local y LM Studio. |
| `local/schemas/tools/` | Esquemas JSON compartidos. MCP y chat incorporan subconjuntos permitidos. |
| `local/webview-runtimes/linux-cef.json` | Manifiesto marcador o generado del runtime CEF de Linux usado para generar el manifiesto de publicación, para los resultados de doctor y como alternativa heredada. Registra la disposición compartida de datos de aplicación y los metadatos del archivo sin colocar CEF dentro del ZIP del addon. |
| `local/Cargo.toml` | Configuración del workspace Rust. |
| `local/Cargo.lock` | Grafo de dependencias bloqueado. |

<a id="gdextension-source"></a>
## Fuente de GDExtension

| Ruta | Responsabilidad |
| --- | --- |
| `fennara-cpp/SConstruct` | Punto de entrada de compilación. |
| `fennara-cpp/include/` | Encabezados C++ públicos. |
| `fennara-cpp/src/` | Implementación C++. |
| `fennara-cpp/src/setup/` | Estado nativo de la primera configuración, arranque de la CLI mediante el manifiesto de publicación, verificación de hashes, inicio de la CLI y lector del progreso duradero de las operaciones. |
| `fennara-cpp/src/release/version.cpp` | Validación y precedencia nativas de SemVer usadas por el descubrimiento de publicaciones y actualizaciones. |
| `fennara-cpp/src/release/identity.cpp` | Validación de la identidad estable o de staging empaquetada y compatibilidad estable heredada. |
| `fennara-cpp/src/release/discovery.cpp` | Descubrimiento de GitHub Latest y de actualizaciones en canales aislados de staging. |
| `fennara-cpp/src/update/` | Coordinación de actualizaciones con destino exacto, descubrimiento de recibos duraderos, traspaso de cierre e instalación, y estado de la interfaz de recuperación. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Panel de primera configuración independiente de la vista web, con progreso, reintento, registros y acciones sobre informes saneados. |
| `fennara-cpp/vendor/cef/` | Instantánea oficial de encabezados CEF 139 usada por el puente OSR de Linux. Los binarios del runtime permanecen fuera del addon. |
| `fennara-cpp/src/ui/webview_host*` | Host nativo de la vista web del chat dentro del editor y backends de plataforma. |
| `fennara-cpp/src/ui/native_webview_occlusion.*` | Detección compartida en Windows y macOS que oculta temporalmente el overlay de webview nativo cuando hay ventanas emergentes de Godot o UI de nivel superior del editor superpuestas. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Descubrimiento exclusivo de Linux del runtime CEF compartido, validación del marcador y base del cargador dinámico de `libcef.so`. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Superficie de renderizado fuera de pantalla de CEF exclusiva de Linux, reenvío de entrada de Godot, carga del ABI del puente y actualización de texturas de Godot para la vista web interna del chat. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Pequeña biblioteca puente exclusiva de Linux, compilada con el código fuente fijado oficial `libcef_dll_wrapper` de CEF 139 y el adaptador CEF OSR de Fennara. La GDExtension principal la abre dinámicamente después de cargar el runtime externo `libcef.so`. |
| `fennara-cpp/src/tools/` | Implementaciones de herramientas para Godot. |
| `fennara-cpp/src/lsp/` | Diagnósticos y auxiliares del servidor de lenguaje. |
| `fennara-cpp/src/csharp/` | Selección de proyectos de C# exclusiva de la compilación, preparación en segundo plano, diagnósticos aislados y comprobación previa del runtime. |
| `fennara-cpp/src/runtime/` | Compatibilidad nativa con el runtime usada por las herramientas, incluida la comprobación previa de escenas del runtime, los diagnósticos de scripts y las instantáneas del depurador. |
| `fennara-cpp/godot-cpp/` | Submódulo de bindings C++ de Godot. |

<a id="addon-payload"></a>
## Paquete del addon

| Ruta | Responsabilidad |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Registro de la GDExtension. |
| `godot_demo/addons/fennara/VERSION` | Versión del paquete. |
| `godot_demo/addons/fennara/release.json` | Identidad estable o de staging empaquetada, incluida la versión exacta, la etiqueta de publicación, el canal y el commit fuente de staging. |
| `godot_demo/addons/fennara/bin/` | Bibliotecas compiladas. |
| `godot_demo/addons/fennara/dist/` | Recursos empaquetados de la interfaz web que usa la vista web del chat dentro del editor. |
| `godot_demo/addons/fennara/runtime/` | Copia empaquetada y sincronizada de `runtime/` que se distribuye dentro del addon. |
| `godot_demo/tests/first_run_setup_test.gd` | Prueba sin interfaz del estado nativo de la primera configuración y de fallos deterministas. |
| `godot_demo/tests/export_plugin_test.gd` | Prueba de regresión nativa sin interfaz de la exclusión durante la exportación y la restauración del autoload. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Prueba de regresión sin interfaz del contrato de argumentos de capturas de pantalla nativo. |
| `godot_demo/tests/image_sheet_test.gd` | Prueba de regresión sin interfaz de la composición compartida de hojas de capturas de pantalla y del runtime. |
| `godot_demo/tests/runtime_image_context_test.gd` | Prueba de regresión sin interfaz de fotogramas sin procesar del runtime, hojas y salida arbitraria de Image. |

<a id="runtime-helper-source"></a>
## Fuente de auxiliares de ejecución

| Ruta | Responsabilidad |
| --- | --- |
| `runtime/game_capture_helper.gd` | Punto de entrada del auxiliar del runtime cargado por la GDExtension para sesiones de escenas y comprobaciones del runtime. |
| `runtime/image_label.gd` | Etiquetas compactas y deterministas que se estampan en celdas Image compuestas después de la captura. |
| `runtime/image_sheet.gd` | Composición compartida de hojas exclusivamente con Image, usada por los contextos de scripts de capturas de pantalla y del runtime. |
| `runtime/screenshot_script_context.gd` | Fachada pública para scripts de capturas de pantalla que añade la composición compartida de Image al contexto de captura nativo. |
| `runtime/runtime_script_context.gd` | Superficie pública de auxiliares `ctx` expuesta a `runtime_script`, incluidos fotogramas sin procesar, composición y salida de Image, esperas, entrada, instantáneas, condiciones, raycasts y clics. |
| `runtime/runtime_input_driver.gd` | Controlador de bajo nivel de eventos de entrada del runtime para teclas, botones del ratón, movimiento absoluto del ratón, movimiento relativo del ratón, modificadores y limpieza de la entrada. |
| `runtime/runtime_node_snapshot.gd` | Búsqueda de nodos del runtime, comprobaciones de existencia, instantáneas seguras frente a referencias obsoletas, lectura de propiedades y resúmenes de hijos. |
| `runtime/runtime_physics_query.gd` | Auxiliares exactos de raycast y examen 2D y 3D del runtime con recibos compactos de impactos. |
| `runtime/runtime_query_utils.gd` | Utilidades compartidas de consultas del runtime para conversión de vectores, resolución segura de nodos y rutas, identidad de objetos y coincidencia genérica de objetivos. |
| `runtime/runtime_capture_store.gd` | Escritor de artefactos de captura y estado del runtime usado por sesiones, scripts y comprobaciones del entorno del runtime. |
| `runtime/runtime_check_runner.gd` | Ejecutor de comprobaciones del runtime para especificaciones de ejecución de escenas no interactivas. |

<a id="scripts-and-workflows"></a>
## Scripts y flujos

| Ruta | Responsabilidad |
| --- | --- |
| `scripts/set-version.mjs` | Actualiza los archivos con versión en todo el repositorio. |
| `scripts/check-version.mjs` | Comprueba que las versiones estén sincronizadas. |
| `scripts/release-identity.mjs` | Validación y generación compartidas de Node para la identidad SemVer de publicación y los punteros de staging por solicitud de incorporación de cambios. |
| `scripts/release-policy.mjs` | Política de la CLI publicada compatible mínima para los manifiestos de publicaciones estables y de staging. |
| `scripts/staging-candidate.mjs` | Generación de confianza de la identidad del candidato de staging y decisiones monotónicas sobre punteros por solicitud de incorporación de cambios. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Validación específica del addon de staging, los archivos, el manifiesto, el sistema de archivos compartido y el paquete de publicación. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Puntos de entrada de validación estricta para resultados de compilación que no son de confianza y para el paquete de publicación de confianza. |
| `scripts/check-staging-channel-advance.mjs` | Aplica comprobaciones monotónicas y de procedencia antes de que avance un puntero de canal de staging. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | Verifican los bytes de los recursos publicados y el comportamiento de las descargas públicas antes de promover el puntero. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Compila un proyecto temporal de Godot ignorado y realiza una prueba de humo de la inspección de solo lectura de un `PackedScene` importado frente a la GDExtension del editor. |
| `scripts/release-targets.mjs` | Define los destinos de plataforma compatibles y los nombres de sus recursos empaquetados. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Escriben la identidad congelada del candidato y su pequeño puntero de canal. |
| `scripts/sync-chat-ui.mjs` | Copia el código fuente de la interfaz de chat sin compilación en el paquete del addon. |
| `scripts/sync-runtime.mjs` | Copia el código fuente de los auxiliares del runtime de la raíz del repositorio en el paquete del addon. |
| `scripts/sync-doc-navigation.mjs` | Añade navegación, hashes y anclas sin traducir. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Valida cobertura, actualidad, Markdown, URL y enlaces. |
| `scripts/package-preview.mjs` | Ensambla los ZIP de vista previa o publicación del addon, la CLI y el runtime local después de las compilaciones por plataforma. |
| `scripts/prepare-linux-cef-runtime.mjs` | Prepara el ZIP independiente del runtime CEF de Linux x64, elimina símbolos de binarios ELF preparados, valida los archivos requeridos y puede escribir el manifiesto de publicación generado. |
| `scripts/prepare-linux-cef-sdk.mjs` | Descarga y extrae el SDK mínimo oficial y fijado de CEF 139 para Linux que necesitan las compilaciones de CI que usan el código fuente del wrapper `libcef_dll/`. |
| `scripts/check-linux-cef-runtime-release.mjs` | Valida el recurso de publicación del runtime CEF de Linux con el manifiesto generado `local/webview-runtimes/linux-cef.json`. |
| `scripts/write-release-manifest.mjs` | Escribe y valida `fennara-release-manifest-v<version>.json` a partir de los recursos de publicación, incluidos los hashes del paquete local, del addon y de los runtimes compartidos. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Código fuente del auxiliar mínimo del subproceso CEF de Linux empaquetado dentro del ZIP independiente del runtime CEF. |
| `.github/workflows/version-check.yml` | Comprobación de coherencia de versiones. |
| `.github/workflows/gdextension-build.yml` | Comprobación de compilación multiplataforma de la GDExtension, además de la prueba sin interfaz del estado nativo de la primera configuración en Windows. |
| `.github/workflows/local-build.yml` | Comprobación de compilación del paquete local de Rust. |
| `.github/workflows/package-preview.yml` | Artefactos manuales de vista previa de paquetes, incluido un artefacto del runtime CEF de Linux exclusivo de pruebas para las pruebas de humo del chat en Linux. |
| `.github/workflows/release.yml` | Publicación manual de versiones de GitHub, incluida la generación del paquete del runtime CEF de Linux, la generación del manifiesto de publicación y la validación final de recursos. |
| `.github/workflows/staging-release.yml` | Compilación manual de staging para un SHA exacto, ejecución en seco solo de validación, publicación de una versión previa exacta y avance del puntero por solicitud de incorporación de cambios. |

<a id="where-to-change-things"></a>
## Dónde realizar cambios

| Tarea | Empieza aquí |
| --- | --- |
| Añadir o cambiar una herramienta | `fennara-cpp/src/tools/` y `local/schemas/tools/` |
| Cambiar texto de esquemas MCP | `local/schemas/tools/` |
| Cambiar `fennara install` o `fennara update` | `local/crates/fennara-cli/src/`; staging y aplicación pertenecen a `release_update.rs`, `update_stage.rs`, `update_stage/` y `update_apply/` |
| Cambiar comandos de CLI | `local/crates/fennara-cli/src/` y `docs/cli.md` |
| Cambiar progreso, cierre, saludo o recuperación | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` y `ui/chat/` |
| Cambiar primera configuración | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` y `fennara-cpp/src/ui/dock.cpp` |
| Cambiar la exclusión del addon durante la exportación | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` y `godot_demo/tests/export_plugin_test.gd` |
| Cambiar registros, fases, errores o informes | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` y `local/crates/fennara-cli/src/diagnostics.rs` |
| Cambiar requisitos de vista web | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` y `fennara-cpp/src/ui/webview_host*` |
| Cambiar instrucciones generadas | `local/templates/` y `local/crates/fennara-cli/src/project_guidance.rs` |
| Sincronizar instrucciones del addon | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` y `godot_demo/addons/fennara/ai/` |
| Cambiar configuración MCP | `local/crates/fennara-cli/src/mcp_setup.rs` y `docs/mcp-setup.md` |
| Cambiar la detección de la Vinculación de proyecto MCP o la igualdad de la Raíz del proyecto | `local/crates/fennara-project-identity/`, `local/crates/fennara-mcp/src/runtime/`, `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs` y `docs/multi-agent-worktrees.md` |
| Cambiar el enrutamiento de destinos del daemon o el estado vinculado | `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs`, `local/crates/fennara-mcp/src/runtime/` y `docs/multi-agent-worktrees.md` |
| Cambiar procesos o registros de ejecución | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` y `fennara-cpp/src/tool_results/` |
| Cambiar la propiedad, la admisión o las concesiones de la Ranura de ejecución | `local/crates/fennara-daemon/src/runtime_daemon/runtime_slot.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `fennara-cpp/src/tools/runtime_session/`, `fennara-cpp/src/tools/runtime_script.cpp` y `local/schemas/tools/runtime_session.json` |
| Cambiar los auxiliares ctx de `runtime_script`, la entrada, las instantáneas, las esperas, los raycasts, las capturas o la limpieza | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` y `docs/tools.md` |
| Cambiar interfaz, comandos o selectores | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` y `fennara-cpp/src/ui/webview_host*` |
| Cambiar proveedores | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` y `ui/chat/` |
| Cambiar telemetría | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` y `docs/telemetry.md` |
| Cambiar bibliotecas de UI de terceros | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` y `THIRD_PARTY_NOTICES.md` |
| Cambiar C# | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/` y los esquemas e instrucciones de herramientas C# |
| Cambiar paquetes, CLI mínima o actualización propia | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` y `.github/workflows/release.yml` |
| Aumentar la versión | `node scripts/set-version.mjs <version>` |
| Actualizar documentación de chat y MCP | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` y `llms.txt` |
| Actualizar traducciones | Página canónica en inglés, `docs/i18n/languages.json`, las páginas del idioma correspondiente, `scripts/sync-doc-navigation.mjs` y `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Notas

- Mantén este archivo actualizado al añadir o mover áreas principales.
- Mantén los pasos de publicación en [release.md](release.md).
- Mantén la configuración en [setup.md](setup.md).
- Mantén el comportamiento de terminal en [cli.md](cli.md).
