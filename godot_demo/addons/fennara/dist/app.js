(function () {
  const DAEMON_WS_URL = "ws://127.0.0.1:41287/chat/ws";
  const PROMPT_MAX_HEIGHT = 126;
  const USER_COLLAPSE_CHARS = 700;
  const AUTO_SCROLL_THRESHOLD = 72;
  const DAEMON_RECONNECT_DELAY_MS = 250;
  const SHOW_RELOAD_BUTTON = true;
  const SETTINGS_SAVED_NOTICE_MS = 1800;
  const SETTINGS_SAVE_TIMEOUT_MS = 8000;
  const DEFAULT_OLLAMA_BASE_URL = "http://127.0.0.1:11434";
  const DEFAULT_LOCAL_BASE_URLS = {
    ollama: DEFAULT_OLLAMA_BASE_URL,
    lmstudio: "http://127.0.0.1:1234/v1",
  };
  const CHAT_SURFACE_EMBEDDED = "embedded";
  const CHAT_SURFACE_BROWSER = "browser";
  const APPROVAL_MODE_ASK = "ask";
  const APPROVAL_MODE_FULL_ACCESS = "full_access";
  const RUNTIME_CHAT_SURFACE = /^https?:$/.test(window.location.protocol)
    ? CHAT_SURFACE_BROWSER
    : CHAT_SURFACE_EMBEDDED;
  const SUPPORTED_IMAGE_TYPES = window.FennaraAttachmentManager.SUPPORTED_IMAGE_TYPES;
  const COPY_ICON = '<svg class="svg-icon" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 11c0-2.83 0-4.24.88-5.12C7.76 5 9.17 5 12 5h3c2.83 0 4.24 0 5.12.88C21 6.76 21 8.17 21 11v5c0 2.83 0 4.24-.88 5.12C19.24 22 17.83 22 15 22h-3c-2.83 0-4.24 0-5.12-.88C6 20.24 6 18.83 6 16v-5Z"></path><path d="M6 19a3 3 0 0 1-3-3v-6c0-3.77 0-5.66 1.17-6.83C5.34 2 7.23 2 11 2h4a3 3 0 0 1 3 3"></path></svg>';
  const CHECK_ICON = '<svg class="svg-icon" viewBox="0 0 24 24" aria-hidden="true"><path d="m20 6-11 11-5-5"></path></svg>';
  const settingsDialog = document.querySelector("[data-settings]");
  const commandPopover = document.querySelector("[data-command-popover]");
  const commandOptionButtons = Array.from(document.querySelectorAll("[data-command-option]"));
  const modelPopover = document.querySelector("[data-model-popover]");
  const providerPopover = document.querySelector("[data-provider-popover]");
  const providerKeyPopover = document.querySelector("[data-provider-key-popover]");
  const ollamaSetupPopover = document.querySelector("[data-ollama-setup-popover]");
  const modelTrigger = document.querySelector("[data-open-model-picker]");
  const modelSearch = document.querySelector("[data-model-search]");
  const modelList = document.querySelector("[data-model-list]");
  const modelDetail = document.querySelector("[data-model-detail]");
  const providerStatuses = document.querySelectorAll("[data-provider-status]");
  const providerDot = document.querySelector(".composer-model-dot");
  const providerOptionsList = document.querySelector("[data-provider-options]");
  const providerSearch = document.querySelector("[data-provider-search]");
  const ollamaForm = document.querySelector("[data-ollama-form]");
  const ollamaBaseUrlInput = document.querySelector("[data-ollama-base-url]");
  const localSetupTitle = document.querySelector("[data-local-setup-title]");
  const localSetupHelp = document.querySelector("[data-local-setup-help]");
  const providerKeyForm = document.querySelector("[data-provider-key-form]");
  const providerKeyTitle = document.querySelector("[data-provider-key-title]");
  const providerKeyInlineInput = document.querySelector("[data-provider-key-inline]");
  const transcript = document.querySelector("[data-transcript]");
  const chatList = document.querySelector("[data-chat-list]");
  const chatTitle = document.querySelector("[data-chat-title]");
  const composer = document.querySelector("[data-composer]");
  const prompt = document.querySelector("[data-prompt]");
  const attachImageButton = document.querySelector("[data-attach-image]");
  const imageInput = document.querySelector("[data-image-input]");
  const attachmentPreview = document.querySelector("[data-attachment-preview]");
  const chatSurfaceBrowserInput = document.querySelector("[data-chat-surface-browser]");
  const chatSurfaceRestartStatus = document.querySelector("[data-chat-surface-restart]");
  const approvalModeControls = document.querySelectorAll("[data-approval-mode]");
  const settingsSavedToast = document.querySelector("[data-settings-saved-toast]");
  const openSettingsProvidersButton = document.querySelector("[data-open-settings-providers]");
  const modelInput = document.querySelector("[data-model]");
  const modelStatuses = document.querySelectorAll("[data-model-status]");
  const chatSizeStatus = document.querySelector("[data-chat-size]");
  const sessionCostStatus = document.querySelector("[data-session-cost]");
  const setMcpTargetButton = document.querySelector("[data-set-mcp-target]");
  const targetPillText = document.querySelector("[data-target-pill-text]");
  const targetMenu = document.querySelector("[data-target-menu]");
  const targetPopoverTitle = document.querySelector("[data-target-popover-title]");
  const targetPopoverText = document.querySelector("[data-target-popover-text]");
  const versionMenu = document.querySelector("[data-version-menu]");
  const versionWarning = document.querySelector("[data-version-warning]");
  const versionPopover = document.querySelector("[data-version-popover]");
  const versionWarningText = document.querySelector("[data-version-warning-text]");
  const versionCommand = document.querySelector("[data-version-command]");
  const usageContainer = document.querySelector(".composer-usage");
  const usagePopover = document.querySelector("[data-usage-popover]");
  const usageTotalCost = document.querySelector("[data-usage-total-cost]");
  const usageContextStatus = document.querySelector("[data-usage-context]");
  const reasoningEffortControls = document.querySelectorAll("[data-reasoning-effort]");
  const effortStatus = document.querySelector("[data-effort-status]");
  const effortToggle = document.querySelector("[data-effort-toggle]");
  const effortOptions = document.querySelector("[data-effort-options]");
  const effortOptionButtons = document.querySelectorAll("[data-effort-option]");
  const sendButton = document.querySelector("[data-send-button]");
  const turnRecoveryBar = document.querySelector("[data-turn-recovery]");
  const turnRecoveryNotice = document.querySelector("[data-turn-recovery-notice]");
  const turnRecoveryNoticeText = document.querySelector("[data-turn-recovery-notice-text]");
  const turnRecoverySkipped = document.querySelector("[data-turn-recovery-skipped]");
  const turnRecoverySkippedList = document.querySelector("[data-turn-recovery-skipped-list]");
  const turnUndoButton = document.querySelector("[data-turn-undo]");
  const turnRedoButton = document.querySelector("[data-turn-redo]");
  const turnRetryButton = document.querySelector("[data-turn-retry]");
  const turnEditRetryButton = document.querySelector("[data-turn-edit-retry]");
  const turnResumeButton = document.querySelector("[data-turn-resume]");
  const turnRecoveryConfirm = document.querySelector("[data-turn-recovery-confirm]");
  const turnRecoveryConfirmText = document.querySelector("[data-turn-recovery-confirm-text]");
  const turnRecoveryConflicts = document.querySelector("[data-turn-recovery-conflicts]");
  const saveSettingsButton = document.querySelector("[data-save-settings]");
  const reloadButton = document.querySelector("[data-reload-ui]");
  const appShell = document.querySelector(".app-shell");
  const markdown = window.markdownit({
    html: false,
    linkify: true,
    typographer: true,
    breaks: false,
  });

  if (window.markdownitTaskLists) {
    markdown.use(window.markdownitTaskLists, { enabled: false, label: false });
  }

  markdown.renderer.rules.fence = function (tokens, index, options, env, self) {
    const token = tokens[index];
    const language = token.info.trim().split(/\s+/)[0] || "text";
    const code = token.content;
    const escapedLanguage = markdown.utils.escapeHtml(language);
    const escapedCode = markdown.utils.escapeHtml(code);
    return [
      '<figure class="code-block">',
      "<figcaption>",
      `<span>${escapedLanguage}</span>`,
      '<button class="copy-code-button" type="button" aria-label="Copy code" data-code-copy>',
      COPY_ICON,
      "</button>",
      "</figcaption>",
      `<pre><code>${escapedCode}</code></pre>`,
      "</figure>",
    ].join("");
  };

  const projectStatusController = window.FennaraProjectStatus.createProjectStatus({
    targetMenu,
    setMcpTargetButton,
    targetPillText,
    targetPopoverTitle,
    targetPopoverText,
    versionMenu,
    versionWarning,
    versionPopover,
    versionWarningText,
    versionCommand,
    escapeHtml: markdown.utils.escapeHtml,
  });
  const applyProjectStatus = projectStatusController.applyProjectStatus;

  let activeChatId = null;
  let currentModel = "";
  let currentProvider = "";
  let currentReasoningEffort = "medium";
  let currentChatSurface = CHAT_SURFACE_EMBEDDED;
  let currentApprovalMode = APPROVAL_MODE_ASK;
  let hasOpenRouterKey = false;
  let hasOllamaCloudKey = false;
  let providerRegistry = [];
  let providerMetadata = new Map();
  let keyPromptProvider = "";
  let defaultModel = "";
  let ollamaBaseUrl = DEFAULT_OLLAMA_BASE_URL;
  let providerBaseUrls = new Map(Object.entries(DEFAULT_LOCAL_BASE_URLS));
  let localModelContextLengths = new Map();
  let ollamaModels = [];
  let ollamaStatus = "unknown";
  let localProviderStatuses = new Map();
  let lastModelCatalog = null;
  let openrouterCatalogStatus = null;
  let catalogRefreshInFlight = false;
  let pendingProviderKeySaves = new Map();
  let chatStreaming = false;
  let recoveryDraftRestoring = false;
  let sessionCost = 0;
  let activeTurnCost = 0;
  let latestPromptTokens = 0;
  let projectStatusTimer = 0;
  const checkpointWarnings = new Map();
  let modelPicker = null;
  let providerPopovers = null;
  let projectFileLinks = null;
  let settingsPanel = null;
  let storedTranscript = null;
  let composerActions = null;
  let turnRecoveryControls = null;
  let effortControls = null;
  const transcriptRenderer = window.FennaraTranscriptRenderer.createTranscriptRenderer({
    transcript,
    markdown,
    copyIcon: COPY_ICON,
    checkIcon: CHECK_ICON,
    toolMediaOrigin: window.FennaraDaemonClient.daemonHttpOrigin(DAEMON_WS_URL),
    userCollapseChars: USER_COLLAPSE_CHARS,
    autoScrollThreshold: AUTO_SCROLL_THRESHOLD,
    onProjectFileReference: handleProjectFileReference,
    onToolApprovalReview: requestToolApprovalReview,
  });

  const attachmentManager = window.FennaraAttachmentManager.createAttachmentManager({
    attachmentPreview,
    imageInput,
    transcriptRenderer,
    appendSystem,
    focusComposer,
    onNativePasteboardSettled: () => window.setTimeout(resizePrompt, 0),
  });
  const addContextSnippet = attachmentManager.addContextSnippet;
  const addImageFiles = attachmentManager.addImageFiles;
  const attachmentPayload = attachmentManager.attachmentPayload;
  const clearAttachments = attachmentManager.clearAttachments;
  const contextSnippetPayload = attachmentManager.contextSnippetPayload;
  const getAttachedContextSnippets = attachmentManager.getContextSnippets;
  const normalizeContextSnippet = attachmentManager.normalizeContextSnippet;
  const requestNativePastedImage = attachmentManager.requestNativePastedImage;
  const restoreAttachments = attachmentManager.restoreAttachments;

  modelPicker = window.FennaraModelPicker?.createModelPicker({
    popover: modelPopover,
    trigger: modelTrigger,
    search: modelSearch,
    list: modelList,
    detail: modelDetail,
    getCurrentModel: () => currentModel,
    getCurrentProvider: () => currentProvider,
    getProviders: () => providerRegistry,
    isProviderConnected: providerConnected,
    providerFromModel,
    hasOpenRouterKey: () => hasOpenRouterKey,
    hasOllamaCloudKey: () => hasOllamaCloudKey,
    getOllamaModels: () => ollamaModels,
    openProviderPicker,
    onSelect: selectModel,
    onEscapeClose: focusComposer,
    onRequestModels: () => requestModelList({ refreshOllama: true }),
    onRefreshCatalog: () => refreshModelCatalog(true),
  });

  const daemonClient = window.FennaraDaemonClient.createDaemonClient({
    defaultWsUrl: DAEMON_WS_URL,
    reconnectDelayMs: DAEMON_RECONNECT_DELAY_MS,
    onOpen() {
      appShell?.setAttribute("data-connection", "online");
      send({ type: "get_settings", request_id: nextRequestId("settings") });
      requestProjectStatus();
      startProjectStatusPolling();
      requestModelList();
      flushPendingSettings();
    },
    onMessage(message) {
      handleDaemonMessage(message);
    },
    onClose() {
      appShell?.setAttribute("data-connection", "offline");
      stopProjectStatusPolling();
      turnRecoveryControls?.handleDisconnect();
    },
    onSendUnavailable() {
      appendSystem("Local daemon is not connected yet.");
    },
    onEnsureUnavailable() {
      appendSystem("Connecting to local daemon...");
    },
  });
  const chatWsUrl = daemonClient.chatWsUrl;
  const connect = daemonClient.connect;
  const ensureDaemonConnected = daemonClient.ensureConnected;
  const nextRequestId = daemonClient.nextRequestId;
  const send = daemonClient.send;

  projectFileLinks = window.FennaraProjectFileLinks.createProjectFileLinks({
    send,
    nextRequestId,
    appendSystem,
    clearSystemStatus,
  });

  const usageSummary = window.FennaraUsageSummary.createUsageSummary({
    chatSizeStatus,
    sessionCostStatus,
    usagePopover,
    usageTotalCost,
    usageContextStatus,
    getSessionCost: () => sessionCost,
    getLatestPromptTokens: () => latestPromptTokens,
    getCurrentModelInfo: currentModelInfo,
  });
  const formatUsageCost = usageSummary.formatUsageCost;
  const hideUsagePopoverSoon = usageSummary.hideUsagePopoverSoon;
  const positionUsagePopover = usageSummary.positionUsagePopover;
  const setUsagePopoverOpen = usageSummary.setUsagePopoverOpen;
  const showUsagePopover = usageSummary.showUsagePopover;
  const updateChatSize = usageSummary.updateChatSize;
  const updateSessionCost = usageSummary.updateSessionCost;
  const usageCost = usageSummary.usageCost;
  const usagePromptTokens = usageSummary.usagePromptTokens;

  storedTranscript = window.FennaraStoredTranscript.createStoredTranscript({
    transcriptRenderer,
    supportedImageTypes: SUPPORTED_IMAGE_TYPES,
    callbacks: {
      clearTranscript,
      updateChatSize,
      setLatestPromptTokens: (tokens) => {
        latestPromptTokens = tokens;
      },
      usagePromptTokens,
      usageCost,
      formatUsageCost,
      normalizeContextSnippet,
    },
  });

  const commandPalette = window.FennaraCommandPalette.createCommandPalette({
    prompt,
    commandPopover,
    commandOptionButtons,
    ensureDaemonConnected,
    resizePrompt,
    openProviderPicker,
    openModelPicker,
  });

  providerPopovers = window.FennaraProviderPopovers.createProviderPopovers({
    elements: {
      modelPopover,
      providerPopover,
      providerKeyPopover,
      ollamaSetupPopover,
      providerOptionsList,
      providerSearch,
      providerKeyTitle,
      providerKeyInlineInput,
      ollamaBaseUrlInput,
      localSetupTitle,
      localSetupHelp,
    },
    callbacks: {
      ensureDaemonConnected,
      setUsagePopoverOpen,
      closeCommandPalette: () => commandPalette.close(),
      getModelPicker: () => modelPicker,
      getProviderRegistry: () => providerRegistry,
      getProviderMetadata: () => providerMetadata,
      getCurrentProvider: () => currentProvider,
      setKeyPromptProvider: (provider) => {
        keyPromptProvider = provider;
      },
      requestModelList,
      providerBaseUrl,
      providerStatusLabel,
      providerUsesBaseUrlSetup,
      chooseProvider,
    },
    settings: {
      defaultOllamaBaseUrl: DEFAULT_OLLAMA_BASE_URL,
      defaultLocalBaseUrls: DEFAULT_LOCAL_BASE_URLS,
    },
  });

  settingsPanel = window.FennaraSettingsPanel.createSettingsPanel({
    elements: {
      settingsDialog,
      chatSurfaceBrowserInput,
      chatSurfaceRestartStatus,
      approvalModeControls,
      settingsSavedToast,
      saveSettingsButton,
      openProvidersButton: openSettingsProvidersButton,
    },
    callbacks: {
      ensureDaemonConnected,
      setUsagePopoverOpen,
      closeProviderPicker,
      closeProviderKeyPrompt: closeOpenRouterKeyPrompt,
      closeLocalSetupPrompt: closeOllamaSetupPrompt,
      closeCommandPalette: () => commandPalette.close(),
      cleanChatSurface,
      cleanApprovalMode,
      getCurrentChatSurface: () => currentChatSurface,
      getCurrentApprovalMode: () => currentApprovalMode,
      openProviderPicker,
      sendIfOpen: daemonClient.sendIfOpen,
      connect,
      appendSystem,
      clearSystemStatus,
      buildSavePayload: ({ chatSurface, approvalMode }) => ({
        type: "save_settings",
        request_id: nextRequestId("save-settings"),
        model: cleanUiModelId(modelInput?.value || currentModel),
        reasoning_effort: currentReasoningEffort,
        ollama_base_url: ollamaBaseUrl,
        provider_base_urls: providerBaseUrlPayload(),
        local_model_context_lengths: localModelContextLengthPayload(),
        chat_surface: chatSurface,
        approval_mode: approvalMode,
      }),
    },
    constants: {
      savedNoticeMs: SETTINGS_SAVED_NOTICE_MS,
      saveTimeoutMs: SETTINGS_SAVE_TIMEOUT_MS,
      chatSurfaceBrowser: CHAT_SURFACE_BROWSER,
      chatSurfaceEmbedded: CHAT_SURFACE_EMBEDDED,
      approvalModeAsk: APPROVAL_MODE_ASK,
      approvalModeFullAccess: APPROVAL_MODE_FULL_ACCESS,
      runtimeChatSurface: RUNTIME_CHAT_SURFACE,
    },
  });

  composerActions = window.FennaraComposerActions.createComposerActions({
    elements: {
      composer,
      prompt,
      modelInput,
      commandPopover,
      sendButton,
    },
    callbacks: {
      isChatStreaming: () => chatStreaming,
      isSubmissionBlocked: () => recoveryDraftRestoring,
      getAttachedContextSnippets,
      getCurrentProvider: () => currentProvider,
      getCurrentModel: () => currentModel,
      getCurrentReasoningEffort: () => currentReasoningEffort,
      setCurrentReasoningEffort: (effort) => {
        currentReasoningEffort = effort;
      },
      cleanReasoningEffort,
      providerRequiresApiKey,
      providerConnected,
      openProviderPicker,
      openModelPicker,
      cleanModelId: cleanUiModelId,
      resetStreamState: () => transcriptRenderer.resetStreamState(),
      nextRequestId,
      getActiveChatId: () => activeChatId,
      send,
      attachmentPayload,
      contextSnippetPayload,
      setStreaming,
      setActiveTurnCost: (cost) => {
        activeTurnCost = cost;
      },
      appendMessage,
      beginStream: () => transcriptRenderer.beginStream(),
      trackOptimisticRequest,
      deleteOptimisticRequest,
      clearAttachments,
      resizePrompt,
      commandPalette,
      addImageFiles,
      requestNativePastedImage,
      chatWsUrl,
      appendSystem,
      onMessageSubmitted: () => turnRecoveryControls?.handleReplacementSubmitted(),
    },
  });

  turnRecoveryControls = window.FennaraTurnRecoveryControls.createTurnRecoveryControls({
    elements: {
      bar: turnRecoveryBar,
      notice: turnRecoveryNotice,
      noticeText: turnRecoveryNoticeText,
      skippedDetails: turnRecoverySkipped,
      skippedList: turnRecoverySkippedList,
      undoButton: turnUndoButton,
      redoButton: turnRedoButton,
      retryButton: turnRetryButton,
      editRetryButton: turnEditRetryButton,
      resumeButton: turnResumeButton,
      confirmDialog: turnRecoveryConfirm,
      confirmText: turnRecoveryConfirmText,
      conflictList: turnRecoveryConflicts,
    },
    callbacks: {
      send,
      nextRequestId,
      getActiveChatId: () => activeChatId,
      isChatStreaming: () => chatStreaming,
      applyAuthoritativeChat: applyOpenedChat,
      submitRetryPayload: (payload) => composerActions?.submitMessage(payload) || false,
      restoreDraftPayload: async (payload) => {
        if (prompt) {
          prompt.value = payload.text || "";
        }
        const restored = await restoreAttachments(payload.images, payload.contextSnippets);
        if (
          restored.restoredImages !== payload.images.length
          || restored.restoredContextSnippets !== payload.contextSnippets.length
        ) {
          appendSystem("Some original attachments could not be restored to the composer.");
        }
        resizePrompt();
        focusComposer();
      },
      setDraftRestoring: (restoring) => {
        recoveryDraftRestoring = restoring;
        composer?.setAttribute("aria-busy", String(restoring));
        if (sendButton) {
          sendButton.disabled = restoring;
        }
      },
      refreshAuthoritativeChat: () => refreshActiveChat("open-chat-after-recovery-error"),
      appendSystem,
    },
  });

  effortControls = window.FennaraEffortControls.createEffortControls({
    elements: {
      reasoningEffortControls,
      effortStatus,
      effortToggle,
      effortOptions,
      effortOptionButtons,
    },
    callbacks: {
      getCurrentReasoningEffort: () => currentReasoningEffort,
      setCurrentReasoningEffort: (effort) => {
        currentReasoningEffort = effort;
      },
      cleanReasoningEffort,
      saveCurrentChatSettings,
    },
  });

  const chatNavigation = window.FennaraChatNavigation.createChatNavigation({
    appShell,
    chatList,
    chatTitle,
    prompt,
    modelInput,
    send,
    nextRequestId,
    clearTranscript,
    clearAttachments,
    resizePrompt,
    cleanModelId: cleanUiModelId,
    getActiveChatId: () => activeChatId,
    getCurrentModel: () => currentModel,
    getCurrentReasoningEffort: () => currentReasoningEffort,
  });
  const closeDrawer = chatNavigation.closeDrawer;
  const closeDrawerFromOutsideClick = chatNavigation.closeDrawerFromOutsideClick;
  const renderChatList = chatNavigation.renderChatList;
  const startNewChat = chatNavigation.startNewChat;
  const toggleDrawer = chatNavigation.toggleDrawer;
  const updateChatTitle = chatNavigation.updateChatTitle;

  window.FennaraOverlayManager.createOverlayManager({
    elements: {
      commandPopover,
      providerPopover,
      providerKeyPopover,
      ollamaSetupPopover,
      modelPopover,
      effortOptions,
      usagePopover,
      prompt,
    },
    callbacks: {
      closeDrawerFromOutsideClick,
      closeDrawer,
      closeProviderPicker,
      closeProviderKeyPrompt: closeOpenRouterKeyPrompt,
      closeLocalSetupPrompt: closeOllamaSetupPrompt,
      setEffortMenuOpen,
      setUsagePopoverOpen,
      focusComposer,
      positionUsagePopover,
      positionProviderPopover,
      positionProviderKeyPrompt,
      positionLocalSetupPrompt: positionOllamaSetupPrompt,
      commandPalette,
      modelPicker,
    },
  });

  window.FennaraShellBindings.createShellBindings({
    elements: {
      providerSearch,
      providerKeyPopover,
      providerKeyInlineInput,
      ollamaSetupPopover,
      providerKeyForm,
      ollamaForm,
      reloadButton,
      attachImageButton,
      imageInput,
      usageContainer,
      usagePopover,
      sessionCostStatus,
      setMcpTargetButton,
      targetPillText,
    },
    callbacks: {
      openModelPicker,
      renderProviderOptions,
      closeProviderPicker,
      closeProviderKeyPrompt: closeOpenRouterKeyPrompt,
      closeLocalSetupPrompt: closeOllamaSetupPrompt,
      focusComposer,
      saveProviderKey: (key) => {
        const provider = keyPromptProvider || currentProvider || "openrouter";
        const requestId = nextRequestId("save-settings-key");
        pendingProviderKeySaves.set(requestId, provider);
        currentProvider = provider;
        if (currentModel && providerFromModel(currentModel) !== provider) {
          currentModel = "";
        }
        updateProviderUi();
        updateModelUi();
        updateChatSize();
        queueSettingsSave({
          type: "save_settings",
          request_id: requestId,
          model: cleanUiModelId(modelInput?.value || currentModel),
          reasoning_effort: currentReasoningEffort,
          ollama_base_url: ollamaBaseUrl,
          provider_base_urls: providerBaseUrlPayload(),
          provider_api_keys: {
            [provider]: key,
          },
        });
      },
      saveLocalProvider: saveOllamaProvider,
      flashCopied: (button, normalLabel, copiedLabel) => {
        transcriptRenderer.flashCopied(button, normalLabel, copiedLabel);
      },
      toggleDrawer,
      startNewChat,
      addImageFiles,
      showUsagePopover,
      hideUsagePopoverSoon,
      send,
      nextRequestId,
    },
    constants: {
      showReloadButton: SHOW_RELOAD_BUTTON,
    },
  });

  function requestModelList(options = {}) {
    const refreshLocal = options.refreshLocal !== false;
    if (options.refreshOllama) {
      ollamaStatus = "checking";
      ollamaModels = [];
      providerRegistry
        .filter((provider) => provider.kind === "local")
        .forEach((provider) => {
          const existing = localProviderStatuses.get(provider.id) || {};
          localProviderStatuses.set(provider.id, { ...existing, state: "checking" });
        });
      updateProviderUi();
      updateModelUi();
    }
    return send({
      type: "list_models",
      request_id: nextRequestId("list-models"),
      refresh_local: refreshLocal,
    });
  }

  function refreshModelCatalog(force = true) {
    catalogRefreshInFlight = true;
    modelPicker?.applyCatalog({
      ...(lastModelCatalog || {}),
      catalog_status: openrouterCatalogStatus,
      refreshing: true,
      providers: providerRegistry,
    });
    const sent = send({
      type: "refresh_model_catalog",
      request_id: nextRequestId("refresh-model-catalog"),
      force,
    });
    if (!sent) {
      catalogRefreshInFlight = false;
    }
    return sent;
  }

  function requestProjectStatus() {
    return send({ type: "get_project_status", request_id: nextRequestId("project-status") });
  }

  function startProjectStatusPolling() {
    stopProjectStatusPolling();
    projectStatusTimer = window.setInterval(requestProjectStatus, 5000);
  }

  function stopProjectStatusPolling() {
    window.clearInterval(projectStatusTimer);
    projectStatusTimer = 0;
  }

  function setStreaming(nextStreaming) {
    chatStreaming = nextStreaming;
    appShell?.classList.toggle("is-streaming", nextStreaming);
    if (sendButton) {
      sendButton.setAttribute("aria-busy", String(nextStreaming));
      sendButton.querySelector(".send-label").textContent = nextStreaming ? "Cancel" : "Send";
    }
    turnRecoveryControls?.handleStreamingChanged();
  }

  function openModelPicker(forceOpen = false) { return providerPopovers?.openModelPicker(forceOpen); }
  function openProviderPicker() { return providerPopovers?.openProviderPicker() || false; }
  function closeProviderPicker() { return providerPopovers?.closeProviderPicker(); }
  function positionProviderPopover() { return providerPopovers?.positionProviderPopover(); }
  function openOpenRouterKeyPrompt() { return providerPopovers?.openOpenRouterKeyPrompt() || false; }
  function openProviderKeyPrompt(providerId) { return providerPopovers?.openProviderKeyPrompt(providerId) || false; }
  function closeOpenRouterKeyPrompt() { return providerPopovers?.closeOpenRouterKeyPrompt(); }
  function openOllamaSetupPrompt() { return providerPopovers?.openOllamaSetupPrompt() || false; }
  function closeOllamaSetupPrompt() { return providerPopovers?.closeOllamaSetupPrompt(); }

  function focusComposer() {
    if (!prompt || chatStreaming) {
      return;
    }
    const restore = () => {
      prompt.focus({ preventScroll: true });
      const end = prompt.value.length;
      prompt.setSelectionRange?.(end, end);
    };
    window.setTimeout(restore, 0);
    window.requestAnimationFrame?.(() => window.setTimeout(restore, 0));
  }

  function positionProviderKeyPrompt() { return providerPopovers?.positionProviderKeyPrompt(); }
  function positionOllamaSetupPrompt() { return providerPopovers?.positionOllamaSetupPrompt(); }
  function renderProviderOptions() { return providerPopovers?.renderProviderOptions(); }
  function syncOllamaSetupFields() { return providerPopovers?.syncOllamaSetupFields(); }

  function clearTranscript(resetCost = true) {
    transcriptRenderer.clear(resetCost, () => {
      sessionCost = 0;
      latestPromptTokens = 0;
      updateChatSize();
      updateSessionCost();
    });
  }

  function appendMessage(role, text, attachments = [], contextSnippets = []) { return storedTranscript?.appendMessage(role, text, attachments, contextSnippets); }
  function appendDaemonUserMessage(userMessage, requestId = "") { return storedTranscript?.appendDaemonUserMessage(userMessage, requestId); }
  function hasConnectedOptimisticUserMessage(requestId) { return storedTranscript?.hasConnectedOptimisticUserMessage(requestId) || false; }
  function restorePendingOptimisticUserMessages(chatId) { return storedTranscript?.restorePendingOptimisticUserMessages(chatId); }
  function renderStoredMessages(messages, contextCompactions = []) {
    return storedTranscript?.renderStoredMessages(messages, contextCompactions);
  }
  function trackOptimisticRequest(requestId, value) { return storedTranscript?.trackOptimisticRequest(requestId, value); }
  function deleteOptimisticRequest(requestId) { return storedTranscript?.deleteOptimisticRequest(requestId); }

  function appendSystem(text) {
    transcriptRenderer.appendSystem(text);
  }

  function clearSystemStatus() {
    transcriptRenderer.clearSystemStatus();
  }

  function updateThinkingText(text, status) {
    transcriptRenderer.updateThinkingText(text, status);
  }

  function updateAssistantText(text) {
    if (String(text || "").trim()) {
      transcriptRenderer.finishActiveThinking();
    }
    transcriptRenderer.updateAssistantText(text);
  }

  function updateToolCall(item) {
    transcriptRenderer.updateToolCall(item);
  }

  function handleContextCompaction(message) {
    const chatId = String(message.chat_id || "");
    if (activeChatId && chatId && chatId !== activeChatId) {
      return;
    }
    activeChatId = chatId || activeChatId;
    transcriptRenderer.updateContextCompaction(message.status);
  }

  function applySettings(settings, options = {}) {
    if (!settings) {
      return;
    }
    applyProviderBaseUrls(settings);
    applyLocalModelContextLengths(settings);
    ollamaBaseUrl = providerBaseUrl("ollama");
    applyProviderRegistry(settings);
    hasOpenRouterKey = providerConnected("openrouter") || Boolean(settings.has_openrouter_key);
    hasOllamaCloudKey = providerConnected("ollama-cloud") || Boolean(settings.has_ollama_cloud_key);
    defaultModel = cleanUiModelId(settings.default_model || defaultModel);
    const savedModel = cleanUiModelId(settings.model || "");
    const savedProvider = providerFromModel(savedModel);
    if (savedModel && !isUnavailableDefaultModel(savedModel, savedProvider)) {
      currentProvider = savedProvider || currentProvider;
      currentModel = savedModel;
    } else if (savedModel === defaultModel && isUnavailableDefaultModel(savedModel, savedProvider)) {
      currentModel = "";
      if (currentProvider === savedProvider) {
        currentProvider = "";
      }
    }
    currentReasoningEffort = cleanReasoningEffort(settings.reasoning_effort);
    currentChatSurface = cleanChatSurface(settings.chat_surface);
    currentApprovalMode = cleanApprovalMode(settings.approval_mode);
    if (!currentProvider && hasOpenRouterKey) {
      currentProvider = "openrouter";
    }
    if (chatSurfaceBrowserInput) {
      chatSurfaceBrowserInput.checked = currentChatSurface === CHAT_SURFACE_BROWSER;
    }
    syncApprovalModeControls();
    updateChatSurfaceRestartNotice();
    if (ollamaBaseUrlInput) {
      syncOllamaSetupFields();
    }
    if (modelInput) {
      modelInput.value = currentModel;
    }
    updateProviderUi();
    updateModelUi();
    updateChatSize();
    syncReasoningControls();
    updateComposerEffort();
    const list = document.querySelector("#model-suggestions");
    if (list && Array.isArray(settings.text_model_suggestions)) {
      list.replaceChildren();
      for (const model of settings.text_model_suggestions) {
        const option = document.createElement("option");
        option.value = model;
        list.append(option);
      }
    }
    if (options.refreshModels !== false) {
      requestModelList();
    }
  }

  function applyProviderRegistry(settings) {
    const providers = Array.isArray(settings.providers) && settings.providers.length
      ? settings.providers
      : fallbackProviderRegistry(settings);
    providerRegistry = providers.map(normalizeProvider).filter((provider) => provider.id);
    providerMetadata = new Map(providerRegistry.map((provider) => [provider.id, provider]));
    renderProviderOptions();
    modelPicker?.applyCatalog({
      ...(lastModelCatalog || {}),
      providers: providerRegistry,
    });
  }

  function normalizeProvider(provider) {
    const id = String(provider?.id || "").trim();
    const setup = provider?.setup || null;
    if (id && setup?.base_url) {
      providerBaseUrls.set(id, String(setup.base_url));
    }
    return {
      id,
      name: String(provider?.name || id || "Provider"),
      kind: String(provider?.kind || "cloud"),
      auth: provider?.auth || { type: "none" },
      connected: Boolean(provider?.connected),
      model_prefix: String(provider?.model_prefix || (id ? `${id}/` : "")),
      setup,
    };
  }

  function fallbackProviderRegistry(settings) {
    return [
      {
        id: "openrouter",
        name: "OpenRouter",
        kind: "cloud",
        auth: { type: "api_key", env: "OPENROUTER_API_KEY" },
        connected: Boolean(settings.has_openrouter_key),
        model_prefix: "openrouter/",
      },
      {
        id: "ollama",
        name: "Ollama (local)",
        kind: "local",
        auth: { type: "none" },
        connected: true,
        model_prefix: "ollama/",
        setup: {
          type: "base_url",
          default_base_url: DEFAULT_OLLAMA_BASE_URL,
          base_url: settings.ollama_base_url || DEFAULT_OLLAMA_BASE_URL,
        },
      },
      {
        id: "ollama-cloud",
        name: "Ollama Cloud",
        kind: "cloud",
        auth: { type: "api_key", env: "OLLAMA_API_KEY" },
        connected: Boolean(settings.has_ollama_cloud_key),
        model_prefix: "ollama-cloud/",
      },
      {
        id: "deepseek",
        name: "DeepSeek",
        kind: "cloud",
        auth: { type: "api_key", env: "DEEPSEEK_API_KEY" },
        connected: false,
        model_prefix: "deepseek/",
      },
      {
        id: "lmstudio",
        name: "LM Studio",
        kind: "local",
        auth: { type: "none" },
        connected: true,
        model_prefix: "lmstudio/",
        setup: {
          type: "base_url",
          default_base_url: DEFAULT_LOCAL_BASE_URLS.lmstudio,
          base_url: providerBaseUrl("lmstudio"),
        },
      },
    ];
  }

  function applyProviderBaseUrls(settings) {
    providerBaseUrls = new Map(Object.entries(DEFAULT_LOCAL_BASE_URLS));
    const baseUrls = settings?.provider_base_urls || {};
    Object.entries(baseUrls).forEach(([provider, baseUrl]) => {
      const clean = String(baseUrl || "").trim().replace(/\/+$/, "");
      if (provider && clean) {
        providerBaseUrls.set(provider, clean);
      }
    });
    if (settings?.ollama_base_url) {
      providerBaseUrls.set("ollama", String(settings.ollama_base_url).trim().replace(/\/+$/, ""));
    }
  }

  function providerBaseUrl(providerId) {
    const id = String(providerId || "");
    return providerBaseUrls.get(id) || DEFAULT_LOCAL_BASE_URLS[id] || "";
  }

  function providerBaseUrlPayload() {
    return Object.fromEntries(providerBaseUrls.entries());
  }

  function applyLocalModelContextLengths(settings) {
    localModelContextLengths = new Map();
    Object.entries(settings?.local_model_context_lengths || {}).forEach(([model, contextLength]) => {
      const clean = cleanUiModelId(model);
      const value = normalizeContextLength(contextLength);
      if (clean && value) {
        localModelContextLengths.set(clean, value);
      }
    });
  }

  function localModelContextLengthPayload() {
    return Object.fromEntries(localModelContextLengths.entries());
  }

  function currentModelInfo() {
    const info = modelPicker?.modelInfo(currentModel) || null;
    const override = localModelContextLengths.get(currentModel);
    const detected = modelContextLength(info);
    if (detected > 0) {
      if (info && Number(info.context_length || 0) <= 0) {
        return { ...info, context_length: detected };
      }
      return info;
    }
    if (!override) {
      return info;
    }
    return {
      ...(info || { id: currentModel }),
      context_length: override,
    };
  }

  function modelContextLength(info) {
    const candidates = [
      info?.context_length,
      info?.contextLength,
      info?.context_tokens,
      info?.contextTokens,
      info?.max_context_length,
      info?.maxContextLength,
      info?.limits?.context_tokens,
      info?.limits?.contextTokens,
      info?.limits?.context,
    ];
    for (const candidate of candidates) {
      const value = normalizeContextLength(candidate);
      if (value > 0) {
        return value;
      }
    }
    return 0;
  }

  function currentModelLabel() {
    if (!currentModel || isUnavailableDefaultModel(currentModel)) {
      return "No model";
    }
    return modelPicker?.displayName(currentModel) || currentModel;
  }

  function isUnavailableDefaultModel(modelId, providerId = providerFromModel(modelId)) {
    const clean = cleanUiModelId(modelId);
    return Boolean(
      clean
        && defaultModel
        && clean === defaultModel
        && providerRequiresApiKey(providerId)
        && !providerConnected(providerId),
    );
  }

  function providerFromModel(modelId) {
    const clean = cleanUiModelId(modelId);
    const provider = providerRegistry
      .slice()
      .sort((a, b) => b.model_prefix.length - a.model_prefix.length)
      .find((candidate) => candidate.model_prefix && clean.startsWith(candidate.model_prefix));
    if (provider) {
      return provider.id;
    }
    if (clean.includes("/")) {
      return "openrouter";
    }
    return "";
  }

  function providerLabel(provider = currentProvider) {
    const label = providerMetadata.get(provider)?.name;
    if (label) {
      return label;
    }
    return "Choose provider";
  }

  function updateProviderUi() {
    providerStatuses.forEach((status) => {
      status.textContent = providerLabel();
      status.title = providerLabel();
    });
    if (providerDot) {
      providerDot.classList.toggle("is-idle", !currentProvider);
      providerDot.classList.toggle("is-ready", hasUsableModel());
    }
    renderProviderOptions();
  }

  function localProviderLabel(providerId, fallbackState = "unknown") {
    const state = localProviderStatuses.get(providerId)?.state || fallbackState;
    if (state === "checking") {
      return "Checking";
    }
    if (state === "ready") {
      return "Connected";
    }
    if (state === "empty") {
      return "No models";
    }
    if (state === "offline") {
      return "Offline";
    }
    return "Not connected";
  }

  function providerStatusLabel(provider) {
    if (provider.kind === "local") {
      return localProviderLabel(provider.id, provider.id === "ollama" ? ollamaStatus : "unknown");
    }
    if (provider.auth?.type === "api_key") {
      return provider.connected ? "Connected" : "Not connected";
    }
    return provider.connected ? "Connected" : "Available";
  }

  function providerRequiresApiKey(providerId) {
    return providerMetadata.get(providerId)?.auth?.type === "api_key";
  }

  function providerConnected(providerId) {
    return Boolean(providerMetadata.get(providerId)?.connected);
  }

  function markApiKeyProviderConnected(providerId) {
    const id = String(providerId || "").trim();
    if (!id || !providerRequiresApiKey(id)) {
      return;
    }
    let changed = false;
    providerRegistry = providerRegistry.map((provider) => {
      if (provider.id !== id || provider.connected) {
        return provider;
      }
      changed = true;
      return { ...provider, connected: true };
    });
    if (!changed) {
      return;
    }
    providerMetadata = new Map(providerRegistry.map((provider) => [provider.id, provider]));
    renderProviderOptions();
    modelPicker?.applyCatalog({
      ...(lastModelCatalog || {}),
      catalog_status: openrouterCatalogStatus,
      refreshing: catalogRefreshInFlight,
      providers: providerRegistry,
    });
  }

  function refreshModelsAfterProviderKeySave(providerId) {
    markApiKeyProviderConnected(providerId);
    requestModelList({ refreshLocal: false });
    modelPicker?.open();
  }

  function providerUsesBaseUrlSetup(providerId) {
    return providerMetadata.get(providerId)?.setup?.type === "base_url";
  }

  function applyModelCatalog(catalog) {
    lastModelCatalog = catalog || { models: [] };
    openrouterCatalogStatus = lastModelCatalog.catalog_status || openrouterCatalogStatus;
    ollamaStatus = String(lastModelCatalog.ollama_status?.state || ollamaStatus || "unknown");
    ollamaBaseUrl = lastModelCatalog.ollama_status?.base_url || ollamaBaseUrl;
    providerBaseUrls.set("ollama", ollamaBaseUrl);
    localProviderStatuses = new Map(
      Object.entries(lastModelCatalog.local_provider_statuses || {}),
    );
    localProviderStatuses.forEach((status, providerId) => {
      if (status?.base_url) {
        providerBaseUrls.set(providerId, String(status.base_url));
      }
    });
    const models = Array.isArray(lastModelCatalog.models) ? lastModelCatalog.models : [];
    const daemonOllamaModels = models.filter((model) => String(model?.id || "").startsWith("ollama/"));
    ollamaModels = daemonOllamaModels;
    if (currentProviderIsLocal() && !localModelAvailable(currentModel)) {
      currentModel = "";
    }
    modelPicker?.applyCatalog({
      ...lastModelCatalog,
      catalog_status: openrouterCatalogStatus,
      refreshing: catalogRefreshInFlight,
      providers: providerRegistry,
    });
    updateProviderUi();
    updateModelUi();
    updateChatSize();
  }

  function updateModelUi() {
    modelStatuses.forEach((status) => {
      status.textContent = currentModelLabel();
      status.title = currentModel || "No model selected";
    });
    if (modelInput) {
      modelInput.value = currentModel;
    }
    sendButton?.classList.toggle("is-blocked", !hasUsableModel());
  }

  function hasUsableModel() {
    if (!currentModel || !currentProvider) {
      return false;
    }
    if (isUnavailableDefaultModel(currentModel, currentProvider)) {
      return false;
    }
    if (currentProviderIsLocal()) {
      return localModelAvailable(currentModel);
    }
    if (providerRequiresApiKey(currentProvider)) {
      return providerConnected(currentProvider);
    }
    return true;
  }

  function currentProviderIsLocal() {
    return providerMetadata.get(currentProvider)?.kind === "local";
  }

  function localModelAvailable(modelId) {
    const clean = cleanUiModelId(modelId);
    if (!clean) {
      return false;
    }
    return (lastModelCatalog?.models || []).some((model) => {
      const id = String(model?.id || "");
      return id === clean && model?.source === "local";
    });
  }

  function selectModel(modelId) {
    const clean = window.FennaraModelPicker?.cleanModelId(modelId) || String(modelId || "").trim();
    if (!clean) {
      return;
    }
    maybePromptForLocalContextLength(clean);
    currentProvider = providerFromModel(clean) || currentProvider;
    currentModel = clean;
    updateProviderUi();
    updateModelUi();
    updateChatSize();
    saveCurrentChatSettings();
  }

  function cleanReasoningEffort(effort) {
    return ["low", "medium", "high"].includes(effort) ? effort : "medium";
  }

  function cleanChatSurface(surface) {
    return surface === CHAT_SURFACE_BROWSER ? CHAT_SURFACE_BROWSER : CHAT_SURFACE_EMBEDDED;
  }

  function cleanApprovalMode(mode) {
    return mode === APPROVAL_MODE_FULL_ACCESS ? APPROVAL_MODE_FULL_ACCESS : APPROVAL_MODE_ASK;
  }

  function syncApprovalModeControls() {
    return settingsPanel?.syncApprovalModeControls();
  }

  function updateChatSurfaceRestartNotice(surface) {
    return settingsPanel?.updateChatSurfaceRestartNotice(surface);
  }

  function syncReasoningControls() {
    return effortControls?.syncReasoningControls(currentReasoningEffort);
  }

  function updateComposerEffort() {
    return effortControls?.updateComposerEffort();
  }

  function setEffortMenuOpen(open) {
    return effortControls?.setEffortMenuOpen(open);
  }

  function saveCurrentChatSettings() {
    const payload = {
      type: "save_settings",
      request_id: nextRequestId("silent-settings"),
      model: cleanUiModelId(modelInput?.value || currentModel),
      reasoning_effort: currentReasoningEffort,
      ollama_base_url: ollamaBaseUrl,
      provider_base_urls: providerBaseUrlPayload(),
      local_model_context_lengths: localModelContextLengthPayload(),
      approval_mode: currentApprovalMode,
    };
    return send(payload);
  }

  function maybePromptForLocalContextLength(modelId) {
    if (!isLocalContextModel(modelId) || localModelContextLengths.has(modelId)) {
      return;
    }
    const detected = modelContextLength(modelPicker?.modelInfo(modelId));
    if (detected > 0) {
      return;
    }
    const raw = window.prompt(
      `Fennara could not detect the context length for ${modelId}. Enter the token count, for example 8192 or 32768.`,
      "",
    );
    const contextLength = normalizeContextLength(raw);
    if (!contextLength) {
      appendSystem("Local model context length is still unknown.");
      window.setTimeout(clearSystemStatus, 1600);
      return;
    }
    localModelContextLengths.set(modelId, contextLength);
    appendSystem(`Saved ${contextLength.toLocaleString("en-US")} token context for ${modelId}.`);
    window.setTimeout(clearSystemStatus, 1600);
  }

  function isLocalContextModel(modelId) {
    const provider = providerFromModel(modelId);
    return provider === "ollama" ||
      provider === "lmstudio" ||
      providerMetadata.get(provider)?.kind === "local";
  }

  function normalizeContextLength(value) {
    const parsed = Number(String(value || "").replace(/,/g, "").trim());
    if (!Number.isFinite(parsed) || parsed <= 0) {
      return 0;
    }
    return Math.floor(parsed);
  }

  function chooseProvider(provider) {
    if (!providerMetadata.has(provider)) {
      return;
    }
    currentProvider = provider;
    closeProviderPicker();
    if (currentModel && providerFromModel(currentModel) !== provider) {
      currentModel = "";
    }
    updateProviderUi();
    updateModelUi();
    updateChatSize();
    if (providerRequiresApiKey(provider) && !providerConnected(provider)) {
      openProviderKeyPrompt(provider);
      requestModelList();
      return;
    }
    if (providerUsesBaseUrlSetup(provider)) {
      openOllamaSetupPrompt();
      requestModelList({ refreshOllama: true });
      return;
    }
    requestModelList();
  }

  function saveOllamaProvider() {
    const provider = currentProvider && providerUsesBaseUrlSetup(currentProvider) ? currentProvider : "ollama";
    const defaultBaseUrl = providerMetadata.get(provider)?.setup?.default_base_url || DEFAULT_OLLAMA_BASE_URL;
    const nextBaseUrl = String(ollamaBaseUrlInput?.value || providerBaseUrl(provider) || defaultBaseUrl).trim() || defaultBaseUrl;
    providerBaseUrls.set(provider, nextBaseUrl.replace(/\/+$/, ""));
    if (provider === "ollama") {
      ollamaBaseUrl = providerBaseUrl(provider);
    }
    currentProvider = provider;
    closeOllamaSetupPrompt();
    send({
      type: "save_settings",
      request_id: nextRequestId("save-local-provider"),
      reasoning_effort: currentReasoningEffort,
      ollama_base_url: ollamaBaseUrl,
      provider_base_urls: providerBaseUrlPayload(),
    });
    requestModelList({ refreshOllama: true });
    modelPicker?.open();
    appendSystem(`Checking local ${providerLabel(provider)} models.`);
    window.setTimeout(clearSystemStatus, 1200);
  }

  function clearSettingsSaveTimeout() {
    return settingsPanel?.clearSaveTimeout();
  }

  function markSettingsClean(options = {}) {
    return settingsPanel?.markClean(options);
  }

  function setSettingsSaving(saving) {
    return settingsPanel?.setSaving(saving);
  }

  function flushPendingSettings() {
    return settingsPanel?.flushPending() || false;
  }

  function queueSettingsSave(payload) {
    return settingsPanel?.queueSave(payload) || false;
  }

  function handleDaemonMessage(message) {
    if (turnRecoveryControls?.handleMessage(message)) {
      return;
    }
    if (turnRecoveryControls?.handleError(message)) {
      return;
    }
    if (message.type === "settings" || message.type === "settings_saved") {
      const requestId = String(message.request_id || "");
      const isKeySave = requestId.startsWith("save-settings-key");
      const keySaveProvider = isKeySave
        ? pendingProviderKeySaves.get(requestId) || keyPromptProvider || currentProvider
        : "";
      const isSettingsDialogSave = requestId.startsWith("save-settings-") && !isKeySave;
      const isProviderSetupSave = requestId.startsWith("save-ollama-provider") || requestId.startsWith("save-local-provider");
      const isSilentSave = requestId.startsWith("silent-settings");
      applySettings(message.settings, {
        preserveTypedKey: !isKeySave && !isSettingsDialogSave,
        refreshModels: message.type === "settings",
      });
      if (isKeySave) {
        pendingProviderKeySaves.delete(requestId);
        currentProvider = keySaveProvider || currentProvider;
        markApiKeyProviderConnected(currentProvider);
        if (currentModel && providerFromModel(currentModel) !== currentProvider) {
          currentModel = "";
        }
        updateProviderUi();
        updateModelUi();
        updateChatSize();
      }
      if (isProviderSetupSave) {
        currentProvider = providerUsesBaseUrlSetup(currentProvider) ? currentProvider : "ollama";
        if (currentModel && providerFromModel(currentModel) !== currentProvider) {
          currentModel = "";
        }
        updateProviderUi();
        updateModelUi();
        updateChatSize();
      }
      if (message.type === "settings_saved") {
        const restartNeeded = currentChatSurface !== RUNTIME_CHAT_SURFACE;
        if (settingsPanel?.hasPendingRequest(message.request_id)) {
          settingsPanel.clearPending();
          if (isSettingsDialogSave) {
            markSettingsClean({ showSaved: true });
          } else {
            clearSettingsSaveTimeout();
            setSettingsSaving(false);
          }
          if (isKeySave) {
            closeOpenRouterKeyPrompt();
          }
        }
        if (!isSilentSave) {
          if (restartNeeded && isSettingsDialogSave) {
            appendSystem("Settings saved. Restart Godot for the chat display change to take effect.");
            window.setTimeout(clearSystemStatus, 7000);
          } else if (!isSettingsDialogSave) {
            appendSystem("Settings saved locally.");
            window.setTimeout(clearSystemStatus, 1200);
          }
        }
        if (!restartNeeded && isKeySave) {
          refreshModelsAfterProviderKeySave(currentProvider);
        } else if (!isSettingsDialogSave && !isSilentSave) {
          requestModelList();
        }
      } else {
        markSettingsClean();
        clearSystemStatus();
      }
      return;
    }
    if (message.type === "chat_reset") {
      clearTranscript();
      setStreaming(false);
      return;
    }
    if (message.type === "chat_list") {
      renderChatList(message.chats || []);
      return;
    }
    if (message.type === "model_list") {
      applyModelCatalog(message.catalog);
      return;
    }
    if (message.type === "catalog_refresh_result") {
      catalogRefreshInFlight = false;
      openrouterCatalogStatus = message.status || openrouterCatalogStatus;
      modelPicker?.applyCatalog({
        ...(lastModelCatalog || {}),
        catalog_status: openrouterCatalogStatus,
        refreshing: false,
        error: message.ok ? null : message.error?.message || "Catalog refresh failed.",
        providers: providerRegistry,
      });
      requestModelList();
      if (!message.ok) {
        appendSystem(message.error?.message || "Could not refresh model catalog.");
        window.setTimeout(clearSystemStatus, 1600);
      }
      return;
    }
    if (message.type === "project_status") {
      applyProjectStatus(message);
      return;
    }
    if (message.type === "chat_context_snippet") {
      addContextSnippet(message);
      return;
    }
    if (message.type === "project_file_opened") {
      if (message.ok === false || message.error) {
        appendSystem(message.error || "Could not open that project file.");
        window.setTimeout(clearSystemStatus, 1800);
      }
      return;
    }
    if (message.type === "chat_opened") {
      applyOpenedChat(message);
      return;
    }
    if (message.type === "chat_created") {
      activeChatId = message.chat?.id || activeChatId;
      updateChatTitle(message.chat);
      sessionCost = Number(message.chat?.total_cost || 0);
      latestPromptTokens = Number(message.chat?.latest_prompt_tokens || 0);
      updateChatSize();
      updateSessionCost();
      return;
    }
    if (message.type === "chat_updated") {
      if (message.chat?.id && (!activeChatId || message.chat.id === activeChatId)) {
        activeChatId = message.chat.id;
        updateChatTitle(message.chat);
        const nextSessionCost = Number(message.chat?.total_cost || sessionCost);
        if (chatStreaming && Number.isFinite(nextSessionCost) && nextSessionCost > sessionCost) {
          activeTurnCost += nextSessionCost - sessionCost;
        }
        sessionCost = nextSessionCost;
        latestPromptTokens = Number(message.chat?.latest_prompt_tokens || latestPromptTokens || 0);
        updateChatSize();
        updateSessionCost();
      }
      return;
    }
    if (message.type === "chat_context_compaction") {
      handleContextCompaction(message);
      return;
    }
    if (message.type === "chat_user_message") {
      clearSystemStatus();
      setStreaming(true);
      transcriptRenderer.resetActiveAssistant();
      activeTurnCost = 0;
      activeChatId = message.chat_id || activeChatId;
      const requestKey = String(message.request_id || "");
      const usedOptimisticMessage = hasConnectedOptimisticUserMessage(requestKey);
      if (message.user_message && !usedOptimisticMessage) {
        appendDaemonUserMessage(message.user_message, requestKey);
      }
      transcriptRenderer.beginStream();
      deleteOptimisticRequest(requestKey);
      return;
    }
    if (message.type === "chat_stream_start") {
      clearSystemStatus();
      setStreaming(true);
      transcriptRenderer.resetActiveAssistant();
      activeTurnCost = 0;
      activeChatId = message.chat_id || activeChatId;
      if (message.user_message) {
        const requestKey = String(message.request_id || "");
        if (!hasConnectedOptimisticUserMessage(requestKey)) {
          appendDaemonUserMessage(message.user_message, requestKey);
        }
      }
      transcriptRenderer.beginStream();
      return;
    }
    if (message.type === "chat_item_update" && message.item?.type === "message") {
      updateAssistantText(message.item.content || "");
      return;
    }
    if (message.type === "chat_item_update" && message.item?.type === "reasoning") {
      updateThinkingText(message.item.content || "", message.item.status);
      return;
    }
    if (
      message.type === "chat_item_update" &&
      (message.item?.type === "function_call" || message.item?.type === "tool_result")
    ) {
      updateToolCall(message.item);
      if (message.item?.type === "tool_result") {
        transcriptRenderer.resetActiveAssistant();
      }
      return;
    }
    if (message.type === "chat_response") {
      clearSystemStatus();
      updateAssistantText(message.response || "");
      transcriptRenderer.flushAssistantRender();
      const cost = usageCost(message.usage);
      if (Number.isFinite(cost)) {
        activeTurnCost += cost;
        sessionCost += cost;
        updateSessionCost();
      }
      latestPromptTokens = usagePromptTokens(message.usage) || latestPromptTokens;
      updateChatSize();
      const turnUsage = { ...(message.usage || {}), cost: activeTurnCost };
      transcriptRenderer.addAssistantActions(turnUsage, formatUsageCost);
      activeTurnCost = 0;
      transcriptRenderer.finishActiveThinking();
      transcriptRenderer.endStream();
      transcriptRenderer.resetActiveAssistant();
      setStreaming(false);
      deleteOptimisticRequest(message.request_id || "");
      if (message.turn_recovery_warning && activeChatId) {
        checkpointWarnings.set(activeChatId, String(message.turn_recovery_warning));
      }
      refreshActiveChat("open-chat-after-turn");
      return;
    }
    if (message.type === "chat_cancelled") {
      clearSystemStatus();
      transcriptRenderer.updateContextCompaction("failed");
      updateAssistantText(message.response || "");
      transcriptRenderer.flushAssistantRender();
      transcriptRenderer.finishActiveThinking();
      transcriptRenderer.endStream();
      transcriptRenderer.resetActiveAssistant();
      setStreaming(false);
      appendSystem("Cancelled.");
      window.setTimeout(clearSystemStatus, 1200);
      deleteOptimisticRequest(message.request_id || "");
      refreshActiveChat("open-chat-after-cancel");
      return;
    }
    if (message.type === "error") {
      const errorText = message.message || "Chat request failed.";
      transcriptRenderer.updateContextCompaction("failed");
      const requestId = String(message.request_id || "");
      if (requestId.startsWith("open-project-file")) {
        appendSystem(errorText);
        window.setTimeout(clearSystemStatus, 2400);
        return;
      }
      if (chatStreaming) {
        updateAssistantText(`Request failed: ${errorText}`);
        transcriptRenderer.flushAssistantRender();
      }
      appendSystem(errorText);
      if (settingsPanel?.hasPendingRequest(message.request_id)) {
        settingsPanel.clearPending();
        clearSettingsSaveTimeout();
        setSettingsSaving(false);
      }
      if (requestId.startsWith("save-settings-key")) {
        pendingProviderKeySaves.delete(requestId);
        send({ type: "get_settings", request_id: nextRequestId("settings-after-key-save-error") });
      }
      transcriptRenderer.finishActiveThinking();
      transcriptRenderer.endStream();
      transcriptRenderer.resetActiveAssistant();
      setStreaming(false);
      deleteOptimisticRequest(message.request_id || "");
      if (message.code === "provider_auth_error" || message.code === "missing_openrouter_key") {
        openProviderPicker();
      }
    }
  }

  function applyOpenedChat(message) {
    activeChatId = message.chat?.id || null;
    updateChatTitle(message.chat);
    const messages = message.messages || [];
    renderStoredMessages(messages, message.context_compactions || []);
    if (!message.request_id) {
      restorePendingOptimisticUserMessages(activeChatId);
    }
    sessionCost = Number(message.chat?.total_cost || 0);
    latestPromptTokens = Number(message.chat?.latest_prompt_tokens || latestPromptTokens || 0);
    const recoveryAvailable = message.turn_recovery?.can_undo
      || message.turn_recovery?.can_redo
      || message.turn_recovery?.operation_state;
    if (recoveryAvailable) {
      checkpointWarnings.delete(activeChatId);
    }
    turnRecoveryControls?.applyState(
      messages,
      message.turn_recovery,
      checkpointWarnings.get(activeChatId) || "",
    );
    if (message.recovery_resume_refresh?.ok === false) {
      appendSystem(
        `Recovery completed, but Godot did not refresh restored files: ${message.recovery_resume_refresh.error || "refresh failed"}`,
      );
    }
    updateChatSize();
    updateSessionCost();
  }

  function refreshActiveChat(prefix) {
    if (!activeChatId) {
      return false;
    }
    return send({
      type: "open_chat",
      request_id: nextRequestId(prefix),
      chat_id: activeChatId,
    });
  }

  function handleProjectFileReference(rawReference) {
    return projectFileLinks?.handleProjectFileReference(rawReference) || false;
  }

  function requestToolApprovalReview(approvalId, decision) {
    return composerActions?.requestToolApprovalReview(approvalId, decision);
  }

  function cleanUiModelId(modelId) {
    return window.FennaraModelPicker?.cleanModelId(modelId) || String(modelId || "").trim();
  }

  function resizePrompt() {
    if (!prompt) {
      return;
    }
    prompt.style.height = "auto";
    const nextHeight = Math.min(prompt.scrollHeight, PROMPT_MAX_HEIGHT);
    prompt.style.height = nextHeight + "px";
    prompt.style.overflowY = prompt.scrollHeight > PROMPT_MAX_HEIGHT ? "auto" : "hidden";
  }

  clearTranscript();
  appendSystem("Connecting to local daemon...");
  resizePrompt();
  updateProviderUi();
  updateModelUi();
  updateChatSize();
  updateSessionCost();
  connect();
})();
