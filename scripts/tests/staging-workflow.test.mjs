import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(new URL("../../.github/workflows/staging-release.yml", import.meta.url), "utf8");

test("dry runs cannot enter the publication job", () => {
  assert.match(workflow, /^  publish:\r?\n[\s\S]*?^    if: inputs\.publish$/m);
  assert.match(workflow, /publish:\r?\n[\s\S]*?default: false[\s\S]*?type: boolean/);
  assert.doesNotMatch(workflow, /pull_request_target|workflow_run/);
});

test("candidate builds are pinned and isolated per pull request", () => {
  assert.match(workflow, /group: staging-release-pr-\$\{\{ inputs\.pull_request \}\}/);
  assert.match(workflow, /cancel-in-progress: true/);
  assert.ok(
    (workflow.match(/ref: \$\{\{ needs\.resolve\.outputs\.source_commit \}\}/g) ?? []).length >= 2,
    "candidate source checkouts must use the frozen pull-request head",
  );
  assert.match(workflow, /test "\$\(git rev-parse HEAD\)" = "\$\{EXPECTED_SOURCE_COMMIT\}"/);
});

test("write credentials are confined to trusted publication", () => {
  const publishIndex = workflow.indexOf("\n  publish:");
  assert.ok(publishIndex > 0);
  assert.doesNotMatch(workflow.slice(0, publishIndex), /contents: write/);
  assert.match(workflow.slice(publishIndex), /contents: write/);
  assert.match(workflow.slice(publishIndex), /ref: \$\{\{ github\.sha \}\}/);
});

test("public smoke validation precedes monotonic pointer advancement", () => {
  const publicSmoke = workflow.indexOf("name: Smoke test public release downloads");
  const monotonicCheck = workflow.indexOf("name: Check monotonic channel advancement");
  const pointerAdvance = workflow.indexOf("name: Advance the per-PR staging pointer last");
  assert.ok(publicSmoke > 0 && publicSmoke < monotonicCheck && monotonicCheck < pointerAdvance);
  assert.doesNotMatch(workflow, /gh release (create|edit|upload) latest/);
});
