use anyhow::{Context, Result};
use colored::*;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

use crate::utils;

/// Embedded template directory
pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/.claude");

/// Get a specific template file content
pub fn get_template(filename: &str) -> Result<&'static str> {
    TEMPLATES
        .get_file(filename)
        .and_then(|f| f.contents_utf8())
        .with_context(|| format!("Template not found: {}", filename))
}

/// Copy all templates to target directory (preserving directory structure)
pub fn copy_all_templates(target_dir: &Path, dry_run: bool) -> Result<()> {
    copy_dir_recursive(&TEMPLATES, target_dir, dry_run)
}

/// Recursively copy directory structure from embedded templates
fn copy_dir_recursive(dir: &Dir, target_base: &Path, dry_run: bool) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(sub_dir) => {
                let target_subdir = target_base.join(sub_dir.path());
                if dry_run {
                    println!("{} {}", "Would create:".yellow(), target_subdir.display());
                } else {
                    utils::ensure_dir(&target_subdir)?;
                }
                copy_dir_recursive(sub_dir, target_base, dry_run)?;
            }
            include_dir::DirEntry::File(file) => {
                let target_file = target_base.join(file.path());
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
