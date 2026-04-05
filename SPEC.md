# agent-rules spec

Format specification for prescribed policy in AI agent instruction files. Rules are declarative statements that shape agent behavior -- conventions, constraints, architecture decisions, and tool configuration.

## File Locations

Rules live in the instruction file for your tool:

| Location | Tool(s) |
|----------|---------|
| `CLAUDE.md` | Claude Code |
| `AGENTS.md` | Codex, Claude Code |
| `.cursorrules` | Cursor |
| `.cursor/rules/*.mdc` | Cursor (scoped) |
| `.windsurfrules` | Windsurf |
| `.windsurf/rules/*.md` | Windsurf (scoped) |
| `.github/copilot-instructions.md` | GitHub Copilot |
| `GEMINI.md` | Gemini CLI |
| `CONVENTIONS.md` | Aider |

Per-directory instruction files (e.g., `src/api/CLAUDE.md`) override or extend root-level rules for that subtree.

## Recommended Sections

Organize rules into these sections within your instruction file:

### Conventions

Coding standards and patterns the agent should follow.

```markdown
## Conventions

- Use snake_case for Python modules and functions.
- Prefer `Result<T, E>` over panicking in library code.
- Name test files `*_test.go`, not `test_*.go`.
```

**Guideline:** Start each rule with an imperative verb (use, prefer, name, write, keep).

### Constraints

Hard boundaries the agent must not cross.

```markdown
## Constraints

- Never commit `.env` files or cleartext secrets.
- Never use `unsafe` without a `// SAFETY:` comment.
- Do not add dependencies without checking the license.
```

**Guideline:** Start with "Never", "Do not", or "Always" to signal the rule is non-negotiable.

### Architecture

Decisions that explain why the system is shaped the way it is.

```markdown
## Architecture

- PostgreSQL for persistence (JSONB support, mature ecosystem).
- Event sourcing for the order pipeline; CRUD for user management.
- Monorepo with workspace packages under `packages/`.
```

**Guideline:** Include the *why* in parentheses or a short clause. This prevents the agent from suggesting alternatives that violate existing decisions.

### Tool Configuration

How to invoke project tooling.

```markdown
## Tool Configuration

- Lint: `cargo clippy --all-targets`
- Test: `make test` (runs unit + integration)
- Pre-commit: follow `.agent/runbooks/precommit.md`
```

**Guideline:** Keep commands concrete. Reference runbooks for multi-step procedures instead of inlining them.

### Project Structure

Brief layout reference so the agent knows where things live.

```markdown
## Project Structure

- `src/` -- application source
- `tests/` -- integration tests
- `migrations/` -- database migrations (never modify existing ones)
```

**Guideline:** Only include paths that affect agent behavior. This is not documentation -- it is navigation context.

## Validation Rules

When auditing instruction files for rule quality:

### 1. Actionable content

Every rule should contain an imperative verb that directs agent behavior. Informational content (tables of data, architecture diagrams, API reference) should be externalized to separate files and referenced.

**Pass:** "Use snake_case for all Python functions."
**Fail:** A 50-line table of environment variables with no directive.

### 2. Line budget

Combined instruction files in a project should stay under 1000 lines. Beyond that, externalize procedures to runbooks and reference material to separate files.

### 3. No machine-local paths

Rules must not contain paths that only resolve on one machine:

**Fail:** `~/projects/myapp/bin/lint`
**Pass:** `./bin/lint` or `make lint`

### 4. No large code blocks

Code blocks over 10 lines suggest a procedure or reference that belongs in a runbook or external file, not inline in the instruction file.

### 5. Declarative, not imperative

Rules state policy. If a rule reads like a numbered procedure ("Step 1... Step 2..."), it belongs in a runbook.

**Fail:** "1. Run `make test`. 2. Check the output. 3. If tests pass, run `make build`."
**Pass:** "Run `make check` before committing (see `.agent/runbooks/precommit.md`)."

## Portability Guidelines

### Write tool-agnostic content

Rule content should be plain markdown that works in any instruction file. Avoid tool-specific syntax (e.g., Cursor's `@file` references, Windsurf's frontmatter tags) in the rule text itself. Use tool-specific wrappers around portable content.

### Use relative paths

Reference project files with relative paths (`./src/`, `tests/`), not absolute paths. This keeps rules valid across machines and CI environments.

### Separate policy from procedure

Keep the instruction file focused on *what* and *why*. Move *how* into runbooks. This makes the instruction file portable -- procedures often reference tool-specific commands, but policy transfers cleanly.

### Keep rules atomic

Each rule should be a single, self-contained statement. Compound rules ("Use snake_case and also run the linter and make sure tests pass") should be split into separate items.

## Relationship to Other Specs

| Spec | Content Type | Format |
|------|-------------|--------|
| **agent-rules** (this spec) | Prescribed policy | Sections in instruction file |
| **[agent-runbooks](https://github.com/btakita/agent-runbooks)** | Imperative procedures | One file per procedure |
| **[agent-memories](https://github.com/btakita/agent-memories)** | Learned lessons | One file per memory with frontmatter |

Together these three specs cover the full lifecycle: rules prescribe policy upfront, runbooks detail procedures on demand, and memories capture lessons from experience.
