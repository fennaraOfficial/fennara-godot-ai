import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { createChannelPointer } from "./release-identity.mjs";
import { validateStagingCandidate } from "./staging-candidate.mjs";

const args = parseArgs(process.argv.slice(2));
const candidate = validateStagingCandidate(readJson(requiredArg("candidate")));
const manifestPath = path.resolve(requiredArg("manifest"));
const manifestBytes = readFileSync(manifestPath);
const manifest = JSON.parse(manifestBytes);
if (manifest.version !== candidate.version) {
  throw new Error(
    `release manifest version ${JSON.stringify(manifest.version)} does not match ${candidate.version}`,
  );
}
const pointer = createChannelPointer(
  candidate,
  createHash("sha256").update(manifestBytes).digest("hex"),
);
const outPath = path.resolve(requiredArg("out"));
mkdirSync(path.dirname(outPath), { recursive: true });
writeFileSync(outPath, `${JSON.stringify(pointer, null, 2)}\n`);
console.log(`Created ${outPath}`);

function readJson(file) {
  return JSON.parse(readFileSync(path.resolve(file), "utf8"));
}

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

function requiredArg(name) {
  if (!args[name]) {
    throw new Error(`Missing --${name}`);
  }
  return args[name];
}
