(function () {
  function defaultEscapeHtml(value) {
    return String(value)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function createProjectStatus(options = {}) {
    const targetMenu = options.targetMenu || null;
    const setMcpTargetButton = options.setMcpTargetButton || null;
    const targetPillText = options.targetPillText || null;
    const targetPopoverTitle = options.targetPopoverTitle || null;
    const targetPopoverText = options.targetPopoverText || null;
    const versionMenu = options.versionMenu || null;
    const versionWarning = options.versionWarning || null;
    const versionPopover = options.versionPopover || null;
    const versionWarningText = options.versionWarningText || null;
    const escapeHtml = options.escapeHtml || defaultEscapeHtml;

    function basename(path) {
      return String(path || "").split(/[\\/]/).filter(Boolean).pop() || "";
    }

    function applyProjectStatus(message) {
      const daemon = message.daemon || {};
      const boundSessionId = message.bound_session_id || "";
      const connectedProjects = Array.isArray(daemon.connected_projects) ? daemon.connected_projects : [];
      const boundProject =
        connectedProjects.find((project) => project.session_id === boundSessionId) ||
        daemon.active_project ||
        {};
      const activeProject = daemon.active_project || null;
      const isTarget = Boolean(daemon.active_session_id && daemon.active_session_id === boundSessionId);
      const targetName = activeProject?.project_name || basename(activeProject?.project_path) || "No MCP target";
      const boundName = boundProject?.project_name || basename(boundProject?.project_path) || "Godot project";

      if (targetMenu) {
        targetMenu.hidden = false;
      }
      if (setMcpTargetButton) {
        setMcpTargetButton.classList.toggle("is-target", isTarget);
        setMcpTargetButton.classList.remove("is-setting");
        setMcpTargetButton.classList.toggle("has-other-target", Boolean(activeProject) && !isTarget);
      }
      if (targetPillText) {
        targetPillText.textContent = isTarget ? "MCP target" : "Use for MCP";
      }
      if (targetPopoverTitle && targetPopoverText) {
        targetPopoverTitle.textContent = isTarget ? `${boundName} is the MCP target` : "Use this project for MCP";
        targetPopoverText.textContent = isTarget
          ? "External MCP clients send Godot tool calls here."
          : activeProject
            ? `Current target: ${targetName}. Click to switch MCP to this project.`
            : "No target is selected. Click to use this project.";
      }

      applyVersionWarning(message.version || {});
    }

    function applyVersionWarning(version) {
      const outdated = Boolean(version.outdated);
      if (!versionMenu || !versionWarning) {
        return;
      }
      versionMenu.hidden = !outdated;
      versionWarning.setAttribute("aria-expanded", "false");
      if (!outdated) {
        return;
      }
      const current = version.current_version || "installed";
      const latest = version.latest_version || "latest";
      if (versionWarningText) {
        versionWarningText.innerHTML = [
          `Current: ${escapeHtml(current)}`,
          `Available: ${escapeHtml(latest)}`,
          "",
          "The update will be verified before Godot asks to close.",
        ].join("<br>");
      }
      if (versionPopover) {
        versionPopover.hidden = false;
      }
    }

    return {
      applyProjectStatus,
      applyVersionWarning,
    };
  }

  window.FennaraProjectStatus = {
    createProjectStatus,
  };
})();
