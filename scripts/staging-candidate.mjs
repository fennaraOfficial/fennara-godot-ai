import {
  createReleaseIdentity,
  parseReleaseVersion,
  validateChannelPointer,
  validateReleaseIdentity,
} from "./release-identity.mjs";

const SOURCE_REPOSITORY_PATTERN = /^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/;

export function createStagingCandidate({
  baseVersion,
  pullRequest,
  candidateNumber,
  sourceCommit,
  sourceRepository,
}) {
  const base = parseReleaseVersion(baseVersion, "base version");
  if (base.prerelease || base.build) {
    throw new Error("base version must be a stable SemVer value without prerelease or build metadata");
  }

  const pullRequestNumber = positiveInteger(pullRequest, "pull request");
  const sequence = positiveInteger(candidateNumber, "candidate number");
  validateSourceRepository(sourceRepository);

  const channel = `pr-${pullRequestNumber}`;
  const version = `${baseVersion}-pr.${pullRequestNumber}.${sequence}`;
  const identity = createReleaseIdentity({
    version,
    track: "staging",
    channel,
    sourceCommit,
  });

  return {
    ...identity,
    base_version: baseVersion,
    pull_request: pullRequestNumber,
    candidate_number: sequence,
    source_repository: sourceRepository,
  };
}

export function validateStagingCandidate(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("staging candidate must be a JSON object");
  }
  const identity = validateReleaseIdentity(value, requiredString(value, "version"));
  if (identity.track !== "staging") {
    throw new Error("staging candidate release identity must use the staging track");
  }

  const expected = createStagingCandidate({
    baseVersion: requiredString(value, "base_version"),
    pullRequest: value.pull_request,
    candidateNumber: value.candidate_number,
    sourceCommit: requiredString(value, "source_commit"),
    sourceRepository: requiredString(value, "source_repository"),
  });
  for (const [field, expectedValue] of Object.entries(expected)) {
    if (value[field] !== expectedValue) {
      throw new Error(
        `staging candidate ${field} must be ${JSON.stringify(expectedValue)}, got ${JSON.stringify(value[field])}`,
      );
    }
  }
  return expected;
}

export function decideChannelAdvance(currentValue, nextValue) {
  const next = validateChannelPointer(nextValue);
  if (currentValue === undefined || currentValue === null) {
    return "create";
  }
  const current = validateChannelPointer(currentValue, next.channel);
  const currentSequence = candidateSequence(current.version, current.channel);
  const nextSequence = candidateSequence(next.version, next.channel);
  if (nextSequence < currentSequence) {
    throw new Error(
      `refusing to move ${next.channel} backward from ${current.version} to ${next.version}`,
    );
  }
  if (nextSequence === currentSequence) {
    if (JSON.stringify(current) === JSON.stringify(next)) {
      return "noop";
    }
    throw new Error(
      `channel ${next.channel} candidate ${next.version} already exists with different provenance`,
    );
  }
  return "advance";
}

function positiveInteger(value, label) {
  const text = String(value ?? "");
  if (!/^[1-9]\d*$/.test(text)) {
    throw new Error(`${label} must be a positive integer`);
  }
  const parsed = Number(text);
  if (!Number.isSafeInteger(parsed)) {
    throw new Error(`${label} is too large`);
  }
  return parsed;
}

function validateSourceRepository(value) {
  const [owner, repository] = String(value ?? "").split("/");
  if (
    !SOURCE_REPOSITORY_PATTERN.test(value ?? "") ||
    owner === "." ||
    owner === ".." ||
    repository === "." ||
    repository === ".."
  ) {
    throw new Error("source repository must use owner/name format");
  }
}

function candidateSequence(version, channel) {
  const pullRequest = channel.slice("pr-".length);
  const parsed = parseReleaseVersion(version);
  const match = new RegExp(`^pr\\.${pullRequest}\\.([1-9]\\d*)$`).exec(parsed.prerelease);
  if (!match) {
    throw new Error(`staging version ${version} does not belong to ${channel}`);
  }
  return positiveInteger(match[1], "candidate number");
}

function requiredString(value, field) {
  const fieldValue = value[field];
  if (typeof fieldValue !== "string" || fieldValue.length === 0) {
    throw new Error(`staging candidate is missing ${field}`);
  }
  return fieldValue;
}
