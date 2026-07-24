# Anonymous Telemetry

Fennara sends one small anonymous activity event at most once per UTC day. The
event is sent only after a compatible Godot editor connects to the local daemon.
It helps maintainers measure active installations, supported platform usage, and
version adoption.

Telemetry is enabled by default. Open **Chat Settings > Chat > Anonymous
telemetry** to disable it. Headless and automated environments can set either:

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

An environment variable takes precedence over the saved UI preference. Turning
telemetry off stops future events and deletes the local telemetry identity and
last-send state. Turning it on again creates a new random identity when Godot
next connects.

## Event contents

The `fennara_active_installation` event contains only:

| Field | Purpose |
| --- | --- |
| `schema_version` | Version of the small telemetry payload contract |
| `event` | Fixed event name |
| `installation_id` | Random UUID generated locally, not derived from hardware or accounts |
| `fennara_version` | Running daemon version |
| `godot_version` | Numeric Godot version, such as `4.6.3` |
| `platform` | `windows`, `macos`, or `linux` |
| `architecture` | `x86_64` or `aarch64` |

Fennara does not send project names, project paths, account information, prompts,
chat messages, provider keys, model names, tool names, tool arguments, tool
results, logs, screenshots, scene contents, filenames, or error text.

## Storage and transport

The daemon stores its random identity and last successful UTC day under the
shared Fennara app-data directory:

```text
Fennara/
  telemetry/
    state.json
```

The daemon sends the event over HTTPS to
`https://fennara.io/api/telemetry`. The receiver validates an exact field
allowlist and replaces the raw installation UUID with a server-side HMAC before
forwarding the event to PostHog. PostHog person profiles and IP geolocation are
disabled for this event.

The Vercel receiver necessarily observes normal network metadata while handling
the HTTPS request. That metadata is not copied into the PostHog event payload.

## Delivery behavior

Telemetry runs outside Godot tool-call paths:

- A bounded queue accepts activity signals without waiting.
- One background worker reuses a single HTTP client.
- Requests have a short timeout.
- A full queue, filesystem problem, network failure, or server rejection is
  silently tolerated and never fails a Fennara tool.
- The UTC day is recorded only after the server accepts an event, so a later
  Godot connection can retry a failed delivery.
- Shutdown waits briefly, then cancels the telemetry worker instead of delaying
  the daemon.

One installation is one persisted random UUID. Using Fennara on two computers
counts as two installations. Clearing Fennara app data or disabling and later
re-enabling telemetry creates a new identity.

Monthly active installations are counted as distinct anonymous installation
identities that sent at least one `fennara_active_installation` event during the
calendar month.
