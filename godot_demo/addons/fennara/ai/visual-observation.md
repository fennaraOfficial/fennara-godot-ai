# Visual Observation

Use this page when the task depends on what a Godot scene looks like, how objects relate in 2D or 3D, or how visible state changes over time. The `screenshot_scene` schema defines the script contract and exact capture API. This page explains how to compose those primitives into useful evidence.

## When The Required Godot Code Is Unclear

Do not research APIs again when existing project evidence or a proven reusable script already answers the question. If a public method is uncertain, observed behavior contradicts expectations, or an imported asset behaves differently from its source, try `get_class_info`, current official documentation, or the Godot source matching the connected version.

Source search can connect a GDScript method to its implementation through the class and `ClassDB::bind_method`, while importer source can explain generated resource behavior. Use this only to find the correct public API or resolve a real uncertainty, not to copy private engine internals. Prove the uncertain part with a small Fennara capture before generalizing it.

## Begin With The Question

Choose the smallest set of images that lets the model answer the visual question. Select exact scene nodes in ordinary Godot code. Do not add bookkeeping IDs, intent objects, or game-specific concepts to Fennara's tool contract. A project worker may still use ordinary project-specific Godot code.

A selected node defines framing evidence. It does not imply that every unselected object should disappear. Preserve surrounding geometry and HUD when they help explain the game. Hide specific content temporarily only when it blocks the subject or when an unobstructed comparison is explicitly needed.

Do not add contours, node paths, legends, measurements, or diagnostic prose inside an ordinary scene image. Such overlays can be mistaken for game content and consume useful pixels. Return explanatory text outside the image. A small chronological frame, time, or pose number inside an animation cell is the intentional exception.

## One Image Or Several

Use one image when one viewpoint clearly answers the question. Use multiple separately published images when:

- important subjects cannot remain readable at one scale
- distant groups would make a whole-world image too small
- front, side, top, or perspective views reveal different facts
- an unobstructed view and the normal gameplay context are both useful
- several states must be compared without shrinking them into one crowded collage

Images published separately remain separate model inputs. Use a collage only when the spatial arrangement inside one image is itself valuable, such as a chronological animation sheet or a strict same-camera comparison.

If more images are produced than the tool can attach to one result, publish them anyway. Fennara saves every output and reports the paths of images omitted from model context. The model can request or read a saved image later.

## Detached Scene Lifecycle

Before its first capture, `ctx.root` is detached from a `SceneTree`. Local properties are available, but tree-dependent APIs such as `global_transform` are not. If framing or measurement requires global-space values before any useful image is rendered, perform one initialization capture and discard its returned Image:

```gdscript
await ctx.capture(subject, {"view": "front"})
```

After that capture, the scene is initialized and global-space measurements are valid. Use a cheap explicit view for this discarded initialization instead of automatic camera search. Do this only when tree-dependent state is required because the initialization is a real render and adds capture cost.

## 2D Framing

Frame the relevant CanvasItem or group of items with enough surrounding context to interpret placement. Prefer a temporary Camera2D when an explicit crop, zoom, or coordinate region matters. Preserve the authored viewport and UI when judging the actual player-facing composition.

For SpriteFrames or other discrete frame animation, exact chronological frames may be more useful than pose selection. `AnimatedSprite2D.set_frame_and_progress()` can set an exact visible frame without playing through earlier frames. Use one or more sheets when the complete sequence is reasonably sized, and keep cells large enough that silhouettes and pixel details remain visible.

## 3D Framing

When the useful direction is already known, use an explicit view or temporary Camera3D. When it is unknown and a small group of subjects must fit together, automatic viewpoint search can choose among deterministic candidates. Camera search is useful for discovery, but it cannot make an enormous world readable in one frame.

For large environments, choose local subjects and capture multiple readable regions. Do not zoom out until the whole world becomes visual clutter. When exact relationships require several directions, publish those directions as separate images rather than shrinking seven views into a default collage.

Treat camera choice as part of the evidence. Keep it fixed when comparing placement, scale, reach, or movement. Vary it only when maximum per-subject detail matters more than spatial comparability.

## Animation Storyboards

An animation storyboard should show representative visual states across the complete animation, not merely the first few frames and not every source frame by default.

Use exact frame sampling when the task is frame-level inspection, such as sprite coverage, deformation errors, popping, or a request to inspect every authored frame. Otherwise select representative poses deterministically.

### Candidate States

Build a bounded chronological candidate set from the information available in the project:

- the start state
- the end state for a non-looping animation
- authored markers or events when they carry visual meaning
- available animation key times
- uniform samples across the full duration, capped to a practical candidate count such as 64 for representative selection

If authored keys exceed the candidate budget, thin them deterministically across the complete duration rather than keeping only the earliest keys. Preserve mandatory states first, then use any remaining capacity for authored and uniform samples.

Do not assume that keys in an imported Godot `Animation` are the creator's original authored keys. Importers may bake source interpolation into generated samples. For example, Godot's glTF importer samples transform curves at the configured bake FPS, so a five-key CUBICSPLINE source curve can become a dense linear Godot track. Treat imported key times as available candidate states unless original authoring metadata is independently known.

Validate the imported clip before building a sheet. An animation name alone is not evidence that useful animation data was imported. Check its length, track count, track types, and paths. If a clip has zero tracks or only a placeholder duration, reimport or repair the asset instead of publishing repeated identical frames. A screenshot worker cannot reconstruct tracks that the importer omitted.

Prefer the project's normally imported `PackedScene` when observing glTF assets. `GLTFDocument.generate_scene()` is useful for import inspection, but Godot can leave descendant geometry as `ImporterMeshInstance3D` intermediates until the editor scene-import pipeline converts it. Those intermediates are not a substitute for the renderable imported scene.

For direct `AnimationPlayer` clip inspection, evaluate each candidate by seeking the animation and updating it immediately. Avoid firing gameplay events merely to inspect a pose. Do not use this shortcut when the question concerns `AnimationTree` blending, transitions, filters, or OneShots because it bypasses the graph being tested.

The safe public Godot pattern is:

```gdscript
player.pause()
player.assigned_animation = clip_name
player.seek(time, true, true)
```

Assigning the animation while paused avoids starting playback. Godot's `update_only` seek applies the visual state while skipping method and audio track execution. Avoid calling `play()` merely to select a clip before pose extraction because starting playback can process the initial key.

Keep `update_only` enabled even if an editor-side method-track test appears harmless. Godot independently suppresses method-track callbacks while `Engine.is_editor_hint()` is true, but that editor protection does not prove the seek is side-effect-free: `update_only` also suppresses audio and animation-playback tracks and remains the explicit contract for pose-only inspection.

### AnimationTree Graph State

Drive an `AnimationTree` through its real public parameters and playback objects. Put it in manual process mode so capture frames cannot advance it implicitly, start from a known state, set graph parameters, and advance it explicitly:

```gdscript
tree.callback_mode_process = AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_MANUAL
tree.active = true
playback.start(&"Move", true)
tree.advance(0.0)
tree.set("parameters/StateMachine/Move/blend_position", 0.5)
tree.advance(sample_delta)
tree.advance(0.0)
```

The final zero-delta advance is an intentional re-evaluation at the resulting graph time. Nodes such as `AnimationNodeOneShot` update fade and internal playback state while processing an advance, so an image taken immediately after only `advance(delta)` can contain the previous blend weight even though the reported graph state has changed.

Unlike `AnimationPlayer.seek(..., update_only=true)`, `AnimationMixer.advance()` has no update-only mode. Before offline graph sampling, temporarily disable enabled method, audio, and nested-animation tracks that are not part of the visual question, then restore them after capture. This keeps the detached observation from triggering callbacks or media while preserving transform, value, and blend-shape output.

Stabilize unrelated autonomous animation before comparison. For example, a character can contain an independently blinking or randomly looking 2D face rendered through a `SubViewport`. Pause that secondary animation at a known neutral state when testing body motion, but leave it running when facial coordination is the question.

For layered output such as a filtered OneShot, publish matched controls at the same graph time: base state, then base plus layer. This distinguishes the layer's contribution from motion that the base state would have produced anyway.

### Skeletons And Modifiers

For a directly sought `AnimationPlayer` pose with `BoneAttachment3D` visuals, flush the skeleton update after seeking and before capture:

```gdscript
player.seek(time, true, true)
skeleton.notification(Skeleton3D.NOTIFICATION_UPDATE_SKELETON)
var image := await ctx.capture(subject, {"camera": camera})
```

Godot normally delivers this notification later in the frame. An immediate offscreen capture can otherwise observe attachment transforms from the previous pose. `force_update_all_bone_transforms()` alone is insufficient for this case because `BoneAttachment3D` follows the subsequent `skeleton_updated` signal.

A `SkeletonModifier3D` bone pose is transient. Godot backs up the unmodified poses, processes modifiers, uploads the modified transforms to the rendered skin, emits the skeleton update, then restores the stored poses. Consequently, `get_bone_pose*()` after modifier evaluation can report the base pose. Observe `modification_processed` when the transient bone value itself is required.

Verify both the modifier signal and the rendered skinned geometry. Treat a visible `BoneAttachment3D` as supplementary evidence only. It can show a restored or stale attachment transform even when modifier evaluation and rendered skin deformation are correct, and following the target still does not prove that the same pose reached the skin.

### Procedural Motion And IK

Procedural motion usually has no seekable clip timestamp. Begin from a fresh known state, initialize the real project solver, and advance it sequentially with a fixed delta. Disable only the autonomous callbacks that would otherwise advance the same system a second time. If the procedural result is applied through `SkeletonModifier3D`, put the skeleton in manual modifier mode and explicitly evaluate it after updating targets and solver state:

```gdscript
skeleton.modifier_callback_mode_process = Skeleton3D.MODIFIER_CALLBACK_MODE_PROCESS_MANUAL

# Update the project's targets and solver state through its real public path.
skeleton.advance(delta)
```

Give floor queries, constraints, and placement state a deterministic warm-up before publishing the first pose when the question concerns settled motion. Keep an unsolved rest pose only when it is useful evidence.

Simulation cadence and observation cadence are separate decisions. A worker may run several fixed simulation ticks between photographs, so pose count does not control physics or solver accuracy. Keep the driven target trajectory explicit, preferably in world space when world movement is being tested.

Record a small numerical trace alongside the images, such as solver state, normalized phase, target transforms, contacts, or foot heights. Confirm that the intended state was actually exercised. Movement below a solver threshold may correctly produce standing or isolated foot placement instead of a sustained gait, even though every capture succeeds. For a complete cycle, verify that its phase progresses through the expected states and wraps or finishes.

Use a fixed side or three-quarter camera when displacement and foot contact matter. Use a separate anchored sheet when articulation needs to remain large. When determinism matters, repeat the worker from a fresh scene and compare both the state trace and final image hashes under the same renderer and environment.

Treat cross-renderer checks as compatibility tests, not determinism tests. Run the same bounded fixture under Forward+, Mobile, and Compatibility when renderer coverage matters, then verify that required content remains recognizable and that diagnostics and renderer errors stay clean. Do not require matching pixels or hashes across rendering methods because lighting, shaders, particles, and other rendering details can legitimately differ.

Matching state traces or image hashes prove repeatability, not motion quality. Different rendered poses prove state change, not smooth or intentional transitions. Use event-aligned states and measurements for correctness; inspect a dense interval or runtime playback when timing and motion quality are the question.

Separate-thread 3D physics can make the active physics space unavailable to editor-thread screenshot code, including forced raycast updates. Prefer a project-owned deterministic advance path using resolved contacts, a real runtime physics observation, or an explicit main-thread test fixture. Never silently change a production project's physics-thread setting to make an offline capture succeed.

### Pose Description

Compare actual scene state, not timestamp distance alone. A useful 3D pose descriptor may include:

- root translation and rotation
- bone positions and rotations
- transforms of other animated nodes
- visual bounds
- blend-shape weights

Normalize skeletal articulation relative to the animated root bone, not only the outer scene node. Measure root motion separately. Otherwise locomotion can dominate pose distance, enlarge union bounds, make the character unnecessarily small, and cause representative selection to favor travel rather than body-shape change.

When comparing bone rotations, account for a quaternion and its negation representing the same rotation. Normalize positional differences by a characteristic subject size so scene units or asset scale do not overwhelm rotational change.

Confirm that seeking between visibly different states changes the rendered mesh, not only the reported bone transforms. `MeshInstance3D` resolves skinning through its `skeleton` NodePath. Godot does not register a skin when that path is empty or resolves to no `Skeleton3D`, and its source notes that the path can become outdated after reparenting. After the scene has entered the tree, `get_skin_reference() == null` confirms that no rendered skin binding was created.

When an imported mesh is a direct child of its intended skeleton but the path is empty, repair the project instance with ordinary Godot code before capture:

```gdscript
if mesh.get_parent() == skeleton and mesh.skeleton.is_empty():
    mesh.skeleton = NodePath("..")
```

Do not apply this blindly to other hierarchies. Resolve the actual intended `Skeleton3D` and use the corresponding relative path.

For pose-aware bounds of a skinned mesh, Godot can bake its current skeleton deformation:

```gdscript
var baked := mesh_instance.bake_mesh_from_current_skeleton_pose()
var posed_bounds := mesh_instance.global_transform * baked.get_aabb()
```

This copies and deforms mesh surface data using the current RenderingServer skeleton transforms. Use lightweight bone or animation-track descriptors across the candidate pool, then bake only the selected states whose rendered bounds are needed. Skeleton-pose baking ignores blend shapes, so do not treat it as complete visual geometry when blend-shape deformation also matters.

Use only components relevant to the asset. Procedural animation, particles, physics, shaders, visibility changes, and AnimationTree-driven state may require advancing or configuring the real scene and measuring the visible or physical state they produce.

`Animation.TYPE_VALUE` tracks can drive visible properties such as material colors, mesh visibility, and light visibility. Include the actual Variant type in the pose descriptor. Compare `Color` components numerically, treat a changed `bool` as a discrete change, and handle other supported value types explicitly rather than silently ignoring all non-transform tracks.

For nearest or step-interpolated property tracks, sample inside each held interval, such as the midpoint between consecutive keys. This captures each displayed state without relying on boundary semantics. Preserve the true non-looping end separately when the complete duration matters.

For several independently animated subjects, distinguish shared scene evolution from individual clip inspection. A shared storyboard should use an explicit scene-time window and preserve each subject's real timing, looping, and completion behavior; do not invent synchronization by assigning unrelated clips the same percentage progress. Normalize descriptor distance per subject before combining it, then apply explicit importance weights if needed, so track count, bone count, units, or asset scale cannot dominate accidentally. When the question concerns each animation rather than their coordination, select and publish separate per-subject storyboards on each clip's local timeline.

For an `ArrayMesh` animated by blend shapes, the base mesh bounds may not describe the current deformation. Godot can bake the current blend-shape mix into temporary geometry:

```gdscript
var baked := mesh_instance.bake_mesh_from_current_blend_shape_mix()
var deformed_bounds := mesh_instance.global_transform * baked.get_aabb()
```

Use the deformed bounds when shape change affects pose significance or when fitting one camera around all selected states. Baking performs real geometry work, so keep candidate counts bounded and do not bake states whose bounds are irrelevant to the question.

### Deterministic Selection

For a requested pose count:

1. Keep mandatory states such as the start, non-looping end, and relevant markers.
2. Repeatedly choose the candidate whose descriptor is farthest from its nearest already selected pose.
3. Resolve equal scores by choosing the earlier time.
4. Sort the selected states chronologically before rendering.

This farthest-point rule favors distinct poses while remaining deterministic. An endpoint may visually match the start and still remain useful because it exposes that the non-looping animation returns to its initial pose.

Maximum diversity is appropriate when the question concerns distinct silhouettes, articulation, deformation, or spatial extremes. It is not a complete description of animation progression because visually similar states revisited later can be discarded and selected times can cluster around one part of the clip.

For a chronological storyboard, prefer cumulative-change sampling: measure descriptor distance between consecutive candidates, accumulate that change over time, then select states at roughly equal portions of the accumulated motion. This preserves the journey through repeated states and spends fewer cells on long pauses, making it the stronger default when the goal is to explain how the animation progresses.

For non-looping clips, keep the actual final timestamp even when cumulative change reaches its total earlier because the clip ends with an unchanged hold. Otherwise the images may show every distinct pose while still failing to prove that the complete animation duration was inspected.

Uniform time sampling remains useful as a simple baseline and when exact timing or evenly spaced timestamps matter. It guarantees duration coverage but can waste cells on pauses or nearly unchanged intervals.

Pose count is a presentation decision, not a Fennara limit. Eight poses in a 4 by 2 sheet is only a convenient starting point. The caller may request any count, or selection may continue until additional poses add too little meaningful difference. Use multiple sheets instead of shrinking cells when more poses are needed.

### Root Motion And Camera Choice

Decide whether the question concerns articulation, world movement, or both:

- For articulation comparison, normalize root motion and keep the subject anchored so body poses remain large and comparable.
- For movement trajectory, preserve root motion and frame its full path with union bounds.
- When both matter, publish an anchored pose sheet and a separate trajectory view.

`AnimationTree` root motion is a per-advance delta, not an independently seekable world position. For a trajectory, begin from a known graph state, advance monotonically, read the delta immediately, and compose it in the same order as the project:

```gdscript
tree.advance(delta)
var root_step := Transform3D(
    Basis(tree.get_root_motion_rotation()),
    tree.get_root_motion_position(),
)
accumulated *= root_step
```

Read the root-motion delta before another `advance()`, including `advance(0.0)`. Godot clears the per-evaluation root-motion result when the next blend begins. Independent pose seeks therefore cannot reconstruct a traveled path; replay sequentially from a fresh state and validate the final accumulated transform against the authored track or the project's own movement result.

Treat static `MeshInstance3D` bounds as an estimate for skinned characters. Validate the completed sheet because a deformed pose can exceed those bounds; add a safety margin or use deformation-aware bounds rather than accepting clipped extremities.

Do not assume an imported character's authored forward axis. If straight-on symmetry matters, make a small exact `+X`, `-X`, `+Z`, `-Z` cardinal probe once, identify the actual front, and reuse that camera direction. Use a side or three-quarter camera as an additional view when depth motion matters, not as an unexplained replacement for the front view.

### Rendering, Labels, And Packing

When a sheet compares spatial facts, compute union bounds across all chosen states, create one camera from those bounds, and reuse its projection and scale for every cell. Do not auto-zoom those cells independently.

Add a temporary `Camera3D` to the initialized scene tree before calling global-space methods such as `look_at_from_position()`. Calling them while the camera is detached can leave it at the wrong transform and produce a technically successful but empty render.

Independent per-pose framing is useful for a detail sheet when each pose should be inspected at maximum readable size and world movement is irrelevant. State that choice outside the image. Never use it for translation, scale, reach, or spatial-extreme comparisons because recentering and rescaling erase those differences.

Pass captured Images in the intended order to `ctx.sheet()`. It packs deterministic row-major pages, preserves aspect ratio by default, and can add compact labels after capture without contaminating the observed scene. The caller still chooses every state, camera, ordering, layout option, and published result.

Preserve the source capture aspect ratio when resizing cells, or crop intentionally before resizing. Stretching a 16:9 render directly into a square cell distorts the subject and invalidates shape comparison.

Do not infer source FPS from the smallest animation-key interval. Imported tracks can contain uneven or subframe key spacing, producing false values such as 40, 60, or 120 FPS for different clips from the same asset. Use frame labels only when an authoritative FPS is known. Otherwise use a compact pose index or time label and return the exact time mapping outside the image.

Keep labels minimal and unobtrusive. Size them for the final packed cell, with roughly 8 percent of cell height as a practical starting point and an outline around 10 percent of the font size. Prefer stamping compact labels onto the packed `Image` after resizing; otherwise scale a source overlay by the source-to-cell ratio. Do not advance engine frames only to refresh a label because that can change or destabilize the observed state. Keep node paths, legends, and explanations outside the image. Verify the completed sheet at its delivered size.

Use a content-unique filename when presenting a saved image by path. Reusing and overwriting one path can leave model clients showing a cached older image.

The screenshot worker may perform selection and custom Image construction with normal Godot code. Use `ctx.sheet()` for ordinary uniform grids, or compose an irregular layout directly when the task needs one. Save substantial reusable workers and project-specific utilities under `res://.fennara/scripts/screenshots/`. Rerun a complete worker through `script_path`, or `preload()`/`load()` a proven utility from a new worker when only part of its logic is needed. Fennara should not own animation-type-specific helper APIs.

An active editor can retain a previously loaded helper resource. If a changed or newly created preloaded utility is missing methods at runtime, use a new versioned path or restart the editor before depending on it; successful diagnostics for the main worker do not prove that every dependency was reloaded.
