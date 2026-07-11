(function () {
  function createChatNavigation(options = {}) {
    const appShell = options.appShell || null;
    const chatList = options.chatList || null;
    const chatTitle = options.chatTitle || null;
    const prompt = options.prompt || null;
    const modelInput = options.modelInput || null;
    const send = options.send || (() => false);
    const nextRequestId = options.nextRequestId || ((prefix) => prefix);
    const clearTranscript = options.clearTranscript || function () {};
    const clearAttachments = options.clearAttachments || function () {};
    const resizePrompt = options.resizePrompt || function () {};
    const cleanModelId = options.cleanModelId || ((value) => String(value || "").trim());
    const getActiveChatId = options.getActiveChatId || (() => null);
    const getCurrentModel = options.getCurrentModel || (() => "");
    const getCurrentReasoningEffort = options.getCurrentReasoningEffort || (() => "medium");

    function updateChatTitle(chat) {
      if (!chatTitle) {
        return;
      }
      chatTitle.textContent = chat?.title || "Scene Diagnostics";
    }

    function showPendingChatTitle(text) {
      if (!chatTitle || chatTitle.textContent.trim() !== "New chat") {
        return;
      }
      chatTitle.textContent = titleFromPrompt(text);
    }

    function renderChatList(chats) {
      if (!chatList) {
        return;
      }
      const heading = chatList.querySelector("h2") || document.createElement("h2");
      heading.textContent = "Chats";
      chatList.replaceChildren(heading);
      for (const chat of chats || []) {
        const row = document.createElement("button");
        row.className = "chat-row";
        row.classList.toggle("active", chat.id === getActiveChatId());
        row.type = "button";
        row.dataset.chatId = chat.id;
        row.innerHTML = [
          '<svg class="svg-icon" viewBox="0 0 24 24" aria-hidden="true">',
          '<path d="M12 3l1.8 5.2L19 10l-5.2 1.8L12 17l-1.8-5.2L5 10l5.2-1.8Z"></path>',
          "</svg>",
          "<span></span>",
          "<time></time>",
        ].join("");
        row.querySelector("span").textContent = chat.title || "New chat";
        row.querySelector("time").textContent = formatChatTime(chat.updated_at_ms);
        row.addEventListener("click", () => {
          send({
            type: "open_chat",
            request_id: nextRequestId("open-chat"),
            chat_id: chat.id,
          });
          appShell?.classList.remove("drawer-open");
        });
        chatList.append(row);
      }
    }

    function formatChatTime(timestampMs) {
      const deltaMs = Date.now() - Number(timestampMs || 0);
      if (!Number.isFinite(deltaMs) || deltaMs < 0) {
        return "now";
      }
      const minutes = Math.floor(deltaMs / 60000);
      if (minutes < 1) {
        return "now";
      }
      if (minutes < 60) {
        return minutes + "m";
      }
      const hours = Math.floor(minutes / 60);
      if (hours < 24) {
        return hours + "h";
      }
      return Math.floor(hours / 24) + "d";
    }

    function toggleDrawer() {
      appShell?.classList.toggle("drawer-open");
    }

    function closeDrawer() {
      appShell?.classList.remove("drawer-open");
    }

    function closeDrawerFromOutsideClick(event) {
      if (!appShell?.classList.contains("drawer-open")) {
        return;
      }
      if (event.target.closest("[data-chat-drawer]") || event.target.closest("[data-toggle-drawer]")) {
        return;
      }
      closeDrawer();
    }

    function startNewChat() {
      closeDrawer();
      clearTranscript(true);
      send({
        type: "new_chat",
        request_id: nextRequestId("new-chat"),
        model: cleanModelId(modelInput?.value || getCurrentModel()),
        reasoning_effort: getCurrentReasoningEffort(),
      });
      if (prompt) {
        prompt.value = "";
      }
      clearAttachments();
      resizePrompt();
      prompt?.focus();
    }

    return {
      closeDrawer,
      closeDrawerFromOutsideClick,
      renderChatList,
      showPendingChatTitle,
      startNewChat,
      toggleDrawer,
      updateChatTitle,
    };
  }

  function titleFromPrompt(text) {
    const title = String(text || "").trim().split(/\s+/).filter(Boolean).slice(0, 8).join(" ");
    if (Array.from(title).length > 60) {
      return Array.from(title).slice(0, 57).join("") + "...";
    }
    return title || "New chat";
  }

  window.FennaraChatNavigation = {
    createChatNavigation,
  };
})();
