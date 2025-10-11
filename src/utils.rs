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
#[allow(dead_code)]
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
pub fn backup_file(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Ok(PathBuf::new());
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = path.with_extension(format!(
        "{}.backup.{}",
        path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or(""),
        timestamp
    ));

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
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&backup_pattern) {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            backup_files.push((path, modified));
                        }
                    }
                }
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

/// Merge two JSON values (second value takes precedence for conflicts)
pub fn merge_json(
    base: &mut serde_json::Value,
    overlay: &serde_json::Value,
) -> Result<()> {
    if let (Some(base_obj), Some(overlay_obj)) = (base.as_object_mut(), overlay.as_object()) {
        for (key, value) in overlay_obj {
            if let Some(base_value) = base_obj.get_mut(key) {
                if base_value.is_object() && value.is_object() {
                    merge_json(base_value, value)?;
                } else {
                    *base_value = value.clone();
                }
            } else {
                base_obj.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}

/// Read JSON file
pub fn read_json_file(path: &Path) -> Result<serde_json::Value> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))
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
        backup_file(target_path)?;
        // Clean up old backups after creating new one
        cleanup_old_backups(target_path, DEFAULT_MAX_BACKUPS)?;
        read_json_file(target_path)?
    } else {
        serde_json::json!({})
    };

    merge_json(&mut base, new_content)?;
    write_json_file(target_path, &base)?;

    Ok(())
}

/// Prompt user for confirmation
#[allow(dead_code)]
pub fn confirm(message: &str) -> Result<bool> {
    print!("{} [Y/n]: ", message.yellow());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y" || input == "yes")
}

/// Copy file with message
#[allow(dead_code)]
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
