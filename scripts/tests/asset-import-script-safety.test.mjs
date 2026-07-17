import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const executor = read(
  "../../fennara-cpp/src/executor/script_diagnostics_batch.cpp",
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

test("asset import receipts include the reusable worker script path", () => {
  assert.match(formatter, /lines\.append\("Script: " \+ script_path\)/);
  assert.match(formatter, /metadata\["script_path"\] =/);
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
