import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(
  new URL("../../.github/workflows/release.yml", import.meta.url),
  "utf8",
);

test("stable publication reconciles matching drafts before immutable promotion", () => {
  assert.match(workflow, /reconcile_draft=false/);
  assert.match(workflow, /Resuming matching draft release/);
  assert.match(
    workflow,
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
