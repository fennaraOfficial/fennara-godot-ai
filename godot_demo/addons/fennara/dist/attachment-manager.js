(function () {
  const MAX_IMAGE_ATTACHMENTS = 4;
  const MAX_CONTEXT_SNIPPETS = 8;
  const MAX_CONTEXT_SNIPPET_CHARS = 64000;
  const MAX_RAW_IMAGE_BYTES = 8 * 1024 * 1024;
  const MAX_SEND_IMAGE_BYTES = 3 * 1024 * 1024;
  const MAX_TOTAL_IMAGE_BYTES = 20 * 1024 * 1024;
  const SUPPORTED_IMAGE_TYPES = new Set(["image/png", "image/jpeg", "image/webp", "image/gif"]);

  function createAttachmentManager(options = {}) {
    const attachmentPreview = options.attachmentPreview || null;
    const imageInput = options.imageInput || null;
    const transcriptRenderer = options.transcriptRenderer || null;
    const appendSystem = options.appendSystem || function () {};
    const focusComposer = options.focusComposer || function () {};
    const onNativePasteboardSettled = options.onNativePasteboardSettled || function () {};
    let attachedImages = [];
    let attachedContextSnippets = [];

    async function addImageFiles(files) {
      const unique = uniqueFiles(files);
      const imageFiles = unique.filter((file) => file && imageMimeType(file));
      if (imageFiles.length === 0) {
        return 0;
      }
      let added = 0;
      for (const file of imageFiles) {
        if (attachedImages.length >= MAX_IMAGE_ATTACHMENTS) {
          appendSystem(`Attach up to ${MAX_IMAGE_ATTACHMENTS} images.`);
          break;
        }
        const mimeType = imageMimeType(file);
        const validationError = validateImageFile(file, mimeType);
        if (validationError) {
          appendSystem(validationError);
          continue;
        }
        try {
          const dataUrl = await readFileAsDataUrl(file);
          const prepared = await prepareImageForChat({
            base64: dataUrl.split(",", 2)[1] || "",
            mimeType,
            name: file.name || "pasted image",
            size: file.size,
          });
          if (!prepared) {
            appendSystem("Image is too large. Try a smaller screenshot.");
            continue;
          }
          const totalSize = attachedImages.reduce((sum, image) => sum + image.size, 0) + prepared.size;
          if (totalSize > MAX_TOTAL_IMAGE_BYTES) {
            appendSystem(`Attached images must be ${formatBytes(MAX_TOTAL_IMAGE_BYTES)} total or less.`);
            continue;
          }
          attachedImages.push({
            id: `${Date.now()}-${Math.random().toString(16).slice(2)}`,
            base64: prepared.base64,
            mime_type: prepared.mimeType,
            name: prepared.name,
            size: prepared.size,
            description: file.name || "user image",
          });
          added += 1;
        } catch {
          appendSystem("Could not read that image.");
        }
      }
      renderAttachmentPreview();
      return added;
    }

    function uniqueFiles(files) {
      const seen = new Set();
      const unique = [];
      for (const file of Array.from(files || [])) {
        if (!file) {
          continue;
        }
        const key = [file.name || "", file.type || "", file.size || 0, file.lastModified || 0].join(":");
        if (seen.has(key)) {
          continue;
        }
        seen.add(key);
        unique.push(file);
      }
      return unique;
    }

    async function addImagePayload(image) {
      const base64 = String(image?.base64 || "");
      const mimeType = String(image?.mime_type || "").toLowerCase();
      const size = Number(image?.size || 0);
      if (!base64 || !mimeType) {
        return false;
      }
      if (attachedImages.length >= MAX_IMAGE_ATTACHMENTS) {
        appendSystem(`Attach up to ${MAX_IMAGE_ATTACHMENTS} images.`);
        return false;
      }
      if (!SUPPORTED_IMAGE_TYPES.has(mimeType)) {
        appendSystem("Unsupported image type. Use PNG, JPEG, WebP, or GIF.");
        return false;
      }
      if (size > MAX_RAW_IMAGE_BYTES) {
        appendSystem("Image is too large. Try a smaller screenshot.");
        return false;
      }
      const prepared = await prepareImageForChat({
        base64,
        mimeType,
        name: String(image?.name || "pasted image"),
        size,
      });
      if (!prepared) {
        appendSystem("Image is too large. Try a smaller screenshot.");
        return false;
      }
      const totalSize = attachedImages.reduce((sum, item) => sum + item.size, 0) + prepared.size;
      if (totalSize > MAX_TOTAL_IMAGE_BYTES) {
        appendSystem(`Attached images must be ${formatBytes(MAX_TOTAL_IMAGE_BYTES)} total or less.`);
        return false;
      }
      attachedImages.push({
        id: `${Date.now()}-${Math.random().toString(16).slice(2)}`,
        base64: prepared.base64,
        mime_type: prepared.mimeType,
        name: prepared.name,
        size: prepared.size,
        description: prepared.name,
      });
      renderAttachmentPreview();
      return true;
    }

    function addContextSnippet(rawSnippet) {
      const snippet = normalizeContextSnippet(rawSnippet);
      if (!snippet) {
        return false;
      }
      const alreadyAttached = attachedContextSnippets.some((item) =>
        item.path === snippet.path &&
        item.start_line === snippet.start_line &&
        item.end_line === snippet.end_line &&
        item.text === snippet.text
      );
      if (alreadyAttached) {
        focusComposer();
        return true;
      }
      if (attachedContextSnippets.length >= MAX_CONTEXT_SNIPPETS) {
        appendSystem(`Attach up to ${MAX_CONTEXT_SNIPPETS} code snippets.`);
        return false;
      }
      attachedContextSnippets.push(snippet);
      renderAttachmentPreview();
      focusComposer();
      return true;
    }

    function normalizeContextSnippet(rawSnippet, normalizeOptions = {}) {
      const path = String(rawSnippet?.path || "").trim();
      const startLine = Number(rawSnippet?.start_line || 0);
      const endLine = Number(rawSnippet?.end_line || 0);
      let text = String(rawSnippet?.text || "").replace(/\r\n/g, "\n").replace(/\r/g, "\n");
      if (!path || !text.trim() || !Number.isInteger(startLine) || !Number.isInteger(endLine)) {
        return null;
      }
      if (startLine <= 0 || endLine < startLine) {
        return null;
      }
      if (text.length > MAX_CONTEXT_SNIPPET_CHARS) {
        text = text.slice(0, MAX_CONTEXT_SNIPPET_CHARS) + "\n... [truncated by Fennara]\n";
      }
      return {
        id: normalizeOptions.keepId && rawSnippet?.id
          ? String(rawSnippet.id)
          : `${Date.now()}-${Math.random().toString(16).slice(2)}`,
        path,
        start_line: startLine,
        end_line: endLine,
        text,
      };
    }

    function imageMimeType(file) {
      const explicitType = String(file?.type || "").toLowerCase();
      if (SUPPORTED_IMAGE_TYPES.has(explicitType)) {
        return explicitType;
      }
      const name = String(file?.name || "").toLowerCase();
      if (name.endsWith(".png")) {
        return "image/png";
      }
      if (name.endsWith(".jpg") || name.endsWith(".jpeg")) {
        return "image/jpeg";
      }
      if (name.endsWith(".webp")) {
        return "image/webp";
      }
      if (name.endsWith(".gif")) {
        return "image/gif";
      }
      return "";
    }

    function validateImageFile(file, mimeType) {
      if (!SUPPORTED_IMAGE_TYPES.has(mimeType)) {
        return "Unsupported image type. Use PNG, JPEG, WebP, or GIF.";
      }
      if (file.size > MAX_RAW_IMAGE_BYTES) {
        return "Image is too large. Try a smaller screenshot.";
      }
      return "";
    }

    async function prepareImageForChat(image) {
      if (!image.base64) {
        return null;
      }
      if (image.size <= MAX_SEND_IMAGE_BYTES) {
        return image;
      }
      if (image.mimeType === "image/gif") {
        return null;
      }
      return compressImageForChat(image);
    }

    async function compressImageForChat(image) {
      const dataUrl = `data:${image.mimeType};base64,${image.base64}`;
      const loaded = await loadImage(dataUrl);
      const canvas = document.createElement("canvas");
      const context = canvas.getContext("2d");
      if (!context) {
        return null;
      }

      let scale = Math.min(1, Math.sqrt(MAX_SEND_IMAGE_BYTES / Math.max(image.size, 1)) * 0.92);
      const qualities = [0.82, 0.72, 0.62, 0.52];
      for (let attempt = 0; attempt < 6; attempt += 1) {
        canvas.width = Math.max(1, Math.round(loaded.width * scale));
        canvas.height = Math.max(1, Math.round(loaded.height * scale));
        context.fillStyle = "#fff";
        context.fillRect(0, 0, canvas.width, canvas.height);
        context.drawImage(loaded, 0, 0, canvas.width, canvas.height);
        for (const quality of qualities) {
          const blob = await canvasToBlob(canvas, "image/jpeg", quality);
          if (blob && blob.size <= MAX_SEND_IMAGE_BYTES) {
            return {
              base64: await blobToBase64(blob),
              mimeType: "image/jpeg",
              name: image.name.replace(/\.[^.]+$/, "") + ".jpg",
              size: blob.size,
            };
          }
        }
        scale *= 0.82;
      }
      return null;
    }

    function loadImage(src) {
      return new Promise((resolve, reject) => {
        const image = new Image();
        image.onload = () => resolve(image);
        image.onerror = reject;
        image.src = src;
      });
    }

    function canvasToBlob(canvas, type, quality) {
      return new Promise((resolve) => {
        canvas.toBlob(resolve, type, quality);
      });
    }

    async function blobToBase64(blob) {
      const dataUrl = await readFileAsDataUrl(blob);
      return dataUrl.split(",", 2)[1] || "";
    }

    function readFileAsDataUrl(file) {
      return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.addEventListener("load", () => resolve(String(reader.result || "")));
        reader.addEventListener("error", reject);
        reader.readAsDataURL(file);
      });
    }

    function renderAttachmentPreview() {
      if (!attachmentPreview) {
        return;
      }
      attachmentPreview.hidden = attachedImages.length === 0 && attachedContextSnippets.length === 0;
      attachmentPreview.replaceChildren();
      for (const image of attachedImages) {
        const chip = document.createElement("figure");
        chip.className = "attachment-chip";
        const preview = document.createElement("button");
        preview.type = "button";
        preview.className = "attachment-preview-button";
        preview.setAttribute("aria-label", `Open ${image.name || "attached image"}`);
        const img = document.createElement("img");
        img.alt = image.name || "Attached image";
        img.src = `data:${image.mime_type};base64,${image.base64}`;
        preview.addEventListener("click", () => transcriptRenderer?.openImagePreview(img.src, img.alt));
        const remove = document.createElement("button");
        remove.type = "button";
        remove.className = "attachment-remove-button";
        remove.setAttribute("aria-label", "Remove image");
        remove.textContent = "x";
        remove.addEventListener("click", () => {
          attachedImages = attachedImages.filter((item) => item.id !== image.id);
          renderAttachmentPreview();
        });
        preview.append(img);
        chip.append(preview, remove);
        attachmentPreview.append(chip);
      }
      for (const snippet of attachedContextSnippets) {
        const chip = document.createElement("figure");
        chip.className = "attachment-chip context-chip";
        const icon = document.createElement("span");
        icon.className = "context-chip-icon";
        icon.textContent = "{}";
        const label = document.createElement("figcaption");
        const title = document.createElement("strong");
        title.textContent = contextSnippetLabel(snippet);
        const path = document.createElement("span");
        path.textContent = snippet.path;
        label.append(title, path);
        const remove = document.createElement("button");
        remove.type = "button";
        remove.className = "attachment-remove-button";
        remove.setAttribute("aria-label", "Remove code context");
        remove.textContent = "x";
        remove.addEventListener("click", (event) => {
          event.stopPropagation();
        });
        remove.addEventListener("click", () => {
          attachedContextSnippets = attachedContextSnippets.filter((item) => item.id !== snippet.id);
          renderAttachmentPreview();
        });
        chip.tabIndex = 0;
        chip.setAttribute("role", "button");
        chip.setAttribute("aria-label", `Preview ${contextSnippetLabel(snippet)}`);
        chip.addEventListener("click", () => transcriptRenderer?.openContextSnippetPreview(snippet));
        chip.addEventListener("keydown", (event) => {
          if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            transcriptRenderer?.openContextSnippetPreview(snippet);
          }
        });
        chip.append(icon, label, remove);
        attachmentPreview.append(chip);
      }
    }

    function clearAttachments() {
      attachedImages = [];
      attachedContextSnippets = [];
      if (imageInput) {
        imageInput.value = "";
      }
      renderAttachmentPreview();
    }

    function attachmentPayload() {
      return attachedImages.map((image) => ({
        base64: image.base64,
        mime_type: image.mime_type,
        description: image.description,
        name: image.name,
        size: image.size,
      }));
    }

    function contextSnippetPayload(snippets) {
      return snippets.map((snippet) => ({
        path: snippet.path,
        start_line: snippet.start_line,
        end_line: snippet.end_line,
        text: snippet.text,
      }));
    }

    function contextSnippetLabel(snippet) {
      const fileName = String(snippet.path || "").split(/[\\/]/).filter(Boolean).pop() || "script";
      const range = snippet.end_line > snippet.start_line
        ? `L${snippet.start_line}-${snippet.end_line}`
        : `L${snippet.start_line}`;
      return `${fileName}#${range}`;
    }

    function languageForScriptPath(path) {
      const clean = String(path || "").toLowerCase();
      if (clean.endsWith(".gd")) {
        return "gdscript";
      }
      if (clean.endsWith(".cs")) {
        return "csharp";
      }
      return "text";
    }

    function formatBytes(bytes) {
      return `${Math.round(bytes / 1024 / 1024)} MB`;
    }

    function nativePasteboardBridge() {
      return window.webkit?.messageHandlers?.fennaraPasteboard;
    }

    function requestNativePastedImage() {
      const bridge = nativePasteboardBridge();
      if (!bridge) {
        return false;
      }
      try {
        bridge.postMessage({ type: "paste_image" });
        return true;
      } catch {
        return false;
      }
    }

    window.FennaraNativePasteboard = {
      receiveImage(image) {
        addImagePayload(image).finally(() => {
          onNativePasteboardSettled();
        });
      },
      receiveError(error) {
        const message = String(error?.message || "Could not paste that image.");
        appendSystem(message);
        onNativePasteboardSettled();
      },
    };

    return {
      addContextSnippet,
      addImageFiles,
      attachmentPayload,
      clearAttachments,
      contextSnippetPayload,
      getContextSnippets: () => attachedContextSnippets.slice(),
      hasAttachments: () => attachedImages.length > 0 || attachedContextSnippets.length > 0,
      languageForScriptPath,
      normalizeContextSnippet,
      requestNativePastedImage,
    };
  }

  window.FennaraAttachmentManager = {
    createAttachmentManager,
    SUPPORTED_IMAGE_TYPES,
  };
})();
