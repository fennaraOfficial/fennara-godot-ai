# Scripts

This directory contains repository automation that is shared by local development, package preview, and release workflows.

Scripts should be small, deterministic, and safe to run from the repository root unless their help text says otherwise. They should not write user-specific state outside the repo.

## Version Scripts

- `set-version.mjs`: updates the repo `VERSION`, addon `VERSION`, local Rust workspace metadata, lockfile package versions, and the C++ plugin version constant.
- `check-version.mjs`: verifies those versioned files are still in sync.

Run `check-version.mjs` in CI and before release packaging. Use `set-version.mjs` only when intentionally changing the Fennara version.

## Packaging Scripts

- `package-preview.mjs`: syncs committed addon payloads, then assembles per-platform preview archives after the GDExtension and local Rust binaries have already been built.
- `package-addon-all.mjs`: combines platform addon parts into the final all-platform addon archive.
- `release-policy.mjs`: defines the minimum compatible published CLI for each release track.
- `write-release-manifest.mjs`: writes `fennara-release-manifest-v<version>.json` from release assets and validates every referenced SHA-256.

Both scripts use `.package-preview/` as temporary staging and write zip outputs under the repo-root `dist/` folder. Those outputs are ignored and should not be committed.

Packaging scripts must keep the addon payload small. In particular, Linux CEF runtime files such as `libcef.so` and `fennara_cef_helper` must not be bundled inside `fennara-addon-*`; CEF is installed once into the user's shared Fennara app-data directory.

## Staging Release Scripts

- `write-staging-candidate.mjs`: creates the exact prerelease identity for one pull request and frozen source commit.
- `validate-staging-build.mjs`: checks addon parts, platform archives, the assembled addon, the release manifest, and Linux CEF before publication.
- `smoke-public-release.mjs`: downloads every published candidate through its unauthenticated browser URL and verifies the trusted asset and manifest hashes before channel advancement.
- `write-staging-pointer.mjs`: writes the small per-PR pointer after hashing the exact release manifest.
- `check-staging-channel-advance.mjs`: rejects backward or conflicting channel movement.
- `validate-staging-publish-bundle.mjs`: revalidates the final artifact bundle without executing candidate code.
- `verify-published-assets.mjs`: compares the expected and downloaded GitHub Release asset names and SHA-256 values.

These scripts support `.github/workflows/staging-release.yml`. Candidate build jobs run without release credentials. Only the trusted final job can publish, and it advances the per-channel Git ref after the exact release has been downloaded and verified.

## Linux CEF Scripts

- `prepare-linux-cef-sdk.mjs`: downloads/extracts the pinned official Linux x64 CEF SDK used to build the Linux CEF bridge.
- `prepare-linux-cef-runtime.mjs`: stages the separate Linux CEF runtime zip, validates required files, strips staged ELF binaries on Linux, and can write the generated `local/webview-runtimes/linux-cef.json` manifest for release packaging.
- `check-linux-cef-runtime-release.mjs`: validates that release assets contain the CEF runtime zip named by the enabled manifest and that its SHA-256 matches.
- `cef/linux/fennara_cef_helper.cpp`: tiny CEF helper process source used when building the runtime helper from the CEF SDK.

The CEF scripts operate on copied staging files only. They must not mutate the downloaded/source CEF SDK tree.

## UI Sync

- `sync-chat-ui.mjs`: copies `ui/chat/` into `godot_demo/addons/fennara/dist/`.

`godot_demo/addons/fennara/dist/` is intentionally committed because released addon zips must contain the built chat webview. Make changes in `ui/chat/`, run the sync script, and commit both source and generated addon assets together.

## Runtime Sync

- `sync-runtime.mjs`: copies `runtime/` into `godot_demo/addons/fennara/runtime/`.

`godot_demo/addons/fennara/runtime/` is intentionally committed because released addon zips must contain the Godot-side runtime helper scripts. Make changes in `runtime/`, run the sync script, and commit both source and generated addon assets together.

## Guidance Sync

- `sync-guidance.mjs`: copies `local/templates/fennara-guidelines.md` into `godot_demo/addons/fennara/ai/guidelines.md`, matching the file that `fennara install` and `fennara update` write into user projects.

`godot_demo/addons/fennara/ai/guidelines.md` is intentionally committed because the demo addon mirrors an installed addon layout. Make changes in `local/templates/`, run the sync script, and commit both source and generated addon guidance together.

## Boundaries

- Scripts may create `.package-preview/` and root `dist/` outputs.
- Scripts may update committed generated payloads only when that is their explicit job, such as `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs`, or `set-version.mjs`.
- Scripts must not write Godot editor cache, local app-data installs, downloaded release artifacts, or VM test output into tracked source folders.
