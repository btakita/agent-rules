use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process;

use agent_rules::validate_all;

#[derive(Parser)]
#[command(name = "agent-rules", about = "Validate AI agent instruction files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a single rules file
    Validate {
        /// Path to the rules file
        file: PathBuf,
        /// Maximum number of lines allowed
        #[arg(long, default_value_t = 1000)]
        line_budget: usize,
    },
    /// Find and validate all instruction files in a directory
    Lint {
        /// Directory to scan
        dir: PathBuf,
        /// Maximum number of lines allowed per file
        #[arg(long, default_value_t = 1000)]
        line_budget: usize,
    },
}

/// Well-known instruction file names.
const INSTRUCTION_FILES: &[&str] = &[
    "CLAUDE.md",
    "CLAUDE.local.md",
    "AGENTS.md",
    "AGENTS.local.md",
    "RULES.md",
    "COPILOT.md",
];

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (issues, file_count) = match cli.command {
        Commands::Validate { file, line_budget } => {
            let content =
                fs::read_to_string(&file).with_context(|| format!("reading {}", file.display()))?;
            let issues = validate_all(&file, &content, line_budget);
            (issues, 1usize)
        }
        Commands::Lint { dir, line_budget } => {
            let files = find_instruction_files(&dir)?;
            let mut all_issues = Vec::new();
            let count = files.len();
            for path in files {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("reading {}", path.display()))?;
                all_issues.extend(validate_all(&path, &content, line_budget));
            }
            (all_issues, count)
        }
    };

    let errors = issues.iter().filter(|i| !i.warning).count();
    let warnings = issues.iter().filter(|i| i.warning).count();

    for issue in &issues {
        eprintln!("{issue}");
    }

    eprintln!(
        "\nScanned {} file(s): {} error(s), {} warning(s)",
        file_count, errors, warnings
    );

    if errors > 0 {
        process::exit(1);
    }

    Ok(())
}

fn find_instruction_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    walk_dir(dir, &mut found)?;
    found.sort();
    Ok(found)
}

fn walk_dir(dir: &PathBuf, found: &mut Vec<PathBuf>) -> Result<()> {
    let entries =
        fs::read_dir(dir).with_context(|| format!("reading directory {}", dir.display()))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Skip hidden dirs and common non-instruction directories
            if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
                continue;
            }
            walk_dir(&path, found)?;
        } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
            && INSTRUCTION_FILES.contains(&file_name)
        {
            found.push(path);
        }
    }
    Ok(())
}
