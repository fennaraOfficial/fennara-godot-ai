import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import {
  channelPointerAssetName,
  channelPointerRef,
  createChannelPointer,
  createReleaseIdentity,
  parseReleaseVersion,
  validateChannelPointer,
  validateReleaseIdentity,
} from "../release-identity.mjs";

const SOURCE_COMMIT = "0123456789abcdef0123456789abcdef01234567";

test("accepts stable and pull-request staging identities", () => {
  assert.deepEqual(createReleaseIdentity({ version: "0.3.9" }), {
    schema_version: 1,
    track: "stable",
    version: "0.3.9",
    release_tag: "v0.3.9",
  });
  assert.deepEqual(
    createReleaseIdentity({
      version: "0.3.9-pr.101.2",
      track: "staging",
      channel: "pr-101",
      sourceCommit: SOURCE_COMMIT,
    }),
    {
      schema_version: 1,
      track: "staging",
      version: "0.3.9-pr.101.2",
      release_tag: "v0.3.9-pr.101.2",
      channel: "pr-101",
      source_commit: SOURCE_COMMIT,
    },
  );
});

test("rejects ambiguous or mismatched staging identity", () => {
  assert.throws(
    () => createReleaseIdentity({
      version: "0.3.9-pr.125.1",
      track: "staging",
      channel: "pr-101",
      sourceCommit: SOURCE_COMMIT,
    }),
    /must end with -pr\.101/,
  );
  assert.throws(
    () => createReleaseIdentity({
      version: "0.3.9-preview-pr.101.2",
      track: "staging",
      channel: "pr-101",
      sourceCommit: SOURCE_COMMIT,
    }),
    /must end with -pr\.101/,
  );
  assert.throws(
    () => createReleaseIdentity({ version: "0.3.9-rc.1" }),
    /stable release versions/,
  );
});

test("native identity validation anchors the PR marker to the prerelease start", () => {
  const source = readFileSync(
    new URL("../../fennara-cpp/src/release/identity.cpp", import.meta.url),
    "utf8",
  );
  assert.match(source, /prerelease\.begins_with\(prefix\)/);
  assert.doesNotMatch(source, /identity\.version\.find\(prefix\)/);
});

test("validates identity tag and VERSION agreement", () => {
  const identity = createReleaseIdentity({ version: "0.3.9" });
  assert.deepEqual(validateReleaseIdentity(identity, "0.3.9"), identity);
  assert.throws(
    () => validateReleaseIdentity({ ...identity, release_tag: "v0.4.0" }),
    /tag must be v0\.3\.9/,
  );
  assert.throws(() => validateReleaseIdentity(identity, "0.4.0"), /does not match VERSION/);
});

test("creates an isolated pointer name and record per pull request", () => {
  const identity = createReleaseIdentity({
    version: "0.3.9-pr.101.2",
    track: "staging",
    channel: "pr-101",
    sourceCommit: SOURCE_COMMIT,
  });
  assert.equal(channelPointerRef("pr-101"), "fennara-staging/pr-101");
  assert.equal(channelPointerAssetName("pr-101"), "fennara-staging-channel-pr-101.json");
  const pointer = createChannelPointer(identity, "a".repeat(64));
  assert.deepEqual(pointer, {
    schema_version: 1,
    channel: "pr-101",
    version: "0.3.9-pr.101.2",
    release_tag: "v0.3.9-pr.101.2",
    source_commit: SOURCE_COMMIT,
    release_manifest_sha256: "a".repeat(64),
  });
  assert.deepEqual(validateChannelPointer(pointer, "pr-101"), pointer);
});

test("accepts SemVer prereleases and rejects invalid numeric identifiers", () => {
  assert.equal(parseReleaseVersion("0.3.9-pr.101.2").prerelease, "pr.101.2");
  assert.throws(() => parseReleaseVersion("0.3.9-pr.0101.2"), /valid SemVer/);
  assert.throws(() => parseReleaseVersion("0.3"), /valid SemVer/);
});
