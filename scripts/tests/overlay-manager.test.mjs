import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const source = readFileSync(
  new URL("../../ui/chat/overlay-manager.js", import.meta.url),
  "utf8",
);

function escapeHarness(closeCustomProviderPrompt) {
  let keydown = null;
  let focusCount = 0;
  const context = {
    document: {
      addEventListener(type, handler) {
        if (type === "keydown") {
          keydown = handler;
        }
      },
    },
    window: {
      addEventListener() {},
    },
  };
  vm.runInNewContext(source, context);
  context.window.FennaraOverlayManager.createOverlayManager({
    elements: {
      customProviderPopover: { hidden: false },
    },
    callbacks: {
      closeCustomProviderPrompt,
      focusComposer: () => {
        focusCount += 1;
      },
    },
  });
  const event = {
    key: "Escape",
    prevented: false,
    stopped: false,
    preventDefault() {
      this.prevented = true;
    },
    stopPropagation() {
      this.stopped = true;
    },
  };
  keydown(event);
  return { event, focusCount };
}

test("Escape preserves dialog focus while a custom provider save is active", () => {
  const { event, focusCount } = escapeHarness(() => false);

  assert.equal(event.prevented, true);
  assert.equal(event.stopped, true);
  assert.equal(focusCount, 0);
});

test("Escape focuses the composer after the custom provider dialog closes", () => {
  const { focusCount } = escapeHarness(() => true);

  assert.equal(focusCount, 1);
});
