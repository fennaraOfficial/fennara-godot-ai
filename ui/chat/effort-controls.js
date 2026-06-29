(function () {
  function createEffortControls(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const reasoningEffortControls = Array.from(elements.reasoningEffortControls || []);
    const effortStatus = elements.effortStatus || null;
    const effortToggle = elements.effortToggle || null;
    const effortOptions = elements.effortOptions || null;
    const effortOptionButtons = Array.from(elements.effortOptionButtons || []);
    const getCurrentReasoningEffort = callbacks.getCurrentReasoningEffort || (() => "medium");
    const setCurrentReasoningEffort = callbacks.setCurrentReasoningEffort || function () {};
    const cleanReasoningEffort = callbacks.cleanReasoningEffort || ((effort) => effort || "medium");
    const saveCurrentChatSettings = callbacks.saveCurrentChatSettings || function () {};

    reasoningEffortControls.forEach((control) => {
      control.addEventListener("change", () => {
        const effort = cleanReasoningEffort(control.value);
        setCurrentReasoningEffort(effort);
        syncReasoningControls(effort);
        updateComposerEffort();
        saveCurrentChatSettings();
      });
    });

    effortToggle?.addEventListener("click", (event) => {
      event.stopPropagation();
      setEffortMenuOpen(effortOptions?.hidden !== false);
    });

    effortOptionButtons.forEach((button) => {
      button.addEventListener("click", (event) => {
        event.stopPropagation();
        const effort = cleanReasoningEffort(button.value);
        setCurrentReasoningEffort(effort);
        syncReasoningControls(effort);
        updateComposerEffort();
        setEffortMenuOpen(false);
        saveCurrentChatSettings();
      });
    });

    function syncReasoningControls(effort = getCurrentReasoningEffort()) {
      reasoningEffortControls.forEach((control) => {
        control.value = effort;
      });
    }

    function effortLabel(effort) {
      return effort.charAt(0).toUpperCase() + effort.slice(1);
    }

    function updateComposerEffort() {
      const effort = getCurrentReasoningEffort();
      if (effortStatus) {
        effortStatus.textContent = effortLabel(effort);
      }
      effortOptionButtons.forEach((button) => {
        const selected = button.value === effort;
        button.setAttribute("aria-selected", String(selected));
      });
    }

    function setEffortMenuOpen(open) {
      if (!effortOptions || !effortToggle) {
        return;
      }
      effortOptions.hidden = !open;
      effortToggle.setAttribute("aria-expanded", String(open));
    }

    return {
      setEffortMenuOpen,
      syncReasoningControls,
      updateComposerEffort,
    };
  }

  window.FennaraEffortControls = {
    createEffortControls,
  };
})();
