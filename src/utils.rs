use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Maximum number of backup files to keep (default)
pub const DEFAULT_MAX_BACKUPS: usize = 3;

/// Get the home directory path
pub fn home_dir() -> Result<PathBuf> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .context("Failed to get HOME environment variable")
}

/// Get the ~/.claude/ directory path
pub fn claude_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".claude"))
}

/// Get the ~/.local/share/claude/ directory path
#[expect(dead_code)]
pub fn claude_data_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".local/share/claude"))
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

/// Create a backup of a file with timestamp
///
/// # Naming Convention
/// - Original: `settings.json`
/// - Backup: `settings.json.backup.20250101_120000`
///
/// This ensures:
/// - Clear identification of backup files
/// - Chronological sorting by timestamp
/// - Easy restoration (remove .backup.TIMESTAMP suffix)
pub fn backup_file(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Ok(PathBuf::new());
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

    Ok(backup_path)
}

/// Clean up old backup files, keeping only the most recent N backups
pub fn cleanup_old_backups(original_path: &Path, max_backups: usize) -> Result<()> {
    if !original_path.exists() {
        return Ok(());
    }

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
    backup_files.sort_by(|a, b| b.1.cmp(&a.1));

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

/// Write JSON file
pub fn write_json_file(path: &Path, value: &serde_json::Value) -> Result<()> {
    let content = serde_json::to_string_pretty(value)
        .context("Failed to serialize JSON")?;

    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    println!("{} {}", "Wrote:".green(), path.display());
    Ok(())
}

/// Merge JSON configuration files (preserving existing configuration)
pub fn merge_json_file(target_path: &Path, new_content: &serde_json::Value) -> Result<()> {
    let mut base = if target_path.exists() {
        println!("{} {}", "Merging into existing:".blue(), target_path.display());
        backup_file(target_path)?;
        // Clean up old backups after creating new one
        cleanup_old_backups(target_path, DEFAULT_MAX_BACKUPS)?;
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

/// Prompt user for confirmation
#[expect(dead_code)]
pub fn confirm(message: &str) -> Result<bool> {
    print!("{} [Y/n]: ", message.yellow());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y" || input == "yes")
}

/// Copy file with message
#[expect(dead_code)]
pub fn copy_file(from: &Path, to: &Path) -> Result<()> {
    fs::copy(from, to).with_context(|| {
        format!(
            "Failed to copy: {} -> {}",
            from.display(),
            to.display()
        )
    })?;

    println!("{} {} -> {}", "Copied:".green(), from.display(), to.display());
    Ok(())
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

/// Expand shell-like environment variable syntax to absolute paths
///
/// Supports the following patterns:
/// - `${HOME}` → actual home directory path
/// - `${XDG_CACHE_HOME:-$HOME/.cache}` → XDG_CACHE_HOME or fallback
/// - `${XDG_DATA_HOME:-$HOME/.local/share}` → XDG_DATA_HOME or fallback
///
/// # Examples
/// ```
/// let expanded = expand_env_vars("${HOME}/.config")?;
/// // Returns: "/home/username/.config"
/// ```
pub fn expand_env_vars(input: &str) -> Result<String> {
    let home = std::env::var("HOME").with_context(|| {
        format!(
            "HOME environment variable not set. Cannot expand: {}",
            input
        )
    })?;

    let xdg_cache = std::env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", home));

    let xdg_data = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", home));

    let mut result = input.to_string();

    // Replace ${HOME}
    result = result.replace("${HOME}", &home);

    // Replace ${XDG_CACHE_HOME:-$HOME/.cache}
    result = result.replace(
        "${XDG_CACHE_HOME:-$HOME/.cache}",
        &xdg_cache
    );

    // Replace ${XDG_DATA_HOME:-$HOME/.local/share}
    result = result.replace(
        "${XDG_DATA_HOME:-$HOME/.local/share}",
        &xdg_data
    );

    Ok(result)
}

/// Recursively expand environment variables in JSON values
///
/// Traverses JSON structure and replaces shell-like environment variable syntax
/// with absolute paths in all string values.
pub fn expand_json_env_vars(value: &mut serde_json::Value) -> Result<()> {
    match value {
        serde_json::Value::String(s) => {
            *s = expand_env_vars(s)
                .with_context(|| format!("Failed to expand environment variables in: {}", s))?;
        }
        serde_json::Value::Array(arr) => {
            for (index, item) in arr.iter_mut().enumerate() {
                expand_json_env_vars(item)
                    .with_context(|| format!("Failed to expand variables in array index {}", index))?;
            }
        }
        serde_json::Value::Object(map) => {
            for (key, v) in map {
                expand_json_env_vars(v)
                    .with_context(|| format!("Failed to expand variables in object key: {}", key))?;
            }
        }
        _ => {}
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
