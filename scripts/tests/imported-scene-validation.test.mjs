import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const header = read("../../fennara-cpp/include/fennara/tools/validate_scene.hpp");
const source = read("../../fennara-cpp/src/tools/validate_scene/validate_scene.cpp");
const executor = read("../../fennara-cpp/src/executor/validate_scene_batch.cpp");

test("runtime validation only launches Godot command-line scene formats", () => {
  assert.match(source, /extension == "tscn"/);
  assert.match(source, /extension == "scn"/);
  assert.match(source, /extension == "escn"/);
  assert.match(source, /extension == "res"/);
  assert.match(source, /extension == "tres"/);
  assert.doesNotMatch(source, /extension == "glb"/);
  assert.doesNotMatch(source, /extension == "gltf"/);
});

test("sync and async validation explain imported-source runtime skips", () => {
  assert.match(header, /static godot::String runtime_skip_reason/);
  assert.match(source, /runtime_skip_reason\(scene_result\)/);
  assert.match(executor, /runtime_skip_reason\(scene_result\)/);
  assert.match(
    source,
    /command-line scene runner cannot launch imported source/,
  );
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
