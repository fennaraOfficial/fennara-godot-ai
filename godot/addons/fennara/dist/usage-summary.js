(function () {
  function createUsageSummary(options = {}) {
    const chatSizeStatus = options.chatSizeStatus || null;
    const sessionCostStatus = options.sessionCostStatus || null;
    const usagePopover = options.usagePopover || null;
    const usageTotalCost = options.usageTotalCost || null;
    const usageContextStatus = options.usageContextStatus || null;
    const getSessionCost = options.getSessionCost || (() => 0);
    const getLatestPromptTokens = options.getLatestPromptTokens || (() => 0);
    const getCurrentModelInfo = options.getCurrentModelInfo || (() => null);
    let usageCloseTimer = 0;
    let latestContextUsage = {
      availableTokens: 0,
      remainingTokens: 0,
      usedTokens: 0,
      hasAvailable: false,
    };

    function formatUsageCost(usage) {
      const cost = usageCost(usage);
      if (!Number.isFinite(cost) || cost <= 0) {
        return "";
      }
      return formatCostValue(cost);
    }

    function usageCost(usage) {
      const rawCost = usage?.cost;
      return Number(rawCost);
    }

    function usagePromptTokens(usage) {
      const value =
        usage?.prompt_tokens ?? usage?.promptTokens ?? usage?.total_tokens ?? usage?.totalTokens;
      const tokens = Number(value);
      return Number.isFinite(tokens) && tokens > 0 ? tokens : 0;
    }

    function formatTokenCount(value) {
      const tokens = Number(value);
      if (!Number.isFinite(tokens) || tokens <= 0) {
        return "0";
      }
      if (tokens < 1000) {
        return String(Math.round(tokens));
      }
      if (tokens < 1000000) {
        return (tokens / 1000).toFixed(tokens < 10000 ? 1 : 0).replace(/\.0$/, "") + "k";
      }
      return (tokens / 1000000).toFixed(tokens < 10000000 ? 1 : 0).replace(/\.0$/, "") + "M";
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
        const tokens = Number(candidate);
        if (Number.isFinite(tokens) && tokens > 0) {
          return Math.floor(tokens);
        }
      }
      return 0;
    }

    function updateChatSize() {
      const availableTokens = modelContextLength(getCurrentModelInfo());
      const hasAvailable = Number.isFinite(availableTokens) && availableTokens > 0;
      const usedTokens = Number(getLatestPromptTokens() || 0);
      const remainingTokens = hasAvailable ? Math.max(availableTokens - usedTokens, 0) : 0;
      latestContextUsage = {
        availableTokens,
        remainingTokens,
        usedTokens,
        hasAvailable,
      };
      if (chatSizeStatus) {
        const usedText = formatTokenCount(usedTokens);
        const availableText = hasAvailable ? formatTokenCount(availableTokens) : "?";
        chatSizeStatus.textContent = `*${usedText} / ${availableText} tokens`;
        chatSizeStatus.title = hasAvailable ? `${formatTokenCount(remainingTokens)} tokens left` : "";
      }
      if (usageContextStatus) {
        usageContextStatus.textContent = hasAvailable
          ? `*${formatTokenCount(availableTokens)} tokens (${formatTokenCount(remainingTokens)} left)`
          : "*Unknown";
      }
      renderUsageBadge();
    }

    function updateSessionCost() {
      const sessionCost = Number(getSessionCost() || 0);
      if (usageTotalCost) {
        usageTotalCost.textContent = sessionCost > 0 ? "*" + formatCostValue(sessionCost) : "*$0.00";
      }
      renderUsageBadge();
    }

    function renderUsageBadge() {
      if (!sessionCostStatus) {
        return;
      }
      const sessionCost = Number(getSessionCost() || 0);
      const hasCost = Number.isFinite(sessionCost) && sessionCost > 0;
      const hasContext = Boolean(latestContextUsage.hasAvailable);
      sessionCostStatus.hidden = !hasCost && !hasContext;
      if (hasCost) {
        sessionCostStatus.textContent = "*" + formatCostValue(sessionCost);
        sessionCostStatus.title = hasContext
          ? `${formatTokenCount(latestContextUsage.usedTokens)} used, ${formatTokenCount(latestContextUsage.remainingTokens)} left`
          : "";
      } else if (hasContext) {
        const usedText = formatTokenCount(latestContextUsage.usedTokens);
        const availableText = formatTokenCount(latestContextUsage.availableTokens);
        sessionCostStatus.textContent = `*${usedText} / ${availableText} tokens`;
        sessionCostStatus.title = `${formatTokenCount(latestContextUsage.remainingTokens)} tokens left`;
      } else {
        sessionCostStatus.textContent = "";
        sessionCostStatus.title = "";
      }
      if (sessionCostStatus.hidden) {
        setUsagePopoverOpen(false);
      }
    }

    function positionUsagePopover() {
      if (!usagePopover || !sessionCostStatus || usagePopover.hidden) {
        return;
      }
      const margin = 12;
      const gap = 10;
      const anchor = sessionCostStatus.getBoundingClientRect();
      const width = usagePopover.offsetWidth;
      const height = usagePopover.offsetHeight;
      const maxLeft = Math.max(margin, window.innerWidth - width - margin);
      let left = anchor.left + anchor.width / 2 - width / 2;
      left = Math.min(Math.max(left, margin), maxLeft);
      let top = anchor.top - height - gap;
      if (top < margin) {
        top = Math.min(window.innerHeight - height - margin, anchor.bottom + gap);
      }
      usagePopover.style.setProperty("--usage-popover-left", `${Math.max(margin, left)}px`);
      usagePopover.style.setProperty("--usage-popover-top", `${Math.max(margin, top)}px`);
    }

    function setUsagePopoverOpen(open) {
      if (!usagePopover || !sessionCostStatus) {
        return;
      }
      const shouldOpen = Boolean(open) && !sessionCostStatus.hidden;
      usagePopover.hidden = !shouldOpen;
      sessionCostStatus.setAttribute("aria-expanded", shouldOpen ? "true" : "false");
      if (shouldOpen) {
        positionUsagePopover();
      }
    }

    function showUsagePopover() {
      window.clearTimeout(usageCloseTimer);
      setUsagePopoverOpen(true);
    }

    function hideUsagePopoverSoon() {
      window.clearTimeout(usageCloseTimer);
      usageCloseTimer = window.setTimeout(() => setUsagePopoverOpen(false), 90);
    }

    function formatCostValue(cost) {
      if (cost > 0 && cost < 0.0001) {
        return "$" + cost.toFixed(6);
      }
      if (cost < 0.01) {
        return "$" + cost.toFixed(4);
      }
      return "$" + cost.toFixed(2);
    }

    return {
      formatCostValue,
      formatUsageCost,
      hideUsagePopoverSoon,
      positionUsagePopover,
      setUsagePopoverOpen,
      showUsagePopover,
      updateChatSize,
      updateSessionCost,
      usageCost,
      usagePromptTokens,
    };
  }

  window.FennaraUsageSummary = {
    createUsageSummary,
  };
})();
