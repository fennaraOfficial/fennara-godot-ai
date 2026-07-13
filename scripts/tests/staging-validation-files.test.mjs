import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { requiredSafeFileName } from "../staging-validation-files.mjs";

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
