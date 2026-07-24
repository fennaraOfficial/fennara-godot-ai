import assert from "node:assert/strict";
import { createHash } from "node:crypto";
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
import { RELEASE_TARGETS } from "../release-targets.mjs";

test("Linux CEF release record includes the native runtime marker identity", () => {
  const tempParent = fileURLToPath(new URL("../../temp/", import.meta.url));
  mkdirSync(tempParent, { recursive: true });
  const directory = mkdtempSync(path.join(tempParent, "release-manifest-"));
  const assetsDir = path.join(directory, "assets");
  const version = "1.2.3";
  const cefAssetName = "fennara-webview-cef-linux-x64-test.zip";

  try {
    mkdirSync(assetsDir, { recursive: true });
    writeReleaseAssets(assetsDir, version);
    const cefBytes = Buffer.from("cef-runtime");
    writeFileSync(path.join(assetsDir, cefAssetName), cefBytes);

    const cefManifestPath = path.join(directory, "linux-cef.json");
    writeFileSync(
      cefManifestPath,
      JSON.stringify({
        schema_version: 1,
        runtime: "cef",
        platform: "linux",
        arch: "x86_64",
        platform_arch: "linux-x64",
        version: "139.0.28+chromium-139.0.7258.139",
        enabled: true,
        required_files: ["libcef.so"],
        archive: {
          format: "zip",
          name: cefAssetName,
          sha256: createHash("sha256").update(cefBytes).digest("hex"),
        },
      }),
    );

    const identityPath = path.join(directory, "release.json");
    writeFileSync(identityPath, JSON.stringify(createReleaseIdentity({ version })));
    const outPath = path.join(directory, "manifest.json");
    const result = spawnSync(
      process.execPath,
      [
        fileURLToPath(new URL("../write-release-manifest.mjs", import.meta.url)),
        "--version",
        version,
        "--assets-dir",
        assetsDir,
        "--linux-cef-manifest",
        cefManifestPath,
        "--release-identity",
        identityPath,
        "--out",
        outPath,
      ],
      { encoding: "utf8" },
    );

    assert.equal(result.status, 0, result.stderr);
    const manifest = JSON.parse(readFileSync(outPath, "utf8"));
    assert.equal(manifest.shared_runtimes[0].kind, "cef");
    assert.equal(manifest.shared_runtimes[0].runtime, "cef");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

function writeReleaseAssets(assetsDir, version) {
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
