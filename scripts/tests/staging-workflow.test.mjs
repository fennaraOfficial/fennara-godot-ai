import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(new URL("../../.github/workflows/staging-release.yml", import.meta.url), "utf8");

test("dry runs cannot enter the publication job", () => {
  assert.match(workflow, /^  publish:\r?\n[\s\S]*?^    if: inputs\.publish$/m);
  assert.match(
    workflow,
    /^      publish:\r?\n        description: [^\r\n]+\r?\n        required: true\r?\n        default: false\r?\n        type: boolean$/m,
  );
  assert.doesNotMatch(workflow, /pull_request_target|workflow_run/);
});

test("candidate builds are pinned and isolated per pull request", () => {
  assert.match(workflow, /group: staging-release-pr-\$\{\{ inputs\.pull_request \}\}/);
  assert.match(workflow, /cancel-in-progress: true/);
  const checkouts = checkoutSteps(workflow);
  assert.ok(checkouts.length >= 2, "workflow must contain trusted and candidate checkouts");
  for (const checkout of checkouts) {
    if (/name: Checkout trusted /.test(checkout)) {
      assert.match(checkout, /ref: \$\{\{ github\.sha \}\}/);
    } else {
      assert.match(checkout, /ref: \$\{\{ needs\.resolve\.outputs\.source_commit \}\}/);
    }
  }
  assert.match(workflow, /test "\$\(git rev-parse HEAD\)" = "\$\{EXPECTED_SOURCE_COMMIT\}"/);
});

test("write credentials are confined to trusted publication", () => {
  assert.equal(
    (workflow.match(/^\s+contents: write$/gm) ?? []).length,
    1,
    "only the publish job may request contents write access",
  );
  const jobs = jobBlocks(workflow);
  for (const [name, block] of jobs) {
    if (name === "publish") {
      assert.match(block, /^    permissions:\r?\n(?:      [^\r\n]+\r?\n)*?      contents: write$/m);
      assert.match(block, /ref: \$\{\{ github\.sha \}\}/);
    } else {
      assert.doesNotMatch(block, /contents: write/);
    }
  }
});

test("public smoke validation precedes monotonic pointer advancement", () => {
  const publicSmoke = workflow.indexOf("name: Smoke test public release downloads");
  const monotonicCheck = workflow.indexOf("name: Check monotonic channel advancement");
  const pointerAdvance = workflow.indexOf("name: Advance the per-PR staging pointer last");
  assert.ok(publicSmoke > 0 && publicSmoke < monotonicCheck && monotonicCheck < pointerAdvance);
  assert.doesNotMatch(workflow, /gh release (create|edit|upload) latest/);
});

test("shell commands do not interpolate resolve outputs directly", () => {
  for (const block of runBlocks(workflow)) {
    assert.doesNotMatch(block, /\$\{\{ needs\.resolve\.outputs\./);
  }
});

function runBlocks(source) {
  const lines = source.split(/\r?\n/);
  const blocks = [];
  for (let index = 0; index < lines.length; index += 1) {
    const match = /^(\s*)run:\s*(.*)$/.exec(lines[index]);
    if (!match) {
      continue;
    }
    const indent = match[1].length;
    const block = [match[2]];
    while (index + 1 < lines.length) {
      const next = lines[index + 1];
      if (next.trim() && next.match(/^\s*/)[0].length <= indent) {
        break;
      }
      block.push(next);
      index += 1;
    }
    blocks.push(block.join("\n"));
  }
  return blocks;
}

function checkoutSteps(source) {
  return source
    .split(/(?=^      - name:)/m)
    .filter((block) => /uses: actions\/checkout@/.test(block));
}

function jobBlocks(source) {
  const jobsSource = source.split(/^jobs:\r?\n/m)[1];
  assert.ok(jobsSource, "workflow must contain jobs");
  const blocks = new Map();
  for (const match of jobsSource.matchAll(/^  ([a-zA-Z0-9_-]+):\r?\n([\s\S]*?)(?=^  [a-zA-Z0-9_-]+:|(?![\s\S]))/gm)) {
    blocks.set(match[1], match[0]);
  }
  return blocks;
}
