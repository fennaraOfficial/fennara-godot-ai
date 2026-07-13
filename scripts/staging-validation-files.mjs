import { createHash } from "node:crypto";
import { readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

export function inspectZip(file, versionEntry, releaseEntry) {
  assertFile(file, "release archive");
  const script = [
    "import json, sys, zipfile",
    "archive, version_entry, release_entry = sys.argv[1:4]",
    "with zipfile.ZipFile(archive, 'r') as zf:",
    "    names = sorted(name.rstrip('/') for name in zf.namelist() if not name.endswith('/'))",
    "    modes = {info.filename.rstrip('/'): (info.external_attr >> 16) & 0o777 for info in zf.infolist() if not info.is_dir()}",
    "    result = {'names': names, 'modes': modes, 'version': zf.read(version_entry).decode('utf-8').strip()}",
    "    if release_entry:",
    "        result['release'] = zf.read(release_entry).decode('utf-8')",
    "    print(json.dumps(result))",
  ].join("\n");
  const result = spawnSync("python", ["-c", script, file, versionEntry, releaseEntry ?? ""], {
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(`failed to inspect ${file}: ${result.stderr.trim()}`);
  }
  return JSON.parse(result.stdout);
}

export function treeHashes(root, excludedTopLevel) {
  const records = {};
  visit(root, "");
  return records;

  function visit(directory, relativeDirectory) {
    for (const entry of readdirSync(directory).sort()) {
      if (!relativeDirectory && entry === excludedTopLevel) {
        continue;
      }
      const file = path.join(directory, entry);
      const relative = path.posix.join(relativeDirectory, entry);
      if (statSync(file).isDirectory()) {
        visit(file, relative);
      } else {
        records[relative] = sha256File(file);
      }
    }
  }
}

export function assertSameNames(actual, expected, label) {
  const left = [...actual].sort();
  const right = [...expected].sort();
  if (JSON.stringify(left) !== JSON.stringify(right)) {
    throw new Error(`${label} mismatch\nexpected: ${JSON.stringify(right)}\nactual: ${JSON.stringify(left)}`);
  }
}

export function assertVersion(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label} version ${JSON.stringify(actual)} does not match ${expected}`);
  }
}

export function assertPath(file, label) {
  if (!exists(file)) {
    throw new Error(`Missing ${label}: ${file}`);
  }
}

export function assertZipExecutable(archive, entry) {
  const mode = archive.modes?.[entry] ?? 0;
  if ((mode & 0o111) === 0) {
    throw new Error(`${entry} is not executable in the release archive (mode ${mode.toString(8)})`);
  }
}

export function assertFile(file, label) {
  if (!exists(file) || !statSync(file).isFile()) {
    throw new Error(`Missing ${label}: ${file}`);
  }
}

export function assertDirectory(directory, label) {
  if (!exists(directory) || !statSync(directory).isDirectory()) {
    throw new Error(`Missing ${label}: ${directory}`);
  }
}

export function sha256File(file) {
  return createHash("sha256").update(readFileSync(file)).digest("hex");
}

export function requiredString(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`${label} is required`);
  }
  return value;
}

export function requiredSha256(value, label) {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/i.test(value)) {
    throw new Error(`${label} must be a SHA-256 value`);
  }
  return value.toLowerCase();
}

export function readJson(file) {
  return JSON.parse(readFileSync(file, "utf8"));
}

function exists(file) {
  try {
    statSync(file);
    return true;
  } catch {
    return false;
  }
}
