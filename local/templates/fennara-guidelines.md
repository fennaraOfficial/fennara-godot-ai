# Fennara MCP Guidelines

Read this file before Godot-specific work. The live Fennara tool schemas are the source of truth for tool names, arguments, limits, behavior, and results. Read `res://addons/fennara/ai/index.md` only when it routes the current task to specialized guidance.

## Connection

Fennara requires Godot 4 with this project open, the addon installed, and the intended project selected in the Fennara dock. Call `fennara_status` when the connection, active project, available tools, renderer, or asset-import readiness is uncertain.

If Godot is scanning or importing, wait for a meaningful interval without making Fennara calls, then check `fennara_status` once. Do not poll rapidly.

When several Godot projects are open, calls go to the project selected as the active MCP target in the Fennara dock.

## Tool Choice

Use ordinary repository tools for source reading, text search, file discovery, and diffs. Use Fennara for Godot-aware state and operations:

- `get_scene_tree` for scene structure and exact node paths.
- `get_node_properties` for current node, resource, connection, and animation state.
- `get_class_info` when a native Godot API is uncertain.
- `write_or_update_file` for GDScript, C#, shaders, and other appropriate project text.
- `run_scene_edit_script` for authored scenes and Godot-serialized resources.
- `run_asset_import_script` for imported source assets and import settings.
- `project_settings` for `project.godot`, autoloads, InputMap, and other project settings.
- `runtime_session` and `runtime_script` for behavior that must be observed in a running game.
- `screenshot_scene` for visual evidence from a detached scene.

Some clients defer MCP tools until searched. If a required Fennara tool is not callable, discover that exact tool and read its returned schema. Client-specific notes are linked from `res://addons/fennara/ai/index.md`.

## Inspection And Editing

Inspect the relevant saved or live state before an uncertain change. Reuse reliable evidence already gathered instead of making ritual inspection calls.

Do not hand-edit `.tscn`, `.tres`, or `.res` as plain text. Use Godot-aware scene or resource operations so Godot owns serialization. Give persistent created nodes explicit, meaningful names.

Use `run_asset_import_script` when changing how a source asset is imported everywhere. Use `run_scene_edit_script` when changing one authored scene instance, its transforms, assigned materials, local overrides, or scene-owned resources.

Fennara addon scripts and autoloads are protected infrastructure. Do not remove, rename, disable, or modify them unless the user explicitly asks to repair, replace, or uninstall Fennara.

## Validation

Treat diagnostics and validation returned by an editing tool as part of that edit. Fix relevant errors before claiming completion.

- GDScript and shader edits require diagnostics. `write_or_update_file` normally runs them automatically.
- Complete a related set of C# edits, then run one project-level C# diagnostic scan.
- Scene and resource edits require scene validation. `run_scene_edit_script` normally runs it automatically.
- Run additional diagnostics or validation only for affected work not already covered.
- Check screenshots for changes whose correctness is visual.

Validation should be proportional to the change. Do not repeat checks whose successful result already covers the final state.

## Visual And Runtime Work

For visual work, inspect the actual rendered result rather than relying only on scene text or properties. Read `res://addons/fennara/ai/visual-observation.md` for framing, multiple captures, large worlds, comparisons, and animation storyboards.

For runtime work, discover the project's real controls and success signals. Do not infer gameplay semantics from action names or common genre conventions. Verify outcomes from observed project state, not attempted inputs or elapsed time. Read `res://addons/fennara/ai/runtime-observation.md` for the full reasoning workflow.

## Reusable Project Scripts

Do not repeatedly rediscover the same project structure or resend substantial scripts. Before writing a non-trivial screenshot worker or runtime probe, check `res://.fennara/scripts/` for relevant existing code. Save useful project-specific scripts there with clear names. Rerun or patch a complete worker through the tool's `script_path`; when writing a different worker, reuse focused utilities from that directory with normal GDScript `preload()` or `load()` instead of copying their logic.

Keep screenshot workers and their utilities under `res://.fennara/scripts/screenshots/`, and runtime probes and their utilities under `res://.fennara/scripts/runtime/`; their script contracts differ. These scripts may preserve knowledge already learned about this project, such as resolved node roles, state summaries, framing, or safe observation logic. For example, after establishing how a particular platformer represents platforms and direction, a runtime probe that finds platforms ahead should be saved and reused directly or loaded by later runtime probes instead of rewritten on every call.

Treat these as private project scratch, not product code or generic Fennara helpers. Patch them when the project changes, and do not save tiny one-off snippets that are cheaper to write once.

## Renderer-Sensitive Work

Before renderer-sensitive changes or recommendations, call `fennara_status` and respect its rendering context and warnings. Consider the target platform as well as the editor renderer. If support remains uncertain, verify it with current official Godot documentation.

## Scale, Timeouts, And Failures

Keep scene traversals, logs, and generated output bounded. Narrow large work before using procedural scripts, and do not treat a long client timeout as permission for an unbounded scan.

Do not blindly repeat a timed-out or failed call while its underlying work may still be running. Read `res://addons/fennara/ai/operations.md` for tool-log recovery and large-project practices.

If the same call fails twice for the same reason, stop repeating it. Report the tool, the high-level failure, and the next concrete step. Never invent tool results or claim unavailable evidence.

## Completion

Keep changes scoped to the user's request and follow the project's existing style. Before finishing, state what changed, the relevant diagnostics or validation result, whether visual output was checked when applicable, and any limitation or unavailable evidence. Do not list tool calls merely to prove they were used.
