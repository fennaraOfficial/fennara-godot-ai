import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  parseArgs,
  requireDescendantPath,
  requiredArg,
} from "./staging-validation-files.mjs";

const args = parseArgs(process.argv.slice(2), [
  "repository",
  "release-tag",
  "expected-dir",
  "download-dir",
]);
const repository = requiredArg(args, "repository");
const releaseTag = requiredArg(args, "release-tag");
const expectedDir = path.resolve(requiredArg(args, "expected-dir"));
const downloadDir = requireDescendantPath(
  process.env.RUNNER_TEMP,
  requiredArg(args, "download-dir"),
  "--download-dir",
);
const METADATA_TIMEOUT_MS = 30_000;
const ASSET_TIMEOUT_MS = 60_000;

if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(repository)) {
  throw new Error("repository must use owner/name format");
}

const metadata = await fetchJson(
  `https://api.github.com/repos/${repository}/releases/tags/${encodeURIComponent(releaseTag)}`,
  {
    headers: {
      Accept: "application/vnd.github+json",
      "User-Agent": "fennara-staging-public-smoke",
      "X-GitHub-Api-Version": "2026-03-10",
    },
  },
  METADATA_TIMEOUT_MS,
  "public release metadata",
);
if (!metadata.response.ok) {
  throw new Error(`public release metadata returned HTTP ${metadata.response.status}`);
}
const release = metadata.body;
if (release.draft || !release.prerelease || release.tag_name !== releaseTag) {
  throw new Error("public release metadata does not describe the expected published prerelease");
}

const expectedManifest = expectedAssetManifest(expectedDir);
const published = new Map(
  (release.assets ?? []).map((asset) => [asset.name, asset.browser_download_url]),
);
assertExactNames([...expectedManifest.keys()], [...published.keys()]);

rmSync(downloadDir, { recursive: true, force: true });
mkdirSync(downloadDir, { recursive: true });
for (const [name, expectedHash] of expectedManifest) {
  const url = published.get(name);
  if (typeof url !== "string" || !url.startsWith("https://github.com/")) {
    throw new Error(`release asset ${name} has no public GitHub browser download URL`);
  }
  const download = await fetchBytes(
    url,
    {
      headers: { "User-Agent": "fennara-staging-public-smoke" },
      redirect: "follow",
    },
    ASSET_TIMEOUT_MS,
    `public download for ${name}`,
  );
  if (!download.response.ok) {
    throw new Error(`public download for ${name} returned HTTP ${download.response.status}`);
  }
  const bytes = download.body;
  const actualHash = createHash("sha256").update(bytes).digest("hex");
  if (actualHash !== expectedHash) {
    throw new Error(`public download hash mismatch for ${name}`);
  }
  writeFileSync(path.join(downloadDir, name), bytes);
}

const manifestName = [...expectedManifest.keys()].find(
  (name) => name.startsWith("fennara-release-manifest-v") && name.endsWith(".json"),
);
if (!manifestName) {
  throw new Error("published assets do not contain a versioned release manifest");
}
const manifest = JSON.parse(readFileSync(path.join(downloadDir, manifestName), "utf8"));
for (const asset of manifestAssets(manifest)) {
  if (!expectedManifest.has(asset.name)) {
    throw new Error(`release manifest references unpublished asset ${asset.name}`);
  }
  if (expectedManifest.get(asset.name) !== asset.sha256) {
    throw new Error(`release manifest hash does not match public asset ${asset.name}`);
  }
}

console.log(`Verified ${expectedManifest.size} assets through public browser download URLs`);

function expectedAssetManifest(directory) {
  return new Map(
    readdirSync(directory)
      .sort()
      .map((name) => {
        const file = path.join(directory, name);
        if (!statSync(file).isFile()) {
          throw new Error(`unexpected directory in expected release assets: ${file}`);
        }
        return [name, createHash("sha256").update(readFileSync(file)).digest("hex")];
      }),
  );
}

function manifestAssets(manifest) {
  const records = [
    ...Object.values(manifest.assets?.cli ?? {}),
    ...Object.values(manifest.assets?.local ?? {}),
    manifest.assets?.addon,
    ...(manifest.shared_runtimes ?? []).map((runtime) => runtime.archive),
  ].filter(Boolean);
  for (const record of records) {
    if (
      typeof record.name !== "string" ||
      typeof record.sha256 !== "string" ||
      !/^[0-9a-f]{64}$/.test(record.sha256)
    ) {
      throw new Error("release manifest contains an invalid asset record");
    }
  }
  return records;
}

function assertExactNames(expected, actual) {
  const left = [...expected].sort();
  const right = [...actual].sort();
  if (JSON.stringify(left) !== JSON.stringify(right)) {
    throw new Error(
      `public release assets differ from validated assets\nexpected: ${JSON.stringify(left)}\nactual: ${JSON.stringify(right)}`,
    );
  }
}


async function fetchJson(url, options, timeoutMs, label) {
  return fetchBody(url, options, timeoutMs, label, async (response) =>
    response.ok ? response.json() : undefined,
  );
}

async function fetchBytes(url, options, timeoutMs, label) {
  return fetchBody(url, options, timeoutMs, label, async (response) =>
    response.ok ? Buffer.from(await response.arrayBuffer()) : undefined,
  );
}

async function fetchBody(url, options, timeoutMs, label, readBody) {
  try {
    const response = await fetch(url, {
      ...options,
      signal: AbortSignal.timeout(timeoutMs),
    });
    return { response, body: await readBody(response) };
  } catch (error) {
    if (error?.name === "TimeoutError") {
      throw new Error(`${label} timed out after ${timeoutMs} ms`, { cause: error });
    }
    throw new Error(`${label} failed: ${error?.message ?? error}`, { cause: error });
  }
}
