import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { createStagingCandidate } from "../staging-candidate.mjs";
import {
  assertSafeZipMembers,
  parseArgs,
  requireDescendantPath,
  requiredArg,
  requiredSafeFileName,
} from "../staging-validation-files.mjs";

const SOURCE_COMMIT = "0123456789abcdef0123456789abcdef01234567";

test("release validation accepts only plain asset file names", () => {
  assert.equal(requiredSafeFileName("fennara-release.zip", "asset"), "fennara-release.zip");
  for (const name of ["../secret", "..\\secret", "/tmp/secret", "C:\\temp\\secret", ".", ".."]) {
    assert.throws(() => requiredSafeFileName(name, "asset"), /plain file name/);
  }
});

test("staging pointer writer rejects unknown and duplicate options", () => {
  const script = fileURLToPath(new URL("../write-staging-pointer.mjs", import.meta.url));
  const unknown = spawnSync(process.execPath, [script, "--unknown", "value"], {
    encoding: "utf8",
  });
  assert.notEqual(unknown.status, 0);
  assert.match(unknown.stderr, /Unknown option --unknown/);

  const duplicate = spawnSync(
    process.execPath,
    [script, "--candidate", "one", "--candidate", "two"],
    { encoding: "utf8" },
  );
  assert.notEqual(duplicate.status, 0);
  assert.match(duplicate.stderr, /Duplicate option --candidate/);
});

test("shared staging argument parsing rejects unknown and duplicate options", () => {
  assert.deepEqual(parseArgs(["--bundle", "candidate"], ["bundle"]), {
    bundle: "candidate",
  });
  assert.throws(() => parseArgs(["--unknown", "value"], ["bundle"]), /Unknown option/);
  assert.throws(
    () => parseArgs(["--bundle", "one", "--bundle", "two"], ["bundle"]),
    /Duplicate option/,
  );
  assert.equal(requiredArg({ bundle: "candidate" }, "bundle"), "candidate");
  assert.throws(() => requiredArg({}, "bundle"), /Missing --bundle/);
});

test("addon archive validation rejects unsafe and non-regular members", () => {
  const regular = 0o100644;
  assert.deepEqual(
    assertSafeZipMembers([
      { name: "addons/fennara/VERSION", mode: regular, create_system: 3, is_dir: false },
      { name: "addons/fennara/bin/tool", mode: 0o100755, create_system: 3, is_dir: false },
    ]),
    ["addons/fennara/VERSION", "addons/fennara/bin/tool"],
  );

  for (const name of [
    "/absolute",
    "C:/absolute",
    "../outside",
    "addons/../outside",
    "addons\\outside",
    "addons//outside",
  ]) {
    assert.throws(
      () => assertSafeZipMembers([{ name, mode: regular, create_system: 3, is_dir: false }]),
      /unsafe member path/,
    );
  }
  assert.throws(
    () => assertSafeZipMembers([
      { name: "duplicate", mode: regular, create_system: 3, is_dir: false },
      { name: "duplicate", mode: regular, create_system: 3, is_dir: false },
    ]),
    /duplicate member/,
  );
  for (const member of [
    { name: "directory", mode: 0o040755, create_system: 3, is_dir: true },
    { name: "symlink", mode: 0o120777, create_system: 3, is_dir: false },
    { name: "unknown", mode: 0, create_system: 0, is_dir: false },
  ]) {
    assert.throws(() => assertSafeZipMembers([member]), /not a regular Unix file/);
  }
});

test("staging pointer writer rejects same-version manifest provenance mismatches", () => {
  const tempParent = fileURLToPath(new URL("../../temp/", import.meta.url));
  mkdirSync(tempParent, { recursive: true });
  const directory = mkdtempSync(path.join(tempParent, "staging-pointer-provenance-"));
  try {
    const candidate = createStagingCandidate({
      baseVersion: "0.3.9",
      pullRequest: "101",
      candidateNumber: "2",
      sourceCommit: SOURCE_COMMIT,
      sourceRepository: "fennaraOfficial/fennara-godot-ai",
    });
    const candidatePath = path.join(directory, "candidate.json");
    const manifestPath = path.join(directory, "manifest.json");
    const pointerPath = path.join(directory, "pointer.json");
    writeFileSync(candidatePath, JSON.stringify(candidate));
    writeFileSync(
      manifestPath,
      JSON.stringify({
        version: candidate.version,
        release: { ...candidate, source_commit: "f".repeat(40) },
      }),
    );

    const script = fileURLToPath(new URL("../write-staging-pointer.mjs", import.meta.url));
    const result = spawnSync(
      process.execPath,
      [
        script,
        "--candidate",
        candidatePath,
        "--manifest",
        manifestPath,
        "--out",
        pointerPath,
      ],
      { encoding: "utf8" },
    );
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /identity does not match staging candidate/);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("recursive release smoke output must stay below RUNNER_TEMP", () => {
  const root = fileURLToPath(new URL("../../temp/smoke-root/", import.meta.url));
  assert.equal(
    requireDescendantPath(root, `${root}/candidate`, "--download-dir"),
    fileURLToPath(new URL("../../temp/smoke-root/candidate", import.meta.url)),
  );
  assert.throws(() => requireDescendantPath(root, root, "--download-dir"), /inside RUNNER_TEMP/);
  assert.throws(
    () => requireDescendantPath(root, `${root}/../outside`, "--download-dir"),
    /inside RUNNER_TEMP/,
  );
});
