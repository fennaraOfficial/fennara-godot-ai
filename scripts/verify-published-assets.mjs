import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

const args = parseArgs(process.argv.slice(2));
const expected = fileHashes(path.resolve(requiredArg("expected-dir")));
const actual = fileHashes(path.resolve(requiredArg("actual-dir")));

if (JSON.stringify(expected) !== JSON.stringify(actual)) {
  throw new Error(
    `published release assets differ from validated assets\nexpected: ${JSON.stringify(expected, null, 2)}\nactual: ${JSON.stringify(actual, null, 2)}`,
  );
}
console.log(`Verified ${Object.keys(expected).length} published release assets`);

function fileHashes(directory) {
  const records = {};
  for (const entry of readdirSync(directory).sort()) {
    const file = path.join(directory, entry);
    if (!statSync(file).isFile()) {
      throw new Error(`Unexpected directory in release assets: ${file}`);
    }
    records[entry] = createHash("sha256").update(readFileSync(file)).digest("hex");
  }
  return records;
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
