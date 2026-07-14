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
  const readinessStart = setupLockSource.indexOf("bool installed_components_match_addon()");
  const readinessEnd = setupLockSource.indexOf(
    "bool FirstRunSetup::_installed_components_match()",
    readinessStart,
  );
  assert.ok(readinessStart >= 0 && readinessEnd > readinessStart, "missing readiness helper");
  const readinessBody = setupLockSource.slice(readinessStart, readinessEnd);
  assert.match(readinessBody, /res:\/\/addons\/fennara\/VERSION/);
  assert.match(readinessBody, /app_paths::cli_binary_path\(\)/);
  assert.match(readinessBody, /app_paths::daemon_binary_path\(\)/);
  assert.match(readinessBody, /current\.get\("version", ""\)\) == expected_version/);

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

  const spawnGate = daemonSource.indexOf("if (!installed_components_match_addon())");
  const daemonSpawn = daemonSource.indexOf("create_process", spawnGate);
  assert.ok(
    spawnGate >= 0 && daemonSpawn > spawnGate,
    "component readiness must be checked before daemon spawning",
  );
});

test("project install stops an idle old daemon before switching versions", () => {
  const helperStart = projectInstallSource.indexOf("fn prepare_version_switch(");
  const helperEnd = projectInstallSource.indexOf("fn active_version(", helperStart);
  assert.ok(helperStart >= 0 && helperEnd > helperStart, "missing version switch helper");
  const helperBody = projectInstallSource.slice(helperStart, helperEnd);
  const conflictCheck = helperBody.indexOf("daemon_setup::ensure_switch_available");
  const shutdown = helperBody.indexOf("daemon_setup::shutdown_if_running");
  assert.match(helperBody, /daemon_setup::ensure_switch_available\(layout, None\)/);
  assert.ok(
    conflictCheck >= 0 && conflictCheck < shutdown,
    "all connected projects must be rejected before the old daemon is stopped",
  );
});

function read(relativePath) {
  return readFileSync(new URL(relativePath, import.meta.url), "utf8");
}
