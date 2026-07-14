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

test("latest promotion freezes the legacy bootstrap without moving its tag", () => {
  const publishStep = namedStep(jobBlock(workflow, "publish"), "Publish release");
  const exactPublish = publishStep.indexOf('gh release edit "${tag}"');
  const latestCheck = publishStep.indexOf(
    'gh api "repos/${GITHUB_REPOSITORY}/releases/latest"',
  );
  const legacyTagCheck = publishStep.indexOf(
    "git ls-remote origin refs/tags/latest",
  );
  const legacyUpload = publishStep.indexOf(
    'gh release upload latest "${promotion_assets[@]}"',
  );
  const legacyVerify = publishStep.indexOf('--actual-dir "${legacy_assets_dir}"');
  const legacyPublish = publishStep.indexOf("gh release edit latest");
  assert.ok(
    exactPublish > 0 &&
      exactPublish < latestCheck &&
      latestCheck < legacyTagCheck &&
      legacyTagCheck < legacyUpload &&
      legacyUpload < legacyVerify &&
      legacyVerify < legacyPublish,
  );
  assert.match(publishStep, /gh release edit "\$\{tag\}"[\s\S]*?--latest/);
  assert.match(
    publishStep,
    /gh release edit latest[\s\S]*?--latest=false[\s\S]*?gh release verify latest --repo "\$\{GITHUB_REPOSITORY\}"[\s\S]*?read -r legacy_immutable/,
  );
  const legacyNoteContinuations = [
    ...publishStep.matchAll(/\n([^\S\r\n]*)Frozen with Fennara/g),
  ];
  assert.equal(legacyNoteContinuations.length, 2);
  assert.ok(legacyNoteContinuations.every((match) => match[1] === "          "));
  assert.match(publishStep, /Legacy latest bootstrap is already frozen; leaving it unchanged/);
  assert.doesNotMatch(publishStep, /git tag --force latest/);
  assert.doesNotMatch(publishStep, /git push --force[^\n]*refs\/tags\/latest/);
  assert.doesNotMatch(publishStep, /gh release edit latest[\s\S]*?--target/);
});

test("immutable-release preflight uses an administration token", () => {
  const preflight = namedStep(jobBlock(workflow, "publish"), "Require immutable GitHub releases");
  assert.match(preflight, /GH_TOKEN: \$\{\{ secrets\.RELEASE_ADMIN_TOKEN \}\}/);
  assert.match(preflight, /RELEASE_ADMIN_TOKEN must provide repository Administration read access/);
  assert.match(preflight, /--jq '\.enabled'/);
  assert.match(preflight, /immutable_enabled\}" != "true"/);
});

test("installers discover stable releases through GitHub Latest", () => {
  for (const relativePath of ["../../install.ps1", "../../install.sh"]) {
    const installer = readFileSync(new URL(relativePath, import.meta.url), "utf8");
    assert.match(installer, /api\.github\.com\/repos\/[^\s"']+\/releases\/latest/);
    assert.doesNotMatch(installer, /releases\/tags\/latest/);
  }

  const nativeDiscovery = readFileSync(
    new URL("../../fennara-cpp/src/release/discovery.cpp", import.meta.url),
    "utf8",
  );
  assert.match(
    nativeDiscovery,
    /\/repos\/fennaraOfficial\/fennara-godot-ai\/releases\/latest/,
  );
  assert.doesNotMatch(nativeDiscovery, /\/releases\/tags\/latest/);
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
