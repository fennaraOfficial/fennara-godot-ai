#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";

import {
  appendSnapshot,
  fetchStarCount,
  fetchStarredAt,
  normalizeSeries,
  seriesFromStarredAt,
} from "./star-history-data.mjs";
import { renderStarHistorySvg } from "./star-history-render.mjs";

export async function generateStarHistory(options) {
  const {
    repository,
    seriesPath,
    seedPath,
    outputPath,
    date,
    token,
    countOverride,
    fetchImpl = fetch,
  } = options;
  let series = await loadFirstSeries([seriesPath, seedPath], repository);
  let exactCount = null;

  if (!series && token) {
    try {
      const timestamps = await fetchStarredAt(repository, token, fetchImpl);
      series = seriesFromStarredAt(repository, timestamps);
      exactCount = timestamps.length;
      console.log(`bootstrapped history from ${exactCount} current stargazers`);
    } catch (error) {
      console.warn(`stargazer bootstrap unavailable: ${error.message}`);
    }
  }

  let currentCount = countOverride;
  if (currentCount === undefined) {
    try {
      currentCount = await fetchStarCount(repository, token, fetchImpl);
    } catch (error) {
      if (exactCount === null) {
        throw error;
      }
      currentCount = exactCount;
      console.warn(`repository count unavailable, using exact count: ${error.message}`);
    }
  }
  if (!series) {
    throw new Error("no stored series is available for the snapshot fallback");
  }

  series = appendSnapshot(series, date, currentCount);
  await writeFile(seriesPath, `${JSON.stringify(series, null, 2)}\n`, "utf8");
  await writeFile(outputPath, renderStarHistorySvg(series, date), "utf8");
  return { points: series.points.length, count: currentCount };
}

async function loadFirstSeries(paths, repository) {
  for (const path of paths) {
    if (!path) {
      continue;
    }
    try {
      const content = await readFile(path, "utf8");
      if (content.trim()) {
        return normalizeSeries(JSON.parse(content), repository);
      }
    } catch (error) {
      if (error.code !== "ENOENT") {
        throw new Error(`cannot load ${path}: ${error.message}`, { cause: error });
      }
    }
  }
  return null;
}

function parseArguments(argv) {
  const options = {
    repository: "fennaraOfficial/fennara-godot-ai",
    seriesPath: "star-history.json",
    seedPath: ".github/star-history-seed.json",
    outputPath: "star-history.svg",
    date: new Date().toISOString().slice(0, 10),
    token: process.env.STAR_HISTORY_TOKEN || process.env.GITHUB_TOKEN || "",
    countOverride: undefined,
  };
  const names = {
    "--repo": "repository",
    "--series": "seriesPath",
    "--seed": "seedPath",
    "--out": "outputPath",
    "--date": "date",
    "--count": "countOverride",
  };
  for (let index = 0; index < argv.length; index += 1) {
    const key = names[argv[index]];
    if (!key || index + 1 >= argv.length) {
      throw new Error(`unknown or incomplete argument: ${argv[index]}`);
    }
    options[key] = argv[index + 1];
    index += 1;
  }
  if (options.countOverride !== undefined) {
    options.countOverride = Number(options.countOverride);
  }
  if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(options.repository)) {
    throw new Error(`invalid repository: ${options.repository}`);
  }
  if (!/^\d{4}-\d{2}-\d{2}$/.test(options.date)) {
    throw new Error(`invalid date: ${options.date}`);
  }
  return options;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    const options = parseArguments(process.argv.slice(2));
    const result = await generateStarHistory(options);
    console.log(
      `wrote ${options.outputPath} with ${result.points} points and ${result.count} stars`,
    );
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
