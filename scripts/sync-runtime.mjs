import {
  copyFileSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const source = path.join(root, "runtime");
const target = path.join(root, "godot_demo", "addons", "fennara", "runtime");

rmSync(target, { recursive: true, force: true });
copyDir(source, target);

console.log(`Synced ${path.relative(root, source)} -> ${path.relative(root, target)}`);

function copyDir(from, to) {
  mkdirSync(to, { recursive: true });
  for (const entry of readdirSync(from)) {
    if (entry === "README.md") {
      continue;
    }

    const sourcePath = path.join(from, entry);
    const targetPath = path.join(to, entry);
    if (statSync(sourcePath).isDirectory()) {
      copyDir(sourcePath, targetPath);
    } else {
      copyFile(sourcePath, targetPath);
    }
  }
}

function copyFile(sourcePath, targetPath) {
  if (path.extname(sourcePath) === ".gd") {
    writeFileSync(targetPath, normalizeLineEndings(readFileSync(sourcePath, "utf8")));
  } else {
    copyFileSync(sourcePath, targetPath);
  }
}

function normalizeLineEndings(text) {
  return text.replace(/\r\n?/g, "\n");
}
