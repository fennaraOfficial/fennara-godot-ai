(function () {
  function createOverlayManager(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const commandPopover = elements.commandPopover || null;
    const providerPopover = elements.providerPopover || null;
    const providerKeyPopover = elements.providerKeyPopover || null;
    const ollamaSetupPopover = elements.ollamaSetupPopover || null;
    const customProviderPopover = elements.customProviderPopover || null;
    const modelPopover = elements.modelPopover || null;
    const effortOptions = elements.effortOptions || null;
    const usagePopover = elements.usagePopover || null;
    const prompt = elements.prompt || null;
    const closeDrawerFromOutsideClick = callbacks.closeDrawerFromOutsideClick || function () {};
    const closeDrawer = callbacks.closeDrawer || function () {};
    const closeProviderPicker = callbacks.closeProviderPicker || function () {};
    const closeProviderKeyPrompt = callbacks.closeProviderKeyPrompt || function () {};
    const closeLocalSetupPrompt = callbacks.closeLocalSetupPrompt || function () {};
    const closeCustomProviderPrompt = callbacks.closeCustomProviderPrompt || function () {};
    const setEffortMenuOpen = callbacks.setEffortMenuOpen || function () {};
    const setUsagePopoverOpen = callbacks.setUsagePopoverOpen || function () {};
    const focusComposer = callbacks.focusComposer || function () {};
    const positionUsagePopover = callbacks.positionUsagePopover || function () {};
    const positionProviderPopover = callbacks.positionProviderPopover || function () {};
    const positionProviderKeyPrompt = callbacks.positionProviderKeyPrompt || function () {};
    const positionLocalSetupPrompt = callbacks.positionLocalSetupPrompt || function () {};
    const commandPalette = callbacks.commandPalette || {};
    const modelPicker = callbacks.modelPicker || {};

    document.addEventListener("click", (event) => {
      closeDrawerFromOutsideClick(event);
      if (
        commandPopover &&
        commandPopover.hidden === false &&
        !commandPopover.contains(event.target) &&
        !prompt?.contains(event.target)
      ) {
        commandPalette.close?.();
      }
      if (
        providerPopover &&
        providerPopover.hidden === false &&
        !providerPopover.contains(event.target)
      ) {
        closeProviderPicker();
      }
      if (
        providerKeyPopover &&
        providerKeyPopover.hidden === false &&
        !providerKeyPopover.contains(event.target)
      ) {
        closeProviderKeyPrompt();
      }
      if (
        ollamaSetupPopover &&
        ollamaSetupPopover.hidden === false &&
        !ollamaSetupPopover.contains(event.target)
      ) {
        closeLocalSetupPrompt();
      }
      if (
        customProviderPopover &&
        customProviderPopover.hidden === false &&
        !customProviderPopover.contains(event.target)
      ) {
        closeCustomProviderPrompt();
      }
      setEffortMenuOpen(false);
      setUsagePopoverOpen(false);
    });

    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        const hadOverlayOpen =
          providerPopover?.hidden === false ||
          providerKeyPopover?.hidden === false ||
          ollamaSetupPopover?.hidden === false ||
          customProviderPopover?.hidden === false ||
          modelPopover?.hidden === false ||
          commandPopover?.hidden === false ||
          effortOptions?.hidden === false ||
          usagePopover?.hidden === false;
        setEffortMenuOpen(false);
        setUsagePopoverOpen(false);
        modelPicker.close?.();
        closeProviderPicker();
        closeProviderKeyPrompt();
        closeLocalSetupPrompt();
        closeCustomProviderPrompt();
        commandPalette.close?.();
        closeDrawer();
        if (hadOverlayOpen) {
          event.preventDefault();
          event.stopPropagation();
          focusComposer();
        }
      }
    }, true);

    function positionOverlays() {
      positionUsagePopover();
      positionProviderPopover();
      positionProviderKeyPrompt();
      positionLocalSetupPrompt();
      callbacks.positionCustomProviderPrompt?.();
      commandPalette.position?.();
    }

    window.addEventListener("resize", positionOverlays);
    window.addEventListener("scroll", positionOverlays, true);

    return {
      positionOverlays,
    };
  }

  window.FennaraOverlayManager = {
    createOverlayManager,
  };
})();
