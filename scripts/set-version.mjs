import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createReleaseIdentity, parseReleaseVersion } from "./release-identity.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const version = process.argv[2];
let args;
try {
  args = parseArgs(process.argv.slice(3));
} catch (error) {
  console.error(error.message);
  printUsage();
  process.exit(1);
}

try {
  const parsed = parseReleaseVersion(version ?? "");
  if (parsed.build) {
    throw new Error("release versions must not contain SemVer build metadata");
  }
} catch (error) {
  console.error(error.message);
  printUsage();
  process.exit(1);
}

let identity;
try {
  identity = createReleaseIdentity({
    version,
    track: args.track ?? "stable",
    channel: args.channel,
    sourceCommit: args["source-commit"],
  });
} catch (error) {
  console.error(error.message);
  printUsage();
  process.exit(1);
}

write("VERSION", `${version}\n`);
write("godot_demo/addons/fennara/VERSION", `${version}\n`);
write(
  "godot_demo/addons/fennara/release.json",
  `${JSON.stringify(identity, null, 2)}\n`,
);

update("local/Cargo.toml", (text) => {
  if (/\[workspace\.package\][\s\S]*?version\s*=/.test(text)) {
    return text.replace(
      /(\[workspace\.package\][\s\S]*?version\s*=\s*)"[^"]+"/,
      `$1"${version}"`,
    );
  }

  return text.replace(
    /(\[workspace\.package\]\r?\n)/,
    `$1version = "${version}"\n`,
  );
});

for (const manifest of [
  "local/crates/fennara-cli/Cargo.toml",
  "local/crates/fennara-daemon/Cargo.toml",
  "local/crates/fennara-mcp/Cargo.toml",
]) {
  update(manifest, (text) => {
    if (/version\.workspace\s*=\s*true/.test(text)) {
      return text.replace(/^version\s*=\s*"[^"]+"\r?\n/gm, "");
    }

    return text.replace(
      /^version\s*=\s*"[^"]+"/m,
      "version.workspace = true",
    );
  });
}

update("fennara-cpp/include/fennara/local_bridge.hpp", (text) =>
  text.replace(/PLUGIN_VERSION\s*=\s*"[^"]+"/, `PLUGIN_VERSION = "${version}"`),
);

run("cargo", ["update", "-w", "--manifest-path", path.join("local", "Cargo.toml")]);
run(process.execPath, [path.join("scripts", "check-version.mjs")]);

function read(relativePath) {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function write(relativePath, text) {
  writeFileSync(path.join(root, relativePath), text);
}

function update(relativePath, updater) {
  write(relativePath, updater(read(relativePath)));
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: root,
    stdio: "inherit",
    shell: process.platform === "win32",
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function parseArgs(rawArgs) {
  const parsed = {};
  const allowed = new Set(["track", "channel", "source-commit"]);
  for (let index = 0; index < rawArgs.length; index += 1) {
    const arg = rawArgs[index];
    if (!arg.startsWith("--") || !rawArgs[index + 1]?.length) {
      throw new Error(`Invalid set-version argument: ${arg}`);
    }
    const name = arg.slice(2);
    if (!allowed.has(name)) {
      throw new Error(`Unknown set-version option: ${arg}`);
    }
    if (parsed[name] !== undefined) {
      throw new Error(`Duplicate set-version option: ${arg}`);
    }
    parsed[name] = rawArgs[index + 1];
    index += 1;
  }
  return parsed;
}

function printUsage() {
  console.error(
    "Usage: node scripts/set-version.mjs <semver> [--track staging --channel pr-<number> --source-commit <full-sha>]",
  );
}
