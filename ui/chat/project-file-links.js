(function () {
  function createProjectFileLinks(options = {}) {
    const send = options.send || (() => false);
    const nextRequestId = options.nextRequestId || (() => "open-project-file");
    const appendSystem = options.appendSystem || function () {};
    const clearSystemStatus = options.clearSystemStatus || function () {};

    function handleProjectFileReference(rawReference) {
      const reference = typeof rawReference === "string"
        ? parseProjectFileReference(rawReference)
        : normalizeProjectFileReference(rawReference);
      if (!reference) {
        appendSystem("That project file link is not valid.");
        window.setTimeout(clearSystemStatus, 1600);
        return false;
      }
      const requestId = nextRequestId("open-project-file");
      if (!send({
        type: "open_project_file",
        request_id: requestId,
        path: reference.path,
        start_line: reference.start_line || null,
        end_line: reference.end_line || null,
      })) {
        appendSystem("Local daemon is not connected yet.");
        window.setTimeout(clearSystemStatus, 1800);
        return false;
      }
      return true;
    }

    function parseProjectFileReference(rawHref) {
      let href = String(rawHref || "").trim();
      try {
        href = decodeURI(href);
      } catch {
        // Keep the original string if it is not URI encoded cleanly.
      }
      if (!href.toLowerCase().startsWith("res://")) {
        return null;
      }
      href = href.replace(/[.,;!?]+$/, "");

      const parsed = splitProjectFileReference(href);
      if (!parsed) {
        return null;
      }
      return normalizeProjectFileReference({
        href,
        path: parsed.path,
        start_line: parsed.startLine,
        end_line: parsed.endLine,
      });
    }

    function splitProjectFileReference(href) {
      const hashMatch = href.match(/^(res:\/\/[^#]+)#L?(\d+)(?:-L?(\d+))?$/i);
      if (hashMatch) {
        const startLine = Number(hashMatch[2]);
        return {
          path: hashMatch[1],
          startLine,
          endLine: Number(hashMatch[3] || startLine),
        };
      }

      const colonMatch = href.match(/^(res:\/\/.*?)(?::L?(\d+)(?:-L?(\d+))?)?$/i);
      if (!colonMatch) {
        return null;
      }
      const startLine = Number(colonMatch[2] || 0);
      return {
        path: colonMatch[1],
        startLine,
        endLine: Number(colonMatch[3] || startLine || 0),
      };
    }

    function normalizeProjectFileReference(rawReference) {
      const path = String(rawReference?.path || "").trim().replace(/\\/g, "/");
      const startLine = Number(rawReference?.start_line || 0);
      const endLine = Number(rawReference?.end_line || startLine || 0);
      if (!path.toLowerCase().startsWith("res://")) {
        return null;
      }
      if ((startLine || endLine) && (!Number.isInteger(startLine) || !Number.isInteger(endLine) || startLine <= 0 || endLine < startLine)) {
        return null;
      }
      const lineSuffix = startLine > 0
        ? ":" + (endLine > startLine ? `${startLine}-${endLine}` : `${startLine}`)
        : "";
      const href = String(rawReference?.href || `${path}${lineSuffix}`);
      return {
        href,
        path,
        start_line: startLine,
        end_line: endLine,
        label: `${path}${lineSuffix}`,
      };
    }

    return {
      handleProjectFileReference,
      normalizeProjectFileReference,
      parseProjectFileReference,
    };
  }

  window.FennaraProjectFileLinks = {
    createProjectFileLinks,
  };
})();
