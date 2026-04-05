# Agent Rules

A convention for organizing prescribed policy in AI agent instruction files (CLAUDE.md, .cursorrules, AGENTS.md, etc.).

## The Problem

AI coding tools each have their own instruction file format:

- Claude Code uses `CLAUDE.md`
- Cursor uses `.cursorrules` or `.cursor/rules/`
- Windsurf uses `.windsurfrules` or `.windsurf/rules/`
- GitHub Copilot uses `.github/copilot-instructions.md`
- Codex uses `AGENTS.md`

There is no shared convention for the *content* of these files -- what makes a good rule, how to organize them, how to keep them portable across tools. The result: rules are ad-hoc, mixed with procedures, cluttered with informational content, and locked to a single tool.

## The Convention

Place rules in instruction files following structured conventions. Rules are **declarative policy** -- not procedures (that's [runbooks](https://github.com/btakita/agent-runbooks)) and not learned lessons (that's [memories](https://github.com/btakita/agent-memories)).

Your instruction file references runbooks and memories but *contains* rules:

```markdown
## Conventions

- Use snake_case for all Python modules and functions.
- Prefer composition over inheritance.

## Constraints

- Never commit `.env` files or cleartext secrets.
- All public API changes require a migration guide.

## Architecture

- PostgreSQL for persistence (chosen for JSONB support and mature tooling).
- Event sourcing for the order pipeline; CRUD for everything else.

## Tool Configuration

- Run `make check` before committing (see `.agent/runbooks/precommit.md`).
- Use `cargo clippy --all-targets` as the lint command.
```

## Rule Types

| Type | Purpose | Example |
|------|---------|---------|
| **Convention** | How to write code in this project | "Use snake_case for Python functions" |
| **Constraint** | What the agent must never do | "Never commit secrets to git" |
| **Architecture** | Why the system is shaped this way | "We use PostgreSQL because..." |
| **Tool config** | How to run project tooling | "Run `make check` before committing" |

## Rules vs Runbooks vs Memories

| | Rules | Runbooks | Memories |
|---|---|---|---|
| **Content** | Declarative policy | Imperative procedures | Learned lessons |
| **Origin** | Prescribed by developer | Prescribed by developer | Captured from experience |
| **Format** | Sections in instruction file | One procedure per file | One lesson per file with frontmatter |
| **Loading** | Always-on (instruction file) | On-demand (file reference) | On-demand or indexed |
| **Example** | "Use snake_case" | "Step 1: run tests, Step 2: stage..." | "Mocking DB masked a broken migration" |

Rules are the backbone of the instruction file. Runbooks are externalized to save tokens. Memories accumulate over time and may graduate into rules.

## Cross-Harness Compatibility

Rules live in whichever instruction file your tool uses:

| Tool | Instruction File | Notes |
|------|-----------------|-------|
| Claude Code | `CLAUDE.md` / `AGENTS.md` | Per-directory hierarchy, merged at runtime |
| Cursor | `.cursorrules` / `.cursor/rules/*.mdc` | Project-level or scoped rules |
| Windsurf | `.windsurfrules` / `.windsurf/rules/*.md` | Similar to Cursor |
| GitHub Copilot | `.github/copilot-instructions.md` | Scoped with `applyTo` globs |
| Gemini CLI | `GEMINI.md` | Supports `@file.md` imports |
| Aider | `CONVENTIONS.md` | Loaded via `--read` or `.aider.conf.yml` |
| Codex (OpenAI) | `AGENTS.md` | Nested per-directory hierarchy |

The rule *content* is portable -- write it once, copy or symlink across formats.

## Design Principles

1. **Declarative, not imperative.** Rules say *what* and *why*, not step-by-step *how*. Procedures belong in runbooks.
2. **Actionable, not informational.** Every rule should change agent behavior. Large reference tables, code samples, or architecture diagrams belong in separate files.
3. **Portable across tools.** Write rules as plain markdown that works in any instruction file format. Avoid tool-specific syntax in the rule content itself.
4. **Version-controlled.** Rules are code. They evolve with the project and belong in the repo.

## Spec

See [SPEC.md](SPEC.md) for the full format specification, including recommended sections, validation rules, and portability guidelines.

## Related Work

- [Agent Runbooks](https://github.com/btakita/agent-runbooks) -- Convention for externalizing procedures into on-demand runbook files
- [Agent Memories](https://github.com/btakita/agent-memories) -- Convention for committed memories (type, scope, why, how to apply)
- [AGENTS.md](https://agents.md/) -- Universal instruction file spec (Linux Foundation)
- [instruction-files](https://github.com/btakita/instruction-files) -- Discovery, auditing, and sync for AI agent instruction files

## License

[CC0 1.0](LICENSE) -- Public domain. Use this convention however you like.
