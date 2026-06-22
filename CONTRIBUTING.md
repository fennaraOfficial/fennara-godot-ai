# Contributing

Thanks for helping improve Fennara Godot AI.

## Good Contributions

- Documentation fixes
- Reproducible bug fixes
- Platform compatibility fixes
- Build and packaging improvements
- Small improvements to setup clarity

## Design Discussion Required

Open an issue or discussion before starting:

- new MCP tools
- tool schema changes
- release workflow changes
- large architecture changes
- changes that affect generated project guidance

## Pull Requests

- Keep pull requests small and focused.
- Explain what changed and why.
- Explain how you verified the change.
- Include screenshots or recordings for visible UI or documentation rendering changes.
- Do not include unrelated formatting or cleanup.
- Do not paste large generated descriptions into issues or pull requests.

## Commit And PR Titles

Use Conventional Commit style:

```text
fix: handle missing daemon status
docs: clarify setup steps
ci: add public pull request checks
```

Common types:

- `feat`: user-facing feature
- `fix`: bug fix
- `docs`: documentation
- `ci`: GitHub Actions and automation
- `build`: build or packaging
- `refactor`: behavior-preserving code restructuring
- `test`: tests
- `chore`: maintenance

## Project Boundaries

Fennara should remain game-agnostic. Avoid APIs or guidance that assume a game's controls, objectives, economy, inventory, combat, pathing, quests, or UI flow.

Agents should inspect a Godot project's real scenes, scripts, resources, settings, runtime state, diagnostics, and screenshots, then compose generic Fennara tools for that project.
