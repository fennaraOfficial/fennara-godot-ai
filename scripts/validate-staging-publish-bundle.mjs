import path from "node:path";
import { validateAddonArchive } from "./staging-addon-validation.mjs";
import {
  validateLinuxCefArchive,
  validatePlatformArchives,
  validateReleaseManifest,
} from "./staging-release-validation.mjs";
import { parseArgs, readJson } from "./staging-validation-files.mjs";
import { validateStagingCandidate } from "./staging-candidate.mjs";

const args = parseArgs(process.argv.slice(2), ["bundle"]);
const root = path.resolve(requiredArg("bundle"));
const assetsDir = path.join(root, "release-assets");
const candidate = validateStagingCandidate(readJson(path.join(root, "metadata", "candidate.json")));
const linuxCef = readJson(path.join(root, "metadata", "linux-cef.json"));
const manifest = readJson(
  path.join(assetsDir, `fennara-release-manifest-v${candidate.version}.json`),
);

validateReleaseManifest(candidate, manifest, assetsDir, linuxCef);
validatePlatformArchives(candidate, assetsDir);
validateAddonArchive(
  candidate,
  path.join(assetsDir, `fennara-release-addon-v${candidate.version}.zip`),
);
validateLinuxCefArchive(assetsDir, linuxCef);
console.log(`Validated publish bundle ${candidate.version}`);

function requiredArg(name) {
  if (!args[name]) {
    throw new Error(`Missing --${name}`);
  }
  return args[name];
}
