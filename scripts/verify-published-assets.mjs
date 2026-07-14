import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { parseArgs, requiredArg } from "./staging-validation-files.mjs";

const args = parseArgs(process.argv.slice(2), ["expected-dir", "actual-dir"]);
const expected = fileHashes(path.resolve(requiredArg(args, "expected-dir")));
const actual = fileHashes(path.resolve(requiredArg(args, "actual-dir")));

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
