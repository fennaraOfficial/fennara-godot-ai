import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const source = readFileSync(
  new URL("../../ui/chat/custom-provider-dialog.js", import.meta.url),
  "utf8",
);
const context = { URL, window: {} };
vm.runInNewContext(source, context);
const validate = context.window.FennaraCustomProviderDialog.validateCustomProvider;

test("custom provider form accepts OmniRoute-compatible configuration", () => {
  const result = validate(
    {
      provider_id: "omniroute",
      display_name: "OmniRoute",
      base_url: "http://localhost:20128/v1/",
      api_key: " secret ",
      models: [{
        id: "zai/glm-5",
        name: "GLM 5",
        context_length: "131072",
        max_output_tokens: "8192",
      }],
      headers: [{ name: "X-Router", value: "primary" }],
    },
    new Set(["openai", "nvidia"]),
  );

  assert.deepEqual(
    JSON.parse(JSON.stringify(result.value)),
    {
      update_existing: false,
      provider_id: "omniroute",
      display_name: "OmniRoute",
      base_url: "http://localhost:20128/v1",
      api_key: "secret",
      models: [{
        id: "zai/glm-5",
        name: "GLM 5",
        context_length: 131072,
        max_output_tokens: 8192,
      }],
      headers: [{ name: "X-Router", value: "primary" }],
    },
  );
});

test("custom provider form rejects duplicate providers, models, and headers", () => {
  const result = validate(
    {
      provider_id: "openai",
      display_name: "Duplicate",
      base_url: "ftp://example.com/v1",
      models: [
        { id: "model", name: "Model", context_length: 64000, max_output_tokens: 4096 },
        { id: "model", name: "Duplicate model", context_length: 64000, max_output_tokens: 4096 },
      ],
      headers: [
        { name: "Authorization", value: "one" },
        { name: "authorization", value: "two" },
      ],
    },
    new Set(["openai"]),
  );

  assert.equal(result.value, undefined);
  assert.equal(result.errors.fields.provider_id, "That provider ID already exists.");
  assert.equal(result.errors.fields.base_url, "Enter a valid http:// or https:// URL.");
  assert.equal(result.errors.models[1].id, "Duplicate");
  assert.equal(result.errors.headers[1].name, "Duplicate");
});

test("custom provider edits preserve update intent and reject URL queries", () => {
  const valid = validate({
    update_existing: true,
    provider_id: "omniroute",
    display_name: "OmniRoute",
    base_url: "http://localhost:20128/v1",
    models: [{ id: "zai/glm-5", name: "GLM 5", context_length: 131072, max_output_tokens: 8192 }],
    headers: [],
  });
  assert.equal(valid.value.update_existing, true);

  const invalid = validate({
    provider_id: "router",
    display_name: "Router",
    base_url: "https://example.com/v1?token=secret",
    models: [{ id: "model", name: "Model", context_length: 64000, max_output_tokens: 4096 }],
    headers: [],
  });
  assert.equal(invalid.value, undefined);
  assert.match(invalid.errors.fields.base_url, /valid http/);
});

test("custom provider form requires valid model limits", () => {
  const missing = validate({
    provider_id: "router",
    display_name: "Router",
    base_url: "https://example.com/v1",
    models: [{ id: "model", name: "Model" }],
    headers: [],
  });
  assert.equal(missing.value, undefined);
  assert.match(missing.errors.models[0].context_length, /positive whole number/);
  assert.match(missing.errors.models[0].max_output_tokens, /positive whole number/);

  const excessiveOutput = validate({
    provider_id: "router",
    display_name: "Router",
    base_url: "https://example.com/v1",
    models: [{ id: "model", name: "Model", context_length: 4096, max_output_tokens: 8192 }],
    headers: [],
  });
  assert.equal(excessiveOutput.value, undefined);
  assert.equal(excessiveOutput.errors.models[0].max_output_tokens, "Cannot exceed context length");
});
