# Release Process

Releases are manual.

This repository is being prepared for source-based releases. Release automation should stay conservative while the public packaging flow is established.

## Expected Flow

1. Update the release version.
2. Build platform artifacts from the release commit.
3. Verify artifacts locally.
4. Create a GitHub release tag such as `v0.1.0`.
5. Upload local tool archives and the Godot plugin package.
6. Update release notes with the build commit and verification notes.

## Rules

- Do not publish releases from pull request workflows.
- Do not add production deploy behavior to pull request workflows.
- Keep release workflows manual unless maintainers decide otherwise.
- Prefer small release workflow changes with clear review.
