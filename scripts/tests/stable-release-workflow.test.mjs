import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(
  new URL("../../.github/workflows/release.yml", import.meta.url),
  "utf8",
);

test("stable publication reconciles matching drafts before immutable promotion", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  assert.match(publishStep, /reconcile_draft=false/);
  assert.match(publishStep, /Resuming matching draft release/);
  assert.match(
    publishStep,
    /verify-published-assets\.mjs[\s\S]*?--actual-dir "\$\{exact_assets_dir\}"[\s\S]*?--draft=false/,
  );
});

test("latest promotion verifies exact bytes and moves the tag last", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  const latestUpload = publishStep.indexOf('gh release upload latest "${promotion_assets[@]}"');
  const latestVerify = publishStep.indexOf('--actual-dir "${latest_assets_dir}"');
  const latestEdit = publishStep.indexOf("gh release edit latest");
  const tagPush = publishStep.indexOf("git push --force origin refs/tags/latest");
  assert.ok(
    latestUpload > 0 && latestUpload < latestVerify && latestVerify < latestEdit && latestEdit < tagPush,
  );
});

test("immutable-release preflight uses an administration token", () => {
  const preflight = namedStep(jobBlock(workflow, "publish"), "Require immutable GitHub releases");
  assert.match(preflight, /GH_TOKEN: \$\{\{ secrets\.RELEASE_ADMIN_TOKEN \}\}/);
  assert.match(preflight, /RELEASE_ADMIN_TOKEN must provide repository Administration read access/);
});

function jobBlock(source, jobName) {
  const match = new RegExp(`^  ${jobName}:\\r?\\n([\\s\\S]*?)(?=^  [a-zA-Z0-9_-]+:|(?![\\s\\S]))`, "m").exec(source);
  assert.ok(match, `missing ${jobName} job`);
  return match[0];
}

function namedStep(job, stepName) {
  const escaped = stepName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = new RegExp(`^      - name: ${escaped}\\r?\\n([\\s\\S]*?)(?=^      - name:|(?![\\s\\S]))`, "m").exec(job);
  assert.ok(match, `missing ${stepName} step`);
  return match[0];
}
