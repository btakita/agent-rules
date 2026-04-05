mod actionable;

pub use actionable::{check_actionable, check_tree_paths, extract_tree_paths};

use regex::Regex;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// A validation issue found in a rules file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub message: String,
    pub warning: bool,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = if self.warning { "warning" } else { "error" };
        match self.line {
            Some(n) => write!(f, "{}:{}:{}: {}", self.file.display(), n, severity, self.message),
            None => write!(f, "{}:{}: {}", self.file.display(), severity, self.message),
        }
    }
}

static IMPERATIVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(use|add|create|run|do|don't|never|must|should|avoid|prefer|ensure|keep|set)\b")
        .expect("imperative regex")
});

static LOCAL_PATH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(~/|/home/\w+/|/Users/\w+/|/tmp/|[A-Z]:\\Users\\)")
        .expect("local path regex")
});

/// Check that rules have imperative verbs indicating actionable directives.
///
/// Returns an issue for each non-empty, non-heading, non-comment line that
/// lacks a recognised imperative verb.
pub fn validate_actionable(file: &Path, content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // Skip blanks, headings, comments, code fences, and list-continuation indented lines
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("//")
            || trimmed.starts_with("<!--")
            || trimmed.starts_with("```")
        {
            continue;
        }
        // Only check top-level list items and bare sentences
        if !IMPERATIVE_RE.is_match(trimmed) {
            issues.push(Issue {
                file: file.to_path_buf(),
                line: Some(i + 1),
                message: format!("line lacks an imperative verb: {}", truncate(trimmed, 60)),
                warning: true,
            });
        }
    }
    issues
}

/// Check for machine-local paths that should not appear in shared rules.
pub fn validate_no_local_paths(file: &Path, content: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if LOCAL_PATH_RE.is_match(line) {
            issues.push(Issue {
                file: file.to_path_buf(),
                line: Some(i + 1),
                message: format!(
                    "machine-local path detected: {}",
                    truncate(line.trim(), 60)
                ),
                warning: false,
            });
        }
    }
    issues
}

/// Check that the file does not exceed a line budget.
pub fn validate_line_budget(file: &Path, content: &str, budget: usize) -> Option<Issue> {
    let count = content.lines().count();
    if count > budget {
        Some(Issue {
            file: file.to_path_buf(),
            line: None,
            message: format!("file has {} lines, exceeding budget of {}", count, budget),
            warning: true,
        })
    } else {
        None
    }
}

/// Run all validations on a single file's content.
pub fn validate_all(file: &Path, content: &str, line_budget: usize) -> Vec<Issue> {
    let mut issues = Vec::new();
    issues.extend(validate_actionable(file, content));
    issues.extend(validate_no_local_paths(file, content));
    if let Some(issue) = validate_line_budget(file, content, line_budget) {
        issues.push(issue);
    }
    issues
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn actionable_passes_imperative() {
        let content = "- Use clap for CLI parsing\n- Never commit secrets\n";
        let issues = validate_actionable(Path::new("test.md"), content);
        assert!(issues.is_empty(), "expected no issues, got: {:?}", issues);
    }

    #[test]
    fn actionable_flags_non_imperative() {
        let content = "This is a description of the project.\n";
        let issues = validate_actionable(Path::new("test.md"), content);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].warning);
    }

    #[test]
    fn actionable_skips_headings_and_blanks() {
        let content = "# Rules\n\n- Use foo\n";
        let issues = validate_actionable(Path::new("test.md"), content);
        assert!(issues.is_empty());
    }

    #[test]
    fn local_paths_detected() {
        let content = "Config lives at ~/config.toml\nAlso /home/user/stuff\n";
        let issues = validate_no_local_paths(Path::new("test.md"), content);
        assert_eq!(issues.len(), 2);
        assert!(!issues[0].warning);
    }

    #[test]
    fn local_paths_clean() {
        let content = "- Use `$HOME/config.toml` instead\n";
        let issues = validate_no_local_paths(Path::new("test.md"), content);
        assert!(issues.is_empty());
    }

    #[test]
    fn line_budget_within() {
        let content = "line\n".repeat(100);
        assert!(validate_line_budget(Path::new("t.md"), &content, 1000).is_none());
    }

    #[test]
    fn line_budget_exceeded() {
        let content = "line\n".repeat(1001);
        let issue = validate_line_budget(Path::new("t.md"), &content, 1000);
        assert!(issue.is_some());
    }

    #[test]
    fn validate_all_combines() {
        let content = "This has no imperative.\nAlso ~/bad/path\n";
        let issues = validate_all(Path::new("test.md"), content, 1000);
        assert!(issues.len() >= 2);
    }
}
