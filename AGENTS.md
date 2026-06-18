# Agent Instructions

Read this file before changing the repository.

## Core Rules

- Keep changes small, focused, and easy to review.
- Prefer simple code and clear ownership boundaries.
- Do not add game-specific MCP tools or guidance. Fennara should expose Godot feedback and primitive controls, not assumptions about a particular game's movement, combat, inventory, quests, UI flow, or objectives.
- Do not publish releases, create tags, or run release workflows unless a maintainer explicitly asks for that exact action.
- Do not change GitHub Actions release behavior casually. Explain any workflow change in the pull request.

## Source Of Truth

- `README.md` is the human-facing project overview.
- `llms.txt` is the short index for language models and coding agents.
- `CONTEXT.md` defines shared Fennara vocabulary.
- `docs/repo-map.md` explains repository layout.
- `docs/architecture.md` explains the high-level system.
- `docs/release.md` explains release expectations.

## Documentation Updates

When changing tool behavior, setup behavior, or release behavior, update the relevant docs in the same pull request.

When adding source areas, update `docs/repo-map.md` so contributors and agents can find the right files quickly.

## Pull Requests

- Use Conventional Commit style for pull request titles.
- Keep descriptions short and specific.
- Explain how the change was verified.
- Avoid unrelated cleanup in feature or fix pull requests.
