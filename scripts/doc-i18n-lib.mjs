import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
export const config = JSON.parse(
  readFileSync(path.join(root, "docs", "i18n", "languages.json"), "utf8"),
);

export const NAV_START = "<!-- fennara-doc-nav:start -->";
export const NAV_END = "<!-- fennara-doc-nav:end -->";
export const MARKER_PATTERN =
  /^<!-- fennara-i18n: locale=([^ ]+) source=([^ ]+) sha256=([a-f0-9]{64}) -->\r?\n/;

export function targetFor(document, localeCode) {
  return document.target.replaceAll("{locale}", localeCode);
}

export function normalize(value) {
  return `${value.replaceAll("\r\n", "\n").trimEnd()}\n`;
}

export function stripNavigation(value) {
  const pattern = new RegExp(
    `${escapeRegExp(NAV_START)}[\\s\\S]*?${escapeRegExp(NAV_END)}\\n*`,
    "g",
  );
  return normalize(value).replace(pattern, "");
}

export function sourceHash(value) {
  return createHash("sha256").update(stripNavigation(value)).digest("hex");
}

export function translationMarker(document, locale, source) {
  return `<!-- fennara-i18n: locale=${locale.code} source=${document.source} sha256=${sourceHash(source)} -->`;
}

export function renderNavigation(document, localeCode) {
  const locale = config.locales.find((candidate) => candidate.code === localeCode);
  const currentPath =
    localeCode === config.canonicalLocale
      ? document.source
      : targetFor(document, localeCode);
  const languageLinks = config.locales.map((candidate) => {
    const target =
      candidate.code === config.canonicalLocale
        ? document.source
        : targetFor(document, candidate.code);
    if (candidate.code === localeCode) {
      return `**${candidate.nativeName}**`;
    }
    return `[${candidate.nativeName}](${relativeLink(currentPath, target)})`;
  });
  const lines = [
    NAV_START,
    languageLinks.join(" · "),
  ];
  if (localeCode !== config.canonicalLocale) {
    lines.push(
      "",
      `> ℹ️ ${locale.reviewNotice} [${locale.sourceLabel}](${relativeLink(currentPath, document.source)})`,
    );
  }
  lines.push(NAV_END);
  return lines.join("\n");
}

export function replaceNavigation(value, navigation) {
  const normalized = normalize(value);
  const pattern = new RegExp(
    `${escapeRegExp(NAV_START)}[\\s\\S]*?${escapeRegExp(NAV_END)}\\n*`,
  );
  if (pattern.test(normalized)) {
    return normalize(normalized.replace(pattern, `${navigation}\n\n`));
  }
  const marker = normalized.match(MARKER_PATTERN)?.[0] ?? "";
  const body = marker ? normalized.slice(marker.length) : normalized;
  const title = body.match(/^# .+$/m);
  if (!title || title.index === undefined) {
    throw new Error("document has no level-one title");
  }
  const insertAt = title.index + title[0].length;
  const remainder = body.slice(insertAt).replace(/^\n+/, "");
  return normalize(
    `${marker}${body.slice(0, insertAt)}\n\n${navigation}\n\n${remainder}`,
  );
}

export function ensureTranslationMetadata(
  document,
  locale,
  source,
  translation,
  { refreshSourceHash = false } = {},
) {
  const normalized = normalize(translation);
  const existingMarker = normalized.match(MARKER_PATTERN)?.[0].trimEnd();
  const marker =
    refreshSourceHash || !existingMarker
      ? translationMarker(document, locale, source)
      : existingMarker;
  const withoutMarker = normalized.replace(MARKER_PATTERN, "");
  const sourceIds = headingIds(stripNavigation(source));
  const withoutStableAnchors = withoutMarker.replace(
    /^<a id="[^"]+"><\/a>\r?\n(?=#{1,6}\s)/gm,
    "",
  );
  const lines = withoutStableAnchors.split("\n");
  let headingIndex = 0;
  const output = [];
  let inFence = false;

  for (const line of lines) {
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
    }
    if (!inFence && /^#{1,6}\s+/.test(line)) {
      if (headingIndex >= sourceIds.length) {
        throw new Error("translation has more headings than its English source");
      }
      output.push(`<a id="${sourceIds[headingIndex]}"></a>`);
      headingIndex += 1;
    }
    output.push(line);
  }
  if (headingIndex !== sourceIds.length) {
    throw new Error(
      `translation has ${headingIndex} headings but its English source has ${sourceIds.length}`,
    );
  }
  return normalize(
    `${marker}\n${output.join("\n")}`,
  );
}

export function relativeLink(from, to) {
  return (
    path.posix.relative(
      path.posix.dirname(from.replaceAll("\\", "/")),
      to.replaceAll("\\", "/"),
    ) || path.posix.basename(to)
  );
}

export function headingIds(markdown) {
  const ids = [];
  const counts = new Map();
  let inFence = false;
  for (const line of normalize(markdown).split("\n")) {
    if (/^\s*(```|~~~)/.test(line)) {
      inFence = !inFence;
      continue;
    }
    if (inFence) {
      continue;
    }
    const match = line.match(/^#{1,6}\s+(.+?)\s*#*\s*$/);
    if (!match) {
      continue;
    }
    const base = githubSlug(match[1]);
    const count = counts.get(base) ?? 0;
    counts.set(base, count + 1);
    ids.push(count === 0 ? base : `${base}-${count}`);
  }
  return ids;
}

export function githubSlug(heading) {
  return heading
    .toLowerCase()
    .replace(/<[^>]*>/g, "")
    .replace(/[`*_~]/g, "")
    .replace(/[^\p{L}\p{N}\s-]/gu, "")
    .trim()
    .replace(/\s+/g, "-");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
