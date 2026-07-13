(function () {
  const REQUEST_TIMEOUT_MS = 65_000;

  function createMcpAppsSettings(options = {}) {
    const callbacks = options.callbacks || {};
    const dialog = document.querySelector("[data-settings]");
    const tabs = Array.from(document.querySelectorAll("[data-settings-tab]"));
    const pages = Array.from(document.querySelectorAll("[data-settings-page]"));
    const result = document.querySelector("[data-mcp-app-result]");
    const appSearch = document.querySelector("[data-mcp-app-search]");
    const appCards = Array.from(document.querySelectorAll("[data-mcp-app]"));
    const emptySearch = document.querySelector("[data-mcp-app-empty]");
    const settingsActions = document.querySelector("[data-settings-actions]");
    const previewMode = Boolean(options.previewMode);
    const pendingRequests = new Map();

    function selectPage(pageName) {
      tabs.forEach((tab) => {
        const selected = tab.dataset.settingsTab === pageName;
        tab.classList.toggle("is-active", selected);
        tab.setAttribute("aria-selected", String(selected));
      });
      pages.forEach((page) => {
        page.hidden = page.dataset.settingsPage !== pageName;
      });
      if (settingsActions) {
        settingsActions.hidden = pageName === "mcp";
      }
    }

    function renderResult(title, detail, failed = false) {
      if (!result) {
        return;
      }
      const heading = document.createElement("strong");
      const message = document.createElement("span");
      heading.textContent = title;
      message.textContent = detail;
      result.replaceChildren(heading, message);
      result.classList.toggle("is-error", failed);
      result.hidden = false;
    }

    function lockSetupActions(activeButton) {
      appCards.forEach((card) => {
        const button = card.querySelector("[data-mcp-app-action]");
        if (button) {
          button.disabled = true;
          button.setAttribute("aria-busy", String(button === activeButton));
        }
      });
    }

    function restoreSetupActions() {
      appCards.forEach((card) => {
        const button = card.querySelector("[data-mcp-app-action]");
        const status = card.querySelector("[data-mcp-app-status]");
        if (!button) {
          return;
        }
        const completed = status?.classList.contains("is-configured");
        const retryable = status?.classList.contains("is-warning") || status?.classList.contains("is-error");
        button.disabled = Boolean(completed && !retryable);
        button.removeAttribute("aria-busy");
      });
    }

    function setRunning(card, button) {
      const status = card.querySelector("[data-mcp-app-status]");
      lockSetupActions(button);
      button.textContent = "Setting up...";
      status.textContent = "Running Fennara CLI";
      status.classList.remove("is-configured", "is-error", "is-warning");
      result.hidden = true;
    }

    function setCompleted(card, warning = "") {
      const appName = card.querySelector("strong")?.textContent || "App";
      const restartName = card.dataset.mcpRestart || appName;
      const status = card.querySelector("[data-mcp-app-status]");
      const button = card.querySelector("[data-mcp-app-action]");
      status.textContent = warning ? "Configured with warning" : "Configured, restart required";
      status.classList.add("is-configured");
      status.classList.toggle("is-warning", Boolean(warning));
      status.classList.remove("is-error");
      button.textContent = warning ? "Retry" : "Configured";
      button.disabled = !warning;
      card.classList.add("is-configured");
      if (warning) {
        renderResult(`${appName} setup needs attention.`, warning, true);
      } else {
        renderResult(`${appName} is connected.`, `Restart ${restartName} so it can load Fennara.`);
      }
      restoreSetupActions();
    }

    function setFailed(card, error) {
      const appName = card.querySelector("strong")?.textContent || "App";
      const status = card.querySelector("[data-mcp-app-status]");
      const button = card.querySelector("[data-mcp-app-action]");
      status.textContent = "Setup failed";
      status.classList.add("is-error");
      status.classList.remove("is-configured", "is-warning");
      button.textContent = "Retry";
      renderResult(`${appName} could not be configured.`, error || "Try again or run Fennara MCP setup from the CLI.", true);
      restoreSetupActions();
    }

    function completeRequest(requestId, message) {
      const pending = pendingRequests.get(requestId);
      if (!pending) {
        return false;
      }
      window.clearTimeout(pending.timeoutId);
      pendingRequests.delete(requestId);
      if (message.type === "mcp_setup_completed") {
        setCompleted(pending.card, message.warning || "");
      } else {
        setFailed(pending.card, message.message);
      }
      return true;
    }

    tabs.forEach((tab) => {
      tab.addEventListener("click", () => selectPage(tab.dataset.settingsTab));
    });

    appSearch?.addEventListener("input", () => {
      const query = appSearch.value.trim().toLowerCase();
      let visibleCount = 0;
      appCards.forEach((card) => {
        const searchText = `${card.dataset.mcpSearch || ""} ${card.textContent || ""}`.toLowerCase();
        const visible = !query || searchText.includes(query);
        card.hidden = !visible;
        if (visible) {
          visibleCount += 1;
        }
      });
      if (emptySearch) {
        emptySearch.hidden = visibleCount > 0;
      }
    });

    document.querySelectorAll("[data-mcp-app-action]").forEach((button) => {
      button.addEventListener("click", () => {
        const card = button.closest("[data-mcp-app]");
        if (!card) {
          return;
        }
        setRunning(card, button);

        if (previewMode) {
          window.setTimeout(() => setCompleted(card), 850);
          return;
        }
        if (!callbacks.ensureDaemonConnected?.()) {
          setFailed(card, "The local Fennara daemon is not connected yet.");
          return;
        }

        const requestId = callbacks.nextRequestId?.("mcp-setup");
        if (!requestId || !callbacks.send?.({
          type: "setup_mcp_app",
          request_id: requestId,
          mcp_target: card.dataset.mcpApp,
        })) {
          setFailed(card, "The setup request could not be sent to the local daemon.");
          return;
        }
        const timeoutId = window.setTimeout(() => {
          if (pendingRequests.delete(requestId)) {
            setFailed(card, "Fennara MCP setup timed out. Try again.");
          }
        }, REQUEST_TIMEOUT_MS);
        pendingRequests.set(requestId, { card, timeoutId });
      });
    });

    if (previewMode) {
      selectPage("mcp");
      window.setTimeout(() => dialog?.showModal(), 0);
    }

    return {
      handleDisconnect() {
        pendingRequests.forEach((pending) => {
          window.clearTimeout(pending.timeoutId);
          setFailed(pending.card, "The local Fennara daemon disconnected during setup.");
        });
        pendingRequests.clear();
      },
      handleMessage(message) {
        const requestId = String(message?.request_id || "");
        if (!pendingRequests.has(requestId)) {
          return false;
        }
        if (message.type !== "mcp_setup_completed" && message.type !== "error") {
          return false;
        }
        return completeRequest(requestId, message);
      },
    };
  }

  window.FennaraMcpAppsSettings = {
    createMcpAppsSettings,
  };
})();
