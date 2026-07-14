import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const discovery = read("../../fennara-cpp/src/release/discovery.cpp");
const identity = read("../../fennara-cpp/src/release/identity.cpp");
const version = read("../../fennara-cpp/src/release/version.cpp");

test("native release discovery uses a bare TLS host and requires a complete body", () => {
  assert.match(discovery, /connect_to_host\("api\.github\.com", 443,/);
  assert.doesNotMatch(discovery, /connect_to_host\("https:\/\//);
  assert.match(discovery, /bool response_complete = false;/);
  assert.match(discovery, /if \(!response_complete\)/);
  assert.match(discovery, /Timed out reading the GitHub response/);
});

test("native manifest versions are strict after explicit boundary normalization", () => {
  const parser = /std::optional<Version> parse\(godot::String input\) \{([\s\S]*?)\n\}/.exec(version);
  assert.ok(parser, "missing native release version parser");
  assert.doesNotMatch(parser[1], /normalize\(/);
  assert.match(discovery, /release_version::normalize\(godot::String\(release\.get\("tag_name"/);
});

test("native identity schema rejects string and boolean coercion", () => {
  assert.match(identity, /schema_type != godot::Variant::INT/);
  assert.match(identity, /schema_type != godot::Variant::FLOAT/);
  assert.match(identity, /\(double\)schema_version != 1\.0/);
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
