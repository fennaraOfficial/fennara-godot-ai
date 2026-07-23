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
const asyncBatch = read(
  "../../fennara-cpp/src/executor/async_batch.cpp",
);
const formatter = read(
  "../../fennara-cpp/src/tool_results/screenshot_scene.cpp",
);

test("screenshot capture is only reachable with an internal ownership token", () => {
  assert.doesNotMatch(header, /static godot::Dictionary capture\(\)/);
  assert.doesNotMatch(registration, /D_METHOD\("capture"\)/);
  assert.doesNotMatch(capture, /FennaraScreenshotSceneTool::capture\(\)/);
  assert.match(header, /capture_owned\(uint64_t owner\)/);
  assert.match(capture, /owner != _active_capture_owner_ref\(\)/);
});

test("screenshot script paths use the addon access boundary", () => {
  const start = asyncBatch.indexOf('name == "screenshot_scene"');
  const end = asyncBatch.indexOf('name == "validate_scene"');
  assert.ok(start >= 0, "screenshot_scene branch marker not found");
  assert.ok(end > start, "validate_scene marker must follow screenshot_scene");
  const screenshotBranch = asyncBatch.slice(start, end);
  assert.match(screenshotBranch, /screenshot_script_path/);
  assert.match(
    screenshotBranch,
    /complete_if_blocked\([\s\S]*screenshot_script_path/,
  );
});

test("screenshot receipts surface unavailable diagnostics", () => {
  assert.match(formatter, /Script diagnostics unavailable:/);
  assert.match(
    formatter,
    /copy_if_present\(envelope, raw_result, "diagnostic_success"\)/,
  );
  assert.match(
    formatter,
    /copy_if_present\(envelope, raw_result, "diagnostic_error"\)/,
  );
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
