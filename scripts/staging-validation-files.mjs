import { createHash } from "node:crypto";
import { readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

let cachedPython;

export function parseArgs(rawArgs, allowedOptions) {
  const allowed = new Set(allowedOptions);
  const parsed = {};
  for (let index = 0; index < rawArgs.length; index += 2) {
    const option = rawArgs[index];
    const value = rawArgs[index + 1];
    if (!option?.startsWith("--") || value === undefined) {
      throw new Error(`Invalid argument ${JSON.stringify(option)}`);
    }
    const name = option.slice(2);
    if (!allowed.has(name)) {
      throw new Error(`Unknown option ${option}`);
    }
    if (parsed[name] !== undefined) {
      throw new Error(`Duplicate option ${option}`);
    }
    parsed[name] = value;
  }
  return parsed;
}

export function requiredArg(args, name) {
  const value = args[name];
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Missing --${name}`);
  }
  return value;
}

export function requireDescendantPath(root, candidate, label) {
  const resolvedRoot = path.resolve(requiredString(root, "RUNNER_TEMP"));
  const resolvedCandidate = path.resolve(requiredString(candidate, label));
  const relative = path.relative(resolvedRoot, resolvedCandidate);
  if (
    !relative ||
    relative === ".." ||
    relative.startsWith(`..${path.sep}`) ||
    path.isAbsolute(relative)
  ) {
    throw new Error(`${label} must be inside RUNNER_TEMP without naming RUNNER_TEMP itself`);
  }
  return resolvedCandidate;
}

export function inspectZip(file, versionEntry, releaseEntry) {
  assertFile(file, "release archive");
  const python = resolvePython();
  const script = [
    "import json, sys, zipfile",
    "archive, version_entry, release_entry = sys.argv[1:4]",
    "with zipfile.ZipFile(archive, 'r') as zf:",
    "    members = [{'name': info.filename, 'mode': (info.external_attr >> 16) & 0xffff, 'create_system': info.create_system, 'is_dir': info.is_dir()} for info in zf.infolist()]",
    "    names = sorted(member['name'] for member in members if not member['is_dir'])",
    "    modes = {member['name']: member['mode'] & 0o777 for member in members if not member['is_dir']}",
    "    result = {'members': members, 'names': names, 'modes': modes, 'version': zf.read(version_entry).decode('utf-8').strip()}",
    "    if release_entry:",
    "        result['release'] = zf.read(release_entry).decode('utf-8')",
    "    print(json.dumps(result))",
  ].join("\n");
  const result = spawnSync(
    python.command,
    [...python.prefixArgs, "-c", script, file, versionEntry, releaseEntry ?? ""],
    { encoding: "utf8", windowsHide: true },
  );
  if (result.status !== 0) {
    throw new Error(`failed to inspect ${file}: ${result.stderr.trim()}`);
  }
  return JSON.parse(result.stdout);
}

export function assertSafeZipMembers(members) {
  if (!Array.isArray(members)) {
    throw new Error("release archive member metadata is missing");
  }
  const names = [];
  const seen = new Set();
  for (const member of members) {
    const name = member?.name;
    if (typeof name !== "string" || name.length === 0) {
      throw new Error("release archive contains a member without a valid name");
    }
    if (
      name.includes("\\") ||
      name.startsWith("/") ||
      /^[A-Za-z]:/.test(name) ||
      name.split("/").some((segment) => segment === "" || segment === "." || segment === "..")
    ) {
      throw new Error(`release archive contains unsafe member path ${JSON.stringify(name)}`);
    }
    if (seen.has(name)) {
      throw new Error(`release archive contains duplicate member ${JSON.stringify(name)}`);
    }
    seen.add(name);
    const fileType = Number(member.mode) & 0o170000;
    if (member.is_dir || member.create_system !== 3 || fileType !== 0o100000) {
      throw new Error(`release archive member ${JSON.stringify(name)} is not a regular Unix file`);
    }
    names.push(name);
  }
  return names.sort();
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

export function requiredSafeFileName(value, label) {
  const name = requiredString(value, label);
  if (
    name === "." ||
    name === ".." ||
    path.isAbsolute(name) ||
    name.includes("/") ||
    name.includes("\\") ||
    path.basename(name) !== name
  ) {
    throw new Error(`${label} must be a plain file name`);
  }
  return name;
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

function resolvePython() {
  if (cachedPython) {
    return cachedPython;
  }
  const candidates =
    process.platform === "win32"
      ? [
          { command: "py", prefixArgs: ["-3"] },
          { command: "python3", prefixArgs: [] },
          { command: "python", prefixArgs: [] },
        ]
      : [
          { command: "python3", prefixArgs: [] },
          { command: "python", prefixArgs: [] },
        ];
  for (const candidate of candidates) {
    const result = spawnSync(
      candidate.command,
      [...candidate.prefixArgs, "--version"],
      { encoding: "utf8", windowsHide: true },
    );
    if (!result.error && result.status === 0) {
      cachedPython = candidate;
      return cachedPython;
    }
  }
  throw new Error("Python 3 is required to inspect release archives");
}
