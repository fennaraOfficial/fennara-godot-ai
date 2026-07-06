(function () {
  function createCommandPalette(options = {}) {
    const prompt = options.prompt || null;
    const commandPopover = options.commandPopover || null;
    const commandOptionButtons = Array.from(options.commandOptionButtons || []);
    const ensureDaemonConnected = options.ensureDaemonConnected || (() => true);
    const resizePrompt = options.resizePrompt || function () {};
    const openProviderPicker = options.openProviderPicker || function () {};
    const openModelPicker = options.openModelPicker || function () {};
    let activeCommandIndex = 0;
    let activeCommandRange = null;
    let lastCommandQuery = "";
    let visibleCommandList = [];

    commandOptionButtons.forEach((button) => {
      button.addEventListener("click", (event) => {
        event.stopPropagation();
        runCommand(button.dataset.commandOption || "");
      });
    });

    function commandMatch() {
      const value = prompt?.value || "";
      const cursor = prompt?.selectionStart ?? value.length;
      const beforeCursor = value.slice(0, cursor);
      const match = beforeCursor.match(/(^|\s)(\/[^\s]*)$/);
      if (!match) {
        return null;
      }
      const token = match[2] || "";
      return {
        query: token.toLowerCase(),
        start: cursor - token.length,
        end: cursor,
      };
    }

    function commandQuery() {
      const match = commandMatch();
      activeCommandRange = match ? { start: match.start, end: match.end } : null;
      return match?.query || "";
    }

    function openCommandPalette() {
      if (!commandPopover || !prompt) {
        return false;
      }
      commandPopover.hidden = false;
      renderCommandPalette();
      positionCommandPalette();
      return true;
    }

    function closeCommandPalette() {
      if (!commandPopover || commandPopover.hidden) {
        return;
      }
      commandPopover.hidden = true;
      activeCommandIndex = 0;
      visibleCommandList = [];
      lastCommandQuery = "";
    }

    function updateCommandPalette() {
      if (!prompt || !commandPopover) {
        return;
      }
      if (commandQuery()) {
        openCommandPalette();
      } else {
        closeCommandPalette();
      }
    }

    function renderCommandPalette() {
      const query = commandQuery();
      if (query !== lastCommandQuery) {
        activeCommandIndex = 0;
        lastCommandQuery = query;
      }
      const ranked = commandOptionButtons
        .map((button, index) => ({
          button,
          index,
          command: String(button.dataset.commandOption || "").toLowerCase(),
          title: String(button.querySelector("span")?.textContent || "").toLowerCase(),
        }))
        .map((item) => ({
          ...item,
          score: commandScore(item.command, item.title, query),
        }))
        .filter((item) => item.score < 99)
        .sort((a, b) => a.score - b.score || a.index - b.index);
      visibleCommandList = ranked.map((item) => item.button);
      const palette = commandPopover?.querySelector(".command-palette");
      visibleCommandList.forEach((button) => {
        button.hidden = false;
        palette?.appendChild(button);
      });
      commandOptionButtons.forEach((button) => {
        if (!visibleCommandList.includes(button)) {
          button.hidden = true;
        }
      });
      if (activeCommandIndex >= visibleCommandList.length) {
        activeCommandIndex = 0;
      }
      visibleCommandList.forEach((button, index) => {
        button.setAttribute("aria-selected", String(index === activeCommandIndex));
      });
      commandPopover.hidden = visibleCommandList.length === 0;
    }

    function commandScore(command, title, query) {
      if (!query) {
        return 99;
      }
      const needle = query.replace(/^\//, "");
      const trigger = command.replace(/^\//, "");
      if (!needle) {
        return 0;
      }
      if (trigger === needle) {
        return 0;
      }
      if (trigger.startsWith(needle)) {
        return 1;
      }
      if (title.startsWith(needle)) {
        return 2;
      }
      if (trigger.includes(needle) || title.includes(needle)) {
        return 3;
      }
      return 99;
    }

    function positionCommandPalette() {
      if (!commandPopover) {
        return;
      }
      const anchor = document.querySelector(".composer-card") || prompt;
      if (!anchor) {
        return;
      }
      const gap = 8;
      const margin = 10;
      const anchorRect = anchor.getBoundingClientRect();
      const width = Math.min(360, Math.max(280, window.innerWidth - margin * 2));
      const height = commandPopover.offsetHeight || 150;
      const left = Math.min(
        Math.max(margin, anchorRect.left),
        window.innerWidth - width - margin,
      );
      const top = Math.max(margin, anchorRect.top - gap - height);
      commandPopover.style.width = width + "px";
      commandPopover.style.left = left + "px";
      commandPopover.style.top = top + "px";
    }

    function visibleCommandButtons() {
      return visibleCommandList.filter((button) => !button.hidden);
    }

    function selectedButton() {
      return visibleCommandButtons()[activeCommandIndex] || null;
    }

    function moveCommandSelection(delta) {
      const buttons = visibleCommandButtons();
      if (!buttons.length) {
        return;
      }
      activeCommandIndex = (activeCommandIndex + delta + buttons.length) % buttons.length;
      renderCommandPalette();
    }

    function runCommand(command) {
      const clean = String(command || "").trim().toLowerCase();
      if (!clean) {
        return;
      }
      if ((clean === "/provider" || clean === "/model") && !ensureDaemonConnected()) {
        return;
      }
      if (prompt && activeCommandRange) {
        const value = prompt.value || "";
        prompt.value = value.slice(0, activeCommandRange.start) + value.slice(activeCommandRange.end);
        prompt.selectionStart = activeCommandRange.start;
        prompt.selectionEnd = activeCommandRange.start;
        resizePrompt();
      }
      activeCommandRange = null;
      closeCommandPalette();
      if (clean === "/provider") {
        openProviderPicker();
      } else if (clean === "/model") {
        openModelPicker(true);
      }
    }

    return {
      close: closeCommandPalette,
      moveSelection: moveCommandSelection,
      position: positionCommandPalette,
      run: runCommand,
      selectedButton,
      update: updateCommandPalette,
    };
  }

  window.FennaraCommandPalette = {
    createCommandPalette,
  };
})();
