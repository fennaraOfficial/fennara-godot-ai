import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  config,
  ensureTranslationMetadata,
  renderNavigation,
  replaceNavigation,
  root,
  targetFor,
} from "./doc-i18n-lib.mjs";

const options = parseOptions(process.argv.slice(2));
const checkOnly = options.checkOnly;
const sourcesOnly = options.sourcesOnly;
const changed = [];

for (const document of config.documents) {
  const source = read(document.source);
  sync(document.source, replaceNavigation(source, renderNavigation(document, "en")));
  if (sourcesOnly) {
    continue;
  }
  for (const locale of config.locales.slice(1)) {
    const target = targetFor(document, locale.code);
    if (!existsSync(path.join(root, target))) {
      changed.push(target);
      continue;
    }
    const translated = ensureTranslationMetadata(
      document,
      locale,
      source,
      read(target),
      { refreshSourceHash: options.acceptedSources.has(document.source) },
    );
    sync(
      target,
      replaceNavigation(translated, renderNavigation(document, locale.code)),
    );
  }
}

if (checkOnly && changed.length > 0) {
  console.error("Documentation metadata is out of sync:");
  for (const file of changed) {
    console.error(`- ${file}`);
  }
  process.exit(1);
}

if (changed.length === 0) {
  console.log("Documentation metadata is already in sync.");
} else if (!checkOnly) {
  console.log(`Updated documentation metadata in ${changed.length} files.`);
}

function read(relativePath) {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function sync(relativePath, content) {
  const absolute = path.join(root, relativePath);
  const before = existsSync(absolute) ? readFileSync(absolute, "utf8") : "";
  if (before === content) {
    return;
  }
  changed.push(relativePath);
  if (!checkOnly) {
    writeFileSync(absolute, content);
  }
}

function parseOptions(args) {
  const acceptedSources = new Set();
  let checkOnly = false;
  let sourcesOnly = false;

  for (let index = 0; index < args.length; index += 1) {
    const argument = args[index];
    if (argument === "--check") {
      checkOnly = true;
    } else if (argument === "--sources-only") {
      sourcesOnly = true;
    } else if (argument === "--accept-source") {
      const source = args[index + 1]?.replaceAll("\\", "/");
      if (!source || source.startsWith("--")) {
        throw new Error("--accept-source requires a canonical source path");
      }
      acceptedSources.add(source);
      index += 1;
    } else {
      throw new Error(`unknown option: ${argument}`);
    }
  }

  if (checkOnly && acceptedSources.size > 0) {
    throw new Error("--check cannot be combined with --accept-source");
  }
  if (sourcesOnly && acceptedSources.size > 0) {
    throw new Error("--sources-only cannot be combined with --accept-source");
  }

  const canonicalSources = new Set(config.documents.map((document) => document.source));
  for (const source of acceptedSources) {
    if (!canonicalSources.has(source)) {
      throw new Error(`unknown canonical source: ${source}`);
    }
  }

  return { acceptedSources, checkOnly, sourcesOnly };
}
