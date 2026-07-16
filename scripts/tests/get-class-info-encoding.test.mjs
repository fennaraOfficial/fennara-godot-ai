import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const source = read(
  "../../fennara-cpp/src/tools/get_class_info/docs_fetch.cpp",
);

test("official documentation uses the byte-preserving HTTP path first", () => {
  const fallbackStart = source.indexOf("HttpFetchResult _get_text_with_fallback");
  const fallbackEnd = source.indexOf("bool _fetch_class_xml", fallbackStart);
  const fallbackBody = source.slice(fallbackStart, fallbackEnd);
  assert.ok(fallbackStart >= 0 && fallbackEnd > fallbackStart);
  assert.ok(
    fallbackBody.indexOf("_http_get_text(path)") <
      fallbackBody.indexOf("_curl_get_text(path)"),
  );
  assert.match(source, /response_bytes\.append_array\(chunk\)/);
  assert.match(source, /response_bytes\.get_string_from_utf8\(\)/);
});

test("legacy mojibake documentation cache is rejected", () => {
  assert.match(source, /text\[i\] == 0x00e2/);
  assert.match(source, /_contains_utf8_mojibake\(lookup\.xml_text\)/);
  assert.match(source, /_contains_utf8_mojibake\(result\.body\)/);
  assert.match(source, /lookup = CachedXmlLookup\(\)/);
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
