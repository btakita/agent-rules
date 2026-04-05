# Eval: Tree Path Validation

Verify that validation catches file paths referenced in rules that do not exist on disk.

## Pass Cases

- Rule references `./src/` and the directory exists.
- Rule references `.agent/runbooks/precommit.md` and the file exists.

## Fail Cases

- Rule references `./scripts/deploy.sh` but the file does not exist.
- Rule references a runbook path that was renamed or deleted.
- Rule contains a machine-local absolute path (`~/projects/myapp/bin/lint`).

## Expected Behavior

The agent reports each broken path with its line number, distinguishing between missing relative paths (likely a bug) and absolute paths (violation of the "no machine-local paths" validation rule).
