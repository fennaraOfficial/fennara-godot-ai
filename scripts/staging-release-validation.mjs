import { readdirSync, statSync } from "node:fs";
import path from "node:path";
import { validateReleaseIdentity } from "./release-identity.mjs";
import { RELEASE_TARGETS } from "./release-targets.mjs";
import {
  assertFile,
  assertSameNames,
  assertVersion,
  inspectZip,
  requiredSha256,
  requiredString,
  sha256File,
} from "./staging-validation-files.mjs";

const LOCAL_BINARIES = [
  "fennara",
  "fennara-daemon",
  "fennara-daemon-runtime",
  "fennara-mcp",
  "fennara-mcp-runtime",
];

export function validateReleaseManifest(candidate, manifest, assetsDir, linuxCef) {
  if (manifest.version !== candidate.version) {
    throw new Error(`release manifest version does not match ${candidate.version}`);
  }
  const manifestIdentity = validateReleaseIdentity(manifest.release, candidate.version);
  const candidateIdentity = validateReleaseIdentity(candidate, candidate.version);
  if (JSON.stringify(manifestIdentity) !== JSON.stringify(candidateIdentity)) {
    throw new Error("release manifest identity does not match staging candidate");
  }

  const expectedArchives = expectedArchiveNames(candidate, linuxCef);
  const manifestAssets = manifestAssetRecords(manifest);
  assertSameNames(
    new Set(manifestAssets.map((asset) => asset.name)),
    expectedArchives,
    "release manifest assets",
  );
  for (const asset of manifestAssets) {
    const file = path.join(assetsDir, asset.name);
    assertFile(file, `release asset ${asset.name}`);
    if (sha256File(file) !== requiredSha256(asset.sha256, `manifest hash for ${asset.name}`)) {
      throw new Error(`release manifest hash mismatch for ${asset.name}`);
    }
  }

  const expectedFiles = new Set([
    ...expectedArchives,
    `fennara-release-manifest-v${candidate.version}.json`,
  ]);
  const actualFiles = new Set(
    readdirSync(assetsDir).filter((entry) => statSync(path.join(assetsDir, entry)).isFile()),
  );
  assertSameNames(actualFiles, expectedFiles, "assembled release files");
}

export function validatePlatformArchives(candidate, assetsDir) {
  for (const target of RELEASE_TARGETS) {
    const extension = target.platform === "windows" ? ".exe" : "";
    const cliName = `fennara-cli-${target.platform}-${target.arch}-v${candidate.version}.zip`;
    const cli = inspectZip(path.join(assetsDir, cliName), "VERSION", undefined);
    assertSameNames(
      new Set(cli.names),
      new Set(["VERSION", `bin/fennara${extension}`]),
      cliName,
    );
    assertVersion(cli.version, candidate.version, cliName);

    const localName = `fennara-release-local-${target.platform}-${target.arch}-v${candidate.version}.zip`;
    const local = inspectZip(path.join(assetsDir, localName), "VERSION", undefined);
    assertSameNames(
      new Set(local.names),
      new Set(["VERSION", ...LOCAL_BINARIES.map((binary) => `bin/${binary}${extension}`)]),
      localName,
    );
    assertVersion(local.version, candidate.version, localName);
  }
}

export function validateLinuxCefArchive(assetsDir, linuxCef) {
  const name = requiredString(linuxCef.archive?.name, "Linux CEF archive name");
  const file = path.join(assetsDir, name);
  if (sha256File(file) !== requiredSha256(linuxCef.archive?.sha256, "Linux CEF hash")) {
    throw new Error(`Linux CEF archive hash mismatch for ${name}`);
  }
}

function expectedArchiveNames(candidate, linuxCef) {
  return new Set([
    ...RELEASE_TARGETS.map(
      ({ platform, arch }) => `fennara-cli-${platform}-${arch}-v${candidate.version}.zip`,
    ),
    ...RELEASE_TARGETS.map(
      ({ platform, arch }) => `fennara-release-local-${platform}-${arch}-v${candidate.version}.zip`,
    ),
    `fennara-release-addon-v${candidate.version}.zip`,
    requiredString(linuxCef.archive?.name, "Linux CEF archive name"),
  ]);
}

function manifestAssetRecords(manifest) {
  return [
    ...Object.values(manifest.assets?.cli ?? {}),
    ...Object.values(manifest.assets?.local ?? {}),
    manifest.assets?.addon,
    ...(manifest.shared_runtimes ?? []).map((runtime) => runtime.archive),
  ].filter(Boolean);
}
