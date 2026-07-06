import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const source = path.join(root, "local", "templates", "fennara-guidelines.md");
const target = path.join(
  root,
  "godot_demo",
  "addons",
  "fennara",
  "ai",
  "guidelines.md",
);

mkdirSync(path.dirname(target), { recursive: true });
writeFileSync(target, normalizeTemplate(readFileSync(source, "utf8")));

console.log(`Synced ${path.relative(root, source)} -> ${path.relative(root, target)}`);

function normalizeTemplate(template) {
  return `${template.trimEnd()}\n`;
}
