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
  const latestUpload = workflow.indexOf('gh release upload latest "${promotion_assets[@]}"');
  const latestVerify = workflow.indexOf('--actual-dir "${latest_assets_dir}"');
  const latestEdit = workflow.indexOf("gh release edit latest");
  const tagPush = workflow.indexOf("git push --force origin refs/tags/latest");
  assert.ok(
    latestUpload > 0 && latestUpload < latestVerify && latestVerify < latestEdit && latestEdit < tagPush,
  );
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
