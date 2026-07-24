# Runtime Observation

Use this page when the answer requires a running scene. The `runtime_session` and `runtime_script` schemas define exact actions, context methods, await rules, coordinate contracts, receipts, and saved artifacts. This page explains how to reason about an unfamiliar game without encoding genre assumptions.

When runtime work needs camera choice, animation-state selection, visual comparisons, or sheet design, also read `res://addons/fennara/ai/visual-observation.md`.

## Discover Before Driving

Treat gameplay meaning as project-local. Inspect the InputMap, controller scripts, input handlers, state properties, scene tree, UI, signals, animations, camera logic, coordinate spaces, and likely success signals before controlling the game.

Action names are clues, not contracts. Do not assume that `left`, `attack`, `interact`, or similar names have conventional effects. Test one short primitive input, wait for the game to process it, and measure what changed.

## Observe, Experiment, Infer, Verify

Use bounded loops:

1. Observe current live state.
2. Apply one small input or controlled experiment.
3. Wait for at least one processed frame or a bounded condition.
4. Re-observe the relevant state.
5. Infer the effect from measured differences.
6. Choose the next action and verify its outcome.

Keep every loop bounded by time or iteration count. If no reliable signal is available, gather more evidence and report uncertainty instead of declaring success.

Do not derive success from intent. Pressing an input, clicking coordinates, reaching proximity, waiting long enough, or receiving a successful helper return does not prove a project outcome. Verify using real state such as a property or counter change, UI text, signal, node removal, visibility or enabled state, animation transition, scene transition, resource count, or another project-specific signal discovered through inspection.

## Live-State Safety

Runtime scenes continue simulating between calls. Re-observe after planning delays. Node references can become stale after awaits, scene reloads, transitions, deletion, or replacement, so re-fetch live roots and nodes before relying on them.

World, viewport, canvas, control, and input-event coordinates are separate contracts. Inspect how the project consumes input and perform the required conversion instead of passing positions between spaces by assumption.

Release held actions, keys, mouse buttons, and other simulated input on success, failure, timeout, direction changes, and early return.

## Physics Checkpoints

`screenshot_scene` is an editor observation path. Awaiting `physics_frame` there does not prove that gameplay physics advanced. Verify movement numerically, and use a real `runtime_session` for rigid bodies, ragdolls, contacts, and other collision-driven behavior unless the project provides an explicit offline simulation step.

For exact runtime checkpoints, pause the SceneTree between observations and count completed `_physics_process()` callbacks with a small project-local script. Keep the tree paused while reading state or capturing pixels so capture latency cannot add simulation ticks. Record transforms, velocities, contacts, or other physical state alongside images.

Reloading a scene may not reset every influence on a physics solver. When repeatability matters after collisions, compare fresh runtime sessions. Report whether numerical traces and images repeat, or validate event order and tolerances instead of assuming byte-identical results.

## Probes Versus Product Changes

Do not modify controller, gameplay, UI, or success semantics merely to make a runtime probe easier and then claim the original behavior worked. Product fixes requested by the user are valid, but distinguish them from temporary probe code and verify the final intended behavior.

For a longer investigation, save project-local probes and focused utilities under `res://.fennara/scripts/runtime/`. Patch and rerun a complete probe through `script_path`, or `preload()`/`load()` a proven utility from a new probe when only part of its logic is needed. Ordinary GDScript helpers may observe state, apply one short input pulse, wait for a condition, clean up input, or summarize progress. Keep them bounded and game-specific rather than adding gameplay concepts to Fennara itself.

## Logs And Captures

Log compact milestones, sampled state, and final summaries. Do not log every frame, input event, mouse movement, or full node dump. Accumulate counts, ranges, and small samples inside loops, then emit one concise summary.

Use captures at decision points where appearance provides evidence that state values cannot. A screenshot supports a claim but does not replace checking the real state that defines success.

For a comparison sheet, pause at each exact state and collect raw Images with `await ctx.frame()`. Compose them with `ctx.sheet()` and publish only the useful completed Images with `ctx.output()`. These helpers do not display derived Images in the game viewport. State selection, timing, ordering, camera behavior, and descriptions remain the worker's responsibility.

Use the editor debugger scraper only for errors from a scene the user ran manually in the Godot editor. For a daemon-managed runtime session, use its status, receipts, and runtime log.
