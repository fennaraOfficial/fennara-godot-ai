const MINIMUM_CLI_VERSION_BY_TRACK = Object.freeze({
  stable: "0.3.11",
  staging: "0.3.8",
});

export function minimumCliVersionForTrack(track) {
  if (!Object.hasOwn(MINIMUM_CLI_VERSION_BY_TRACK, track)) {
    throw new Error(`release policy does not define track ${JSON.stringify(track)}`);
  }
  return MINIMUM_CLI_VERSION_BY_TRACK[track];
}

export const RELEASE_POLICY = Object.freeze({
  minimumCliVersionByTrack: MINIMUM_CLI_VERSION_BY_TRACK,
});
