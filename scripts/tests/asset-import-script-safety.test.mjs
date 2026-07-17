import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const executor = read(
  "../../fennara-cpp/src/executor/script_diagnostics_batch.cpp",
);
const tool = read(
  "../../fennara-cpp/src/tools/run_asset_import_script/run_asset_import_script.cpp",
);
const sidecar = read(
  "../../fennara-cpp/src/tools/run_asset_import_script/sidecar.cpp",
);
const formatter = read(
  "../../fennara-cpp/src/tool_results/run_asset_import_script.cpp",
);

test("asset import diagnostic failures are finalized before dispatch", () => {
  const failure = executor.indexOf(
    "Script diagnostics reported errors. Patch the saved script_path and rerun.",
  );
  const finalize = executor.indexOf(
    "FennaraRunAssetImportScriptTool::finalize_result(merged)",
    failure,
  );
  const dispatch = executor.indexOf("_on_async_tool_complete(", finalize);
  assert.ok(failure >= 0 && finalize > failure && dispatch > finalize);
});

test("asset import results only report verified persistence as modified", () => {
  const staged = tool.indexOf(
    'result["changes"] = context->get_staged_changes();',
  );
  const initiallyUnmodified = tool.indexOf('result["modified"] = false;', staged);
  const apply = tool.indexOf("apply_and_reimport(snapshot, changes)", staged);
  const persisted = tool.indexOf('result["modified"] = true;', apply);
  assert.ok(
    staged >= 0 &&
      initiallyUnmodified > staged &&
      apply > initiallyUnmodified &&
      persisted > apply,
  );
});

test("asset import verification checks all outputs but bounds its receipt", () => {
  assert.match(sidecar, /variant_paths\([^;]+dest_files[^;]+\);/s);
  assert.match(
    sidecar,
    /for \(int i = 0; i < generated_files\.size\(\); i\+\+\)/,
  );
  assert.match(
    sidecar,
    /reported_generated_files\.size\(\) < kMaximumCollectedPaths/,
  );
  assert.match(sidecar, /missing_output_count == 0/);
});

test("asset import receipts include the reusable worker script path", () => {
  assert.match(formatter, /lines\.append\("Script: " \+ script_path\)/);
  assert.match(formatter, /metadata\["script_path"\] =/);
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
