# Tools

Fennara gives coding agents Godot-aware inspection, editing, validation,
screenshots, and runtime feedback. It complements normal repository and shell
tools rather than replacing them.

This page explains what each tool can do, what a successful call means, and the
important limitations or failure cases. The live tool schemas remain the source
of truth for exact arguments, result fields, limits, and agent instructions.
Installed projects also receive compact guidelines and on-demand knowledge at
`addons/fennara/ai/`.

## Tool Surfaces

External MCP clients, including Codex, Claude Code, Cursor, and Gemini, connect
through the local `fennara-mcp` process. They use their own model account and
their normal file, search, diff, and shell tools alongside Fennara.

The built-in Fennara chat uses the same daemon and Godot bridge. It can call the
same Godot tools and also provides project-scoped `read_file` and
`exec_command` tools. Provider and model setup belong to the built-in chat, not
the MCP server.

`fennara_status` is available to external MCP clients. The built-in chat already
receives connection and active-project state from the daemon.

## Typical Workflow

1. Confirm the connected project when using an external MCP client.
2. Inspect the relevant scene, resource, class, import state, or project setting.
3. Make the smallest useful edit.
4. Run diagnostics or scene validation.
5. Use screenshots or runtime tools when visual or behavioral evidence matters.

The editor filesystem can temporarily be busy scanning or importing. Asset
tools should be used after it reports ready.

## Connection

### `fennara_status`

Reports the MCP server, daemon, active Godot project, connected editor sessions,
component versions, rendering context, advertised tools, and editor filesystem
readiness.

Working behavior:

- Returns one plain-text status block.
- Distinguishes a ready editor filesystem from one that is scanning or importing.
- Reports whether asset-facing tools are currently ready.
- Shows version differences so mismatched installations can be diagnosed.

Important limits and failures:

- It reports project-level readiness, not readiness for one specific asset path.
- A disconnected daemon, missing active project, or disconnected Godot plugin is
  reported directly instead of being treated as a ready project.
- Readiness can change briefly while Godot reimports files.

## Inspection

### `get_scene_tree`

Loads a scene through Godot and returns its node hierarchy, node classes,
attached scripts, and instanced subscenes. The returned paths can be used by
other scene tools.

Working behavior:

- Reads authored scenes without rewriting them.
- Makes node and instance structure visible before an edit.
- Keeps the result focused on hierarchy rather than expanding every resource.

Important limits and failures:

- It is not a complete 3D asset, mesh, material, skeleton, or animation report.
- A scene that Godot cannot load returns a failure instead of a guessed tree.
- Large resource details belong in targeted property or script inspection.

### `get_node_properties`

Shows properties that differ from class defaults for selected nodes and expands
useful summaries of embedded resources.

Working behavior:

- Supports up to five node targets in one call.
- Reads exported GDScript properties and available C# script metadata.
- Summarizes resources such as animations, themes, tile data, mesh libraries,
  sprite frames, and animation graphs instead of dumping opaque values.

Important limits and failures:

- It is node-targeted, not a full-scene resource inventory.
- Imported source assets may expose less information than authored `.tscn`
  nodes. Use `run_asset_import_script` when the generated imported resource must
  be inspected directly.
- Invalid node paths are reported rather than silently ignored.

### `get_class_info`

Returns the real API surface for a Godot class, including inheritance,
properties, methods, signals, enums, constants, and available documentation.

Working behavior:

- Runtime ClassDB information comes from the connected Godot editor.
- Built-in classes use official Godot XML documentation matching the connected
  major and minor version, with an explicit `master` fallback.
- GDExtension and native addon classes return their available runtime class and
  property information without pretending they have official Godot docs.

Important limits and failures:

- Documentation lookup can be incomplete when the matching upstream class XML
  is unavailable or a response cannot be received completely.
- Runtime-only behavior may still require a small editor-side script probe.
- A class name that does not exist is reported as missing.

## Editing

### `write_or_update_file`

Creates, rewrites, or performs an exact replacement in a project text file.

Working behavior:

- `write` creates or replaces a file from complete content.
- `update` replaces one unique exact text block.
- GDScript and shader edits automatically return Godot diagnostics.
- Shader edits also try to reserialize referencing scenes and resources through
  Godot so embedded material data does not stay stale.
- C# writes are allowed to form a multi-file edit before one project diagnostic
  build is requested.

Important limits and failures:

- Ambiguous or missing update text fails instead of changing an arbitrary match.
- Protected Fennara, Git, Godot cache, plugin manifest, and project-setting paths
  cannot be edited through this tool.
- It is not intended for raw `.tscn`, `.tres`, or `.res` surgery.
- C# validation is not run after every individual write. Use a project diagnostic
  scan after the related C# edits are complete.
- Referencing shader owners that cannot be safely reserialized are reported as
  skipped or warned.

### `run_scene_edit_script`

Runs one editor-time GDScript worker against one authored scene or Godot resource
graph. This is the structured way to inspect or edit scenes through Godot's
object model and serializer.

Working behavior:

- Inspect mode loads a detached read-only scene graph and never saves it.
- Edit mode can add, remove, rename, or reparent nodes; assign resources; change
  properties; create scenes; and save through Godot serialization.
- Existing scenes are saved only when the worker marks the context modified.
- New nodes and PackedScene instances use explicit ownership helpers so Godot
  serializes the intended structure.
- Script diagnostics run before execution, and saved scenes receive follow-up
  validation.
- Inherited scene roots are preserved when Godot can serialize the requested
  overrides safely.
- Every call returns the effective temporary worker path, so a failed worker can
  be corrected without recreating it from scratch.

Important limits and failures:

- The loaded graph is not the same as pressing Run Scene. SceneTree-dependent
  gameplay APIs, timers, frame processing, and global transforms can behave
  differently or fail when used on detached nodes.
- Inspect mode blocks Fennara context mutation helpers, but arbitrary GDScript
  must still avoid direct filesystem, editor, OS, and resource-saving side
  effects.
- Imported source files such as `.glb` and `.gltf` are not saved by this tool.
  Import settings belong to `run_asset_import_script`.
- Incorrect ownership of PackedScene internals is rejected because it can
  flatten or duplicate instance contents.
- If saving would flatten an inherited root, Fennara restores the original file
  and reports failure.
- Diagnostics or runtime errors stop the edit. A failed result does not create or
  update the target scene, although the temporary worker script may remain for a
  retry.

### `run_asset_import_script`

Runs one bounded editor-time GDScript worker against an imported source asset
and its Godot import configuration. It supports models, textures, audio, fonts,
and other formats that already have a matching `.import` sidecar.

Working behavior in inspect mode:

- Reports the importer, generated resource class, import validity, typed current
  options, generated files, and upstream dependencies.
- Loads the generated resource without reusing stale nested cache entries.
- Can temporarily instantiate an imported PackedScene inside the live editor
  SceneTree for bounded inspection, then removes it without saving it.
- Provides bounded summaries for generated subresources.
- Never persists import-option changes in inspect mode.

Working behavior in edit mode:

- Stages supported existing import options while preserving their native Godot
  Variant types.
- Lets the live editor perform the reimport through `EditorFileSystem`.
- Reports success only after the canonical import settings, generated outputs,
  editor filesystem state, and a fresh deep resource load verify.
- Attempts to restore and reimport the previous configuration when verification
  fails, and reports whether that recovery succeeded.

Important limits and failures:

- The source file must already be imported and have a valid `.import` sidecar.
- Version one edits only options classified as safe generated-cache changes for
  supported built-in texture and scene importers.
- Importer identity, import scripts, `_subresources`, external extraction paths,
  and options with unknown effects remain inspect-only.
- Unknown options, unsupported options, and values with the wrong Variant type
  fail instead of being coerced.
- Direct `.import` file modification is detected, restored, and reported as a
  failure. Fennara owns sidecar persistence.
- Imported scenes configured with a root script are not temporarily instantiated
  by the inspection helper.
- Dependencies describe files needed to import the selected asset. They do not
  identify downstream project consumers such as scenes using a model, materials
  using a texture, scripts playing audio, or themes using a font.
- Script diagnostics, runtime errors, reimport errors, missing generated files,
  invalid filesystem state, or reload failures prevent a successful result.
- Large arrays and resource internals are bounded or summarized to protect tool
  output. A bounded result is not a promise that every vertex, key, or dependency
  was printed inline.

### `project_settings`

Reads and changes structured Godot project settings, autoloads, application
metadata, rendering and display settings, and input actions.

Working behavior:

- Uses Godot-aware structured operations instead of raw `project.godot` text
  replacement.
- Lists input actions with deadzones, event counts, and readable event summaries.
- Supports structured input events when adding or updating controls.

Important limits and failures:

- Unknown operations or invalid setting values are reported.
- This tool does not replace scene or script editing.
- Changes should still be validated when they affect startup, rendering, input,
  or addon behavior.

## Checks

### `script_diagnostics`

Runs Godot-aware diagnostics for scripts and shaders.

Working behavior:

- Targeted GDScript and shader calls support up to five files.
- GDScript diagnostics come from Godot's language server.
- Shader diagnostics come from Godot's shader parser.
- Targeted GDScript checks also load relevant scenes in memory so errors caused
  by scene attachment can be associated with the script and scene.
- Project scans check GDScript and shaders, then perform one isolated incremental
  C# build when a C# project is present.
- Diagnostic C# assemblies are kept separate from the editor's normal runtime
  assemblies.

Important limits and failures:

- Targeted C# file diagnostics are not supported. C# uses a project scan.
- Project-wide scans skip per-scene instantiation and can miss problems that only
  appear when a script is loaded through a particular scene.
- Language-server, parser, or build failures are returned as diagnostic failures,
  not treated as clean results.
- Diagnostics prove that the checked code can be parsed or compiled in the tested
  context. They do not prove gameplay correctness.

### `validate_scene`

Checks one or more scenes for structural problems and, where supported, runs a
brief headless startup pass.

Working behavior:

- Accepts up to ten scene paths.
- Structural checks cover missing scripts and resources, invalid node paths,
  duplicate sibling names, cyclic scene dependencies, and relevant exported
  references.
- Optional or runtime-assigned exported references are reported as notes rather
  than unconditional failures.
- Authored scenes with clean structural results receive a three-second headless
  startup pass with logs and artifacts retained.
- Repeated findings are grouped so large scenes do not flood the result.

Important limits and failures:

- Imported source scenes receive structural validation only because they cannot
  be launched directly as authored project scenes.
- Fennara intentionally stops the process after the validation window. That stop
  code alone is not treated as a scene failure.
- A brief startup pass cannot validate all gameplay paths, visuals, performance,
  animation quality, or user interaction.
- Structural errors prevent the runtime pass for that scene.

## Visual And Runtime Feedback

### `screenshot_scene`

Captures visual evidence from authored scenes and supported imported 3D assets.

Working behavior:

- Every scene is instantiated in an isolated SubViewport. Screenshot capture
  does not open or modify the authored scene.
- Automatic 3D framing can add neutral preview lighting when the asset has no
  environment or lights.
- `scene_path` is the only required input. When both `code` and `script_path`
  are omitted, Fennara captures the detached root with automatic framing.
- GDScript can select one node or an array of nodes with ordinary
  Godot code, group subjects freely, show or hide scene parts, temporarily
  alter the detached scene, and request captures with `ctx.capture(...)`.
  Those temporary changes are rendered but never saved to the authored scene.
- `await ctx.capture(...)` renders the scene state at that exact point and
  returns an ordinary Godot `Image`. The worker may inspect, compare, resize,
  discard, or combine captured images before publishing selected results with
  `ctx.output(image, description)`.
- For up to eight selected subjects, when a scripted 3D capture omits `view`
  and `camera`, Fennara checks 17
  deterministic viewpoints and chooses one that favors selected-node
  visibility, readable size, edge clearance, and low overlap. Use an explicit
  view or camera when the useful direction is already known, and use multiple
  captures when distant subjects would become too small in one frame.
- A screenshot worker receives only `ctx.root`, `await ctx.capture(...)`,
  `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)`, and `ctx.error(...)`.
  `ctx.sheet(...)` composes caller-ordered Images into deterministic,
  optionally labelled pages without choosing states or publishing them. It can pass a
  temporary Camera2D or Camera3D under `ctx.root` in the capture options when
  it needs exact authored framing.
- Camera paths, target paths, view rectangles, and top-level framing parameters
  are not accepted. All selection and framing lives in the worker script.
- Every published image is saved and listed. Image-capable MCP clients and
  built-in chat models receive the first six published outputs as separate
  image context in call order. Later outputs remain available by saved path,
  with an explicit omitted-image count in the receipt.
- Sparse captures are returned with framing metrics and partial status instead
  of hiding the image.

Important limits and failures:

- Automatic framing cannot always infer the artistically useful viewpoint for a
  large interior, room, level, or unusual skinned asset.
- A returned image can be valid while content validation reports that framing is
  sparse or uncertain.
- Text-only models receive the receipt and saved paths but cannot directly see
  attached image pixels.
- Loading, rendering, capture ownership, or file-save failures are reported.
- Unknown legacy screenshot arguments are rejected with a migration error.
- Script parse errors, runtime errors, missing capture calls, nodes outside the
  detached root, and invalid temporary cameras are reported without capturing.

### `runtime_session`

Starts, checks, or stops a daemon-managed windowed Godot scene.

Working behavior:

- Startup gates run before a scene process is launched.
- A successful start returns a session identifier, process state, log paths,
  startup findings, and available capture information.
- Status returns new runtime output without discarding the full session log.
- Stop returns final process and log information.
- C# projects receive a real runtime build into Godot's normal Debug output before
  launch so the process uses current assemblies.
- The runtime log is the source of truth for Godot output, runtime errors, helper
  markers, captures, and stop events.

Important limits and failures:

- Only one daemon-managed runtime session is active globally at a time.
- Failed startup gates prevent the scene from opening.
- A C# runtime build can trigger the open editor's normal assembly reload.
- Startup readiness markers may arrive after the initial response and appear in
  a later status call.
- Managed sessions are separate Godot processes, not the scene manually running
  inside the editor.

### `runtime_script`

Runs a bounded GDScript probe or input driver inside an active managed runtime
session.

Working behavior:

- Can inspect live nodes, log findings, wait for state, send mapped or low-level
  input, perform raycasts, interact with basic UI, and capture frames.
- Can collect unsaved viewport Images with `ctx.frame()`, compose the same
  caller-controlled sheets available to screenshot workers with `ctx.sheet()`,
  and publish derived Images directly with `ctx.output()` without displaying
  them inside the game.
- A script can finish while the managed scene remains open for another probe.
- Results include diagnostics, runtime findings, capture paths, log paths, and
  session state when available.

Important limits and failures:

- It requires a valid active `runtime_session` identifier.
- Runtime scripts are not editor `@tool` scripts and cannot be used as scene edit
  workers.
- Invalid diagnostics, timeouts, runtime errors, closed sessions, or unavailable
  nodes are reported.
- Probes must remain bounded. They are not a replacement for a permanent gameplay
  automation framework.

### `scrape_editor`

Reads a compact debugger snapshot after the user manually runs a scene through
the Godot editor.

Working behavior:

- Groups repeated debugger issues and caps noisy details.
- Helps inspect editor-run output that is not owned by a managed runtime session.

Important limits and failures:

- It is intentionally narrower than reading every editor UI element or log line.
- It should not be used for scenes launched through `runtime_session`; the
  managed runtime log is more complete.
- No useful debugger state may be available when nothing was manually run.

## Built-In Chat Tools And Controls

### `read_file`

Reads project-scoped text files and supported images using Godot path handling.
It is useful when `res://` normalization or image handling matters. Broad source
navigation still belongs to normal repository tools.

### `exec_command`

Runs one non-interactive command with the active project root as the default
working directory.

Working behavior:

- Captures standard output and error with time and output limits.
- Rejects working directories outside the active project root.
- Stores a raw daemon-side receipt so large output does not need to remain in the
  model conversation.

Important limits and failures:

- It is project-root restriction and approval handling, not an operating-system
  sandbox.
- It does not provide an interactive terminal, PTY, background session, standard
  input stream, or arbitrary environment configuration.
- Non-zero exits, timeouts, and output truncation are reported.

### Chat Controls

The built-in chat supports approval modes for project-changing and runtime tool
calls. Read-only inspection can run immediately, while mutation or execution can
require explicit approval. Full access removes those prompts but does not bypass
hard safety checks.

Selected code from Godot's script editor can be attached with **Add to Chat**.
The composer shows the attachment before sending. `/provider` opens provider
setup and `/model` opens model selection; these are chat commands, not MCP tools.

## What Fennara Does Not Replace

Use normal development tools for:

- broad repository search and navigation
- ordinary text file reading
- diffs and version control
- edits that do not need Godot feedback
- general shell work

Use Fennara when the answer depends on Godot understanding, importing,
serializing, rendering, validating, or running the project.
