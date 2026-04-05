# Eval: Cross-Tool Portability

Verify that rules written for one tool's instruction file work correctly across all supported tools.

## Pass Cases

- Plain markdown rules with relative paths and no tool-specific syntax.
- Rules using standard markdown features (headings, lists, code blocks, tables).

## Fail Cases

- Rules containing Cursor-specific `@file` references.
- Rules using Windsurf frontmatter tags in the rule body.
- Rules using Copilot `applyTo` globs inline instead of in the wrapper format.

## Expected Behavior

The agent flags tool-specific syntax in rule content and suggests portable alternatives. Tool-specific syntax is acceptable only in wrapper/configuration sections, not in the rule text itself.
