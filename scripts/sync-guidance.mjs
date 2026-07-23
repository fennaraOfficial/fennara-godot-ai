import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const files = [
  ["local/templates/fennara-guidelines.md", "guidelines.md"],
  ["local/templates/fennara-ai/index.md", "index.md"],
  ["local/templates/fennara-ai/visual-observation.md", "visual-observation.md"],
  ["local/templates/fennara-ai/runtime-observation.md", "runtime-observation.md"],
  ["local/templates/fennara-ai/operations.md", "operations.md"],
  ["local/templates/fennara-ai/clients/cursor.md", "clients/cursor.md"],
];

for (const [sourceRelative, targetRelative] of files) {
  const source = path.join(root, sourceRelative);
  const target = path.join(
    root,
    "godot_demo",
    "addons",
    "fennara",
    "ai",
    targetRelative,
  );
  mkdirSync(path.dirname(target), { recursive: true });
  writeFileSync(target, normalizeTemplate(readFileSync(source, "utf8")));
  console.log(`Synced ${path.relative(root, source)} -> ${path.relative(root, target)}`);
}

function normalizeTemplate(template) {
  return `${template.trimEnd()}\n`;
}
