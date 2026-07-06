(function () {
  function daemonWsUrl(defaultWsUrl) {
    if (/^https?:$/.test(window.location.protocol) && /^(127\.0\.0\.1|localhost)$/.test(window.location.hostname)) {
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      return `${protocol}//${window.location.host}/chat/ws`;
    }
    return defaultWsUrl;
  }

  function chatWsUrl(defaultWsUrl) {
    const params = new URLSearchParams(window.location.search);
    const token = params.get("chat_token") || params.get("session") || "";
    const baseUrl = daemonWsUrl(defaultWsUrl);
    return token ? baseUrl + "?chat_token=" + encodeURIComponent(token) : baseUrl;
  }

  function createDaemonClient(options = {}) {
    const defaultWsUrl = options.defaultWsUrl || "ws://127.0.0.1:41287/chat/ws";
    const reconnectDelayMs = Number(options.reconnectDelayMs || 250);
    let socket = null;
    let daemonConnected = false;
    let reconnectTimer = 0;
    let requestCounter = 0;

    function currentChatWsUrl() {
      return chatWsUrl(defaultWsUrl);
    }

    function nextRequestId(prefix) {
      requestCounter += 1;
      return prefix + "-" + Date.now() + "-" + requestCounter;
    }

    function isOpen() {
      return Boolean(socket && socket.readyState === WebSocket.OPEN);
    }

    function connect() {
      window.clearTimeout(reconnectTimer);
      socket = new WebSocket(currentChatWsUrl());

      socket.addEventListener("open", () => {
        daemonConnected = true;
        options.onOpen?.();
      });

      socket.addEventListener("message", (event) => {
        let message = null;
        try {
          message = JSON.parse(event.data);
        } catch {
          return;
        }
        options.onMessage?.(message);
      });

      socket.addEventListener("close", () => {
        daemonConnected = false;
        options.onClose?.();
        reconnectTimer = window.setTimeout(connect, reconnectDelayMs);
      });
    }

    function send(payload) {
      if (!isOpen()) {
        options.onSendUnavailable?.(payload);
        return false;
      }
      socket.send(JSON.stringify(payload));
      return true;
    }

    function sendIfOpen(payload) {
      if (!isOpen()) {
        return false;
      }
      socket.send(JSON.stringify(payload));
      return true;
    }

    function ensureConnected() {
      if (daemonConnected && isOpen()) {
        return true;
      }
      options.onEnsureUnavailable?.();
      if (!socket || socket.readyState === WebSocket.CLOSED || socket.readyState === WebSocket.CLOSING) {
        connect();
      }
      return false;
    }

    return {
      chatWsUrl: currentChatWsUrl,
      connect,
      ensureConnected,
      nextRequestId,
      send,
      sendIfOpen,
    };
  }

  window.FennaraDaemonClient = {
    createDaemonClient,
    daemonWsUrl,
    chatWsUrl,
  };
})();
