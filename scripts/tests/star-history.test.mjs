import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { generateStarHistory } from "../generate-star-history.mjs";
import {
  appendSnapshot,
  normalizeSeries,
  seriesFromStarredAt,
} from "../star-history-data.mjs";
import { niceScale, renderStarHistorySvg } from "../star-history-render.mjs";

const repository = "fennaraOfficial/fennara-godot-ai";

test("starred_at timestamps become cumulative daily points", () => {
  assert.deepEqual(
    seriesFromStarredAt(repository, [
      "2026-06-02T12:00:00Z",
      "2026-06-01T12:00:00Z",
      "2026-06-02T13:00:00Z",
    ]),
    {
      repository,
      points: [
        ["2026-06-01", 1],
        ["2026-06-02", 3],
      ],
    },
  );
});

test("snapshots replace the same date and preserve sorted history", () => {
  const series = normalizeSeries({
    repository,
    points: [
      ["2026-06-02", 2],
      ["2026-06-01", 1],
      ["2026-06-02", 3],
    ],
  });
  assert.deepEqual(appendSnapshot(series, "2026-06-02", 4).points, [
    ["2026-06-01", 1],
    ["2026-06-02", 4],
  ]);
});

test("the chart uses a readable 250-star scale for the current series", () => {
  assert.deepEqual(niceScale(235), { step: 50, ceiling: 250 });
});

test("the SVG is minimal, theme-aware, and directly labeled", () => {
  const svg = renderStarHistorySvg(
    {
      repository,
      points: [
        ["2026-06-01", 1],
        ["2026-07-27", 235],
      ],
    },
    "2026-07-27",
  );
  assert.match(svg, /fennaraOfficial\/fennara-godot-ai/);
  assert.match(svg, /GitHub stars over time/);
  assert.match(svg, /235 stars/);
  assert.match(svg, /@media \(prefers-color-scheme: dark\)/);
  assert.doesNotMatch(svg, /glow|linearGradient|★/);
});

test("offline generation starts from the tracked seed and writes both outputs", async () => {
  const tempRoot = fileURLToPath(new URL("../../temp/", import.meta.url));
  mkdirSync(tempRoot, { recursive: true });
  const directory = mkdtempSync(path.join(tempRoot, "star-history-"));
  const seedPath = path.join(directory, "seed.json");
  const seriesPath = path.join(directory, "series.json");
  const outputPath = path.join(directory, "chart.svg");
  writeFileSync(
    seedPath,
    JSON.stringify({
      repository,
      points: [
        ["2026-06-01", 1],
        ["2026-06-02", 2],
      ],
    }),
  );

  try {
    const result = await generateStarHistory({
      repository,
      seriesPath,
      seedPath,
      outputPath,
      date: "2026-06-03",
      token: "",
      countOverride: 3,
    });
    assert.deepEqual(result, { points: 3, count: 3 });
    assert.deepEqual(JSON.parse(readFileSync(seriesPath, "utf8")).points.at(-1), [
      "2026-06-03",
      3,
    ]);
    assert.match(readFileSync(outputPath, "utf8"), /3 stars/);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("stored snapshots are not rewritten from a later stargazer list", async () => {
  const tempRoot = fileURLToPath(new URL("../../temp/", import.meta.url));
  mkdirSync(tempRoot, { recursive: true });
  const directory = mkdtempSync(path.join(tempRoot, "star-history-preserve-"));
  const seedPath = path.join(directory, "seed.json");
  const seriesPath = path.join(directory, "series.json");
  const outputPath = path.join(directory, "chart.svg");
  writeFileSync(
    seriesPath,
    JSON.stringify({
      repository,
      points: [
        ["2026-06-01", 10],
        ["2026-06-02", 12],
      ],
    }),
  );

  try {
    await generateStarHistory({
      repository,
      seriesPath,
      seedPath,
      outputPath,
      date: "2026-06-03",
      token: "available-token",
      countOverride: 11,
      fetchImpl: async () => {
        throw new Error("stored history must not request exact stargazers");
      },
    });
    assert.deepEqual(JSON.parse(readFileSync(seriesPath, "utf8")).points, [
      ["2026-06-01", 10],
      ["2026-06-02", 12],
      ["2026-06-03", 11],
    ]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
