use anyhow::{Context, Result};
use colored::*;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

use crate::utils;

/// Embedded template directory
pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/.claude");

/// Embedded chat template directory
pub static CHAT_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/.chat");

/// Get a specific template file content
pub fn get_template(filename: &str) -> Result<&'static str> {
    TEMPLATES
        .get_file(filename)
        .and_then(|f| f.contents_utf8())
        .with_context(|| format!("Template not found: {}", filename))
}

/// Copy all templates to target directory with skip list
pub fn copy_all_templates_with_skip(target_dir: &Path, dry_run: bool, skip_paths: &[String]) -> Result<()> {
    copy_dir_recursive(&TEMPLATES, target_dir, dry_run, skip_paths)
}

/// Recursively copy directory structure from embedded templates
fn copy_dir_recursive(dir: &Dir, target_base: &Path, dry_run: bool, skip_paths: &[String]) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(sub_dir) => {
                copy_sub_dir(sub_dir, target_base, dry_run, skip_paths)?;
            }
            include_dir::DirEntry::File(file) => {
                copy_file_entry(file, target_base, dry_run, skip_paths)?;
            }
        }
    }
    Ok(())
}


/// Copy a subdirectory entry from embedded templates
fn copy_sub_dir(sub_dir: &Dir, target_base: &Path, dry_run: bool, skip_paths: &[String]) -> Result<()> {
    let relative_path = sub_dir.path();

    if should_skip(relative_path, skip_paths) {
        let msg = if dry_run { "Would skip directory:" } else { "Skipped directory:" };
        println!("{} {}", msg.yellow(), relative_path.display());
        return Ok(());
    }

    let target_subdir = target_base.join(relative_path);
    if dry_run {
        println!("{} {}", "Would create:".yellow(), target_subdir.display());
    } else {
        utils::ensure_dir(&target_subdir)?;
    }

    copy_dir_recursive(sub_dir, target_base, dry_run, skip_paths)
}

/// Copy a file entry from embedded templates
fn copy_file_entry(file: &include_dir::File, target_base: &Path, dry_run: bool, skip_paths: &[String]) -> Result<()> {
    let relative_path = file.path();

    if should_skip(relative_path, skip_paths) {
        let msg = if dry_run { "Would skip file:" } else { "Skipped file:" };
        println!("{} {}", msg.yellow(), relative_path.display());
        return Ok(());
    }

    let target_file = target_base.join(relative_path);
    let content = file
        .contents_utf8()
        .context("Template file is not valid UTF-8")?;

    if dry_run {
        let action = if target_file.exists() { "Would overwrite:" } else { "Would create:" };
        println!("{} {}", action.yellow(), target_file.display());
        return Ok(());
    }

    // Backup existing file
    if target_file.exists() {
        utils::backup_file(&target_file)?;
        utils::cleanup_old_backups(&target_file, utils::DEFAULT_MAX_BACKUPS)?;
    }

    // Ensure parent directory exists
    if let Some(parent) = target_file.parent() {
        utils::ensure_dir(parent)?;
    }

    fs::write(&target_file, content).with_context(|| {
        format!("Failed to write template file: {}", target_file.display())
    })?;

    println!("{} {}", "Wrote:".green(), target_file.display());

    Ok(())
}

/// Check if a path should be skipped based on skip_paths
fn should_skip(path: &Path, skip_paths: &[String]) -> bool {
    for skip in skip_paths {
        let skip_path = Path::new(skip);

        // Match exact file name or directory name
        if path == skip_path {
            return true;
        }

        // Match if path starts with skip_path (directory prefix)
        if path.starts_with(skip_path) {
            return true;
        }

        // Match file name only (for convenience)
        if let Some(file_name) = path.file_name()
            && file_name.to_string_lossy() == skip.as_str()
        {
            return true;
        }
    }
    false
}


/// Copy chat templates to target directory
pub fn copy_chat_templates(target_dir: &Path, dry_run: bool) -> Result<()> {
    copy_dir_recursive_simple(&CHAT_TEMPLATES, target_dir, dry_run)
}

/// Recursively copy directory structure from embedded templates (simple version without skip)
fn copy_dir_recursive_simple(dir: &Dir, target_base: &Path, dry_run: bool) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(sub_dir) => {
                let relative_path = sub_dir.path();
                let target_subdir = target_base.join(relative_path);
                if dry_run {
                    println!("{} {}", "Would create:".yellow(), target_subdir.display());
                } else {
                    utils::ensure_dir(&target_subdir)?;
                }
                copy_dir_recursive_simple(sub_dir, target_base, dry_run)?;
            }
            include_dir::DirEntry::File(file) => {
                let relative_path = file.path();
                let target_file = target_base.join(relative_path);
                let content = file
                    .contents_utf8()
                    .context("Template file is not valid UTF-8")?;

                if dry_run {
                    if target_file.exists() {
                        println!("{} {}", "Would overwrite:".yellow(), target_file.display());
                    } else {
                        println!("{} {}", "Would create:".yellow(), target_file.display());
                    }
                } else {
                    if target_file.exists() {
                        utils::backup_file(&target_file)?;
                        utils::cleanup_old_backups(&target_file, utils::DEFAULT_MAX_BACKUPS)?;
                    }
                    if let Some(parent) = target_file.parent() {
                        utils::ensure_dir(parent)?;
                    }
                    fs::write(&target_file, content).with_context(|| {
                        format!("Failed to write template file: {}", target_file.display())
                    })?;
                    println!("{} {}", "Wrote:".green(), target_file.display());
                }
            }
        }
    }
    Ok(())
}
