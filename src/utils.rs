use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Number of timestamped backups kept next to a file that hagi rewrites
const MAX_BACKUPS: usize = 1;

/// Name of the Claude Code config directory, both under $HOME and inside a project
pub const CLAUDE_DIR_NAME: &str = ".claude";

/// Get the home directory path
pub fn home_dir() -> Result<PathBuf> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .context("Failed to get HOME environment variable")
}

/// Get the ~/.claude/ directory path
pub fn claude_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(CLAUDE_DIR_NAME))
}

/// Get the current project's .claude/ directory path (whether or not it exists)
pub fn project_claude_dir() -> Result<PathBuf> {
    let project_dir = std::env::current_dir().context("Failed to get current directory")?;
    Ok(project_dir.join(CLAUDE_DIR_NAME))
}

/// Name of the private repository that `hagi sync` uses for this project's .claude/
pub fn claude_repo_name() -> String {
    format!("{}-claude", get_repository_name())
}

/// Create a directory if it doesn't exist
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        println!("{} {}", "Created:".green(), path.display());
    } else {
        println!("{} {}", "Exists:".blue(), path.display());
    }
    Ok(())
}

/// Back up a file as `<name>.backup.<timestamp>` and drop older backups beyond MAX_BACKUPS
///
/// Missing files are a no-op so callers can back up unconditionally before writing.
pub fn backup_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .with_context(|| format!("Invalid filename: {}", path.display()))?;

    let backup_filename = format!("{}.backup.{}", filename, timestamp);
    let backup_path = path.with_file_name(backup_filename);

    fs::copy(path, &backup_path).with_context(|| {
        format!(
            "Failed to create backup: {} -> {}",
            path.display(),
            backup_path.display()
        )
    })?;

    println!(
        "{} {} -> {}",
        "Backed up:".yellow(),
        path.display(),
        backup_path.display()
    );

    cleanup_old_backups(path, MAX_BACKUPS)
}

/// Remove backup files of `original_path` beyond the most recent `max_backups`
fn cleanup_old_backups(original_path: &Path, max_backups: usize) -> Result<()> {
    let parent_dir = original_path
        .parent()
        .context("Failed to get parent directory")?;

    let filename = original_path
        .file_name()
        .and_then(|f| f.to_str())
        .context("Invalid filename")?;

    // Find all backup files for this original file
    let backup_pattern = format!("{}.backup.", filename);
    let mut backup_files: Vec<(PathBuf, SystemTime)> = Vec::new();

    if let Ok(entries) = fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str())
                && name.starts_with(&backup_pattern)
                && let Ok(metadata) = fs::metadata(&path)
                && let Ok(modified) = metadata.modified()
            {
                backup_files.push((path, modified));
            }
        }
    }

    // Sort by modification time (newest first)
    backup_files.sort_by_key(|(_, modified)| std::cmp::Reverse(*modified));

    // Remove old backups beyond max_backups
    let to_remove: Vec<_> = backup_files
        .iter()
        .skip(max_backups)
        .map(|(path, _)| path)
        .collect();

    for path in to_remove {
        fs::remove_file(path)
            .with_context(|| format!("Failed to remove old backup: {}", path.display()))?;
        println!("{} {}", "Removed old backup:".dimmed(), path.display());
    }

    if backup_files.len() > max_backups {
        println!(
            "{} Kept {} most recent backups for {}",
            "Cleaned:".green(),
            max_backups,
            filename
        );
    }

    Ok(())
}

/// Merge two JSON values recursively (overlay takes precedence for conflicts)
///
/// # Behavior
/// - Objects are merged recursively
/// - Arrays and primitives in overlay replace base values
/// - New keys in overlay are added to base
///
/// # Examples
/// - Base: {"a": 1, "b": {"c": 2}}
/// - Overlay: {"b": {"c": 3, "d": 4}, "e": 5}
/// - Result: {"a": 1, "b": {"c": 3, "d": 4}, "e": 5}
pub fn merge_json(
    base: &mut serde_json::Value,
    overlay: &serde_json::Value,
) -> Result<()> {
    if let (Some(base_obj), Some(overlay_obj)) = (base.as_object_mut(), overlay.as_object()) {
        for (key, value) in overlay_obj {
            if let Some(base_value) = base_obj.get_mut(key) {
                // If both are objects, merge recursively
                if base_value.is_object() && value.is_object() {
                    merge_json(base_value, value)?;
                } else {
                    // Otherwise, overlay value replaces base value
                    *base_value = value.clone();
                }
            } else {
                // New key from overlay - add to base
                base_obj.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}

/// Read JSON file with detailed error reporting
pub fn read_json_file(path: &Path) -> Result<serde_json::Value> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    serde_json::from_str(&content).with_context(|| {
        // Show first 500 chars of content for debugging
        let preview = if content.len() > 500 {
            format!("{}...", &content[..500])
        } else {
            content.clone()
        };
        format!(
            "Failed to parse JSON in file: {}\nContent preview:\n{}",
            path.display(),
            preview
        )
    })
}

/// Write a text file, creating parent directories; in dry-run mode only report what would happen
pub fn write_file(path: &Path, content: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        let action = if path.exists() { "Would overwrite:" } else { "Would create:" };
        println!("{} {}", action.yellow(), path.display());
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    println!("{} {}", "Wrote:".green(), path.display());
    Ok(())
}

/// Write JSON file
pub fn write_json_file(path: &Path, value: &serde_json::Value) -> Result<()> {
    let content = serde_json::to_string_pretty(value)
        .context("Failed to serialize JSON")?;
    write_file(path, &content, false)
}

/// Merge JSON configuration files (preserving existing configuration)
pub fn merge_json_file(target_path: &Path, new_content: &serde_json::Value) -> Result<()> {
    let mut base = if target_path.exists() {
        println!("{} {}", "Merging into existing:".blue(), target_path.display());
        backup_file(target_path)?;
        read_json_file(target_path)
            .with_context(|| format!("Failed to read existing JSON file: {}", target_path.display()))?
    } else {
        println!("{} {}", "Creating new:".green(), target_path.display());
        serde_json::json!({})
    };

    merge_json(&mut base, new_content)
        .with_context(|| format!("Failed to merge JSON configurations for: {}", target_path.display()))?;

    write_json_file(target_path, &base)
        .with_context(|| format!("Failed to write merged JSON to: {}", target_path.display()))?;

    Ok(())
}

/// Parse "major.minor[.patch]" into (major, minor); patch is ignored for compatibility checks
pub fn major_minor(version: &str) -> Option<(u64, u64)> {
    let mut parts = version.trim().split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    Some((major, minor))
}

/// Outcome of comparing the version recorded in `.claude/hagi-version` with the running hagi
#[derive(Debug, PartialEq)]
pub enum VersionCheck {
    UpToDate,
    /// The project's templates predate the installed hagi
    TemplatesBehind,
    /// The templates came from a newer hagi (e.g. via `hagi sync` from another machine)
    BinaryBehind,
    Unrecognized,
}

impl VersionCheck {
    /// Command that resolves the mismatch, if there is one
    pub fn fix(&self) -> Option<&'static str> {
        match self {
            VersionCheck::TemplatesBehind => Some("hagi install --refresh"),
            VersionCheck::BinaryBehind => Some("hagi update"),
            VersionCheck::UpToDate | VersionCheck::Unrecognized => None,
        }
    }
}

/// Minor differences matter, patch differences do not (hagi bumps minor for Claude Code follow-ups)
pub fn check_version(recorded: &str, current: &str) -> VersionCheck {
    match (major_minor(recorded), major_minor(current)) {
        (Some(r), Some(c)) if c > r => VersionCheck::TemplatesBehind,
        (Some(r), Some(c)) if c < r => VersionCheck::BinaryBehind,
        (Some(_), Some(_)) => VersionCheck::UpToDate,
        _ => VersionCheck::Unrecognized,
    }
}

/// Ask a yes/no question; Enter and "y" mean yes
pub fn confirm(message: &str) -> Result<bool> {
    print!("{} [Y/n]: ", message.yellow());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y" || input == "yes")
}

/// Add lines to .gitignore if they don't exist
pub fn update_gitignore(project_dir: &Path, entries: &[&str]) -> Result<()> {
    let gitignore_path = project_dir.join(".gitignore");

    let mut content = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)
            .context("Failed to read .gitignore")?
    } else {
        String::new()
    };

    let mut added = Vec::new();
    for entry in entries {
        if !content.lines().any(|line| line.trim() == *entry) {
            content.push_str(&format!("\n{}", entry));
            added.push(*entry);
        }
    }

    if !added.is_empty() {
        fs::write(&gitignore_path, content)
            .context("Failed to write .gitignore")?;

        println!("{} {}", "Updated:".green(), gitignore_path.display());
        for entry in added {
            println!("  {} {}", "+".green(), entry);
        }
    } else {
        println!("{} {} (no changes needed)", "Checked:".blue(), gitignore_path.display());
    }

    Ok(())
}

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get repository name from git remote or current directory
pub fn get_repository_name() -> String {
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output();

    if let Ok(output) = output
        && output.status.success()
    {
        let url = String::from_utf8_lossy(&output.stdout);
        if let Some(name) = url.split('/').next_back() {
            return name.trim_end_matches(".git\n").trim().to_string();
        }
    }

    std::env::current_dir()
        .ok()
        .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "myproject".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_minor_ignores_patch_and_rejects_garbage() {
        assert_eq!(major_minor("0.5.1\n"), Some((0, 5)));
        assert_eq!(major_minor("1.2"), Some((1, 2)));
        assert_eq!(major_minor("hagi"), None);
        assert_eq!(major_minor("1"), None);
        assert!(major_minor("0.5.0") > major_minor("0.4.9"));
    }

    #[test]
    fn test_check_version_outcomes() {
        assert_eq!(check_version("0.4.1\n", "0.5.0"), VersionCheck::TemplatesBehind);
        assert_eq!(check_version("0.6.0", "0.5.0"), VersionCheck::BinaryBehind);
        assert_eq!(check_version("0.5.3", "0.5.0"), VersionCheck::UpToDate);
        assert_eq!(check_version("garbage", "0.5.0"), VersionCheck::Unrecognized);
        assert_eq!(VersionCheck::TemplatesBehind.fix(), Some("hagi install --refresh"));
        assert_eq!(VersionCheck::UpToDate.fix(), None);
    }

    #[test]
    fn test_merge_json() {
        let mut base = serde_json::json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": 3
            }
        });

        let overlay = serde_json::json!({
            "b": {
                "c": 20,
                "e": 4
            },
            "f": 5
        });

        merge_json(&mut base, &overlay).unwrap();

        assert_eq!(base["a"], 1);
        assert_eq!(base["b"]["c"], 20);
        assert_eq!(base["b"]["d"], 3);
        assert_eq!(base["b"]["e"], 4);
        assert_eq!(base["f"], 5);
    }
}
