# Release Process

Releases are manual. Do not publish from pull request workflows.

> [!IMPORTANT]
> Run releases from `main`, keep `VERSION` and the workflow input identical, and
> explicitly decide whether the release needs a higher minimum CLI version.

## Release At A Glance

| Step | Result |
| --- | --- |
| Prepare and merge the version change | Repository version sources agree |
| Run Package Preview | Release-shaped artifacts are built without publishing |
| Inspect the preview | Archives, manifest, hashes, and Linux CEF layout are verified |
| Run Release from `main` | Tag and GitHub Release are published |
| Smoke test install and update | The public user flow is verified |

## Versioning

`VERSION` is the source of truth.

Release tooling accepts SemVer values. Stable releases use `X.Y.Z`. Staging
candidates use an isolated pull-request prerelease such as
`0.3.9-pr.101.2`, where `pr-101` is the staging channel and `2` is that
channel's candidate number.

To bump the repo version:

```bash
node scripts/set-version.mjs X.Y.Z
```

The script updates:

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- plugin version constants
- Rust workspace package version under `local/`
- `local/Cargo.lock`

The addon also carries `addons/fennara/release.json`. Stable identity is
written automatically by the normal command above. A staging build workspace
uses the explicit identity inputs:

```bash
node scripts/set-version.mjs 0.3.9-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

The staging version, channel, source commit, and immutable release tag must
agree. A prerelease addon without this identity is rejected. Existing stable
addons from before `release.json` continue to default to the stable track.

Check version sync:

```bash
node scripts/check-version.mjs
```

## 1. Prepare The Release Commit

1. Run the version script.
2. Review the diff.
3. Run local checks that match the changed surface.
4. Merge the release prep PR into `main`.

Common checks:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

For GDExtension changes, also build the addon locally when possible:

```bash
cd fennara-cpp
scons platform=windows target=editor
```

## 2. Run Package Preview

Use this before publishing when packaging changed or when you want a dry run.

GitHub:

```text
Actions > Package Preview > Run workflow
```

The workflow builds Windows, Linux, and macOS packages and uploads temporary
artifacts. It does not create tags, GitHub Releases, or `latest`.

Package Preview mirrors the non-publishing parts of Release closely enough to
exercise release packaging before merge:

- syncs buildless chat UI and runtime helper source into the addon payload
- builds the Linux CEF runtime zip
- writes the generated Linux CEF runtime manifest
- feeds that generated manifest into platform package builds
- assembles the all-platform addon archive
- renames local/addon packages to the manifest-managed release asset names
- validates the Linux CEF runtime asset against the generated manifest
- writes `fennara-release-manifest-v<version>.json`
- uploads one `fennara-package-preview-release-assets` artifact containing the
  release-shaped zips and manifest

Preview artifacts are useful for checking zip contents and manifest shape before
publishing. They are Actions artifacts, not public release assets.

## 3. Run Release

Run the manual release workflow from `main`:

```text
Actions > Release > Run workflow
```

Inputs:

```text
version: X.Y.Z
promote_latest: true
```

The `version` input must match `VERSION`.

The workflow publishes:

- `v<version>`
- `latest` when `promote_latest` is true

The release workflow prepares the Linux CEF runtime before platform packaging.
It downloads the pinned official CEF 139 Linux minimal SDK, assembles the
separate `fennara-webview-cef-linux-x64-<cef-version>.zip`, strips staged ELF
binaries, writes a generated enabled `local/webview-runtimes/linux-cef.json`
manifest, and feeds that manifest into the CLI packages. The publish job then
validates that the release assets include the exact CEF zip named by the
generated manifest and that its SHA-256 matches. It also writes
`fennara-release-manifest-v<version>.json`, validates every referenced asset and
hash, and uploads that manifest with the release.

Pull request workflows do not publish releases. The Package Preview workflow
creates release-shaped test artifacts, including the manifest and Linux CEF
runtime payload, so maintainers can smoke-test packaging before merging. Package
Preview is not the user-facing release channel.

## Release Assets

Each release should contain per-platform CLI/local runtime packages and one shared all-platform addon package.

| Target | Assets |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| All platforms | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Package roles:

| Pattern | Role |
| --- | --- |
| `fennara-cli-*` | Install script payload containing only the `fennara` CLI for one platform |
| `fennara-release-local-*` | MCP and daemon launchers plus versioned runtime binaries for one platform |
| `fennara-release-addon-v*` | Versioned all-platform addon resolved through the release manifest |
| `fennara-addon-latest.zip` | Stable all-platform addon URL for the Godot Asset Library and documentation |
| `fennara-webview-cef-linux-x64-*` | Linux-only shared CEF runtime installed once in Fennara app data |
| `fennara-release-manifest-v*` | Install and update plan containing asset names, SHA-256 values, install primitives, and shared runtimes |

The `fennara-release-local-*` prefix prevents older CLIs from silently bypassing
the manifest-managed package path.

## Release Manifest

Starting in 0.3.0, `fennara install` and `fennara update` prefer the release
manifest whenever the release publishes one. The manifest records:

- `schema_version`
- `version`
- `minimum_cli_version`
- supported install primitives
- per-platform CLI and local runtime assets with SHA-256 hashes
- the shared addon asset with SHA-256
- platform-specific shared runtime assets, currently Linux CEF

The current manifest generator and release workflows use
`minimum_cli_version: 0.3.8` by default. Normal package layout or asset name
changes should be handled by manifest data, not by changing the outer CLI.
Raise `minimum_cli_version` only when a release needs a new manifest schema or
install primitive that older CLIs truly cannot perform.

When the CLI is too old, `fennara update` should use the manifest's
per-platform `assets.cli` entry to update the installed CLI first, then resume
the package update with `--no-self-update`. If self-update is not available for
that release or install location, it should fail before installing packages and
print a clear instruction to rerun `install.sh` or `install.ps1`.

The optional release identity added to manifest schema 1 does not require a
minimum CLI increase. Older schema-1 clients ignore unknown fields, while
staging-aware clients validate the identity when it is present. A future
release that depends on channel-aware activation or updater handoff must
revisit the minimum CLI before publication.

## Staging Identity And Discovery Contract

Staging channels are isolated per pull request:

| Value | PR 101 example |
| --- | --- |
| Channel | `pr-101` |
| Candidate version | `0.3.9-pr.101.2` |
| Immutable release | `v0.3.9-pr.101.2` |
| Channel ref | `fennara-staging/pr-101` |
| Pointer file | `fennara-staging-channel-pr-101.json` |

The per-channel Git ref contains only a small pointer file to an immutable
exact release. Release binaries never live under the moving channel ref. The
CLI can resolve this pointer with the internal version request
`channel:pr-101`, then continues using only the exact immutable version.

PR 101 and PR 125 therefore use different release tags and pointer assets.
Updating one channel cannot redirect testers on the other channel. Publishing
one channel never changes stable `latest` or another pull request's channel.

## Staging Candidate Workflow

The manual **Staging Release** workflow builds a candidate from the current
head of an open pull request. Run it from `main` and provide:

| Input | Meaning |
| --- | --- |
| `pull_request` | Open pull request to build |
| `base_version` | Planned stable version, such as `0.3.9` |
| `candidate` | Increasing candidate number for this pull request |
| `source_commit` | Optional full SHA that must still be the pull request head |
| `publish` | Off for artifact-only validation, on to publish the candidate |

The workflow freezes the pull request head SHA before any platform build. The
Windows, Linux, and macOS jobs check out that exact commit with read-only
permissions, no persisted Git credentials, no release credentials, and no
shared dependency caches. Candidate code can produce build artifacts, but it
cannot publish a GitHub Release.

Trusted repository scripts then validate the candidate identity, exact archive
inventory, addon contents, platform package layout, release manifest, and every
SHA-256 value. Publication remains disabled unless `publish` is explicitly
selected.

When publication is enabled, the trusted final job:

1. Requires GitHub release immutability to be enabled for the repository.
2. Revalidates the candidate artifacts as data.
3. Creates a draft, uploads every asset, publishes it as the immutable
   `v<exact-version>` prerelease, and verifies its release attestation.
4. Downloads the published assets and compares their names and hashes.
5. Rejects a backward or conflicting channel change.
6. Updates the small `fennara-staging/pr-<number>` pointer ref last through a
   conditional GitHub Contents API write.
7. Downloads the active pointer and verifies its exact contents.

Runs for one pull request are serialized. Different pull requests use separate
concurrency groups, release tags, and pointer refs. Retrying the same
candidate verifies the existing immutable release instead of mixing files into
it. The workflow never creates, uploads to, or promotes stable `latest`.

GitHub release immutability applies only to releases created after the setting
is enabled. Fennara intentionally preserves the existing pre-policy `latest`
release as the one mutable compatibility endpoint used by current installers.
The stable Release workflow updates that release in place and fails if it is
missing or immutable. Exact stable and staging releases are created as drafts,
receive all assets before publication, and must pass `gh release verify` after
publication.

The stable and staging publication jobs use the `RELEASE_ADMIN_TOKEN` repository
secret only for the immutable-release preflight. Configure it as a fine-grained
token with repository Administration read access. Asset publication continues
to use the job-scoped `GITHUB_TOKEN` with contents write access.

Staging-capable and stable release workflows use `minimum_cli_version: 0.3.8`.
Channel handoff, exact-target preservation across CLI replacement, and safe
shared-runtime activation depend on the updater behavior introduced in that CLI.

The shared addon zip contains every built GDExtension binary referenced by `godot_demo/addons/fennara/fennara.gdextension`. Godot loads the matching library for the user's OS and ignores the others.

Linux CEF webview runtime payloads are separate from the addon archive. Release
packaging generates the enabled runtime manifest and embeds that data into
`fennara-release-manifest-v<version>.json`. The CLI installs the matching CEF
payload once under the user's Fennara app-data directory:

```text
webview/cef/linux-x64/<cef-version>/
```

Do not place `libcef.so`, CEF helper executables, CEF resources, or locale packs
inside `fennara-addon-*`. Package Preview builds a separate CEF artifact for
testing and writes the same kind of generated runtime manifest used by Release,
but release publishing remains the only user-facing source of release assets.

Linux GDExtension builds also need the official CEF SDK wrapper source, but not
the CEF runtime files in the addon. CI runs:

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

and passes the extracted directory as `FENNARA_CEF_ROOT` to SCons. SCons uses
`FENNARA_CEF_ROOT/libcef_dll/` to build the small
`libfennara_linux_cef_bridge.so` addon library against the pinned CEF 139 C++
wrapper. The SDK download is version- and hash-checked because the generated
wrapper source must match the runtime CEF ABI. The bridge is packaged with the
addon; `libcef.so`, resources, locale packs, and `fennara_cef_helper` remain in
the separate shared CEF runtime.

Package scripts fail if CEF runtime files are found inside the addon archive.
The runtime asset name must be:

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

The zip must extract with required files at its root:

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

Optional CEF runtime files such as `chrome-sandbox`, `libEGL.so`,
`libGLESv2.so`, `libvk_swiftshader.so`, `libvulkan.so.1`,
`vk_swiftshader_icd.json`, `snapshot_blob.bin`, and additional `locales/*.pak`
should be included when present in the selected CEF distribution.

To assemble the runtime zip manually from a maintainer-selected CEF binary tree:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

On Linux, the script builds `fennara_cef_helper` from
`scripts/cef/linux/fennara_cef_helper.cpp` against the official CEF headers in
`fennara-cpp/vendor/cef/`. On another OS, build that helper on Linux first and
pass `--helper /path/to/fennara_cef_helper`. Use `--dry-run` to inspect the
selected files before writing the zip.

After the script prints the SHA-256, update
`local/webview-runtimes/linux-cef.json`:

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

For normal releases, the workflow writes the Linux CEF runtime manifest
automatically with `--write-manifest`, then `scripts/write-release-manifest.mjs`
copies the runtime fields into `fennara-release-manifest-v<version>.json`. Do
not hand-enable the checked-in placeholder manifest unless intentionally
debugging a manual runtime asset path or legacy fallback behavior. If generated
manifest data points at an asset that is missing or whose SHA-256 does not
match, the Release workflow and Linux `fennara install` / `fennara update` fail
clearly.

The CLI must publish Linux CEF runtime updates atomically: extract and validate
in a staging directory, write the runtime marker only after required files are
present, then publish the version directory and update `current.json` with a
temp-file rename. Running editors keep using the runtime they already loaded.

The CLI embeds the generated project guidance templates from `local/templates/`.
When release packaging builds the CLI, those templates are compiled into the binary with the rest of the CLI code.

## What `latest` Means

`latest` is the moving release used by normal install and update flows.

- `install.ps1` and `install.sh` fetch the latest CLI asset by default.
- `fennara update` fetches the release manifest from `latest` by default, self-updates the installed CLI when needed, then resolves local/addon/shared runtime assets from it.
- In-editor updates stage verified assets before shutdown, recheck the complete staged-addon digest before replacement, keep the previous addon, launchers, and runtime manifest until activation validation succeeds, and require the reopened GDExtension handshake before deleting rollback data.
- `fennara install` fetches the release manifest from `latest` by default, then resolves local/addon/shared runtime assets from it.
- The Godot plugin update check compares against GitHub's latest release.

Use `promote_latest: false` only when publishing a version that should not become the default user install.

Installers and release downloads should print release metadata, asset download,
extract, install, and verification steps. Network fetches should use bounded
timeouts so GitHub/CDN stalls fail with a diagnostic instead of looking frozen.
On Windows, `install.ps1` must check the CLI verification exit code before
printing success. Exit code `-1073741515` (`0xC0000135`) means the CLI executable
was written but Windows could not start it because a required DLL is missing;
tell the user to install Microsoft Visual C++ Redistributable 2015-2022 x64 and
then rerun `fennara --version`, `fennara doctor`, and `fennara install`.
Download URL: `https://aka.ms/vs/17/release/vc_redist.x64.exe`.

## Smoke Test After Release

On Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

In a Godot project:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Check that the project received:

```text
AGENTS.md
addons/fennara/ai/guidelines.md
```

Open the project in Godot, then ask the MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Update test:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

## Rules

- Release workflow runs from `main` only.
- Release version input must match `VERSION`.
- Pull request workflows may build and upload test artifacts, but must not publish releases.
- Keep `latest` pointed at the newest release intended for normal users.
- Do not rewrite published release tags unless maintainers intentionally decide to replace a broken release.
