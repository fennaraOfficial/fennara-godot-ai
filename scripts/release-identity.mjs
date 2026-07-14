const SEMVER_PATTERN =
  /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/;
const PULL_REQUEST_CHANNEL_PATTERN = /^pr-([1-9]\d*)$/;
const FULL_GITHUB_SHA_PATTERN = /^[0-9a-f]{40}$/;

export function parseReleaseVersion(value, label = "version") {
  const match = SEMVER_PATTERN.exec(value);
  if (!match) {
    throw new Error(`${label} must be a valid SemVer value, got ${JSON.stringify(value)}`);
  }
  return {
    value,
    prerelease: match[4] ?? "",
    build: match[5] ?? "",
  };
}

export function validatePullRequestChannel(channel) {
  const match = PULL_REQUEST_CHANNEL_PATTERN.exec(channel);
  if (!match) {
    throw new Error(
      `staging channel must use pr-<number> format, got ${JSON.stringify(channel)}`,
    );
  }
  const pullRequest = Number(match[1]);
  if (!Number.isSafeInteger(pullRequest)) {
    throw new Error(`staging channel number is too large: ${JSON.stringify(channel)}`);
  }
  return pullRequest;
}

export function createReleaseIdentity({
  version,
  track = "stable",
  channel,
  sourceCommit,
}) {
  const parsed = parseReleaseVersion(version);
  if (parsed.build) {
    throw new Error("release versions must not contain SemVer build metadata");
  }

  const identity = {
    schema_version: 1,
    track,
    version,
    release_tag: `v${version}`,
  };

  if (track === "stable") {
    if (parsed.prerelease) {
      throw new Error("stable release versions must not contain a prerelease suffix");
    }
    if (channel !== undefined) {
      throw new Error("stable release identity must not include a staging channel");
    }
    if (sourceCommit !== undefined) {
      assertFullGitHubSha(sourceCommit);
      identity.source_commit = sourceCommit;
    }
    return identity;
  }

  if (track !== "staging") {
    throw new Error(`release track must be stable or staging, got ${JSON.stringify(track)}`);
  }

  const pullRequest = validatePullRequestChannel(channel ?? "");
  const prereleaseParts = parsed.prerelease.split(".");
  if (
    prereleaseParts.length !== 3 ||
    prereleaseParts[0] !== "pr" ||
    prereleaseParts[1] !== String(pullRequest) ||
    !/^[1-9]\d*$/.test(prereleaseParts[2])
  ) {
    throw new Error(
      `staging version for ${channel} must end with -pr.${pullRequest}.<candidate-number>`,
    );
  }
  assertFullGitHubSha(sourceCommit);
  identity.channel = channel;
  identity.source_commit = sourceCommit;
  return identity;
}

export function validateReleaseIdentity(value, expectedVersion) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("release identity must be a JSON object");
  }
  if (value.schema_version !== 1) {
    throw new Error(`unsupported release identity schema ${JSON.stringify(value.schema_version)}`);
  }

  const identity = createReleaseIdentity({
    version: requiredString(value, "version"),
    track: requiredString(value, "track"),
    channel: optionalString(value, "channel"),
    sourceCommit: optionalString(value, "source_commit"),
  });
  if (value.release_tag !== identity.release_tag) {
    throw new Error(
      `release identity tag must be ${identity.release_tag}, got ${JSON.stringify(value.release_tag)}`,
    );
  }
  if (expectedVersion !== undefined && identity.version !== expectedVersion) {
    throw new Error(
      `release identity version ${identity.version} does not match VERSION ${expectedVersion}`,
    );
  }
  return identity;
}

export function createChannelPointer(identity, releaseManifestSha256) {
  const validated = validateReleaseIdentity(identity);
  if (validated.track !== "staging") {
    throw new Error("only staging identities can create channel pointers");
  }
  if (!/^[0-9a-f]{64}$/i.test(releaseManifestSha256)) {
    throw new Error("release manifest SHA-256 must contain exactly 64 hexadecimal characters");
  }
  return {
    schema_version: 1,
    channel: validated.channel,
    version: validated.version,
    release_tag: validated.release_tag,
    source_commit: validated.source_commit,
    release_manifest_sha256: releaseManifestSha256.toLowerCase(),
  };
}

export function validateChannelPointer(value, expectedChannel) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("staging channel pointer must be a JSON object");
  }
  if (value.schema_version !== 1) {
    throw new Error(`unsupported staging channel pointer schema ${JSON.stringify(value.schema_version)}`);
  }
  const channel = requiredString(value, "channel");
  validatePullRequestChannel(channel);
  if (expectedChannel !== undefined && channel !== expectedChannel) {
    throw new Error(
      `staging channel pointer ${JSON.stringify(channel)} does not match ${JSON.stringify(expectedChannel)}`,
    );
  }
  const identity = validateReleaseIdentity({
    schema_version: 1,
    track: "staging",
    channel,
    version: requiredString(value, "version"),
    release_tag: requiredString(value, "release_tag"),
    source_commit: requiredString(value, "source_commit"),
  });
  const releaseManifestSha256 = requiredString(value, "release_manifest_sha256");
  if (!/^[0-9a-f]{64}$/i.test(releaseManifestSha256)) {
    throw new Error("release manifest SHA-256 must contain exactly 64 hexadecimal characters");
  }
  return {
    schema_version: 1,
    channel: identity.channel,
    version: identity.version,
    release_tag: identity.release_tag,
    source_commit: identity.source_commit,
    release_manifest_sha256: releaseManifestSha256.toLowerCase(),
  };
}

export function channelPointerRef(channel) {
  validatePullRequestChannel(channel);
  return `fennara-staging/${channel}`;
}

export function channelPointerAssetName(channel) {
  validatePullRequestChannel(channel);
  return `fennara-staging-channel-${channel}.json`;
}

function assertFullGitHubSha(value) {
  if (!FULL_GITHUB_SHA_PATTERN.test(value ?? "")) {
    throw new Error("staging source commit must be a full 40-character lowercase Git SHA");
  }
}

function requiredString(value, field) {
  const fieldValue = value[field];
  if (typeof fieldValue !== "string" || fieldValue.length === 0) {
    throw new Error(`release identity is missing ${field}`);
  }
  return fieldValue;
}

function optionalString(value, field) {
  const fieldValue = value[field];
  if (fieldValue === undefined) {
    return undefined;
  }
  if (typeof fieldValue !== "string" || fieldValue.length === 0) {
    throw new Error(`release identity ${field} must be a non-empty string when present`);
  }
  return fieldValue;
}
