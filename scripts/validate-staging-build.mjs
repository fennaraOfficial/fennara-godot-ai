import path from "node:path";
import { validateStagingBuild } from "./staging-build-validation.mjs";
import { parseArgs } from "./staging-validation-files.mjs";

const args = parseArgs(process.argv.slice(2), [
  "candidate",
  "assets-dir",
  "addon-parts-dir",
  "release-manifest",
  "linux-cef-manifest",
]);
const candidate = validateStagingBuild({
  candidatePath: resolveArg("candidate"),
  assetsDir: resolveArg("assets-dir"),
  addonPartsDir: resolveArg("addon-parts-dir"),
  releaseManifestPath: resolveArg("release-manifest"),
  linuxCefManifestPath: resolveArg("linux-cef-manifest"),
});
console.log(`Validated staging build ${candidate.version} from ${candidate.source_commit}`);

function resolveArg(name) {
  if (!args[name]) {
    throw new Error(`Missing --${name}`);
  }
  return path.resolve(args[name]);
}
