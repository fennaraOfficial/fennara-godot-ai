import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const TEXT_EXTENSIONS = new Set([
  ".cfg",
  ".css",
  ".gd",
  ".gdextension",
  ".gdshader",
  ".html",
  ".import",
  ".js",
  ".json",
  ".md",
  ".svg",
  ".toml",
  ".tres",
  ".tscn",
  ".txt",
]);

export function copyFile(source, target) {
  mkdirSync(path.dirname(target), { recursive: true });
  if (isTextAsset(source)) {
    writeFileSync(target, normalizeLineEndings(readFileSync(source, "utf8")));
  } else {
    copyFileSync(source, target);
  }
}

export function isTextAsset(filePath) {
  const name = path.basename(filePath);
  if (name === "VERSION" || name === "LICENSE") {
    return true;
  }
  return TEXT_EXTENSIONS.has(path.extname(filePath));
}

export function normalizeLineEndings(text) {
  return text.replace(/\r\n?/g, "\n");
}
