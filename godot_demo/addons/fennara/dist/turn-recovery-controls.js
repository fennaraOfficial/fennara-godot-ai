(function () {
  const MAX_RENDERED_RECOVERY_PATHS = 100;

  function recoveryNow() {
    return typeof performance !== "undefined" ? performance.now() : Date.now();
  }

  function recoveryLog(event, details = {}) {
    const fields = Object.entries(details)
      .filter(([, value]) => value !== undefined && value !== null && value !== "")
      .map(([key, value]) => `${key}=${String(value)}`)
      .join(" ");
    const message = `[Fennara recovery] ${event}${fields ? ` ${fields}` : ""}`;
    console.log(message);
    try {
      const result = window.__fennaraRecoveryLog?.(message);
      result?.catch?.(() => {});
    } catch {
      // Browser chat and unsupported webviews keep the console fallback.
    }
  }

  function createTurnRecoveryControls(options = {}) {
    const elements = options.elements || {};
    const callbacks = options.callbacks || {};
    const bar = elements.bar || null;
    const notice = elements.notice || null;
    const noticeText = elements.noticeText || null;
    const changedDetails = elements.changedDetails || null;
    const changedSummary = changedDetails?.querySelector("summary") || null;
    const changedList = elements.changedList || null;
    const skippedDetails = elements.skippedDetails || null;
    const skippedSummary = skippedDetails?.querySelector("summary") || null;
    const skippedList = elements.skippedList || null;
    const undoButton = elements.undoButton || null;
    const redoButton = elements.redoButton || null;
    const retryButton = elements.retryButton || null;
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
    let generationSettledAt = 0;

    undoButton?.addEventListener("click", () => {
      recoveryLog("control_clicked", { control: "undo", chat: getActiveChatId() });
      requestUndo("undo");
    });
    redoButton?.addEventListener("click", () => {
      recoveryLog("control_clicked", { control: "redo", chat: getActiveChatId() });
      requestRecovery("redo", "redo");
    });
    retryButton?.addEventListener("click", retryTurn);
    resumeButton?.addEventListener("click", () => {
      recoveryLog("control_clicked", { control: "resume", chat: getActiveChatId() });
      requestRecovery("resume", "resume");
    });
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

    function applyState(messages, nextRecovery, nextCheckpointWarning = "", sourceRequestId = "") {
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
      recoveryLog("state_applied", {
        request: sourceRequestId,
        can_undo: recovery.can_undo,
        can_redo: recovery.can_redo,
        operation: recovery.operation_state,
        changed: recovery.changed_paths.length,
        excluded: recovery.skipped_paths.length,
        since_generation_ms: generationSettledAt
          ? Math.round(recoveryNow() - generationSettledAt)
          : undefined,
      });
      render();
    }

    function requestUndo(intent) {
      if (!recovery.can_undo || !recovery.eligible_user_message_id) {
        return false;
      }
      return requestRecovery("undo", intent, false, retryPayload);
    }

    function retryTurn() {
      recoveryLog("control_clicked", { control: "retry", chat: getActiveChatId() });
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
      pending = { requestId, chatId, action, intent, payload, startedAt: recoveryNow() };
      render();
      if (!send(request)) {
        recoveryLog("request_not_sent", { action, intent, request: requestId });
        pending = null;
        render();
        return false;
      }
      recoveryLog("request_sent", { action, intent, request: requestId, force });
      return true;
    }

    function handleMessage(message) {
      if (message?.type !== "turn_recovery_result") {
        return false;
      }
      const completed = pending;
      recoveryLog("result_received", {
        action: message.result?.action || completed?.action,
        intent: completed?.intent,
        request: message.request_id,
        elapsed_ms: completed?.startedAt
          ? Math.round(recoveryNow() - completed.startedAt)
          : undefined,
        confirmation: Boolean(message.result?.confirmation_required),
      });
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
      } else if (completed?.intent === "undo" && payload) {
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
      recoveryLog("request_failed", {
        request: requestId,
        elapsed_ms: pending?.startedAt ? Math.round(recoveryNow() - pending.startedAt) : undefined,
        message: message.message || "Turn recovery failed.",
      });
      pending = null;
      confirmation = null;
      render();
      appendSystem(message.message || "Turn recovery failed.");
      refreshAuthoritativeChat();
      return true;
    }

    function handleDisconnect() {
      recoveryLog("socket_disconnected", { pending: pending?.action });
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
      recoveryLog("replacement_submitted", { chat: getActiveChatId() });
      render();
    }

    function handleStreamingChanged() {
      render();
    }

    function submitRetry(payload) {
      const startedAt = recoveryNow();
      const sent = submitRetryPayload(payload);
      recoveryLog("retry_submit_finished", {
        sent,
        elapsed_ms: Math.round(recoveryNow() - startedAt),
        images: payload?.images?.length || 0,
        context: payload?.contextSnippets?.length || 0,
      });
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
      const startedAt = recoveryNow();
      recoveryLog("draft_restore_started", {
        images: payload?.images?.length || 0,
        context: payload?.contextSnippets?.length || 0,
      });
      setDraftRestoring(true);
      render();
      try {
        await restoreDraftPayload(payload);
        recoveryLog("draft_restore_finished", {
          elapsed_ms: Math.round(recoveryNow() - startedAt),
        });
        appendSystem("Original prompt restored. Edit it, then send when ready.");
      } catch (error) {
        recoveryLog("draft_restore_failed", {
          elapsed_ms: Math.round(recoveryNow() - startedAt),
          message: error?.message || error,
        });
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

    function handleGenerationSettled(requestId) {
      generationSettledAt = recoveryNow();
      recoveryLog("generation_response_received", { request: requestId });
    }

    function handleRefreshRequested(reason, requestId) {
      recoveryLog("chat_refresh_sent", { reason, request: requestId });
    }

    function handleDebugEvent(message) {
      recoveryLog(`daemon_${message.event || "event"}`, {
        request: message.request_id,
        duration_ms: message.duration_ms,
        ok: message.ok,
        coverage: message.coverage,
        unavailable_reason: message.unavailable_reason,
      });
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
      toggleAction(resumeButton, applying, disabled);
      renderNotice(applying);
      bar?.setAttribute("aria-busy", String(Boolean(pending) || draftRestoring));
    }

    function renderNotice(applying) {
      const changed = Array.from(recovery.changed_paths || []);
      const skipped = Array.from(recovery.skipped_paths || []);
      let text = "";
      if (applying) {
        text = "Recovery was interrupted. Resume it before starting another turn.";
      } else if (recovery.can_redo) {
        if (recovery.coverage === "conversation_only") {
          text = "Turn undone in chat only. Project files were not restored. Redo remains available until you send a replacement prompt.";
        } else if (recovery.coverage === "partial") {
          text = `Turn undone. ${changed.length} changed file${changed.length === 1 ? " was" : "s were"} restored; excluded paths were left unchanged. Redo remains available.`;
        } else {
          text = `Turn undone. ${changed.length} changed file${changed.length === 1 ? " was" : "s were"} restored. Redo remains available.`;
        }
      } else if (checkpointWarning) {
        text = `Undo is unavailable for this turn: ${checkpointWarning}`;
      } else if (recovery.coverage === "conversation_only") {
        text = "Chat-only recovery. Project files cannot be restored for this turn.";
      } else if (recovery.coverage === "partial") {
        text = `Undo can restore ${changed.length} changed file${changed.length === 1 ? "" : "s"}. ${skipped.length} excluded path${skipped.length === 1 ? "" : "s"} won't be changed.`;
      } else if (recovery.can_undo && changed.length > 0) {
        text = `Undo can restore ${changed.length} changed file${changed.length === 1 ? "" : "s"}.`;
      }
      if (notice) {
        notice.hidden = !text && changed.length === 0 && skipped.length === 0;
      }
      if (noticeText) {
        noticeText.textContent = text;
      }
      if (changedDetails) {
        changedDetails.hidden = changed.length === 0;
      }
      if (changedSummary) {
        changedSummary.textContent = `Show ${changed.length} changed file${changed.length === 1 ? "" : "s"}`;
      }
      if (changedList) {
        const visibleChanged = changed.slice(0, MAX_RENDERED_RECOVERY_PATHS);
        changedList.replaceChildren(
          ...visibleChanged.map((path) => pathListItem(path)),
          ...remainingPathItems(changed.length - visibleChanged.length),
        );
      }
      if (skippedDetails) {
        skippedDetails.hidden = skipped.length === 0;
      }
      if (skippedSummary) {
        skippedSummary.textContent = `Show ${skipped.length} affected path${skipped.length === 1 ? "" : "s"}`;
      }
      if (skippedList) {
        const visibleSkipped = skipped.slice(0, MAX_RENDERED_RECOVERY_PATHS);
        skippedList.replaceChildren(
          ...skippedPathGroups(visibleSkipped),
          ...remainingPathItems(skipped.length - visibleSkipped.length),
        );
      }
    }

    return {
      applyState,
      handleDisconnect,
      handleDebugEvent,
      handleError,
      handleGenerationSettled,
      handleMessage,
      handleRefreshRequested,
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
      changed_paths: [],
      skipped_paths: [],
      operation_state: null,
      rewound_user_message: null,
    };
  }

  function normalizeRecovery(value) {
    return {
      ...unavailableRecovery(),
      ...(value || {}),
      changed_paths: Array.isArray(value?.changed_paths) ? value.changed_paths : [],
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

  function skippedPathGroups(skippedPaths) {
    const groups = new Map();
    for (const item of skippedPaths) {
      const reason = String(item?.reason || "unknown");
      const paths = groups.get(reason) || [];
      paths.push(String(item?.path || "unknown path"));
      groups.set(reason, paths);
    }
    return Array.from(groups, ([reason, paths]) => {
      const group = document.createElement("li");
      group.className = "turn-recovery-path-group";
      const heading = document.createElement("div");
      heading.className = "turn-recovery-path-group-title";
      heading.textContent = `${paths.length} ${skippedReasonLabel(reason, paths.length)}`;
      const list = document.createElement("ul");
      list.className = "turn-recovery-path-group-items";
      list.replaceChildren(...paths.map((path) => pathListItem(path)));
      group.append(heading, list);
      return group;
    });
  }

  function skippedReasonLabel(reason, count) {
    const singular = count === 1;
    const labels = {
      ignored_path: singular ? "Git-ignored path" : "Git-ignored paths",
      large_untracked_file: singular ? "large untracked file" : "large untracked files",
      nested_git_repository: singular ? "nested Git repository" : "nested Git repositories",
      untracked_byte_budget_exceeded: singular ? "path beyond the checkpoint size limit" : "paths beyond the checkpoint size limit",
      unverified_lfs_object: singular ? "unverified Git LFS file" : "unverified Git LFS files",
      unverified_content_filter: singular ? "file with an unsupported Git filter" : "files with unsupported Git filters",
    };
    return labels[reason] || String(reason).replace(/_/g, " ");
  }

  function remainingPathItems(count) {
    if (count <= 0) {
      return [];
    }
    const item = document.createElement("li");
    item.className = "turn-recovery-path-group-title";
    item.textContent = `${count} more path${count === 1 ? "" : "s"} not shown`;
    return [item];
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
