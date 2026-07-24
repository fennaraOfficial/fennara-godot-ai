# Cursor MCP Naming

This note applies only when the current model is running inside Cursor.

The Fennara MCP server is normally named `fennara` in Cursor settings and `mcp.json`. Cursor may expose the same user-added server internally as `user-fennara`. Treat those names as one server. If `fennara` is not discoverable through Cursor's internal tool metadata, check `user-fennara`.

Do not ask the user to find `user-fennara` in Cursor settings. It is an internal identifier, not a second Fennara installation.
