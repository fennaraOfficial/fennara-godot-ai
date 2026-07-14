import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { createStagingCandidate } from "./staging-candidate.mjs";
import { parseArgs, requiredArg } from "./staging-validation-files.mjs";

const args = parseArgs(process.argv.slice(2), [
  "base-version",
  "pull-request",
  "candidate",
  "source-commit",
  "source-repository",
  "out",
  "github-output",
]);
const outPath = path.resolve(requiredArg(args, "out"));
const candidate = createStagingCandidate({
  baseVersion: requiredArg(args, "base-version"),
  pullRequest: requiredArg(args, "pull-request"),
  candidateNumber: requiredArg(args, "candidate"),
  sourceCommit: requiredArg(args, "source-commit"),
  sourceRepository: requiredArg(args, "source-repository"),
});

mkdirSync(path.dirname(outPath), { recursive: true });
writeFileSync(outPath, `${JSON.stringify(candidate, null, 2)}\n`);

if (args["github-output"]) {
  const outputs = {
    version: candidate.version,
    channel: candidate.channel,
    pull_request: candidate.pull_request,
    source_commit: candidate.source_commit,
    source_repository: candidate.source_repository,
    release_tag: candidate.release_tag,
  };
  appendFileSync(
    path.resolve(args["github-output"]),
    Object.entries(outputs)
      .map(([name, value]) => `${name}=${value}`)
      .join("\n") + "\n",
  );
}

console.log(`Created ${outPath}`);
console.log(`Staging candidate ${candidate.version} from ${candidate.source_repository}@${candidate.source_commit}`);
