<!-- fennara-i18n: locale=es source=docs/tools.md sha256=4cf72381fada4fec347f29da5995d9768b39235f71b437dd698088ac0acb3518 -->
<a id="tools"></a>
# Herramientas

<!-- fennara-doc-nav:start -->
[Documentación](README.md)

🌐 **Idiomas**

[English](../../tools.md) · [简体中文](../zh-CN/tools.md) · **Español** · [Português do Brasil](../pt-BR/tools.md) · [日本語](../ja/tools.md) · [한국어](../ko/tools.md) · [Русский](../ru/tools.md) · [Français](../fr/tools.md) · [Deutsch](../de/tools.md) · [Türkçe](../tr/tools.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../tools.md)
<!-- fennara-doc-nav:end -->

Fennara proporciona a los agentes de programación inspección, edición, validación,
capturas de pantalla e información del runtime que comprenden Godot. Complementa
las herramientas normales del repositorio y del shell, en lugar de sustituirlas.

Esta página explica qué puede hacer cada herramienta, qué significa una llamada
correcta y cuáles son las limitaciones o los casos de fallo importantes. Los esquemas
de herramientas activos siguen siendo la fuente canónica de los argumentos exactos,
los campos de resultado, los límites y las instrucciones para agentes. Los proyectos
instalados también reciben instrucciones compactas y conocimiento bajo demanda en
`addons/fennara/ai/`.

<a id="tool-surfaces"></a>
## Superficies de herramientas

Los clientes MCP externos, incluidos Codex, Claude Code, Cursor y Gemini, se conectan
mediante el proceso local `fennara-mcp`. Usan su propia cuenta del modelo y sus
herramientas habituales de archivos, búsqueda, diferencias y shell junto con Fennara.

El chat integrado de Fennara usa el mismo daemon y el mismo puente de Godot. Puede
llamar las mismas herramientas de Godot y también proporciona las herramientas
`read_file` y `exec_command` limitadas al proyecto. La configuración del proveedor
y del modelo pertenece al chat integrado, no al servidor MCP.

`fennara_status` está disponible para clientes MCP externos. El chat integrado ya
recibe del daemon el estado de conexión y del proyecto activo.

<a id="typical-workflow"></a>
## Flujo de trabajo habitual

1. Confirma el proyecto conectado cuando uses un cliente MCP externo.
2. Inspecciona la escena, el recurso, la clase, el estado de importación o el ajuste del proyecto pertinente.
3. Realiza la edición útil más pequeña.
4. Ejecuta diagnósticos o la validación de la escena.
5. Usa capturas de pantalla o herramientas del runtime cuando las pruebas visuales o de comportamiento sean importantes.

El sistema de archivos del editor puede estar temporalmente ocupado examinando o
importando. Las herramientas de recursos deben usarse después de que indique que
está preparado.

<a id="connection"></a>
## Conexión

<a id="fennarastatus"></a>
### `fennara_status`

Informa sobre el servidor MCP, el daemon, el proyecto activo de Godot, las sesiones
conectadas del editor, las versiones de los componentes, el contexto de renderizado,
las herramientas anunciadas y la disponibilidad del sistema de archivos del editor.

Comportamiento correcto:

- Devuelve un bloque de estado de texto sin formato.
- Distingue entre un sistema de archivos del editor preparado y uno que está examinando o importando.
- Informa si las herramientas orientadas a recursos están preparadas en ese momento.
- Muestra las diferencias de versión para poder diagnosticar instalaciones que no coinciden.

Limitaciones y fallos importantes:

- Informa sobre la disponibilidad a nivel de proyecto, no sobre la disponibilidad de una ruta de recurso concreta.
- Un daemon desconectado, la ausencia de un proyecto activo o un plugin de Godot desconectado se comunican directamente, en lugar de tratarse como un proyecto preparado.
- La disponibilidad puede cambiar brevemente mientras Godot vuelve a importar archivos.

<a id="inspection"></a>
## Inspección

<a id="getscenetree"></a>
### `get_scene_tree`

Carga una escena mediante Godot y devuelve su jerarquía de nodos, las clases de los
nodos, los scripts adjuntos y las subescenas instanciadas. Las rutas devueltas pueden
usarse con otras herramientas de escenas.

Comportamiento correcto:

- Lee escenas creadas sin reescribirlas.
- Hace visible la estructura de nodos e instancias antes de una edición.
- Mantiene el resultado centrado en la jerarquía, en lugar de expandir cada recurso.

Limitaciones y fallos importantes:

- No es un informe completo sobre recursos 3D, mallas, materiales, esqueletos o animaciones.
- Una escena que Godot no pueda cargar devuelve un fallo, en lugar de un árbol inventado.
- Los detalles extensos de recursos pertenecen a una inspección dirigida de propiedades o scripts.

<a id="getnodeproperties"></a>
### `get_node_properties`

Muestra las propiedades que difieren de los valores predeterminados de la clase para
los nodos seleccionados y expande resúmenes útiles de los recursos integrados.

Comportamiento correcto:

- Admite hasta cinco nodos objetivo en una sola llamada.
- Lee propiedades exportadas de GDScript y los metadatos disponibles de scripts de C#.
- Resume recursos como animaciones, temas, datos de tiles, bibliotecas de mallas, fotogramas de sprites y grafos de animación, en lugar de volcar valores opacos.

Limitaciones y fallos importantes:

- Está dirigida a nodos, no es un inventario de recursos de toda la escena.
- Los recursos fuente importados pueden exponer menos información que los nodos creados en `.tscn`. Usa `run_asset_import_script` cuando sea necesario inspeccionar directamente el recurso importado generado.
- Las rutas de nodos no válidas se comunican, en lugar de ignorarse silenciosamente.

<a id="getclassinfo"></a>
### `get_class_info`

Devuelve la superficie real de la API de una clase de Godot, incluida la herencia,
las propiedades, los métodos, las señales, las enumeraciones, las constantes y la
documentación disponible.

Comportamiento correcto:

- La información de ClassDB del runtime procede del editor de Godot conectado.
- Las clases integradas usan la documentación XML oficial de Godot que coincide con las versiones principal y secundaria conectadas, con una alternativa explícita a `master`.
- Las clases de GDExtension y de addons nativos devuelven la información disponible sobre su clase y propiedades del runtime, sin fingir que disponen de documentación oficial de Godot.

Limitaciones y fallos importantes:

- La consulta de documentación puede estar incompleta cuando el XML de la clase ascendente correspondiente no está disponible o no puede recibirse una respuesta completa.
- El comportamiento exclusivo del runtime todavía puede requerir una pequeña prueba con un script del lado del editor.
- Un nombre de clase que no existe se comunica como ausente.

<a id="editing"></a>
## Edición

<a id="writeorupdatefile"></a>
### `write_or_update_file`

Crea, reescribe o realiza una sustitución exacta en un archivo de texto del proyecto.

Comportamiento correcto:

- `write` crea o sustituye un archivo a partir de su contenido completo.
- `update` sustituye un único bloque de texto exacto.
- Las ediciones de GDScript y shaders devuelven automáticamente diagnósticos de Godot.
- Las ediciones de shaders también intentan volver a serializar mediante Godot las escenas y los recursos que los referencian, para que los datos de materiales integrados no queden obsoletos.
- Se permiten escrituras de C# que formen una edición de varios archivos antes de solicitar una compilación de diagnóstico del proyecto.

Limitaciones y fallos importantes:

- Un texto de actualización ambiguo o ausente falla, en lugar de cambiar una coincidencia arbitraria.
- Las rutas protegidas de Fennara, Git, la caché de Godot, los manifiestos de plugins y los ajustes del proyecto no pueden editarse mediante esta herramienta.
- No está pensada para modificar directamente `.tscn`, `.tres` o `.res`.
- La validación de C# no se ejecuta después de cada escritura individual. Usa un examen de diagnóstico del proyecto después de completar las ediciones de C# relacionadas.
- Los propietarios que referencian un shader y que no pueden volver a serializarse de forma segura se comunican como omitidos o mediante una advertencia.

<a id="runsceneeditscript"></a>
### `run_scene_edit_script`

Ejecuta un script trabajador GDScript en tiempo de edición sobre una escena creada
o un grafo de recursos de Godot. Esta es la forma estructurada de inspeccionar o
editar escenas mediante el modelo de objetos y el serializador de Godot.

Comportamiento correcto:

- El modo de inspección carga un grafo de escena separado y de solo lectura, y nunca lo guarda.
- El modo de edición puede añadir, eliminar, cambiar de nombre o reparentar nodos; asignar recursos; cambiar propiedades; crear escenas; y guardar mediante la serialización de Godot.
- Las escenas existentes se guardan solo cuando el trabajador marca el contexto como modificado.
- Los nodos nuevos y las instancias de PackedScene usan auxiliares explícitos de propiedad para que Godot serialice la estructura deseada.
- Los diagnósticos del script se ejecutan antes de su ejecución y las escenas guardadas reciben una validación posterior.
- Las raíces de escenas heredadas se conservan cuando Godot puede serializar de forma segura las sustituciones solicitadas.
- Cada llamada devuelve la ruta temporal efectiva del trabajador, para poder corregir un trabajador fallido sin volver a crearlo desde cero.

Limitaciones y fallos importantes:

- El grafo cargado no equivale a pulsar Run Scene. Las API de juego que dependen de SceneTree, los temporizadores, el procesamiento de fotogramas y las transformaciones globales pueden comportarse de forma diferente o fallar cuando se usan con nodos separados.
- El modo de inspección bloquea los auxiliares de mutación del contexto de Fennara, pero un GDScript arbitrario debe seguir evitando efectos secundarios directos sobre el sistema de archivos, el editor, el sistema operativo y el guardado de recursos.
- Esta herramienta no guarda archivos fuente importados como `.glb` y `.gltf`. Los ajustes de importación pertenecen a `run_asset_import_script`.
- Se rechaza una propiedad incorrecta de los elementos internos de PackedScene porque puede aplanar o duplicar el contenido de una instancia.
- Si el guardado fuera a aplanar una raíz heredada, Fennara restaura el archivo original y comunica el fallo.
- Los diagnósticos o errores del runtime detienen la edición. Un resultado fallido no crea ni actualiza la escena objetivo, aunque el script trabajador temporal puede permanecer para volver a intentarlo.

<a id="runassetimportscript"></a>
### `run_asset_import_script`

Ejecuta un script trabajador GDScript limitado en tiempo de edición sobre un recurso
fuente importado y su configuración de importación de Godot. Admite modelos, texturas,
audio, fuentes y otros formatos que ya tengan un archivo auxiliar `.import`
correspondiente.

Comportamiento correcto en el modo de inspección:

- Informa sobre el importador, la clase del recurso generado, la validez de la importación, las opciones actuales con tipos, los archivos generados y las dependencias ascendentes.
- Carga el recurso generado sin reutilizar entradas anidadas obsoletas de la caché.
- Puede instanciar temporalmente un PackedScene importado dentro del SceneTree activo del editor para una inspección limitada y después lo elimina sin guardarlo.
- Proporciona resúmenes limitados de los subrecursos generados.
- Nunca conserva cambios en las opciones de importación en el modo de inspección.

Comportamiento correcto en el modo de edición:

- Prepara las opciones de importación existentes compatibles conservando sus tipos Variant nativos de Godot.
- Permite que el editor activo realice la reimportación mediante `EditorFileSystem`.
- Solo comunica éxito después de verificar los ajustes de importación canónicos, los resultados generados, el estado del sistema de archivos del editor y una nueva carga profunda del recurso.
- Intenta restaurar y volver a importar la configuración anterior cuando falla la verificación, e informa si la recuperación tuvo éxito.

Limitaciones y fallos importantes:

- El archivo fuente ya debe estar importado y tener un archivo auxiliar `.import` válido.
- La primera versión solo edita las opciones clasificadas como cambios seguros de la caché generada para importadores integrados y compatibles de texturas y escenas.
- La identidad del importador, los scripts de importación, `_subresources`, las rutas de extracción externas y las opciones cuyos efectos se desconocen siguen siendo de solo inspección.
- Las opciones desconocidas, las opciones no compatibles y los valores con un tipo Variant incorrecto fallan, en lugar de convertirse.
- La modificación directa de un archivo `.import` se detecta, se restaura y se comunica como un fallo. Fennara es responsable de conservar los archivos auxiliares.
- El auxiliar de inspección no instancia temporalmente las escenas importadas configuradas con un script raíz.
- Las dependencias describen los archivos necesarios para importar el recurso seleccionado. No identifican a los consumidores posteriores del proyecto, como las escenas que usan un modelo, los materiales que usan una textura, los scripts que reproducen audio o los temas que usan una fuente.
- Los diagnósticos de scripts, los errores del runtime, los errores de reimportación, la ausencia de archivos generados, un estado no válido del sistema de archivos o los fallos de recarga impiden un resultado correcto.
- Los arrays grandes y los elementos internos de los recursos se limitan o resumen para proteger el resultado de la herramienta. Un resultado limitado no promete que cada vértice, clave o dependencia se haya mostrado en línea.

<a id="projectsettings"></a>
### `project_settings`

Lee y cambia ajustes estructurados del proyecto de Godot, autoloads, metadatos de
la aplicación, ajustes de renderizado y visualización, y acciones de entrada.

Comportamiento correcto:

- Usa operaciones estructuradas que comprenden Godot, en lugar de sustituir texto directamente en `project.godot`.
- Enumera las acciones de entrada con zonas muertas, cantidades de eventos y resúmenes legibles de los eventos.
- Admite eventos de entrada estructurados al añadir o actualizar controles.

Limitaciones y fallos importantes:

- Se comunican las operaciones desconocidas o los valores de ajustes no válidos.
- Esta herramienta no sustituye la edición de escenas o scripts.
- Los cambios deben seguir validándose cuando afecten al inicio, el renderizado, la entrada o el comportamiento de addons.

<a id="checks"></a>
## Comprobaciones

<a id="scriptdiagnostics"></a>
### `script_diagnostics`

Ejecuta diagnósticos que comprenden Godot para scripts y shaders.

Comportamiento correcto:

- Las llamadas dirigidas a GDScript y shaders admiten hasta cinco archivos.
- Los diagnósticos de GDScript proceden del servidor de lenguaje de Godot.
- Los diagnósticos de shaders proceden del analizador de shaders de Godot.
- Las comprobaciones dirigidas de GDScript también cargan en memoria las escenas pertinentes para poder asociar con el script y la escena los errores causados por estar adjunto a una escena.
- Los exámenes de proyecto comprueban GDScript y shaders, y después realizan una compilación incremental aislada de C# cuando existe un proyecto de C#.
- Los ensamblados de diagnóstico de C# se mantienen separados de los ensamblados normales del runtime del editor.

Limitaciones y fallos importantes:

- No se admiten diagnósticos dirigidos a archivos de C#. C# usa un examen del proyecto.
- Los exámenes de todo el proyecto omiten la instanciación por escena y pueden no detectar problemas que solo aparecen cuando se carga un script mediante una escena concreta.
- Los fallos del servidor de lenguaje, el analizador o la compilación se devuelven como fallos de diagnóstico, no se tratan como resultados sin problemas.
- Los diagnósticos demuestran que el código comprobado puede analizarse o compilarse en el contexto probado. No demuestran que el juego sea correcto.

<a id="validatescene"></a>
### `validate_scene`

Comprueba una o más escenas en busca de problemas estructurales y, cuando se admite,
ejecuta una breve pasada de inicio sin interfaz.

Comportamiento correcto:

- Acepta hasta diez rutas de escenas.
- Las comprobaciones estructurales cubren scripts y recursos ausentes, rutas de nodos no válidas, nombres duplicados entre hermanos, dependencias cíclicas entre escenas y referencias exportadas pertinentes.
- Las referencias exportadas opcionales o asignadas durante el runtime se comunican como notas, en lugar de fallos incondicionales.
- Las escenas creadas con resultados estructurales sin problemas reciben una pasada de inicio sin interfaz de tres segundos, conservando los registros y los artefactos.
- Los hallazgos repetidos se agrupan para que las escenas grandes no saturen el resultado.

Limitaciones y fallos importantes:

- Las escenas fuente importadas solo reciben validación estructural porque no pueden iniciarse directamente como escenas creadas del proyecto.
- Fennara detiene intencionalmente el proceso después del periodo de validación. Ese código de detención por sí solo no se trata como un fallo de la escena.
- Una breve pasada de inicio no puede validar todos los recorridos del juego, los elementos visuales, el rendimiento, la calidad de las animaciones ni la interacción del usuario.
- Los errores estructurales impiden la pasada del runtime para esa escena.

<a id="visual-and-runtime-feedback"></a>
## Información visual y del runtime

<a id="screenshotscene"></a>
### `screenshot_scene`

Captura pruebas visuales de escenas creadas y de recursos 3D importados compatibles.

Comportamiento correcto:

- Cada escena se instancia en un SubViewport aislado. La captura de pantalla no abre ni modifica la escena creada.
- El encuadre 3D automático puede añadir una iluminación neutra de previsualización cuando el recurso no tiene entorno ni luces.
- `scene_path` es la única entrada obligatoria. Cuando se omiten tanto `code` como `script_path`, Fennara captura la raíz separada con encuadre automático.
- GDScript puede seleccionar un nodo o un array de nodos con código normal de Godot, agrupar libremente sujetos, mostrar u ocultar partes de la escena, modificar temporalmente la escena separada y solicitar capturas con `ctx.capture(...)`. Esos cambios temporales se renderizan, pero nunca se guardan en la escena creada.
- `await ctx.capture(...)` renderiza el estado de la escena en ese punto exacto y devuelve un `Image` normal de Godot. El trabajador puede inspeccionar, comparar, cambiar de tamaño, descartar o combinar las imágenes capturadas antes de publicar los resultados seleccionados con `ctx.output(image, description)`.
- Para un máximo de ocho sujetos seleccionados, cuando una captura 3D con script omite `view` y `camera`, Fennara comprueba 17 puntos de vista deterministas y elige uno que favorece la visibilidad de los nodos seleccionados, un tamaño legible, la separación de los bordes y un solapamiento bajo. Usa una vista o cámara explícita cuando ya conozcas la dirección útil y usa varias capturas cuando los sujetos distantes quedarían demasiado pequeños en un solo fotograma.
- Un trabajador de capturas de pantalla solo recibe `ctx.root`, `await ctx.capture(...)`, `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)` y `ctx.error(...)`. `ctx.sheet(...)` compone imágenes ordenadas por quien llama en páginas deterministas y, opcionalmente, etiquetadas sin elegir estados ni publicarlas. Puede pasar una Camera2D o Camera3D temporal bajo `ctx.root` en las opciones de captura cuando necesita un encuadre creado exacto.
- No se aceptan rutas de cámaras, rutas de objetivos, rectángulos de vista ni parámetros de encuadre de nivel superior. Toda la selección y el encuadre reside en el script trabajador.
- Cada imagen publicada se guarda y se enumera. Los clientes MCP capaces de usar imágenes y los modelos del chat integrado reciben los primeros seis resultados publicados como contexto de imagen independiente, en el orden de llamada. Los resultados posteriores siguen disponibles mediante la ruta guardada, con una cantidad explícita de imágenes omitidas en el recibo.
- Las capturas dispersas se devuelven con métricas de encuadre y un estado parcial, en lugar de ocultar la imagen.

Limitaciones y fallos importantes:

- El encuadre automático no siempre puede inferir el punto de vista artísticamente útil para un interior, habitación o nivel grande, ni para un recurso esquelético inusual.
- Una imagen devuelta puede ser válida aunque la validación del contenido indique que el encuadre es disperso o incierto.
- Los modelos de solo texto reciben el recibo y las rutas guardadas, pero no pueden ver directamente los píxeles de las imágenes adjuntas.
- Se comunican los fallos de carga, renderizado, propiedad de la captura o guardado de archivos.
- Los argumentos antiguos desconocidos de captura de pantalla se rechazan con un error de migración.
- Los errores de análisis del script, los errores del runtime, la ausencia de llamadas de captura, los nodos fuera de la raíz separada y las cámaras temporales no válidas se comunican sin realizar la captura.

<a id="runtimesession"></a>
### `runtime_session`

Inicia, comprueba o detiene una escena de Godot en una ventana administrada por el
daemon.

Comportamiento correcto:

- Las barreras de inicio se ejecutan antes de iniciar un proceso de escena.
- Un inicio correcto devuelve un identificador de sesión, el estado del proceso, las rutas de registros, los hallazgos de inicio y la información de captura disponible.
- El estado devuelve nuevos resultados del runtime sin descartar el registro completo de la sesión.
- La detención devuelve la información final del proceso y del registro.
- Los proyectos de C# reciben una compilación real del runtime en el resultado Debug normal de Godot antes de iniciarse, para que el proceso use ensamblados actuales.
- El registro del runtime es la fuente canónica de los resultados de Godot, los errores del runtime, los marcadores de los auxiliares, las capturas y los eventos de detención.

Limitaciones y fallos importantes:

- Solo puede haber activa globalmente una sesión del runtime administrada por el daemon a la vez.
- Las barreras de inicio fallidas impiden que se abra la escena.
- Una compilación del runtime de C# puede activar la recarga normal de ensamblados del editor abierto.
- Los marcadores de disponibilidad del inicio pueden llegar después de la respuesta inicial y aparecer en una llamada de estado posterior.
- Las sesiones administradas son procesos de Godot independientes, no son la escena ejecutada manualmente dentro del editor.

<a id="runtimescript"></a>
### `runtime_script`

Ejecuta una prueba GDScript o un controlador de entrada limitado dentro de una sesión
activa y administrada del runtime.

Comportamiento correcto:

- Puede inspeccionar nodos activos, registrar hallazgos, esperar estados, enviar entradas asignadas o de bajo nivel, realizar raycasts, interactuar con interfaces básicas y capturar fotogramas.
- Puede recopilar imágenes sin guardar del viewport con `ctx.frame()`, componer las mismas hojas controladas por quien llama disponibles para los trabajadores de capturas de pantalla con `ctx.sheet()` y publicar directamente imágenes derivadas con `ctx.output()` sin mostrarlas dentro del juego.
- Un script puede finalizar mientras la escena administrada permanece abierta para otra prueba.
- Los resultados incluyen diagnósticos, hallazgos del runtime, rutas de capturas, rutas de registros y el estado de la sesión cuando están disponibles.

Limitaciones y fallos importantes:

- Requiere un identificador válido de una `runtime_session` activa.
- Los scripts del runtime no son scripts `@tool` del editor y no pueden usarse como trabajadores de edición de escenas.
- Se comunican los diagnósticos no válidos, los tiempos de espera agotados, los errores del runtime, las sesiones cerradas o los nodos no disponibles.
- Las pruebas deben permanecer limitadas. No sustituyen un framework permanente de automatización del juego.

<a id="scrapeeditor"></a>
### `scrape_editor`

Lee una instantánea compacta del depurador después de que el usuario ejecute
manualmente una escena mediante el editor de Godot.

Comportamiento correcto:

- Agrupa los problemas repetidos del depurador y limita los detalles ruidosos.
- Ayuda a inspeccionar resultados de una ejecución del editor que no pertenece a una sesión administrada del runtime.

Limitaciones y fallos importantes:

- Es intencionalmente más limitada que leer cada elemento de la interfaz o cada línea de registro del editor.
- No debe usarse para escenas iniciadas mediante `runtime_session`; el registro del runtime administrado es más completo.
- Puede no haber un estado útil del depurador cuando no se ha ejecutado nada manualmente.

<a id="built-in-chat-tools-and-controls"></a>
## Herramientas y controles del chat integrado

<a id="readfile"></a>
### `read_file`

Lee archivos de texto limitados al proyecto e imágenes compatibles mediante el
manejo de rutas de Godot. Es útil cuando importan la normalización de `res://` o
el manejo de imágenes. La navegación amplia por el código fuente sigue perteneciendo
a las herramientas normales del repositorio.

<a id="execcommand"></a>
### `exec_command`

Ejecuta un comando no interactivo con la raíz del proyecto activo como directorio
de trabajo predeterminado.

Comportamiento correcto:

- Captura la salida y el error estándar con límites de tiempo y de tamaño.
- Rechaza directorios de trabajo fuera de la raíz del proyecto activo.
- Almacena un recibo sin procesar del lado del daemon para que un resultado grande no tenga que permanecer en la conversación del modelo.

Limitaciones y fallos importantes:

- Es una restricción a la raíz del proyecto con manejo de aprobaciones, no un sandbox del sistema operativo.
- No proporciona un terminal interactivo, PTY, sesión en segundo plano, flujo de entrada estándar ni configuración arbitraria del entorno.
- Se comunican las salidas distintas de cero, los tiempos de espera agotados y el truncamiento de resultados.

<a id="chat-controls"></a>
### Controles del chat

El chat integrado admite modos de aprobación para las llamadas de herramientas
que cambian el proyecto y para las del runtime. La inspección de solo lectura puede
ejecutarse inmediatamente, mientras que la mutación o la ejecución pueden requerir
una aprobación explícita. El acceso completo elimina esas solicitudes, pero no evita
las comprobaciones estrictas de seguridad.

El código seleccionado en el editor de scripts de Godot puede adjuntarse con
**Add to Chat**. El cuadro de redacción muestra el archivo adjunto antes de enviarlo.
`/provider` abre la configuración del proveedor y `/model` abre la selección del
modelo; son comandos de chat, no herramientas MCP.

<a id="what-fennara-does-not-replace"></a>
## Lo que Fennara no sustituye

Usa las herramientas normales de desarrollo para:

- búsqueda y navegación amplias por el repositorio
- lectura normal de archivos de texto
- diferencias y control de versiones
- ediciones que no necesitan información de Godot
- trabajo general con el shell

Usa Fennara cuando la respuesta dependa de que Godot comprenda, importe, serialice,
renderice, valide o ejecute el proyecto.
