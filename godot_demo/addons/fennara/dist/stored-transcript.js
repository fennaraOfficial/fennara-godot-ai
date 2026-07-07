(function () {
  const terminalToolStatuses = new Set(["done", "failed", "timed_out", "cancelled", "denied"]);

  function createStoredTranscript(options = {}) {
    const transcriptRenderer = options.transcriptRenderer || null;
    const supportedImageTypes = options.supportedImageTypes || new Set();
    const callbacks = options.callbacks || {};
    const clearTranscript = callbacks.clearTranscript || function () {};
    const updateChatSize = callbacks.updateChatSize || function () {};
    const setLatestPromptTokens = callbacks.setLatestPromptTokens || function () {};
    const usagePromptTokens = callbacks.usagePromptTokens || (() => 0);
    const usageCost = callbacks.usageCost || (() => 0);
    const formatUsageCost = callbacks.formatUsageCost || ((cost) => String(cost || ""));
    const normalizeContextSnippet = callbacks.normalizeContextSnippet || ((snippet) => snippet);
    const optimisticUserMessageRequests = new Map();

    function appendMessage(role, text, attachments = [], contextSnippets = []) {
      return transcriptRenderer?.appendMessage(role, text, attachments, contextSnippets);
    }

    function appendDaemonUserMessage(userMessage, requestId = "") {
      const node = appendMessage(
        "user",
        userMessage.content || "",
        imagesFromMetadata(userMessage.metadata_json),
        contextSnippetsFromMetadata(userMessage.metadata_json),
      );
      const optimistic = optimisticUserMessageRequests.get(String(requestId || ""));
      if (optimistic) {
        optimistic.node = node;
      }
      return node;
    }

    function hasConnectedOptimisticUserMessage(requestId) {
      const optimistic = optimisticUserMessageRequests.get(String(requestId || ""));
      return Boolean(optimistic?.node?.isConnected);
    }

    function restorePendingOptimisticUserMessages(chatId) {
      for (const optimistic of optimisticUserMessageRequests.values()) {
        if (optimistic.node?.isConnected) {
          continue;
        }
        if (optimistic.chatId && chatId && optimistic.chatId !== chatId) {
          continue;
        }
        optimistic.node = appendMessage(
          "user",
          optimistic.text || "",
          optimistic.images || [],
          optimistic.contextSnippets || [],
        );
      }
    }

    function renderStoredMessages(messages, contextCompactions = []) {
      clearTranscript(false);
      const compactionMarkers = normalizeContextCompactions(contextCompactions);
      let compactionMarkerIndex = 0;
      let pendingHiddenAssistantCost = 0;
      let storedPromptTokens = 0;
      for (const message of messages || []) {
        if (message.role === "user") {
          pendingHiddenAssistantCost = 0;
        }
        const storedUsage = parseUsage(message.usage_json);
        const promptTokens = usagePromptTokens(storedUsage);
        if (promptTokens > 0) {
          storedPromptTokens = promptTokens;
        }
        if (message.role === "assistant" && message.reasoning_content) {
          appendStoredThinking(message.reasoning_content);
        }
        if (message.role === "tool") {
          appendStoredTool(message);
          appendDueContextCompactions(message);
          continue;
        }
        if (isStoredToolCallAssistant(message)) {
          pendingHiddenAssistantCost += storedMessageCost(message);
          appendDueContextCompactions(message);
          continue;
        }
        const node = appendMessage(
          message.role,
          message.content || "",
          imagesFromMetadata(message.metadata_json),
          contextSnippetsFromMetadata(message.metadata_json),
        );
        if (hasStoredToolCalls(message)) {
          pendingHiddenAssistantCost += storedMessageCost(message);
          continue;
        }
        if (message.role === "assistant" && shouldShowStoredAssistantActions(message)) {
          const usage = parseUsage(message.usage_json) || { cost: message.cost };
          const visibleCost = usageCost(usage);
          const combinedCost = pendingHiddenAssistantCost + (Number.isFinite(visibleCost) ? visibleCost : 0);
          transcriptRenderer?.addActionsToMessage(
            node,
            combinedCost > 0 ? { ...usage, cost: combinedCost } : usage,
            formatUsageCost,
          );
          pendingHiddenAssistantCost = 0;
        }
        appendDueContextCompactions(message);
      }
      appendRemainingContextCompactions();
      if (storedPromptTokens > 0) {
        setLatestPromptTokens(storedPromptTokens);
        updateChatSize();
      }

      function appendDueContextCompactions(message) {
        while (
          compactionMarkerIndex < compactionMarkers.length &&
          contextCompactionBelongsAfterMessage(compactionMarkers[compactionMarkerIndex], message)
        ) {
          appendStoredContextCompaction();
          compactionMarkerIndex += 1;
        }
      }

      function appendRemainingContextCompactions() {
        while (compactionMarkerIndex < compactionMarkers.length) {
          appendStoredContextCompaction();
          compactionMarkerIndex += 1;
        }
      }
    }

    function isStoredToolCallAssistant(message) {
      return message.role === "assistant" &&
        !(message.content || "").trim() &&
        Boolean(message.tool_calls_json);
    }

    function hasStoredToolCalls(message) {
      return message.role === "assistant" && Boolean(message.tool_calls_json);
    }

    function shouldShowStoredAssistantActions(message) {
      return Boolean((message.content || "").trim()) || usageCost(parseUsage(message.usage_json)) > 0 || Number(message.cost) > 0;
    }

    function storedMessageCost(message) {
      const usage = parseUsage(message.usage_json);
      const cost = usageCost(usage) || Number(message.cost);
      return Number.isFinite(cost) && cost > 0 ? cost : 0;
    }

    function appendStoredTool(message) {
      const id = message.tool_call_id || message.id;
      const name = message.tool_name || "tool";
      const status = terminalToolStatuses.has(message.status) ? message.status : "done";
      transcriptRenderer?.updateToolCall({
        id,
        name,
        status,
        content: message.content || "",
        images: toolImagesFromMetadata(message.metadata_json),
      });
    }

    function appendStoredContextCompaction() {
      transcriptRenderer?.updateContextCompaction("done");
    }

    function normalizeContextCompactions(compactions) {
      return (Array.isArray(compactions) ? compactions : [])
        .map((item) => ({
          id: String(item?.id || ""),
          createdAtMs: positiveNumber(item?.created_at_ms),
          coveredEndSequence: positiveNumber(item?.covered_end_sequence),
        }))
        .filter((item) => item.id || item.createdAtMs > 0 || item.coveredEndSequence > 0)
        .sort((left, right) =>
          (left.createdAtMs - right.createdAtMs) ||
          (left.coveredEndSequence - right.coveredEndSequence) ||
          left.id.localeCompare(right.id)
        );
    }

    function contextCompactionBelongsAfterMessage(marker, message) {
      const messageCreatedAtMs = positiveNumber(message?.created_at_ms);
      if (marker.createdAtMs > 0 && messageCreatedAtMs > 0) {
        return messageCreatedAtMs >= marker.createdAtMs;
      }
      const sequence = positiveNumber(message?.sequence);
      return marker.coveredEndSequence > 0 && sequence >= marker.coveredEndSequence;
    }

    function positiveNumber(value) {
      const number = Number(value);
      return Number.isFinite(number) && number > 0 ? number : 0;
    }

    function imagesFromMetadata(raw) {
      if (!raw) {
        return [];
      }
      try {
        const metadata = typeof raw === "string" ? JSON.parse(raw) : raw;
        const images = Array.isArray(metadata?.images) ? metadata.images : [];
        return images.filter((image) =>
          image &&
          supportedImageTypes.has(String(image.mime_type || "").toLowerCase()) &&
          typeof image.base64 === "string" &&
          image.base64.length > 0
        );
      } catch {
        return [];
      }
    }

    function toolImagesFromMetadata(raw) {
      if (!raw) {
        return [];
      }
      try {
        const metadata = typeof raw === "string" ? JSON.parse(raw) : raw;
        const images = Array.isArray(metadata?.tool_images) ? metadata.tool_images : [];
        return images.filter((image) =>
          image &&
          supportedImageTypes.has(String(image.mime_type || "").toLowerCase()) &&
          typeof image.url === "string" &&
          image.url.length > 0
        );
      } catch {
        return [];
      }
    }

    function contextSnippetsFromMetadata(raw) {
      if (!raw) {
        return [];
      }
      try {
        const metadata = typeof raw === "string" ? JSON.parse(raw) : raw;
        const snippets = Array.isArray(metadata?.context_snippets) ? metadata.context_snippets : [];
        return snippets
          .map((snippet) => normalizeContextSnippet(snippet, { keepId: true }))
          .filter(Boolean);
      } catch {
        return [];
      }
    }

    function appendStoredThinking(text) {
      transcriptRenderer?.appendStoredThinking(text);
    }

    function parseUsage(raw) {
      if (!raw) {
        return null;
      }
      try {
        return JSON.parse(raw);
      } catch {
        return null;
      }
    }

    function trackOptimisticRequest(requestId, value) {
      optimisticUserMessageRequests.set(String(requestId || ""), value);
    }

    function deleteOptimisticRequest(requestId) {
      optimisticUserMessageRequests.delete(String(requestId || ""));
    }

    return {
      appendDaemonUserMessage,
      appendMessage,
      deleteOptimisticRequest,
      hasConnectedOptimisticUserMessage,
      renderStoredMessages,
      restorePendingOptimisticUserMessages,
      trackOptimisticRequest,
    };
  }

  window.FennaraStoredTranscript = {
    createStoredTranscript,
  };
})();
