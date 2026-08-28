<!-- fennara-i18n: locale=es source=docs/multi-agent-worktrees.md sha256=7b266e260017a37b18e3d8e36a6bed75e76c3bcc4ead88c49bec146302495014 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# Varios agentes y árboles de trabajo de Godot

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · **Español** · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Ejecuta varios agentes de programación en repositorios o árboles de trabajo
separados en un mismo equipo sin permitir que la elección de destino de un agente
redirija a otro. Cada proyecto recibe su propio proceso y conexión MCP de
Fennara; todos los proyectos comparten el mismo daemon por usuario.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Las llamadas de edición, inspección, validación y captura de pantalla pueden
ejecutarse simultáneamente. Las ejecuciones de juegos administradas por el daemon
se serializan mediante una Ranura de ejecución para todo el equipo.

<a id="one-mcp-connection-per-project"></a>
## Una conexión MCP por proyecto

Un proceso MCP selecciona una Raíz del proyecto estable cuando se inicia. Esa
Vinculación de proyecto MCP es una identidad canónica del sistema de archivos
para el directorio que contiene `project.godot`; no es el nombre de un proyecto
ni un ID de proceso de Godot.

Utiliza un proceso y una conexión MCP independientes para cada repositorio o
árbol de trabajo. Una conexión puede servir a varios agentes únicamente cuando
todos trabajan de forma intencionada en el mismo proyecto. Las herramientas de
Fennara no exponen un selector de proyecto por llamada, por lo que el modelo no
puede cambiar accidentalmente un proceso a otro proyecto.

Cada proyecto también necesita un editor de Godot habilitado para Fennara y
conectado. Si un editor se cierra y vuelve a conectarse con un nuevo ID de
proceso, el proceso MCP existente reanuda el enrutamiento cuando se vuelve a
conectar la misma Raíz del proyecto.

<a id="how-a-process-chooses-its-project"></a>
## Cómo elige su proyecto un proceso

El runtime MCP captura su directorio de trabajo de inicio y selecciona una vez
su vinculación, en este orden:

1. `--project-path <path>` o `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. El ancestro más cercano del directorio de inicio que contenga `project.godot`.
4. El modo de compatibilidad heredado sin vinculación cuando la detección
   automática no encuentra ningún proyecto de Godot.

Las rutas de la línea de comandos y del entorno son afirmaciones explícitas. Una
ruta vacía, inaccesible, inexistente, que no sea un directorio, que no sea de
Godot o que no sea compatible impide que se inicie el servidor MCP; nunca
recurre a otro proyecto. Las rutas relativas se resuelven desde el directorio de
inicio capturado. Es preferible usar una ruta absoluta cuando no está claro el
directorio de inicio del host MCP.

Fennara no consume implícitamente variables de espacio de trabajo específicas
del host. Un host MCP puede asignar su propio valor del espacio de trabajo a
`--project-path` o `FENNARA_PROJECT_PATH`.

<a id="configure-a-project-bound-connection"></a>
## Configurar una conexión vinculada a un proyecto

`fennara mcp-setup` sigue siendo global y neutral respecto al proyecto.
Ejecutarlo dentro de un proyecto no vincula todos los procesos MCP futuros a
ese proyecto. Conserva la ruta de su iniciador estable y utiliza después la
configuración de proyecto o espacio de trabajo del host MCP para añadir una
vinculación.

Para una configuración al estilo JSON:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

También puedes utilizar el entorno:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Para TOML al estilo de Codex:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Configura al siguiente agente en su propia configuración de proyecto o espacio
de trabajo con `/absolute/path/to/worktree-b`. Si un host inicia de forma fiable
un proceso MCP independiente desde el directorio de cada proyecto, la detección
de ancestros puede proporcionar la misma vinculación sin una ruta explícita.

<a id="mcp-host-boundaries"></a>
## Límites de los hosts MCP

La configuración local del proyecto y el comportamiento del directorio de inicio
varían según el host:

- Los espacios de trabajo de una sola carpeta de VS Code pueden utilizar el
  directorio de trabajo hijo documentado por el host, aunque una vinculación de
  proyecto explícita sigue siendo la configuración más clara.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro y Codex
  pueden utilizar configuración de proyecto o espacio de trabajo. Usa una
  vinculación explícita o un directorio de inicio de proyecto documentado cuando
  debas garantizar el aislamiento.
- La configuración de Claude Desktop y de las versiones heredadas de
  Windsurf/Cascade es global. Su entrada predeterminada de Fennara permanece en
  modo heredado sin vinculación y no puede proporcionar aislamiento local del
  proyecto automático. Los usuarios avanzados pueden crear entradas globales con
  nombres distintos y rutas explícitas diferentes, pero deben elegir la entrada
  correcta.

<a id="worktree-isolated-subagents"></a>
### Subagentes aislados en worktree

Algunos hosts lanzan un agente hijo en un worktree Git separado y heredan las
conexiones MCP del padre. Claude Code `isolation: worktree` y el aislamiento
de worktree de Grok Build `spawn_subagent` hacen esto.

Las herramientas nativas de archivos y de shell operan entonces en el worktree
hijo. Fennara permanece vinculado al proyecto padre, de modo que el hijo
puede editar un árbol e inspeccionar o mutar otro.

Da a ese subagente su propia conexión Fennara MCP vinculada al worktree hijo,
o mantenlo en el proyecto padre sin aislamiento de worktree. Los subagentes
estándar de Codex y OpenCode no están documentados para heredar Fennara de
esta forma.

La generación automática de configuración local del proyecto y la nueva
compatibilidad con Windsurf/Devin Local quedan fuera de este flujo de trabajo.

<a id="start-and-verify-the-editors"></a>
## Iniciar y verificar los editores

Cada árbol de trabajo necesita su propio editor de Godot habilitado para
Fennara. Los editores sin interfaz pueden utilizar puertos LSP de Godot
distintos mientras comparten el daemon de Fennara:

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

Los puertos LSP pertenecen a Godot. Fennara sigue utilizando un daemon
compartido en su dirección de bucle invertido habitual.

Ejecuta `fennara_status` desde cada agente antes del trabajo simultáneo. Confirma
que indique:

- el modo de enrutamiento `bound`
- la fuente de vinculación y la Raíz del proyecto canónica esperadas
- el estado del editor vinculado `connected`
- la disponibilidad del sistema de archivos para ese editor

Si la detección automática no encuentra ningún proyecto, el estado indica
`legacy_unbound` y muestra una advertencia de concurrencia. En ese modo de
compatibilidad se utiliza primero el Destino MCP seleccionado en el panel y
después el único editor conectado. No utilices una conexión sin vinculación para
trabajo concurrente aislado.

<a id="missing-and-duplicate-editors"></a>
## Editores ausentes y duplicados

Una Vinculación de proyecto válida permanece activa cuando su editor está
ausente. Las llamadas a herramientas devuelven el error reintentable
`bound_project_not_connected` hasta que esa Raíz del proyecto vuelve a
conectarse; nunca recurren al destino del panel.

Dos editores que se resuelven como la misma Raíz del proyecto producen
`ambiguous_project_binding`. Cierra el editor duplicado o asígnale un árbol de
trabajo distinto. Fennara no elige por ID de proceso, orden de conexión, nombre
del proyecto ni destino del panel.

Los alias de enlaces simbólicos al mismo proyecto se resuelven como la misma
identidad activa del sistema de archivos. Cambiar el destino de un enlace
simbólico después de iniciar MCP no modifica una vinculación; reinicia ese
proceso MCP para volver a vincularlo.

<a id="serialized-runtime-sessions"></a>
## Sesiones de ejecución serializadas

Todos los proyectos comparten una Ranura de ejecución para todo el equipo para
las ejecuciones de juegos administradas por el daemon. Cuando otro proyecto
está iniciando o ejecutando una sesión, `runtime_session.start` devuelve un
resultado de dominio `busy` correcto con `availability: "busy"`,
`slot_acquired: false` y un valor sugerido de `retry_after_ms`. No revela el
propietario, el ID de sesión, el ID de proceso, la escena, los registros, la
posición en la cola ni la duración esperada.

No hay una cola FIFO. Consulta con variación aleatoria cerca del retraso de
reintento sugerido y trata cada `runtime_session.start` como la reclamación
atómica definitiva. Un estado libre es solo orientativo, ya que otro agente
puede ganar la carrera después de la comprobación previa.

Solo la Raíz del proyecto propietaria puede inspeccionar, renovar, ejecutar
scripts o detener su Sesión de ejecución. El estado del propietario renueva un
plazo de inactividad de 120 segundos. Una operación de ejecución acotada del
propietario suspende el vencimiento por inactividad mientras está activa y
renueva el plazo solo después de devolver un resultado terminal del script; un
tiempo de espera agotado, un error de preparación o una cancelación no lo
renuevan. Los agentes deben consultar el estado del propietario aproximadamente
cada 30 segundos con variación aleatoria mientras continúa una ejecución.

La Concesión de ejecución absoluta predeterminada es de 900 segundos.
`max_run_seconds` acepta un entero positivo de hasta 86.400 segundos; por
ejemplo, una regresión prevista de una hora puede solicitar 4.500 segundos para
disponer de un margen de seguridad. El plazo absoluto nunca se suspende. Una
salida natural, una detención explícita, un fallo de inicio, la inactividad o el
vencimiento absoluto detienen o recogen el juego y liberan la Ranura de
ejecución.

<a id="safe-multi-agent-checklist"></a>
## Lista de comprobación segura para varios agentes

1. Crea un repositorio o árbol de trabajo distinto para cada proyecto.
2. Instala Fennara y abre un editor de Godot para cada Raíz del proyecto.
3. Configura un proceso MCP vinculado al proyecto por cada proyecto.
4. Ejecuta `fennara_status` desde cada agente y verifica su raíz canónica.
5. Permite que la edición, la inspección, la validación acotada de escenas y las
   capturas de pantalla independientes continúen simultáneamente.
6. Consulta y reintenta los resultados `busy` que no son errores para las
   pruebas de juego; mantén activa la sesión ganadora mediante el estado del
   propietario mientras se ejecuta.
