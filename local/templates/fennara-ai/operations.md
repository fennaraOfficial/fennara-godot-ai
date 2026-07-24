# Operations And Large Projects

Use this page only when a Fennara call timed out, its result was lost by the client, or the project is large enough that broad work needs special care.

## Recovering Tool Results

Fennara records tool lifecycle events under `user://.fennara/tool_logs/<session_id>/calls.jsonl`. Search by request ID, tool name, scene path, script path, or another distinctive input value.

Lifecycle events include `received`, `started`, `completed`, and `failed`. A final event links to the saved result or artifact. If the client stopped waiting but the call completed or failed, read that saved result instead of repeating the operation.

If the latest event is still `started`, the Godot-side work may still be running. Wait before checking once more. Do not launch an identical expensive call concurrently. If it remains unfinished beyond the tool's supported window, narrow the work before retrying.

Artifact-producing tools keep their result JSON, logs, screenshots, and related payloads together. Runtime scripts normally point back to their owning runtime-session artifact directory, so use the session log when older output is absent from a small receipt.

## Large Scenes And Assets

Start with the cheapest targeted evidence. Use scene structure, known node properties, file search, or a tiny procedural probe to narrow the target before traversing a large world.

When a broad procedural pass is truly necessary:

- skip irrelevant subtrees early
- cache expensive calculations
- avoid recomputing parent, category, bounds, or resource data in several loops
- cap examples and returned arrays
- log concise progress checkpoints and final counts
- rerun and patch a complete saved worker through `script_path`, or load its focused utilities from a new worker, instead of resending or copying large scripts

A longer client timeout does not make an unbounded algorithm safe. Expensive work should remain bounded, cancellable where supported, and free of unnecessary editor or runtime side effects.

After a timeout or repeated failure, reduce paths, nodes, candidates, frames, or output volume. Report what remains unknown rather than hiding an incomplete result.
