# Eval: Actionable Detection

Verify that validation correctly flags informational-only sections in instruction files.

## Pass Cases

- A section containing only imperative rules ("Use snake_case", "Never commit secrets").
- A section with a brief reference table followed by actionable rules.

## Fail Cases

- A 50-line environment variable table with no directive.
- An architecture diagram with no decision statement.
- A changelog or release notes section pasted into the instruction file.

## Expected Behavior

The agent reports each informational-only section with its line number and suggests externalizing the content to a separate file.
