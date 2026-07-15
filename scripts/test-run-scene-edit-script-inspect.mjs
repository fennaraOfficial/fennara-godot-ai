import { spawnSync } from "node:child_process";
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const godot = process.argv[2] || process.env.GODOT_BIN;
if (!godot || !existsSync(godot)) {
  throw new Error(
    "Pass the Godot editor executable as the first argument or set GODOT_BIN.",
  );
}

const tempRoot = path.join(repoRoot, "temp");
const projectRoot = path.join(tempRoot, "run-scene-edit-script-inspect-test");
if (!projectRoot.startsWith(`${tempRoot}${path.sep}`)) {
  throw new Error("Refusing to create the smoke project outside the repository temp directory.");
}

rmSync(projectRoot, { recursive: true, force: true });
mkdirSync(projectRoot, { recursive: true });
cpSync(
  path.join(repoRoot, "godot_demo", "addons", "fennara"),
  path.join(projectRoot, "addons", "fennara"),
  { recursive: true },
);

const extensionPath = path.join(
  projectRoot,
  "addons",
  "fennara",
  "fennara.gdextension",
);
writeFileSync(
  extensionPath,
  readFileSync(extensionPath, "utf8").replace("reloadable = true", "reloadable = false"),
);

writeProjectFile("project.godot", `[application]

config/name="Fennara Imported Inspect Test"

[rendering]

renderer/rendering_method="gl_compatibility"
renderer/rendering_method.mobile="gl_compatibility"
`);
writeProjectFile(
  ".godot/extension_list.cfg",
  "res://addons/fennara/fennara.gdextension\n",
);
writeProjectFile(".godot/.gdignore", "");

writeProjectFile("assets/minimal_imported_scene.gltf", `${JSON.stringify({
  asset: { version: "2.0", generator: "Fennara integration test" },
  scene: 0,
  scenes: [{ nodes: [0] }],
  nodes: [{ name: "ImportedNode" }],
}, null, 2)}\n`);

writeProjectFile("worker_inspect.gd", `@tool
extends RefCounted

func run(ctx: Variant) -> void:
\tvar root: Node = ctx.get_scene_root()
\tif root == null:
\t\tctx.error("missing root")
\t\treturn
\tctx.log("root_class=%s" % root.get_class())
\tctx.log("child_count=%d" % root.get_child_count())
\tctx.log("read_only=%s" % ctx.is_read_only())
`);

writeProjectFile("worker_mutation.gd", `@tool
extends RefCounted

func run(ctx: Variant) -> void:
\tctx.set_scene_root(Node.new())
\tctx.own(Node.new())
\tctx.instance_scene(null, "res://missing.tscn", "Missing")
\tctx.remove_node("Missing")
\tctx.clear_children(Node.new())
\tctx.mark_modified()
`);

writeProjectFile("smoke.gd", `extends SceneTree

const IMPORT_WAIT_TIMEOUT_MSEC: int = 60_000

func _initialize() -> void:
\t_run.call_deferred()

func _run() -> void:
\tvar filesystem: EditorFileSystem = EditorInterface.get_resource_filesystem()
\tvar import_deadline_msec: int = Time.get_ticks_msec() + IMPORT_WAIT_TIMEOUT_MSEC
\tvar packed_scene_exists: bool = ResourceLoader.exists("res://assets/minimal_imported_scene.gltf", "PackedScene")
\twhile filesystem.is_scanning() or not packed_scene_exists:
\t\tif Time.get_ticks_msec() >= import_deadline_msec:
\t\t\tpush_error("FENNARA_IMPORT_TIMEOUT scanning=%s packed_scene_exists=%s" % [filesystem.is_scanning(), packed_scene_exists])
\t\t\tquit(1)
\t\t\treturn
\t\tawait process_frame
\t\tpacked_scene_exists = ResourceLoader.exists("res://assets/minimal_imported_scene.gltf", "PackedScene")

\tvar executor: FennaraExecutor = FennaraExecutor.new()
\tget_root().add_child(executor)
\texecutor.execute_tool_calls_async([{
\t\t"name": "run_scene_edit_script",
\t\t"args": {
\t\t\t"scene_path": "res://assets/minimal_imported_scene.gltf",
\t\t\t"script_path": "res://worker_inspect.gd",
\t\t\t"mode": "inspect",
\t\t},
\t}])
\tvar async_results: Array = await executor.all_tools_completed
\tvar inspect_result: Dictionary = async_results[0]["raw_result"]
\texecutor.queue_free()
\tprint("FENNARA_INSPECT_RESULT=" + JSON.stringify(inspect_result))

\tvar mutation_result: Dictionary = FennaraRunSceneEditScriptTool.execute({
\t\t"scene_path": "res://assets/minimal_imported_scene.gltf",
\t\t"script_path": "res://worker_mutation.gd",
\t\t"mode": "inspect",
\t})
\tprint("FENNARA_MUTATION_RESULT=" + JSON.stringify(mutation_result))

\tvar missing_result: Dictionary = FennaraRunSceneEditScriptTool.execute({
\t\t"scene_path": "res://assets/missing.gltf",
\t\t"script_path": "res://worker_inspect.gd",
\t\t"mode": "inspect",
\t})
\tprint("FENNARA_MISSING_RESULT=" + JSON.stringify(missing_result))
\tquit()
`);

const sourcePath = path.join(projectRoot, "assets", "minimal_imported_scene.gltf");
const sourceBefore = readFileSync(sourcePath, "utf8");
const smoke = runGodot([
  "--editor",
  "--headless",
  "--path",
  projectRoot,
  "--script",
  "res://smoke.gd",
]);

const inspect = markerResult(smoke, "FENNARA_INSPECT_RESULT=");
assert(inspect.success === true, "imported scene inspection should succeed");
assert(inspect.mode === "inspect", "inspection result should preserve mode");
assert(inspect.scene_saved === false, "inspection must not save the imported source");
assert(inspect.modified === false, "inspection must not report modification");
assert(inspect.validation === undefined, "inspection must skip saved-scene validation");
assert(inspect.logs.includes("read_only=true"), "inspection context should report read-only");

const mutation = markerResult(smoke, "FENNARA_MUTATION_RESULT=");
assert(mutation.success === false, "mutating context helpers must fail in inspect mode");
assert(mutation.scene_saved === false, "rejected inspection mutation must not save");
const mutationMessages = mutation.runtime_errors.map((error) => error.message).join("\n");
for (const helper of [
  "set_scene_root()",
  "own()",
  "instance_scene()",
  "remove_node()",
  "clear_children()",
  "mark_modified()",
]) {
  assert(mutationMessages.includes(helper), `${helper} should be rejected in inspect mode`);
}

const missing = markerResult(smoke, "FENNARA_MISSING_RESULT=");
assert(missing.success === false, "missing imported scene inspection should fail");
assert(missing.scene_saved === false, "missing scene failure must not save");
assert(
  String(missing.error).includes("not available for inspection"),
  "missing scene failure should explain that the source is unavailable",
);
assert(
  readFileSync(sourcePath, "utf8") === sourceBefore,
  "inspect mode must leave the imported source unchanged",
);

console.log("run_scene_edit_script inspect smoke passed");

function writeProjectFile(relativePath, content) {
  const destination = path.join(projectRoot, relativePath);
  mkdirSync(path.dirname(destination), { recursive: true });
  writeFileSync(destination, content);
}

function runGodot(args, executable = godot) {
  const result = spawnSync(executable, args, {
    encoding: "utf8",
    timeout: 120_000,
    maxBuffer: 16 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(
      `Godot exited with ${result.status}.\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`,
    );
  }
  return `${result.stdout}\n${result.stderr}`;
}

function markerResult(output, marker) {
  const line = output.split(/\r?\n/u).find((candidate) => candidate.startsWith(marker));
  if (!line) {
    throw new Error(`Godot output did not include ${marker}\n${output}`);
  }
  return JSON.parse(line.slice(marker.length));
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
