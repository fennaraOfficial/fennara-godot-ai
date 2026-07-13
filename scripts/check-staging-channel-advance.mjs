import { appendFileSync, existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { decideChannelAdvance } from "./staging-candidate.mjs";

const args = parseArgs(process.argv.slice(2));
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
