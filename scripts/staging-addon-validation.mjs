import { readFileSync } from "node:fs";
import path from "node:path";
import { validateReleaseIdentity } from "./release-identity.mjs";
import { RELEASE_TARGETS } from "./release-targets.mjs";
import {
  assertDirectory,
  assertPath,
  assertZipExecutable,
  assertVersion,
  inspectZip,
  readJson,
  treeHashes,
} from "./staging-validation-files.mjs";

export function validateAddonParts(candidate, addonPartsDir) {
  let commonFiles;
  for (const target of RELEASE_TARGETS) {
    const addonRoot = path.join(
      addonPartsDir,
      `fennara-addon-part-${target.platform}-${target.arch}`,
      "addons",
      "fennara",
    );
    assertDirectory(addonRoot, `addon part ${target.key}`);
    assertAddonIdentity(addonRoot, candidate);
    for (const relative of addonBinaryRequirements(target)) {
      assertPath(path.join(addonRoot, ...relative.split("/")), `${target.key} addon file ${relative}`);
    }
    const currentCommonFiles = treeHashes(addonRoot, "bin");
    if (commonFiles === undefined) {
      commonFiles = currentCommonFiles;
    } else if (JSON.stringify(commonFiles) !== JSON.stringify(currentCommonFiles)) {
      throw new Error(`non-binary addon files differ in ${target.key}`);
    }
  }
}

export function validateAddonArchive(candidate, addonFile) {
  const addon = inspectZip(
    addonFile,
    "addons/fennara/VERSION",
    "addons/fennara/release.json",
  );
  assertVersion(addon.version, candidate.version, path.basename(addonFile));
  const addonIdentity = validateReleaseIdentity(JSON.parse(addon.release), candidate.version);
  const candidateIdentity = validateReleaseIdentity(candidate, candidate.version);
  if (JSON.stringify(addonIdentity) !== JSON.stringify(candidateIdentity)) {
    throw new Error("all-platform addon identity does not match staging candidate");
  }
  for (const target of RELEASE_TARGETS) {
    for (const relative of addonBinaryRequirements(target)) {
      const entry = `addons/fennara/${relative}`;
      if (!addon.names.includes(entry) && !addon.names.some((name) => name.startsWith(`${entry}/`))) {
        throw new Error(`${path.basename(addonFile)} is missing ${entry}`);
      }
    }
  }
  for (const relative of [
    "bin/rg-linux-x86_64",
    "bin/rg-macos-arm64",
    "bin/libfennara.macos.editor.framework/libfennara.macos.editor",
  ]) {
    assertZipExecutable(addon, `addons/fennara/${relative}`);
  }
}

function assertAddonIdentity(addonRoot, candidate) {
  const version = readFileSync(path.join(addonRoot, "VERSION"), "utf8").trim();
  assertVersion(version, candidate.version, addonRoot);
  const identity = validateReleaseIdentity(readJson(path.join(addonRoot, "release.json")), version);
  const expected = validateReleaseIdentity(candidate, candidate.version);
  if (JSON.stringify(identity) !== JSON.stringify(expected)) {
    throw new Error(`addon part identity does not match staging candidate: ${addonRoot}`);
  }
}

function addonBinaryRequirements(target) {
  if (target.platform === "windows") {
    return ["bin/libfennara.windows.editor.x86_64.dll", "bin/rg-windows-x86_64.exe"];
  }
  if (target.platform === "linux") {
    return [
      "bin/libfennara.linux.editor.x86_64.so",
      "bin/libfennara_linux_cef_bridge.so",
      "bin/rg-linux-x86_64",
    ];
  }
  return ["bin/libfennara.macos.editor.framework", "bin/rg-macos-arm64"];
}
