(function () {
  function createComposerActions(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const composer = elements.composer || null;
    const prompt = elements.prompt || null;
    const modelInput = elements.modelInput || null;
    const commandPopover = elements.commandPopover || null;
    const sendButton = elements.sendButton || null;
    const isChatStreaming = callbacks.isChatStreaming || (() => false);
    const getAttachedContextSnippets = callbacks.getAttachedContextSnippets || (() => []);
    const hasAttachments = callbacks.hasAttachments || (() => false);
    const getCurrentProvider = callbacks.getCurrentProvider || (() => "");
    const getCurrentModel = callbacks.getCurrentModel || (() => "");
    const getCurrentReasoningEffort = callbacks.getCurrentReasoningEffort || (() => "medium");
    const setCurrentReasoningEffort = callbacks.setCurrentReasoningEffort || function () {};
    const cleanReasoningEffort = callbacks.cleanReasoningEffort || ((effort) => effort || "medium");
    const providerRequiresApiKey = callbacks.providerRequiresApiKey || (() => false);
    const providerConnected = callbacks.providerConnected || (() => false);
    const openProviderPicker = callbacks.openProviderPicker || function () {};
    const openModelPicker = callbacks.openModelPicker || function () {};
    const cleanModelId = callbacks.cleanModelId || ((modelId) => String(modelId || "").trim());
    const resetStreamState = callbacks.resetStreamState || function () {};
    const nextRequestId = callbacks.nextRequestId || (() => "chat");
    const getActiveChatId = callbacks.getActiveChatId || (() => null);
    const send = callbacks.send || (() => false);
    const attachmentPayload = callbacks.attachmentPayload || (() => []);
    const contextSnippetPayload = callbacks.contextSnippetPayload || (() => []);
    const setStreaming = callbacks.setStreaming || function () {};
    const setActiveTurnCost = callbacks.setActiveTurnCost || function () {};
    const appendMessage = callbacks.appendMessage || function () {};
    const beginStream = callbacks.beginStream || function () {};
    const trackOptimisticRequest = callbacks.trackOptimisticRequest || function () {};
    const deleteOptimisticRequest = callbacks.deleteOptimisticRequest || function () {};
    const clearAttachments = callbacks.clearAttachments || function () {};
    const resizePrompt = callbacks.resizePrompt || function () {};
    const commandPalette = callbacks.commandPalette || {};
    const addImageFiles = callbacks.addImageFiles || (() => Promise.resolve(0));
    const requestNativePastedImage = callbacks.requestNativePastedImage || function () {};
    const chatWsUrl = callbacks.chatWsUrl || (() => "");
    const appendSystem = callbacks.appendSystem || function () {};

    composer?.addEventListener("submit", (event) => {
      event.preventDefault();
      if (isChatStreaming()) {
        return;
      }
      const typedText = prompt?.value.trim() || "";
      const snippets = getAttachedContextSnippets();
      if (!typedText && !hasAttachments()) {
        return;
      }
      const currentProvider = getCurrentProvider();
      const currentModel = getCurrentModel();
      if (!currentProvider) {
        openProviderPicker();
        return;
      }
      if (!currentModel) {
        openModelPicker();
        return;
      }
      if (providerRequiresApiKey(currentProvider) && !providerConnected(currentProvider)) {
        openProviderPicker();
        return;
      }
      const model = cleanModelId(modelInput?.value || currentModel);
      const text = typedText;
      const reasoningEffort = cleanReasoningEffort(getCurrentReasoningEffort());
      setCurrentReasoningEffort(reasoningEffort);
      resetStreamState();
      const requestId = nextRequestId("chat");
      const payload = {
        type: "send_chat",
        request_id: requestId,
        chat_id: getActiveChatId(),
        message: text,
        model,
        reasoning_effort: reasoningEffort,
      };
      const images = attachmentPayload();
      if (images.length > 0) {
        payload.images = images;
      }
      if (snippets.length > 0) {
        payload.context_snippets = contextSnippetPayload(snippets);
      }
      if (send(payload)) {
        setStreaming(true);
        setActiveTurnCost(0);
        const optimisticNode = appendMessage("user", text, images, snippets);
        beginStream();
        trackOptimisticRequest(requestId, {
          node: optimisticNode,
          text,
          images,
          contextSnippets: snippets,
          chatId: getActiveChatId(),
        });
        if (prompt) {
          prompt.value = "";
        }
        clearAttachments();
        resizePrompt();
      } else {
        deleteOptimisticRequest(requestId);
      }
    });

    sendButton?.addEventListener("click", (event) => {
      if (!isChatStreaming()) {
        return;
      }
      event.preventDefault();
      requestCancel();
    });

    prompt?.addEventListener("keydown", (event) => {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "v") {
        window.setTimeout(requestNativePastedImage, 0);
        return;
      }
      if (commandPopover && commandPopover.hidden === false) {
        if (event.key === "ArrowDown") {
          event.preventDefault();
          commandPalette.moveSelection?.(1);
          return;
        }
        if (event.key === "ArrowUp") {
          event.preventDefault();
          commandPalette.moveSelection?.(-1);
          return;
        }
        if (event.key === "Escape") {
          event.preventDefault();
          commandPalette.close?.();
          return;
        }
        if (event.key === "Enter" && !event.shiftKey && !event.ctrlKey && !event.altKey && !event.metaKey) {
          const button = commandPalette.selectedButton?.();
          if (button) {
            event.preventDefault();
            commandPalette.run?.(button.dataset.commandOption || "");
            return;
          }
        }
      }
      if (event.key !== "Enter" || event.shiftKey || event.ctrlKey || event.altKey || event.metaKey) {
        return;
      }
      event.preventDefault();
      composer?.requestSubmit();
    });

    prompt?.addEventListener("input", () => {
      resizePrompt();
      commandPalette.update?.();
    });

    prompt?.addEventListener("paste", (event) => {
      const directFiles = Array.from(event.clipboardData?.files || []);
      const itemFiles = Array.from(event.clipboardData?.items || [])
        .filter((item) => item.kind === "file")
        .map((item) => item.getAsFile())
        .filter(Boolean);
      const files = [...directFiles, ...itemFiles];
      if (files.length > 0) {
        addImageFiles(files).then((added) => {
          if (added === 0) {
            requestNativePastedImage();
          }
        });
      } else {
        requestNativePastedImage();
      }
      window.setTimeout(resizePrompt, 0);
    });

    function requestCancel() {
      const activeChatId = getActiveChatId();
      if (!activeChatId) {
        return;
      }
      appendSystem("Cancelling...");
      const cancelSocket = new WebSocket(chatWsUrl());
      cancelSocket.addEventListener("open", () => {
        cancelSocket.send(JSON.stringify({
          type: "cancel_chat",
          request_id: nextRequestId("cancel-chat"),
          chat_id: activeChatId,
        }));
        window.setTimeout(() => cancelSocket.close(), 120);
      });
      cancelSocket.addEventListener("error", () => {
        appendSystem("Cancel request failed.");
      });
    }

    function requestToolApprovalReview(approvalId, decision) {
      const cleanDecision = ["approved", "denied", "cancelled"].includes(decision) ? decision : "denied";
      if (!approvalId) {
        return;
      }
      appendSystem(cleanDecision === "approved" ? "Approving tool call..." : "Denying tool call...");
      const reviewSocket = new WebSocket(chatWsUrl());
      reviewSocket.addEventListener("open", () => {
        reviewSocket.send(JSON.stringify({
          type: "review_tool_approval",
          request_id: nextRequestId("tool-approval"),
          approval_id: approvalId,
          decision: cleanDecision,
        }));
        window.setTimeout(() => reviewSocket.close(), 160);
      });
      reviewSocket.addEventListener("error", () => {
        appendSystem("Approval response failed.");
      });
    }

    return {
      requestCancel,
      requestToolApprovalReview,
    };
  }

  window.FennaraComposerActions = {
    createComposerActions,
  };
})();
