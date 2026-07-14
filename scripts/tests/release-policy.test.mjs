import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { createReleaseIdentity } from "../release-identity.mjs";
import { minimumCliVersionForTrack, RELEASE_POLICY } from "../release-policy.mjs";
import { RELEASE_TARGETS } from "../release-targets.mjs";

const workflows = ["package-preview.yml", "release.yml", "staging-release.yml"].map(
  (name) =>
    readFileSync(new URL(`../../.github/workflows/${name}`, import.meta.url), "utf8"),
);

test("release policy owns the minimum CLI version for every release track", () => {
  assert.deepEqual(RELEASE_POLICY.minimumCliVersionByTrack, {
    stable: "0.3.12",
    staging: "0.3.8",
  });
  assert.equal(minimumCliVersionForTrack("stable"), "0.3.12");
  assert.equal(minimumCliVersionForTrack("staging"), "0.3.8");
  assert.throws(() => minimumCliVersionForTrack("preview"), /does not define track/);
  assert.throws(() => minimumCliVersionForTrack("toString"), /does not define track/);
});

test("release workflows cannot override the policy minimum", () => {
  for (const workflow of workflows) {
    assert.doesNotMatch(workflow, /--minimum-cli-version/);
  }
});

test("manifest writer applies release policy to stable and staging identities", () => {
  const tempParent = fileURLToPath(new URL("../../temp/", import.meta.url));
  mkdirSync(tempParent, { recursive: true });
  const directory = mkdtempSync(path.join(tempParent, "release-policy-"));
  const script = fileURLToPath(new URL("../write-release-manifest.mjs", import.meta.url));

  try {
    for (const identity of [
      createReleaseIdentity({ version: "1.2.3" }),
      createReleaseIdentity({
        version: "1.2.3-pr.101.1",
        track: "staging",
        channel: "pr-101",
        sourceCommit: "0123456789abcdef0123456789abcdef01234567",
      }),
    ]) {
      const assetsDir = path.join(directory, identity.track, "assets");
      const identityPath = path.join(directory, identity.track, "release.json");
      const outPath = path.join(directory, identity.track, "manifest.json");
      mkdirSync(assetsDir, { recursive: true });
      writeAssets(assetsDir, identity.version);
      writeFileSync(identityPath, JSON.stringify(identity));

      const result = spawnSync(
        process.execPath,
        [
          script,
          "--version",
          identity.version,
          "--assets-dir",
          assetsDir,
          "--linux-cef-manifest",
          path.join(directory, "missing-linux-cef.json"),
          "--release-identity",
          identityPath,
          "--out",
          outPath,
        ],
        { encoding: "utf8" },
      );
      assert.equal(result.status, 0, result.stderr);
      const manifest = JSON.parse(readFileSync(outPath, "utf8"));
      assert.equal(manifest.release.track, identity.track);
      assert.equal(
        manifest.minimum_cli_version,
        minimumCliVersionForTrack(identity.track),
      );
    }
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("manifest writer rejects the removed minimum CLI override", () => {
  const script = fileURLToPath(new URL("../write-release-manifest.mjs", import.meta.url));
  const result = spawnSync(process.execPath, [script, "--minimum-cli-version", "9.9.9"], {
    encoding: "utf8",
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Unknown option: --minimum-cli-version/);
});

function writeAssets(assetsDir, version) {
  for (const target of RELEASE_TARGETS) {
    writeFileSync(
      path.join(assetsDir, `fennara-cli-${target.platform}-${target.arch}-v${version}.zip`),
      `cli-${target.key}`,
    );
    writeFileSync(
      path.join(
        assetsDir,
        `fennara-release-local-${target.platform}-${target.arch}-v${version}.zip`,
      ),
      `local-${target.key}`,
    );
  }
  writeFileSync(
    path.join(assetsDir, `fennara-release-addon-v${version}.zip`),
    "addon",
  );
}
