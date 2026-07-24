import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const source = readFileSync(
  new URL("../../ui/chat/settings-panel.js", import.meta.url),
  "utf8",
);
const context = { window: {} };
vm.runInNewContext(source, context);
const includeTelemetryPreference =
  context.window.FennaraSettingsPanel.includeTelemetryPreference;

test("environment-controlled settings saves preserve the stored telemetry preference", () => {
  const payload = { type: "save_settings" };

  includeTelemetryPreference(payload, false, true);

  assert.equal(Object.hasOwn(payload, "telemetry_enabled"), false);
});

test("user-controlled settings saves include the selected telemetry preference", () => {
  const payload = { type: "save_settings" };

  includeTelemetryPreference(payload, false, false);

  assert.equal(payload.telemetry_enabled, false);
});
