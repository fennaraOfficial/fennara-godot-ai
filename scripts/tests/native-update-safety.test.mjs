import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const discoveryHeader = read("../../fennara-cpp/include/fennara/release/discovery.hpp");
const pluginSource = read("../../fennara-cpp/src/ui/fennara_plugin.cpp");
const updateNoticeSource = read("../../fennara-cpp/src/update_notice.cpp");

test("native update discovery cancellation returns the global check to idle", () => {
  assert.match(discoveryHeader, /bool cancelled = false;/);
  assert.match(
    updateNoticeSource,
    /if \(result\.cancelled\) \{\s*g_check_started = false;\s*g_checked = false;\s*return;/,
  );
});

test("plugin teardown signals cancellation before joining discovery", () => {
  const stop = /void FennaraPlugin::_stop_update_check\(\) \{([\s\S]*?)\n\}/.exec(pluginSource);
  assert.ok(stop, "missing update-check teardown helper");
  const cancel = stop[1].indexOf("update_check_cancelled.store(true");
  const join = stop[1].indexOf("update_check_thread.join()");
  assert.ok(cancel >= 0 && cancel < join, "teardown must cancel before joining the worker");
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
