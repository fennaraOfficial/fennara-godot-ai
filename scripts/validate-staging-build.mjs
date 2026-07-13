import path from "node:path";
import { validateStagingBuild } from "./staging-build-validation.mjs";

const args = parseArgs(process.argv.slice(2));
const candidate = validateStagingBuild({
  candidatePath: resolveArg("candidate"),
  assetsDir: resolveArg("assets-dir"),
  addonPartsDir: resolveArg("addon-parts-dir"),
  releaseManifestPath: resolveArg("release-manifest"),
  linuxCefManifestPath: resolveArg("linux-cef-manifest"),
});
console.log(`Validated staging build ${candidate.version} from ${candidate.source_commit}`);

function parseArgs(rawArgs) {
  const parsed = {};
  for (let index = 0; index < rawArgs.length; index += 2) {
    const option = rawArgs[index];
    const value = rawArgs[index + 1];
    if (!option?.startsWith("--") || value === undefined) {
      throw new Error(`Invalid argument ${JSON.stringify(option)}`);
    }
    parsed[option.slice(2)] = value;
  }
  return parsed;
}

function resolveArg(name) {
  if (!args[name]) {
    throw new Error(`Missing --${name}`);
  }
  return path.resolve(args[name]);
}
