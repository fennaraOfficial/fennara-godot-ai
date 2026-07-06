# Runtime Helpers

This folder is the source for the Godot-side runtime helper scripts used by
`runtime_session` and `runtime_script`.

The packaged addon copy lives at:

```text
godot_demo/addons/fennara/runtime/
```

After editing files here, run:

```bash
node scripts/sync-runtime.mjs
```

Runtime scripts still load these helpers from `res://addons/fennara/runtime/`
inside an installed Godot project. Keep helpers primitive and project-agnostic:
input, waiting, node snapshots, captures, physics queries, and scene lifecycle
support are good fits; game-specific movement, combat, quests, inventory, or UI
flow assumptions are not.
