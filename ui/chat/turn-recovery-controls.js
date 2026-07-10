(function () {
  function createTurnRecoveryControls(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const bar = elements.bar || null;
    const notice = elements.notice || null;
    const noticeText = elements.noticeText || null;
    const skippedDetails = elements.skippedDetails || null;
    const skippedList = elements.skippedList || null;
    const undoButton = elements.undoButton || null;
    const redoButton = elements.redoButton || null;
    const retryButton = elements.retryButton || null;
    const editRetryButton = elements.editRetryButton || null;
    const resumeButton = elements.resumeButton || null;
    const confirmDialog = elements.confirmDialog || null;
    const confirmText = elements.confirmText || null;
    const conflictList = elements.conflictList || null;
    const send = callbacks.send || (() => false);
    const nextRequestId = callbacks.nextRequestId || (() => "turn-recovery");
    const getActiveChatId = callbacks.getActiveChatId || (() => null);
    const isChatStreaming = callbacks.isChatStreaming || (() => false);
    const applyAuthoritativeChat = callbacks.applyAuthoritativeChat || function () {};
    const submitRetryPayload = callbacks.submitRetryPayload || (() => false);
    const restoreDraftPayload = callbacks.restoreDraftPayload || (() => Promise.resolve());
    const setDraftRestoring = callbacks.setDraftRestoring || function () {};
    const refreshAuthoritativeChat = callbacks.refreshAuthoritativeChat || function () {};
    const appendSystem = callbacks.appendSystem || function () {};

    let recovery = unavailableRecovery();
    let retryPayload = null;
    let pending = null;
    let confirmation = null;
    let replacementPending = false;
    let checkpointWarning = "";
    let draftRestoring = false;

    undoButton?.addEventListener("click", () => requestUndo("undo"));
    redoButton?.addEventListener("click", () => requestRecovery("redo", "redo"));
    retryButton?.addEventListener("click", retryTurn);
    editRetryButton?.addEventListener("click", editAndRetry);
    resumeButton?.addEventListener("click", () => requestRecovery("resume", "resume"));
    confirmDialog?.addEventListener("close", () => {
      const accepted = confirmDialog.returnValue === "confirm";
      const next = confirmation;
      confirmation = null;
      if (accepted && next) {
        requestRecovery(next.action, next.intent, true, next.payload);
      } else {
        render();
      }
    });

    function applyState(messages, nextRecovery, nextCheckpointWarning = "") {
      recovery = normalizeRecovery(nextRecovery);
      checkpointWarning = String(nextCheckpointWarning || "");
      replacementPending = false;
      const source = recovery.rewound_user_message
        || (messages || []).find((message) => message?.id === recovery.eligible_user_message_id);
      if (source) {
        retryPayload = payloadFromMessage(source);
      } else if (!recovery.can_redo && !isApplying(recovery.operation_state)) {
        retryPayload = null;
      }
      render();
    }

    function requestUndo(intent) {
      if (!recovery.can_undo || !recovery.eligible_user_message_id) {
        return false;
      }
      return requestRecovery("undo", intent, false, retryPayload);
    }

    function retryTurn() {
      if (!retryPayload) {
        appendSystem("The original prompt payload is unavailable for Retry.");
        return false;
      }
      if (recovery.can_undo) {
        return requestUndo("retry");
      }
      if (recovery.can_redo) {
        return submitRetry(retryPayload);
      }
      return false;
    }

    function editAndRetry() {
      if (!retryPayload) {
        appendSystem("The original prompt payload is unavailable for Edit and retry.");
        return false;
      }
      if (recovery.can_undo) {
        return requestUndo("edit_retry");
      }
      if (recovery.can_redo) {
        restoreDraft(retryPayload);
        return true;
      }
      return false;
    }

    function requestRecovery(action, intent, force = false, payload = retryPayload) {
      if (pending || draftRestoring || isChatStreaming()) {
        return false;
      }
      const chatId = getActiveChatId();
      if (!chatId) {
        return false;
      }
      const requestId = nextRequestId("turn-recovery");
      const request = {
        type: requestType(action),
        request_id: requestId,
        chat_id: chatId,
      };
      if (action === "undo") {
        request.user_message_id = recovery.eligible_user_message_id;
      }
      if (force) {
        request.force = true;
      }
      pending = { requestId, chatId, action, intent, payload };
      render();
      if (!send(request)) {
        pending = null;
        render();
        return false;
      }
      return true;
    }

    function handleMessage(message) {
      if (message?.type !== "turn_recovery_result") {
        return false;
      }
      const completed = pending;
      const responseChatId = String(message.result?.chat_id || "");
      const activeChatId = String(getActiveChatId() || "");
      if (responseChatId && activeChatId && responseChatId !== activeChatId) {
        pending = null;
        render();
        appendSystem("Turn recovery completed in another chat.");
        return true;
      }
      if (message.result?.confirmation_required) {
        pending = null;
        confirmation = {
          action: completed?.action || message.result.action,
          intent: completed?.intent || message.result.action,
          payload: completed?.payload || retryPayload,
        };
        showConfirmation(message.result.conflicts || []);
        return true;
      }
      pending = null;
      if (message.chat) {
        applyAuthoritativeChat(message);
      } else {
        render();
      }
      if (message.editor_refresh?.ok === false) {
        appendSystem(
          `Project files were restored, but Godot did not refresh them: ${message.editor_refresh.error || "refresh failed"}`,
        );
      }
      const payload = completed?.payload || retryPayload;
      if (completed?.intent === "retry" && payload) {
        submitRetry(payload);
      } else if (completed?.intent === "edit_retry" && payload) {
        restoreDraft(payload);
      }
      return true;
    }

    function handleError(message) {
      if (message?.type !== "error") {
        return false;
      }
      const requestId = String(message.request_id || "");
      if (!requestId.startsWith("turn-recovery")) {
        return false;
      }
      pending = null;
      confirmation = null;
      render();
      appendSystem(message.message || "Turn recovery failed.");
      refreshAuthoritativeChat();
      return true;
    }

    function handleDisconnect() {
      pending = null;
      confirmation = null;
      if (confirmDialog?.open) {
        confirmDialog.close("cancel");
      }
      render();
    }

    function handleReplacementSubmitted() {
      if (!recovery.can_redo) {
        return;
      }
      replacementPending = true;
      render();
    }

    function handleStreamingChanged() {
      render();
    }

    function submitRetry(payload) {
      const sent = submitRetryPayload(payload);
      if (sent) {
        replacementPending = true;
        render();
        return true;
      }
      restoreDraft(payload);
      appendSystem("Undo completed. The original prompt was restored to the composer.");
      return false;
    }

    async function restoreDraft(payload) {
      if (draftRestoring) {
        return;
      }
      draftRestoring = true;
      setDraftRestoring(true);
      render();
      try {
        await restoreDraftPayload(payload);
        appendSystem("Original prompt restored. Edit it, then send when ready.");
      } catch (error) {
        appendSystem(`The original prompt is still recoverable, but its attachments could not be restored: ${error?.message || error}`);
      } finally {
        draftRestoring = false;
        setDraftRestoring(false);
        render();
      }
    }

    function showConfirmation(conflicts) {
      const cleanConflicts = Array.from(conflicts || []).map(String).filter(Boolean);
      if (confirmText) {
        confirmText.textContent = cleanConflicts.length === 1
          ? "This path changed after the turn. Restoring will overwrite the later change."
          : `${cleanConflicts.length} paths changed after the turn. Restoring will overwrite those later changes.`;
      }
      if (conflictList) {
        conflictList.replaceChildren(...cleanConflicts.map(pathListItem));
      }
      if (confirmDialog?.showModal) {
        confirmDialog.showModal();
      } else if (window.confirm(confirmText?.textContent || "Overwrite later changes?")) {
        const next = confirmation;
        confirmation = null;
        if (next) {
          requestRecovery(next.action, next.intent, true, next.payload);
        }
      } else {
        confirmation = null;
        render();
      }
    }

    function render() {
      const applying = isApplying(recovery.operation_state);
      const visible = !replacementPending
        && (recovery.can_undo || recovery.can_redo || applying || Boolean(checkpointWarning));
      if (bar) {
        bar.hidden = !visible;
      }
      if (!visible) {
        return;
      }
      const disabled = Boolean(pending) || draftRestoring || isChatStreaming();
      toggleAction(undoButton, recovery.can_undo, disabled);
      toggleAction(redoButton, recovery.can_redo, disabled);
      toggleAction(retryButton, (recovery.can_undo || recovery.can_redo) && Boolean(retryPayload), disabled);
      toggleAction(editRetryButton, (recovery.can_undo || recovery.can_redo) && Boolean(retryPayload), disabled);
      toggleAction(resumeButton, applying, disabled);
      renderNotice(applying);
      bar?.setAttribute("aria-busy", String(Boolean(pending) || draftRestoring));
    }

    function renderNotice(applying) {
      const skipped = Array.from(recovery.skipped_paths || []);
      let text = "";
      if (applying) {
        text = "Recovery was interrupted. Resume it before starting another turn.";
      } else if (recovery.can_redo) {
        if (recovery.coverage === "conversation_only") {
          text = "Turn undone in chat only. Project files were not restored. Redo remains available until you send a replacement prompt.";
        } else if (recovery.coverage === "partial") {
          text = "Turn undone with partial project coverage. Redo remains available until you send a replacement prompt.";
        } else {
          text = "Turn undone. Redo remains available until you send a replacement prompt.";
        }
      } else if (checkpointWarning) {
        text = `Undo is unavailable for this turn: ${checkpointWarning}`;
      } else if (recovery.coverage === "conversation_only") {
        text = "Chat-only recovery. Project files cannot be restored for this turn.";
      } else if (recovery.coverage === "partial") {
        text = `${skipped.length} project path${skipped.length === 1 ? " was" : "s were"} skipped by this checkpoint.`;
      }
      if (notice) {
        notice.hidden = !text && skipped.length === 0;
      }
      if (noticeText) {
        noticeText.textContent = text;
      }
      if (skippedDetails) {
        skippedDetails.hidden = skipped.length === 0;
      }
      if (skippedList) {
        skippedList.replaceChildren(...skipped.map((item) => pathListItem(item.path, item.reason)));
      }
    }

    return {
      applyState,
      handleDisconnect,
      handleError,
      handleMessage,
      handleReplacementSubmitted,
      handleStreamingChanged,
    };
  }

  function unavailableRecovery() {
    return {
      can_undo: false,
      can_redo: false,
      eligible_user_message_id: null,
      coverage: null,
      skipped_paths: [],
      operation_state: null,
      rewound_user_message: null,
    };
  }

  function normalizeRecovery(value) {
    return {
      ...unavailableRecovery(),
      ...(value || {}),
      skipped_paths: Array.isArray(value?.skipped_paths) ? value.skipped_paths : [],
    };
  }

  function payloadFromMessage(message) {
    if (!message) {
      return null;
    }
    let metadata = {};
    try {
      metadata = typeof message.metadata_json === "string"
        ? JSON.parse(message.metadata_json)
        : message.metadata_json || {};
    } catch {
      metadata = {};
    }
    const images = Array.isArray(metadata.images)
      ? metadata.images.map((image) => ({
        ...image,
        size: Number(image?.size || image?.size_bytes || 0),
      }))
      : [];
    const contextSnippets = Array.isArray(metadata.context_snippets)
      ? metadata.context_snippets.map((snippet) => ({ ...snippet }))
      : [];
    return {
      text: String(message.content || ""),
      images,
      contextSnippets,
    };
  }

  function requestType(action) {
    if (action === "undo") {
      return "undo_chat_turn";
    }
    if (action === "redo") {
      return "redo_chat_turn";
    }
    return "resume_chat_turn_recovery";
  }

  function isApplying(state) {
    return state === "applying_undo" || state === "applying_redo";
  }

  function toggleAction(button, visible, disabled) {
    if (!button) {
      return;
    }
    button.hidden = !visible;
    button.disabled = disabled;
  }

  function pathListItem(path, reason = "") {
    const item = document.createElement("li");
    const code = document.createElement("code");
    code.textContent = String(path || "unknown path");
    item.append(code);
    if (reason) {
      const detail = document.createElement("span");
      detail.textContent = String(reason).replace(/_/g, " ");
      item.append(detail);
    }
    return item;
  }

  window.FennaraTurnRecoveryControls = {
    createTurnRecoveryControls,
    payloadFromMessage,
  };
})();
