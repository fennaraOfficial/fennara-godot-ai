import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(
  new URL("../../.github/workflows/release.yml", import.meta.url),
  "utf8",
);

test("stable publication reconciles matching drafts and verifies exact bytes", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  assert.match(publishStep, /reconcile_draft=false/);
  assert.match(publishStep, /Resuming matching draft release/);
  assert.match(
    publishStep,
    /verify-published-assets\.mjs[\s\S]*?--actual-dir "\$\{exact_assets_dir\}"[\s\S]*?--draft=false/,
  );
});

test("stable promotion marks the exact release as GitHub Latest", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  assert.match(
    publishStep,
    /gh release edit "\$\{tag\}"[\s\S]*?--draft=false[\s\S]*?--latest[\s\S]*?releases\/latest[\s\S]*?expected \$\{tag\}/,
  );
});

test("stable publication safely resumes a matching published release", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  assert.match(publishStep, /resume_published=false/);
  assert.match(publishStep, /Resuming matching published release/);
  assert.match(publishStep, /exact_tag_sha[\s\S]*?desired_assets[\s\S]*?published_exact_assets/);
  assert.match(publishStep, /for attempt in \{1\.\.12\}[\s\S]*?sleep 5/);
});

test("stable releases include version, linked source commit, and release date", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  assert.match(publishStep, /short_sha="\$\{GITHUB_SHA:0:7\}"/);
  assert.match(publishStep, /released_date="\$\(date -u \+'%d-%m-%Y'\)"/);
  assert.match(
    publishStep,
    /Fennara Godot AI %s\\nBuilt from \[%s\]\(https:\/\/github\.com\/%s\/commit\/%s\)\.\\nReleased: %s/,
  );
});

test("stable publication does not depend on the retired latest tag or immutability", () => {
  const publishJob = jobBlock(workflow, "publish");
  assert.doesNotMatch(publishJob, /Require immutable GitHub releases/);
  assert.doesNotMatch(publishJob, /gh release verify/);
  assert.doesNotMatch(publishJob, /release (?:view|create|edit|upload|delete) latest/);
  assert.doesNotMatch(publishJob, /refs\/tags\/latest/);
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
