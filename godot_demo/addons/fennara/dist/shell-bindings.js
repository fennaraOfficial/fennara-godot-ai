(function () {
  function createShellBindings(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const constants = options.constants || {};
    const providerSearch = elements.providerSearch || null;
    const providerKeyPopover = elements.providerKeyPopover || null;
    const providerKeyInlineInput = elements.providerKeyInlineInput || null;
    const ollamaSetupPopover = elements.ollamaSetupPopover || null;
    const providerKeyForm = elements.providerKeyForm || null;
    const ollamaForm = elements.ollamaForm || null;
    const reloadButton = elements.reloadButton || null;
    const attachImageButton = elements.attachImageButton || null;
    const imageInput = elements.imageInput || null;
    const usageContainer = elements.usageContainer || null;
    const usagePopover = elements.usagePopover || null;
    const sessionCostStatus = elements.sessionCostStatus || null;
    const setMcpTargetButton = elements.setMcpTargetButton || null;
    const targetPillText = elements.targetPillText || null;
    const showReloadButton = constants.showReloadButton ?? true;
    const openModelPicker = callbacks.openModelPicker || function () {};
    const renderProviderOptions = callbacks.renderProviderOptions || function () {};
    const closeProviderPicker = callbacks.closeProviderPicker || function () {};
    const closeProviderKeyPrompt = callbacks.closeProviderKeyPrompt || function () {};
    const closeLocalSetupPrompt = callbacks.closeLocalSetupPrompt || function () {};
    const focusComposer = callbacks.focusComposer || function () {};
    const saveProviderKey = callbacks.saveProviderKey || function () {};
    const saveLocalProvider = callbacks.saveLocalProvider || function () {};
    const flashCopied = callbacks.flashCopied || function () {};
    const toggleDrawer = callbacks.toggleDrawer || function () {};
    const startNewChat = callbacks.startNewChat || function () {};
    const addImageFiles = callbacks.addImageFiles || (() => Promise.resolve());
    const showUsagePopover = callbacks.showUsagePopover || function () {};
    const hideUsagePopoverSoon = callbacks.hideUsagePopoverSoon || function () {};
    const send = callbacks.send || (() => false);
    const nextRequestId = callbacks.nextRequestId || (() => "request");

    document.querySelectorAll("[data-open-model-picker]").forEach((button) => {
      button.addEventListener("click", openModelPicker);
    });

    providerSearch?.addEventListener("input", renderProviderOptions);
    providerSearch?.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        closeProviderPicker();
        focusComposer();
      }
    });

    providerKeyPopover?.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        closeProviderKeyPrompt();
        focusComposer();
      }
    });

    ollamaSetupPopover?.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopPropagation();
        closeLocalSetupPrompt();
        focusComposer();
      }
    });

    providerKeyForm?.addEventListener("submit", (event) => {
      event.preventDefault();
      const key = providerKeyInlineInput?.value.trim() || "";
      if (!key) {
        providerKeyInlineInput?.focus();
        return;
      }
      saveProviderKey(key);
    });

    ollamaForm?.addEventListener("submit", (event) => {
      event.preventDefault();
      saveLocalProvider();
    });

    if (reloadButton) {
      reloadButton.hidden = !showReloadButton;
      if (showReloadButton) {
        reloadButton.addEventListener("click", reloadUi);
      }
    }

    document.querySelectorAll("[data-copy-code]").forEach((button) => {
      button.addEventListener("click", async () => {
        const code = button.closest(".code-block")?.querySelector("code")?.textContent ?? "";
        if (!code) {
          return;
        }
        await navigator.clipboard?.writeText(code);
        flashCopied(button, "Copy code", "Copied code");
      });
    });

    document.querySelectorAll("[data-toggle-drawer]").forEach((button) => {
      button.addEventListener("click", toggleDrawer);
    });

    document.querySelectorAll("[data-new-chat]").forEach((button) => {
      button.addEventListener("click", startNewChat);
    });

    attachImageButton?.addEventListener("click", () => {
      imageInput?.click();
    });

    imageInput?.addEventListener("change", () => {
      addImageFiles(imageInput.files).finally(() => {
        imageInput.value = "";
      });
    });

    usageContainer?.addEventListener("mouseenter", showUsagePopover);
    usageContainer?.addEventListener("mouseleave", hideUsagePopoverSoon);
    usagePopover?.addEventListener("mouseenter", showUsagePopover);
    usagePopover?.addEventListener("mouseleave", hideUsagePopoverSoon);
    sessionCostStatus?.addEventListener("focus", showUsagePopover);
    sessionCostStatus?.addEventListener("blur", hideUsagePopoverSoon);

    setMcpTargetButton?.addEventListener("click", () => {
      if (setMcpTargetButton.classList.contains("is-target")) {
        return;
      }
      setMcpTargetButton.classList.add("is-setting");
      if (targetPillText) {
        targetPillText.textContent = "Setting";
      }
      send({ type: "set_mcp_target", request_id: nextRequestId("set-target") });
    });
  }

  function reloadUi() {
    const nextUrl = new URL(window.location.href);
    nextUrl.searchParams.set("v", String(Date.now()));
    window.location.replace(nextUrl.toString());
  }

  window.FennaraShellBindings = {
    createShellBindings,
  };
})();
