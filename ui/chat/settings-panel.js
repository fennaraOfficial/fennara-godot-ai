(function () {
  function createSettingsPanel(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const constants = options.constants || {};
    const settingsDialog = elements.settingsDialog || null;
    const chatSurfaceBrowserInput = elements.chatSurfaceBrowserInput || null;
    const chatSurfaceRestartStatus = elements.chatSurfaceRestartStatus || null;
    const approvalModeControls = Array.from(elements.approvalModeControls || []);
    const settingsSavedToast = elements.settingsSavedToast || null;
    const saveSettingsButton = elements.saveSettingsButton || null;
    const openProvidersButton = elements.openProvidersButton || null;
    const savedNoticeMs = constants.savedNoticeMs || 1800;
    const saveTimeoutMs = constants.saveTimeoutMs || 8000;
    const chatSurfaceBrowser = constants.chatSurfaceBrowser || "browser";
    const chatSurfaceEmbedded = constants.chatSurfaceEmbedded || "embedded";
    const approvalModeAsk = constants.approvalModeAsk || "ask";
    const approvalModeFullAccess = constants.approvalModeFullAccess || "full_access";
    const runtimeChatSurface = constants.runtimeChatSurface || chatSurfaceEmbedded;
    const ensureDaemonConnected = callbacks.ensureDaemonConnected || (() => true);
    const setUsagePopoverOpen = callbacks.setUsagePopoverOpen || function () {};
    const closeProviderPicker = callbacks.closeProviderPicker || function () {};
    const closeProviderKeyPrompt = callbacks.closeProviderKeyPrompt || function () {};
    const closeLocalSetupPrompt = callbacks.closeLocalSetupPrompt || function () {};
    const closeCommandPalette = callbacks.closeCommandPalette || function () {};
    const cleanChatSurface = callbacks.cleanChatSurface || ((surface) => surface === chatSurfaceBrowser ? chatSurfaceBrowser : chatSurfaceEmbedded);
    const cleanApprovalMode = callbacks.cleanApprovalMode || ((mode) => mode === approvalModeFullAccess ? approvalModeFullAccess : approvalModeAsk);
    const getCurrentChatSurface = callbacks.getCurrentChatSurface || (() => chatSurfaceEmbedded);
    const getCurrentApprovalMode = callbacks.getCurrentApprovalMode || (() => approvalModeAsk);
    const openProviderPicker = callbacks.openProviderPicker || function () {};
    const buildSavePayload = callbacks.buildSavePayload || (() => null);
    const sendIfOpen = callbacks.sendIfOpen || (() => false);
    const connect = callbacks.connect || function () {};
    const appendSystem = callbacks.appendSystem || function () {};
    const clearSystemStatus = callbacks.clearSystemStatus || function () {};
    let pendingSettingsPayload = null;
    let settingsDirty = false;
    let settingsSaving = false;
    let settingsSavedTimer = 0;
    let settingsSaveTimeout = 0;

    document.querySelectorAll("[data-open-settings]").forEach((button) => {
      button.addEventListener("click", openSettings);
    });

    saveSettingsButton?.addEventListener("click", (event) => {
      event.preventDefault();
      if (settingsSaving || !settingsDirty) {
        updateSaveButton();
        return;
      }
      const payload = buildSavePayload({
        chatSurface: selectedChatSurface(),
        approvalMode: selectedApprovalMode(),
      });
      if (payload) {
        queueSave(payload);
      }
    });

    chatSurfaceBrowserInput?.addEventListener("change", () => {
      updateChatSurfaceRestartNotice();
      setDirty(true);
    });

    approvalModeControls.forEach((control) => {
      control.addEventListener("change", () => {
        setDirty(true);
      });
    });

    openProvidersButton?.addEventListener("click", (event) => {
      event.preventDefault();
      event.stopPropagation();
      settingsDialog?.close();
      openProviderPicker();
    });

    function openSettings() {
      if (!ensureDaemonConnected()) {
        return;
      }
      setUsagePopoverOpen(false);
      closeProviderPicker();
      closeProviderKeyPrompt();
      closeLocalSetupPrompt();
      closeCommandPalette();
      if (chatSurfaceBrowserInput) {
        chatSurfaceBrowserInput.checked = getCurrentChatSurface() === chatSurfaceBrowser;
      }
      syncApprovalModeControls();
      updateChatSurfaceRestartNotice(getCurrentChatSurface());
      markClean();
      if (settingsDialog && typeof settingsDialog.showModal === "function") {
        settingsDialog.showModal();
      }
    }

    function selectedChatSurface() {
      return chatSurfaceBrowserInput?.checked ? chatSurfaceBrowser : chatSurfaceEmbedded;
    }

    function selectedApprovalMode() {
      const selected = approvalModeControls.find((control) => control.checked);
      return cleanApprovalMode(selected?.value || getCurrentApprovalMode());
    }

    function syncApprovalModeControls() {
      const currentApprovalMode = getCurrentApprovalMode();
      approvalModeControls.forEach((control) => {
        control.checked = cleanApprovalMode(control.value) === currentApprovalMode;
      });
    }

    function updateChatSurfaceRestartNotice(surface = selectedChatSurface()) {
      if (chatSurfaceRestartStatus) {
        chatSurfaceRestartStatus.hidden = cleanChatSurface(surface) === runtimeChatSurface;
      }
    }

    function clearSavedNotice() {
      window.clearTimeout(settingsSavedTimer);
      settingsSavedTimer = 0;
      if (settingsSavedToast) {
        settingsSavedToast.hidden = true;
      }
    }

    function clearSaveTimeout() {
      window.clearTimeout(settingsSaveTimeout);
      settingsSaveTimeout = 0;
    }

    function updateSaveButton() {
      if (!saveSettingsButton) {
        return;
      }
      if (settingsSaving) {
        saveSettingsButton.disabled = true;
        saveSettingsButton.textContent = "Saving...";
        return;
      }
      saveSettingsButton.disabled = !settingsDirty;
      saveSettingsButton.textContent = "Save";
    }

    function setDirty(dirty = true) {
      settingsDirty = Boolean(dirty);
      if (settingsDirty) {
        clearSavedNotice();
      }
      updateSaveButton();
    }

    function markClean(options = {}) {
      const showSaved = Boolean(options.showSaved);
      settingsDirty = false;
      settingsSaving = false;
      clearSaveTimeout();
      clearSavedNotice();
      if (showSaved) {
        if (settingsSavedToast) {
          settingsSavedToast.hidden = false;
        }
        settingsSavedTimer = window.setTimeout(() => {
          if (settingsSavedToast) {
            settingsSavedToast.hidden = true;
          }
          updateSaveButton();
        }, savedNoticeMs);
      }
      updateSaveButton();
    }

    function setSaving(saving) {
      settingsSaving = Boolean(saving);
      if (settingsSaving) {
        clearSavedNotice();
      }
      updateSaveButton();
    }

    function startSaveTimeout(requestId) {
      clearSaveTimeout();
      settingsSaveTimeout = window.setTimeout(() => {
        if (!pendingSettingsPayload || pendingSettingsPayload.request_id !== requestId) {
          return;
        }
        pendingSettingsPayload = null;
        settingsSaving = false;
        appendSystem("Settings save timed out. Try again.");
        window.setTimeout(clearSystemStatus, 2400);
        updateSaveButton();
      }, saveTimeoutMs);
    }

    function flushPending() {
      if (!pendingSettingsPayload) {
        return false;
      }
      return sendIfOpen(pendingSettingsPayload);
    }

    function queueSave(payload) {
      pendingSettingsPayload = payload;
      setSaving(true);
      startSaveTimeout(payload.request_id);
      if (flushPending()) {
        return true;
      }
      appendSystem("Connecting to local daemon...");
      connect();
      return true;
    }

    function hasPendingRequest(requestId) {
      return Boolean(pendingSettingsPayload && pendingSettingsPayload.request_id === requestId);
    }

    function clearPending() {
      pendingSettingsPayload = null;
    }

    return {
      clearPending,
      clearSaveTimeout,
      flushPending,
      hasPendingRequest,
      markClean,
      queueSave,
      selectedApprovalMode,
      selectedChatSurface,
      setDirty,
      setSaving,
      syncApprovalModeControls,
      updateChatSurfaceRestartNotice,
      updateSaveButton,
    };
  }

  window.FennaraSettingsPanel = {
    createSettingsPanel,
  };
})();
