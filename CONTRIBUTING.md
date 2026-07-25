# Contributing

<!-- fennara-doc-nav:start -->
**English** · [简体中文](docs/i18n/zh-CN/CONTRIBUTING.md) · [Español](docs/i18n/es/CONTRIBUTING.md) · [Português do Brasil](docs/i18n/pt-BR/CONTRIBUTING.md) · [日本語](docs/i18n/ja/CONTRIBUTING.md) · [한국어](docs/i18n/ko/CONTRIBUTING.md) · [Русский](docs/i18n/ru/CONTRIBUTING.md) · [Français](docs/i18n/fr/CONTRIBUTING.md) · [Deutsch](docs/i18n/de/CONTRIBUTING.md) · [Türkçe](docs/i18n/tr/CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

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
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
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

## Documentation Translations

English is the canonical source. Correct English first, then update every
affected locale. The translated set and locale metadata live in
`docs/i18n/languages.json`.

- Read the complete English page and write the translation directly. Do not use bulk machine-translation services or prose-generation scripts.
- Keep code blocks, inline code, commands, paths, configuration keys, URLs, and product names exact.
- Preserve the source marker and explicit English anchor aliases maintained by the documentation scripts.
- Do not mark a translation as native-reviewed unless a fluent reviewer checked it.
- Do not translate legal text, internal agent prompts, generated project guidance, vendor files, or test fixtures as independent sources.

After changing canonical or translated documentation, run:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

These commands maintain navigation metadata and validate structure. They do not
write translated prose.

Ordinary navigation sync preserves every existing source hash. After changing
an English source, directly update that page in all nine translated locales,
then deliberately acknowledge only that canonical source:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Repeat `--accept-source <path>` for each English page whose translations were
reviewed and updated. Never accept a source hash before all nine translations
contain the new meaning.
