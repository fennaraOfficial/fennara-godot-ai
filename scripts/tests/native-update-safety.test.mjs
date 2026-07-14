import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const discoveryHeader = read("../../fennara-cpp/include/fennara/release/discovery.hpp");
const pluginSource = read("../../fennara-cpp/src/ui/fennara_plugin.cpp");
const bridgeSource = read("../../fennara-cpp/src/local_bridge/local_bridge.cpp");
const daemonSource = read("../../fennara-cpp/src/local_bridge/daemon.cpp");
const setupLockSource = read("../../fennara-cpp/src/setup/first_run_lock.cpp");
const projectInstallSource = read("../../local/crates/fennara-cli/src/project_install.rs");
const updateNoticeSource = read("../../fennara-cpp/src/update_notice.cpp");

test("native update discovery cancellation returns the shared check to idle", () => {
  assert.match(discoveryHeader, /bool cancelled = false;/);
  assert.match(
    updateNoticeSource,
    /if \(result\.cancelled\) \{\s*current\.check_started = false;\s*current\.checked = false;\s*return;/,
  );
});

test("native update state is initialized lazily after the extension loads", () => {
  assert.match(updateNoticeSource, /State &state\(\) \{\s*static State instance;/);
  assert.doesNotMatch(updateNoticeSource, /release_discovery::Result g_[a-z_]+;/);
});

test("plugin teardown signals cancellation before joining discovery", () => {
  const stop = /void FennaraPlugin::_stop_update_check\(\) \{([\s\S]*?)\n\}/.exec(pluginSource);
  assert.ok(stop, "missing update-check teardown helper");
  const cancel = stop[1].indexOf("update_check_cancelled.store(true");
  const join = stop[1].indexOf("update_check_thread.join()");
  assert.ok(cancel >= 0 && cancel < join, "teardown must cancel before joining the worker");
});

test("first-run setup keeps the local bridge dormant until components match", () => {
  assert.match(setupLockSource, /bool installed_components_match_addon\(\)/);
  assert.match(daemonSource, /if \(!installed_components_match_addon\(\)\)/);

  const connectStart = bridgeSource.indexOf("void FennaraLocalBridge::_connect_socket()");
  const connectEnd = bridgeSource.indexOf("void FennaraLocalBridge::_close_socket()", connectStart);
  assert.ok(connectStart >= 0 && connectEnd > connectStart, "missing local bridge connect body");
  const connectBody = bridgeSource.slice(connectStart, connectEnd);
  const readinessGate = connectBody.indexOf("if (!installed_components_match_addon())");
  const daemonAuthentication = connectBody.indexOf("_daemon_auth_future");
  assert.ok(
    readinessGate >= 0 && readinessGate < daemonAuthentication,
    "component readiness must be checked before daemon authentication",
  );
});

test("project install stops an idle old daemon before switching versions", () => {
  const helperStart = projectInstallSource.indexOf("fn prepare_version_switch(");
  const helperEnd = projectInstallSource.indexOf("fn active_version(", helperStart);
  assert.ok(helperStart >= 0 && helperEnd > helperStart, "missing version switch helper");
  const helperBody = projectInstallSource.slice(helperStart, helperEnd);
  const conflictCheck = helperBody.indexOf("daemon_setup::ensure_switch_available");
  const shutdown = helperBody.indexOf("daemon_setup::shutdown_if_running");
  assert.ok(
    conflictCheck >= 0 && conflictCheck < shutdown,
    "other projects must be rejected before the old daemon is stopped",
  );
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
