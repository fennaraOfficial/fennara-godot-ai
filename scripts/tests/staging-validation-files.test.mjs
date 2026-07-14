import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import test from "node:test";
import { fileURLToPath } from "node:url";

import {
  parseArgs,
  requireDescendantPath,
  requiredSafeFileName,
} from "../staging-validation-files.mjs";

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
