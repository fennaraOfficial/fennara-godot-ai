import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(
  new URL("../../.github/workflows/star-history.yml", import.meta.url),
  "utf8",
);
const readmes = [
  "README.md",
  "README.de.md",
  "README.es.md",
  "README.fr.md",
  "README.ja.md",
  "README.ko.md",
  "README.pt-BR.md",
  "README.ru.md",
  "README.tr.md",
  "README.zh-CN.md",
].map((name) => readFileSync(new URL(`../../${name}`, import.meta.url), "utf8"));

test("the chart workflow retains its stored series and publishes off main", () => {
  assert.match(workflow, /schedule:/);
  assert.match(workflow, /origin\/star-history:star-history\.json/);
  assert.match(workflow, /commit-tree/);
  assert.match(workflow, /refs\/heads\/star-history/);
  assert.match(workflow, /permissions:\s+contents: write/);
});

test("every public README uses the repository-owned chart", () => {
  for (const readme of readmes) {
    assert.match(
      readme,
      /raw\.githubusercontent\.com\/fennaraOfficial\/fennara-godot-ai\/star-history\/star-history\.svg/,
    );
    assert.doesNotMatch(readme, /api\.star-history\.com|sealed_token/);
  }
});
