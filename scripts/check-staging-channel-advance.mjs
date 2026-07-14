import { appendFileSync, existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { decideChannelAdvance } from "./staging-candidate.mjs";
import { parseArgs } from "./staging-validation-files.mjs";

const args = parseArgs(process.argv.slice(2), ["next", "current", "github-output"]);
const next = readJson(requiredArg("next"));
const currentPath = args.current ? path.resolve(args.current) : undefined;
const current = currentPath && existsSync(currentPath) ? readJson(currentPath) : undefined;
const action = decideChannelAdvance(current, next);

if (args["github-output"]) {
  appendFileSync(path.resolve(args["github-output"]), `action=${action}\n`);
}
console.log(action);

function readJson(file) {
  return JSON.parse(readFileSync(path.resolve(file), "utf8"));
}

function requiredArg(name) {
  if (!args[name]) {
    throw new Error(`Missing --${name}`);
  }
  return args[name];
}
