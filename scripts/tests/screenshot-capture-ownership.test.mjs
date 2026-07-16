import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const header = read(
  "../../fennara-cpp/include/fennara/tools/screenshot_scene.hpp",
);
const registration = read(
  "../../fennara-cpp/src/tools/screenshot_scene/screenshot_scene.cpp",
);
const capture = read(
  "../../fennara-cpp/src/tools/screenshot_scene/capture.cpp",
);

test("screenshot capture is only reachable with an internal ownership token", () => {
  assert.doesNotMatch(header, /static godot::Dictionary capture\(\)/);
  assert.doesNotMatch(registration, /D_METHOD\("capture"\)/);
  assert.doesNotMatch(capture, /FennaraScreenshotSceneTool::capture\(\)/);
  assert.match(header, /capture_owned\(uint64_t owner\)/);
  assert.match(capture, /owner != _active_capture_owner_ref\(\)/);
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
