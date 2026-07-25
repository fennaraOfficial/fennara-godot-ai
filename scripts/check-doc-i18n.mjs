import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import {
  MARKER_PATTERN,
  config,
  githubSlug,
  headingIds,
  normalize,
  renderNavigation,
  replaceNavigation,
  root,
  sourceHash,
  targetFor,
} from "./doc-i18n-lib.mjs";

const failures = [];
const translatedLocales = config.locales.slice(1);
const expectedTargets = new Set();
let translatedCount = 0;

for (const document of config.documents) {
  const source = read(document.source);
  expectManagedNavigation(document, "en", document.source, source);
  checkLinks(document.source, source);

  const expectedHash = sourceHash(source);
  const sourceHeadingLevels = headingLevels(source);
  const sourceIds = headingIds(source);
  const sourceInlineCode = inlineCode(source);
  const sourceStrictFences = strictFences(source);
  const sourceFenceSignatures = fenceSignatures(source);
  const sourceAdmonitions = admonitions(source);
  const sourceExternalUrls = externalUrls(source);
  const sourceTableShapes = tableShapes(source);
  const sourceHtmlTags = htmlTagNames(source);
  const sourceListShapes = listShapes(source);
  const sourceBlockShapes = blockShapes(source);

  for (const locale of translatedLocales) {
    const target = targetFor(document, locale.code);
    expectedTargets.add(target);
    if (!existsSync(path.join(root, target))) {
      failures.push(`${target}: missing AI-authored translation`);
      continue;
    }
    translatedCount += 1;
    const translation = read(target);
    const marker = translation.match(MARKER_PATTERN);
    if (!marker) {
      failures.push(`${target}: missing source marker`);
    } else {
      if (marker[1] !== locale.code || marker[2] !== document.source) {
        failures.push(`${target}: source marker points to the wrong document`);
      }
      if (marker[3] !== expectedHash) {
        failures.push(`${target}: translation is stale for ${document.source}`);
      }
    }

    expectManagedNavigation(document, locale.code, target, translation);
    expectEqual(`${target}: heading structure changed`, sourceHeadingLevels, headingLevels(translation));
    expectEqual(`${target}: inline code changed`, sourceInlineCode, inlineCode(translation));
    expectEqual(`${target}: strict code fence changed`, sourceStrictFences, strictFences(translation));
    expectEqual(`${target}: fence structure changed`, sourceFenceSignatures, fenceSignatures(translation));
    expectEqual(`${target}: admonition type changed`, sourceAdmonitions, admonitions(translation));
    expectEqual(`${target}: external URL changed`, sourceExternalUrls, externalUrls(translation));
    expectEqual(`${target}: table structure changed`, sourceTableShapes, tableShapes(translation));
    expectEqual(`${target}: raw HTML structure changed`, sourceHtmlTags, htmlTagNames(translation));
    expectEqual(`${target}: list structure changed`, sourceListShapes, listShapes(translation));
    expectEqual(`${target}: document block structure changed`, sourceBlockShapes, blockShapes(translation));

    for (const id of sourceIds) {
      const anchor = `<a id="${id}"></a>`;
      const count = translation.split(anchor).length - 1;
      if (count !== 1) {
        failures.push(`${target}: expected one stable source anchor #${id}, found ${count}`);
      }
    }
    if (translation.includes("\u2014")) {
      failures.push(`${target}: contains an em dash`);
    }
    checkLinks(target, translation);
  }
}

const expectedCount = config.documents.length * translatedLocales.length;
if (translatedCount !== expectedCount) {
  failures.push(`translation coverage is ${translatedCount}/${expectedCount} files`);
}
for (const target of localizedMarkdownFiles()) {
  if (!expectedTargets.has(target)) {
    failures.push(`${target}: unexpected localized document`);
  }
}

if (failures.length > 0) {
  console.error("Documentation translation check failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `Documentation translation check passed for ${translatedCount} AI-authored files across ${translatedLocales.length} translated locales.`,
);

function expectManagedNavigation(document, localeCode, relativePath, markdown) {
  const expected = replaceNavigation(markdown, renderNavigation(document, localeCode));
  if (normalize(markdown) !== expected) {
    failures.push(`${relativePath}: managed navigation is out of sync`);
  }
}

function checkLinks(relativePath, markdown) {
  const links = [
    ...markdown.matchAll(/!?\[[^\]]*]\(([^)\s]+)(?:\s+"[^"]*")?\)/g),
    ...markdown.matchAll(/<(?:a|img)\b[^>]*(?:href|src)="([^"]+)"/gi),
  ];
  for (const match of links) {
    const destination = match[1];
    if (/^(?:https?:\/\/|mailto:)/.test(destination)) {
      continue;
    }
    const [filePart, fragment] = destination.split("#", 2);
    const targetRelative = filePart
      ? path.posix.normalize(path.posix.join(path.posix.dirname(relativePath), filePart))
      : relativePath;
    if (!existsSync(path.join(root, targetRelative))) {
      failures.push(`${relativePath}: link target does not exist: ${destination}`);
      continue;
    }
    if (!fragment) {
      continue;
    }
    const target = read(targetRelative);
    const available = new Set(headingIds(target));
    for (const anchor of target.matchAll(/<a id="([^"]+)"><\/a>/g)) {
      available.add(anchor[1]);
    }
    const decoded = githubSlug(decodeURIComponent(fragment));
    if (!available.has(decoded)) {
      failures.push(`${relativePath}: link anchor does not exist: ${destination}`);
    }
  }
}

function fencedBlocks(markdown) {
  return normalize(markdown).match(/^(?:```|~~~)[\s\S]*?^(?:```|~~~)\s*$/gm) ?? [];
}

function strictFences(markdown) {
  return fencedBlocks(markdown).filter((block) => {
    const info = block.split("\n", 1)[0].replace(/^(?:```|~~~)\s*/, "").trim().toLowerCase();
    return info !== "text";
  });
}

function fenceSignatures(markdown) {
  return fencedBlocks(markdown).map((block) => {
    const lines = block.split("\n");
    return `${lines[0].trim()}\n${lines.at(-1).trim()}`;
  });
}

function inlineCode(markdown) {
  const withoutFences = normalize(markdown).replace(
    /^(?:```|~~~)[\s\S]*?^(?:```|~~~)\s*$/gm,
    "",
  );
  return (withoutFences.match(/(?<!`)`[^`]+`(?!`)/g) ?? [])
    .map((value) => value.replace(/\s+/g, " "))
    .sort();
}

function headingLevels(markdown) {
  const withoutFences = normalize(markdown).replace(
    /^(?:```|~~~)[\s\S]*?^(?:```|~~~)\s*$/gm,
    "",
  );
  return [...withoutFences.matchAll(/^(#{1,6})\s+/gm)].map((match) => match[1].length);
}

function admonitions(markdown) {
  return [...normalize(markdown).matchAll(
    /^\s*>\s*\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*$/gm,
  )].map((match) => match[1]);
}

function externalUrls(markdown) {
  return (normalize(markdown).match(/https?:\/\/[^\s<>)"'`]+/g) ?? [])
    .map((value) => value.replace(/[.,;:]$/, ""))
    .sort();
}

function tableShapes(markdown) {
  return normalize(markdown)
    .split("\n")
    .filter((line) => /^\s*\|.*\|\s*$/.test(line))
    .map((line) => (line.match(/\|/g) ?? []).length);
}

function htmlTagNames(markdown) {
  return [...structuralMarkdown(markdown).matchAll(/<\/?([A-Za-z][A-Za-z0-9-]*)\b[^>]*>/g)]
    .map((match) => `${match[0].startsWith("</") ? "/" : ""}${match[1].toLowerCase()}`);
}

function listShapes(markdown) {
  return structuralMarkdown(markdown)
    .split("\n")
    .flatMap((line) => {
      const unordered = line.match(/^(\s*)[-*+]\s+/);
      if (unordered) {
        return [`ul:${unordered[1].length}`];
      }
      const ordered = line.match(/^(\s*)\d+[.)]\s+/);
      return ordered ? [`ol:${ordered[1].length}`] : [];
    });
}

function blockShapes(markdown) {
  return structuralMarkdown(markdown)
    .split(/\n{2,}/)
    .map((block) => block.trim())
    .filter(Boolean)
    .map((block) => {
      if (/^(?:```|~~~)/.test(block)) return "fence";
      if (/^#{1,6}\s+/.test(block)) return "heading";
      if (/^(?:\s*[-*+]\s+|\s*\d+[.)]\s+)/.test(block)) return "list";
      if (/^\s*\|.*\|\s*$/m.test(block)) return "table";
      if (/^\s*>/.test(block)) return "quote";
      if (/^<\/?[A-Za-z]/.test(block)) return "html";
      return "paragraph";
    });
}

function structuralMarkdown(markdown) {
  return normalize(markdown)
    .replace(
      /<!-- fennara-doc-nav:start -->[\s\S]*?<!-- fennara-doc-nav:end -->\n*/g,
      "",
    )
    .replace(
      /<!-- fennara-i18n: locale=[^ ]+ source=[^ ]+ sha256=[a-f0-9]{64} -->\n*/g,
      "",
    )
    .replace(/<a id="[^"]+"><\/a>\n*/g, "");
}

function localizedMarkdownFiles() {
  const files = readdirSync(root, { withFileTypes: true })
    .filter(
      (entry) =>
        entry.isFile() &&
        /^README\.[A-Za-z]{2}(?:-[A-Za-z]{2})?\.md$/.test(entry.name),
    )
    .map((entry) => entry.name);
  walk(path.join(root, "docs", "i18n"), files);
  return files.map((file) => file.replaceAll("\\", "/"));
}

function walk(directory, files) {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      walk(absolute, files);
    } else if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(path.relative(root, absolute));
    }
  }
}

function read(relativePath) {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function expectEqual(message, expected, actual) {
  if (JSON.stringify(expected) !== JSON.stringify(actual)) {
    failures.push(message);
  }
}
