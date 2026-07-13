import assert from "node:assert/strict";
import test from "node:test";

import {
  createStagingCandidate,
  decideChannelAdvance,
  validateStagingCandidate,
} from "../staging-candidate.mjs";
import { createChannelPointer } from "../release-identity.mjs";

const SOURCE_COMMIT = "0123456789abcdef0123456789abcdef01234567";

test("creates an exact pull-request staging candidate", () => {
  const candidate = createStagingCandidate({
    baseVersion: "0.3.9",
    pullRequest: "101",
    candidateNumber: "2",
    sourceCommit: SOURCE_COMMIT,
    sourceRepository: "fennaraOfficial/fennara-godot-ai",
  });

  assert.deepEqual(candidate, {
    schema_version: 1,
    track: "staging",
    version: "0.3.9-pr.101.2",
    release_tag: "v0.3.9-pr.101.2",
    channel: "pr-101",
    source_commit: SOURCE_COMMIT,
    base_version: "0.3.9",
    pull_request: 101,
    candidate_number: 2,
    source_repository: "fennaraOfficial/fennara-godot-ai",
  });
  assert.deepEqual(validateStagingCandidate(candidate), candidate);
});

test("rejects invalid base versions and channel numbers", () => {
  const common = {
    pullRequest: "101",
    candidateNumber: "1",
    sourceCommit: SOURCE_COMMIT,
    sourceRepository: "fennaraOfficial/fennara-godot-ai",
  };
  assert.throws(
    () => createStagingCandidate({ ...common, baseVersion: "0.3.9-rc.1" }),
    /base version must be a stable SemVer/,
  );
  assert.throws(
    () => createStagingCandidate({ ...common, baseVersion: "0.3.9", pullRequest: "01" }),
    /pull request must be a positive integer/,
  );
  assert.throws(
    () => createStagingCandidate({ ...common, baseVersion: "0.3.9", candidateNumber: "0" }),
    /candidate number must be a positive integer/,
  );
});

test("rejects malformed source repository provenance", () => {
  const candidate = createStagingCandidate({
    baseVersion: "0.3.9",
    pullRequest: "101",
    candidateNumber: "1",
    sourceCommit: SOURCE_COMMIT,
    sourceRepository: "fennaraOfficial/fennara-godot-ai",
  });
  assert.throws(
    () => validateStagingCandidate({ ...candidate, source_repository: "../repository" }),
    /source repository must use owner\/name format/,
  );
});

test("advances a channel monotonically and makes exact retries idempotent", () => {
  const first = createStagingCandidate({
    baseVersion: "0.3.9",
    pullRequest: "101",
    candidateNumber: "1",
    sourceCommit: SOURCE_COMMIT,
    sourceRepository: "fennaraOfficial/fennara-godot-ai",
  });
  const second = createStagingCandidate({
    ...first,
    baseVersion: "0.3.9",
    pullRequest: "101",
    candidateNumber: "2",
    sourceCommit: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    sourceRepository: "fennaraOfficial/fennara-godot-ai",
  });
  const firstPointer = createChannelPointer(first, "1".repeat(64));
  const secondPointer = createChannelPointer(second, "2".repeat(64));

  assert.equal(decideChannelAdvance(undefined, firstPointer), "create");
  assert.equal(decideChannelAdvance(firstPointer, firstPointer), "noop");
  assert.equal(decideChannelAdvance(firstPointer, secondPointer), "advance");
  assert.throws(
    () => decideChannelAdvance(secondPointer, firstPointer),
    /refusing to move pr-101 backward/,
  );
});
