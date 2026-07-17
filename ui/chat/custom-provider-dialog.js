(function () {
  const PROVIDER_ID = /^[a-z0-9][a-z0-9-_]*$/;
  const MAX_MODELS = 100;
  const MAX_HEADERS = 32;
  const MAX_TOKEN_COUNT = 4_294_967_295;
  const SAVE_TIMEOUT_MS = 20_000;
  const LATE_RESPONSE_GRACE_MS = 60_000;

  function createPendingSaveRegistry(options = {}) {
    const graceMs = options.graceMs ?? LATE_RESPONSE_GRACE_MS;
    const schedule = options.setTimeout || ((callback, delay) => window.setTimeout(callback, delay));
    const cancel = options.clearTimeout || ((timer) => window.clearTimeout(timer));
    const entries = new Map();

    function add(requestId, providerId) {
      take(requestId);
      entries.set(String(requestId), {
        providerId: String(providerId || ""),
        timedOut: false,
        expiryTimer: null,
      });
    }

    function markTimedOut(requestId) {
      const key = String(requestId);
      const entry = entries.get(key);
      if (!entry) {
        return false;
      }
      entry.timedOut = true;
      if (entry.expiryTimer !== null) {
        cancel(entry.expiryTimer);
      }
      entry.expiryTimer = schedule(() => entries.delete(key), graceMs);
      return true;
    }

    function peek(requestId) {
      const entry = entries.get(String(requestId));
      return entry ? { providerId: entry.providerId, timedOut: entry.timedOut } : null;
    }

    function take(requestId) {
      const key = String(requestId);
      const entry = entries.get(key);
      if (!entry) {
        return null;
      }
      if (entry.expiryTimer !== null) {
        cancel(entry.expiryTimer);
      }
      entries.delete(key);
      return { providerId: entry.providerId, timedOut: entry.timedOut };
    }

    function clear() {
      entries.forEach((entry) => {
        if (entry.expiryTimer !== null) {
          cancel(entry.expiryTimer);
        }
      });
      entries.clear();
    }

    return {
      add,
      clear,
      markTimedOut,
      peek,
      take,
      get size() {
        return entries.size;
      },
    };
  }

  function matchesSaveResponse(requestId, activeRequestId, timedOutRequestId) {
    if (!requestId) {
      return true;
    }
    return requestId === activeRequestId
      || (!activeRequestId && requestId === timedOutRequestId);
  }

  function validateCustomProvider(raw, existingProviderIds = new Set()) {
    const value = {
      update_existing: Boolean(raw?.update_existing),
      provider_id: String(raw?.provider_id || "").trim(),
      display_name: String(raw?.display_name || "").trim(),
      base_url: String(raw?.base_url || "").trim().replace(/\/+$/, ""),
      api_key: String(raw?.api_key || "").trim(),
      models: Array.isArray(raw?.models) ? raw.models : [],
      headers: Array.isArray(raw?.headers) ? raw.headers : [],
    };
    const errors = { fields: {}, models: [], headers: [] };

    if (!value.provider_id) {
      errors.fields.provider_id = "Provider ID is required.";
    } else if (!PROVIDER_ID.test(value.provider_id)) {
      errors.fields.provider_id = "Use lowercase letters, numbers, hyphens, or underscores.";
    } else if (existingProviderIds.has(value.provider_id)) {
      errors.fields.provider_id = "That provider ID already exists.";
    }
    if (!value.display_name) {
      errors.fields.display_name = "Display name is required.";
    }
    if (!value.base_url) {
      errors.fields.base_url = "Base URL is required.";
    } else if (!validHttpUrl(value.base_url)) {
      errors.fields.base_url = "Enter a valid http:// or https:// URL.";
    }

    const seenModels = new Set();
    value.models = value.models.map((model, index) => {
      const clean = {
        id: String(model?.id || "").trim(),
        name: String(model?.name || "").trim(),
        context_length: requiredTokenCount(model?.context_length),
        max_output_tokens: requiredTokenCount(model?.max_output_tokens),
      };
      const rowErrors = {};
      if (!clean.id) {
        rowErrors.id = "Required";
      } else if (seenModels.has(clean.id)) {
        rowErrors.id = "Duplicate";
      } else {
        seenModels.add(clean.id);
      }
      if (!clean.name) {
        rowErrors.name = "Required";
      }
      if (clean.context_length === null) {
        rowErrors.context_length = "Enter a positive whole number";
      }
      if (clean.max_output_tokens === null) {
        rowErrors.max_output_tokens = "Enter a positive whole number";
      } else if (
        clean.context_length !== null
        && clean.max_output_tokens > clean.context_length
      ) {
        rowErrors.max_output_tokens = "Cannot exceed context length";
      }
      errors.models[index] = rowErrors;
      return clean;
    });
    if (!value.models.length) {
      errors.models[0] = { id: "Add at least one model.", name: "Required" };
    }

    const seenHeaders = new Set();
    value.headers = value.headers
      .map((header, index) => {
        const clean = {
          name: String(header?.name || "").trim(),
          value: String(header?.value || "").trim(),
        };
        const rowErrors = {};
        if (!clean.name && !clean.value) {
          errors.headers[index] = rowErrors;
          return null;
        }
        const normalizedName = clean.name.toLowerCase();
        if (!clean.name) {
          rowErrors.name = "Required";
        } else if (seenHeaders.has(normalizedName)) {
          rowErrors.name = "Duplicate";
        } else {
          seenHeaders.add(normalizedName);
        }
        if (!clean.value) {
          rowErrors.value = "Required";
        }
        errors.headers[index] = rowErrors;
        return clean;
      })
      .filter(Boolean);

    const invalid = Object.keys(errors.fields).length > 0
      || errors.models.some((row) => Object.keys(row || {}).length > 0)
      || errors.headers.some((row) => Object.keys(row || {}).length > 0);
    return invalid ? { errors } : { errors, value };
  }

  function requiredTokenCount(value) {
    const text = String(value ?? "").trim();
    if (!/^\d+$/.test(text)) {
      return null;
    }
    const count = Number(text);
    return Number.isSafeInteger(count) && count > 0 && count <= MAX_TOKEN_COUNT ? count : null;
  }

  function validHttpUrl(value) {
    try {
      const url = new URL(value);
      return (url.protocol === "http:" || url.protocol === "https:")
        && Boolean(url.hostname)
        && !url.username
        && !url.password
        && !url.search
        && !url.hash;
    } catch {
      return false;
    }
  }

  function createCustomProviderDialog(options = {}) {
    const popover = options.popover || null;
    const callbacks = options.callbacks || {};
    const ensureDaemonConnected = callbacks.ensureDaemonConnected || (() => true);
    const closeProviderPicker = callbacks.closeProviderPicker || (() => {});
    const openProviderPicker = callbacks.openProviderPicker || (() => false);
    const getProviderIds = callbacks.getProviderIds || (() => new Set());
    const onSubmit = callbacks.onSubmit || (() => false);
    const onTimeout = callbacks.onTimeout || (() => {});
    if (!popover) {
      return emptyController();
    }

    const form = popover.querySelector("[data-custom-provider-form]");
    const providerIdInput = popover.querySelector("[data-custom-provider-id]");
    const displayNameInput = popover.querySelector("[data-custom-provider-name]");
    const baseUrlInput = popover.querySelector("[data-custom-provider-base-url]");
    const apiKeyInput = popover.querySelector("[data-custom-provider-api-key]");
    const modelRows = popover.querySelector("[data-custom-provider-models]");
    const headerRows = popover.querySelector("[data-custom-provider-headers]");
    const addModelButton = popover.querySelector("[data-custom-provider-add-model]");
    const addHeaderButton = popover.querySelector("[data-custom-provider-add-header]");
    const submitButton = popover.querySelector("[data-custom-provider-submit]");
    const errorBox = popover.querySelector("[data-custom-provider-error]");
    const title = popover.querySelector("[data-custom-provider-title]");
    const apiKeyHelp = popover.querySelector("[data-custom-provider-api-key-help]");
    const headersHelp = popover.querySelector("[data-custom-provider-headers-help]");
    let saving = false;
    let editingProviderId = "";
    let activeRequestId = "";
    let timedOutRequestId = "";
    let saveTimeout = null;

    function open(provider = null) {
      if (!ensureDaemonConnected()) {
        return false;
      }
      closeProviderPicker();
      reset();
      if (provider?.kind === "custom" && provider.custom) {
        beginEdit(provider);
      }
      popover.hidden = false;
      popover.setAttribute("tabindex", "-1");
      position();
      window.setTimeout(
        () => (editingProviderId ? displayNameInput : providerIdInput)?.focus({ preventScroll: true }),
        0,
      );
      return true;
    }

    function close() {
      if (popover.hidden || saving) {
        return false;
      }
      popover.hidden = true;
      return true;
    }

    function back(event) {
      event?.stopPropagation();
      if (close()) {
        openProviderPicker();
      }
    }

    function reset() {
      clearSaveTimeout();
      saving = false;
      activeRequestId = "";
      timedOutRequestId = "";
      editingProviderId = "";
      form?.reset();
      if (providerIdInput) {
        providerIdInput.disabled = false;
      }
      modelRows?.replaceChildren();
      headerRows?.replaceChildren();
      addModelRow();
      addHeaderRow();
      clearErrors();
      syncSaving();
      syncModeCopy();
    }

    function beginEdit(provider) {
      editingProviderId = String(provider.id || "").trim();
      if (providerIdInput) {
        providerIdInput.value = editingProviderId;
        providerIdInput.disabled = true;
      }
      if (displayNameInput) {
        displayNameInput.value = String(provider.name || "");
      }
      if (baseUrlInput) {
        baseUrlInput.value = String(provider.custom?.base_url || "");
      }
      modelRows?.replaceChildren();
      const models = Array.isArray(provider.custom?.models) ? provider.custom.models : [];
      if (models.length) {
        models.forEach((model) => addModelRow(model));
      } else {
        addModelRow();
      }
      syncModeCopy(provider.custom?.header_count || 0);
      syncSaving();
    }

    function position() {
      if (popover.hidden) {
        return;
      }
      const viewportPad = 10;
      const width = Math.min(660, Math.max(300, window.innerWidth - viewportPad * 2));
      const height = popover.offsetHeight || 620;
      popover.style.width = width + "px";
      popover.style.left = Math.max(viewportPad, (window.innerWidth - width) / 2) + "px";
      popover.style.top = Math.max(viewportPad, (window.innerHeight - height) / 2) + "px";
      popover.dataset.side = "center";
    }

    function addModelRow(model = null) {
      if (!modelRows || modelRows.children.length >= MAX_MODELS) {
        return;
      }
      modelRows.append(createRow("model", model));
      syncRemoveButtons(modelRows);
      schedulePosition();
    }

    function addHeaderRow(header = null) {
      if (!headerRows || headerRows.children.length >= MAX_HEADERS) {
        return;
      }
      headerRows.append(createRow("header", header));
      syncRemoveButtons(headerRows);
      schedulePosition();
    }

    function createRow(kind, value = null) {
      const row = document.createElement("div");
      row.className = kind === "model"
        ? "custom-provider-row custom-provider-model-row"
        : "custom-provider-row";
      row.dataset.customProviderRow = kind;
      const first = rowField(
        kind === "model" ? "model-id" : "header-name",
        kind === "model" ? "model-id" : "Header-Name",
        kind === "model" ? "Model ID" : "Header name",
      );
      const second = rowField(
        kind === "model" ? "model-name" : "header-value",
        kind === "model" ? "Display name" : "value",
        kind === "model" ? "Model display name" : "Header value",
      );
      const fields = [first, second];
      if (kind === "model") {
        fields.push(
          rowField("model-context-length", "64000", "Context length", {
            caption: "Context length",
            help: "The model's full context window in tokens. Fennara uses this value to compact the conversation before the provider rejects an oversized prompt.",
            numeric: true,
          }),
          rowField("model-max-output-tokens", "4096", "Max output tokens", {
            caption: "Max output tokens",
            help: "The largest response this model can produce, in tokens. Fennara uses this value to keep compaction summaries within the model's output limit.",
            numeric: true,
          }),
        );
      }
      const inputs = fields.map((field) => field.querySelector("input"));
      if (kind === "model") {
        inputs[0].value = String(value?.id || "");
        inputs[1].value = String(value?.name || "");
        inputs[2].value = String(value?.context_length || "");
        inputs[3].value = String(value?.max_output_tokens || "");
        syncDefaultValueAppearance(inputs[2]);
        syncDefaultValueAppearance(inputs[3]);
      } else {
        inputs[0].value = String(value?.name || "");
        inputs[1].value = String(value?.value || "");
      }
      const remove = document.createElement("button");
      remove.type = "button";
      remove.className = "custom-provider-remove";
      remove.dataset.customProviderRemove = "";
      remove.textContent = "×";
      remove.setAttribute("aria-label", kind === "model" ? "Remove model" : "Remove header");
      remove.addEventListener("click", () => {
        const container = row.parentElement;
        if (!container || container.children.length <= 1) {
          return;
        }
        row.remove();
        syncRemoveButtons(container);
        schedulePosition();
      });
      row.append(...fields, remove);
      return row;
    }

    function rowField(field, placeholder, labelText, options = {}) {
      const label = document.createElement("label");
      if (options.caption) {
        label.className = "custom-provider-limit-field";
        const caption = document.createElement("span");
        caption.className = "custom-provider-limit-caption";
        caption.textContent = options.caption;
        const help = document.createElement("span");
        help.className = "custom-provider-limit-help";
        help.textContent = "?";
        help.tabIndex = 0;
        help.title = options.help;
        help.setAttribute("aria-label", options.help);
        caption.append(help);
        label.append(caption);
      }
      const input = document.createElement("input");
      input.type = options.numeric ? "number" : "text";
      if (options.numeric) {
        input.inputMode = "numeric";
        input.min = "1";
        input.max = String(MAX_TOKEN_COUNT);
        input.step = "1";
        input.dataset.customProviderDefaultValue = placeholder;
        input.addEventListener("focus", () => {
          if (input.classList.contains("is-default-value")) {
            window.requestAnimationFrame(() => input.select());
          }
        });
      }
      input.autocomplete = "off";
      input.placeholder = placeholder;
      input.dataset.customProviderField = field;
      input.setAttribute("aria-label", labelText);
      input.addEventListener("input", () => {
        clearInputError(input);
        syncDefaultValueAppearance(input);
      });
      const error = document.createElement("small");
      error.dataset.customProviderFieldError = field;
      error.hidden = true;
      label.append(input, error);
      return label;
    }

    function syncDefaultValueAppearance(input) {
      const defaultValue = input?.dataset.customProviderDefaultValue;
      input?.classList.toggle(
        "is-default-value",
        Boolean(defaultValue && input.value === defaultValue),
      );
    }

    function syncRemoveButtons(container) {
      const disabled = container.children.length <= 1;
      container.querySelectorAll("[data-custom-provider-remove]").forEach((button) => {
        button.disabled = disabled;
      });
    }

    function collect() {
      return {
        provider_id: providerIdInput?.value,
        update_existing: Boolean(editingProviderId),
        display_name: displayNameInput?.value,
        base_url: baseUrlInput?.value,
        api_key: apiKeyInput?.value,
        models: Array.from(modelRows?.children || []).map((row) => ({
          id: row.querySelector('[data-custom-provider-field="model-id"]')?.value,
          name: row.querySelector('[data-custom-provider-field="model-name"]')?.value,
          context_length: row.querySelector('[data-custom-provider-field="model-context-length"]')?.value,
          max_output_tokens: row.querySelector('[data-custom-provider-field="model-max-output-tokens"]')?.value,
        })),
        headers: Array.from(headerRows?.children || []).map((row) => ({
          name: row.querySelector('[data-custom-provider-field="header-name"]')?.value,
          value: row.querySelector('[data-custom-provider-field="header-value"]')?.value,
        })),
      };
    }

    function submit(event) {
      event.preventDefault();
      if (saving) {
        return;
      }
      clearErrors();
      const existingIds = new Set(
        Array.from(getProviderIds()).filter((providerId) => providerId !== editingProviderId),
      );
      const result = validateCustomProvider(collect(), existingIds);
      if (!result.value) {
        renderErrors(result.errors);
        return;
      }
      saving = true;
      syncSaving();
      const requestId = onSubmit(result.value);
      if (!requestId) {
        handleError("The daemon is not connected yet.");
        return;
      }
      activeRequestId = String(requestId);
      timedOutRequestId = "";
      saveTimeout = window.setTimeout(() => {
        timedOutRequestId = activeRequestId;
        activeRequestId = "";
        saveTimeout = null;
        saving = false;
        syncSaving();
        onTimeout(timedOutRequestId);
        showError("Saving the provider timed out. Check the daemon and try again.");
      }, SAVE_TIMEOUT_MS);
    }

    function renderErrors(errors) {
      setInputError(providerIdInput, errors.fields.provider_id);
      setInputError(displayNameInput, errors.fields.display_name);
      setInputError(baseUrlInput, errors.fields.base_url);
      renderRowErrors(
        modelRows,
        errors.models,
        ["model-id", "model-name", "model-context-length", "model-max-output-tokens"],
        ["id", "name", "context_length", "max_output_tokens"],
      );
      renderRowErrors(headerRows, errors.headers, ["header-name", "header-value"], ["name", "value"]);
      const firstError = popover.querySelector('[aria-invalid="true"]');
      firstError?.focus({ preventScroll: true });
      firstError?.scrollIntoView({ block: "nearest" });
    }

    function renderRowErrors(container, errors, fields, keys) {
      Array.from(container?.children || []).forEach((row, index) => {
        fields.forEach((field, fieldIndex) => {
          const input = row.querySelector(`[data-custom-provider-field="${field}"]`);
          const message = errors[index]?.[keys[fieldIndex]];
          setInputError(input, message);
        });
      });
    }

    function setInputError(input, message) {
      if (!input) {
        return;
      }
      input.setAttribute("aria-invalid", message ? "true" : "false");
      const field = input.dataset.customProviderField;
      const error = field
        ? input.parentElement?.querySelector(`[data-custom-provider-field-error="${field}"]`)
        : null;
      if (error) {
        error.textContent = message || "";
        error.hidden = !message;
      }
      if (message && !error) {
        input.title = message;
      } else {
        input.removeAttribute("title");
      }
    }

    function clearInputError(input) {
      setInputError(input, "");
      hideError();
    }

    function clearErrors() {
      popover.querySelectorAll("input").forEach((input) => setInputError(input, ""));
      popover.querySelectorAll("[data-custom-provider-field-error]").forEach((error) => {
        error.textContent = "";
        error.hidden = true;
      });
      hideError();
    }

    function handleSaved(requestId = "") {
      if (!matchesSaveResponse(requestId, activeRequestId, timedOutRequestId)) {
        return false;
      }
      clearSaveTimeout();
      activeRequestId = "";
      timedOutRequestId = "";
      saving = false;
      syncSaving();
      close();
      return true;
    }

    function handleError(message, requestId = "") {
      if (!matchesSaveResponse(requestId, activeRequestId, timedOutRequestId)) {
        return false;
      }
      clearSaveTimeout();
      activeRequestId = "";
      timedOutRequestId = "";
      saving = false;
      syncSaving();
      showError(message);
      return true;
    }

    function showError(message) {
      if (errorBox) {
        errorBox.textContent = message || "Could not add the custom provider.";
        errorBox.hidden = false;
        errorBox.scrollIntoView({ block: "nearest" });
      }
    }

    function hideError() {
      if (errorBox) {
        errorBox.textContent = "";
        errorBox.hidden = true;
      }
    }

    function syncSaving() {
      if (submitButton) {
        submitButton.disabled = saving;
        submitButton.textContent = saving
          ? editingProviderId ? "Saving..." : "Adding..."
          : editingProviderId ? "Save provider" : "Add provider";
      }
    }

    function syncModeCopy(savedHeaderCount = 0) {
      if (title) {
        title.textContent = editingProviderId ? "Edit custom provider" : "Custom provider";
      }
      if (apiKeyInput) {
        apiKeyInput.placeholder = editingProviderId ? "Leave empty to keep saved key" : "API key";
      }
      if (apiKeyHelp) {
        apiKeyHelp.textContent = editingProviderId
          ? "Leave empty to keep the saved API key."
          : "Optional. Leave empty if authentication is handled through headers.";
      }
      if (headersHelp) {
        headersHelp.textContent = editingProviderId && savedHeaderCount
          ? `(${savedHeaderCount} saved; new values merge)`
          : "(optional)";
      }
    }

    function schedulePosition() {
      window.requestAnimationFrame(position);
    }

    function clearSaveTimeout() {
      if (saveTimeout !== null) {
        window.clearTimeout(saveTimeout);
        saveTimeout = null;
      }
    }

    form?.addEventListener("submit", submit);
    addModelButton?.addEventListener("click", addModelRow);
    addHeaderButton?.addEventListener("click", addHeaderRow);
    popover.querySelector("[data-custom-provider-back]")?.addEventListener("click", back);
    popover.querySelector("[data-custom-provider-close]")?.addEventListener("click", close);
    [providerIdInput, displayNameInput, baseUrlInput, apiKeyInput].forEach((input) => {
      input?.addEventListener("input", () => clearInputError(input));
    });
    reset();

    return { back, close, handleError, handleSaved, open, position };
  }

  function emptyController() {
    const noop = () => {};
    return { back: noop, close: noop, handleError: noop, handleSaved: noop, open: () => false, position: noop };
  }

  window.FennaraCustomProviderDialog = {
    createPendingSaveRegistry,
    createCustomProviderDialog,
    matchesSaveResponse,
    validateCustomProvider,
  };
})();
