import path from "node:path";
import { validateAddonArchive, validateAddonParts } from "./staging-addon-validation.mjs";
import {
  validateLinuxCefArchive,
  validatePlatformArchives,
  validateReleaseManifest,
} from "./staging-release-validation.mjs";
import { readJson } from "./staging-validation-files.mjs";
import { validateStagingCandidate } from "./staging-candidate.mjs";

export function validateStagingBuild({
  candidatePath,
  assetsDir,
  addonPartsDir,
  releaseManifestPath,
  linuxCefManifestPath,
}) {
  const candidate = validateStagingCandidate(readJson(candidatePath));
  const linuxCef = readJson(linuxCefManifestPath);
  const manifest = readJson(releaseManifestPath);

  validateAddonParts(candidate, addonPartsDir);
  validateReleaseManifest(candidate, manifest, assetsDir, linuxCef);
  validatePlatformArchives(candidate, assetsDir);
  validateAddonArchive(
    candidate,
    path.join(assetsDir, `fennara-release-addon-v${candidate.version}.zip`),
  );
  validateLinuxCefArchive(assetsDir, linuxCef);
  return candidate;
}
