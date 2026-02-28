use anyhow::{Context, Result};
use colored::*;
use include_dir::{include_dir, Dir};
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::utils;

/// Embedded template directory
pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/.claude");

/// Embedded chat template directory
pub static CHAT_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/.chat");

// ============================================================================
// Category System
// ============================================================================

/// Template categories for selective installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Instructions,
    Skills,
    Hooks,
    Config,
    Docs,
    Designs,
}

impl Category {
    /// All valid category names
    pub const ALL: &[Category] = &[
        Category::Instructions,
        Category::Skills,
        Category::Hooks,
        Category::Config,
        Category::Docs,
        Category::Designs,
    ];

    /// Category name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Instructions => "instructions",
            Category::Skills => "skills",
            Category::Hooks => "hooks",
            Category::Config => "config",
            Category::Docs => "docs",
            Category::Designs => "designs",
        }
    }

    /// Check if a template path belongs to this category
    fn matches_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        match self {
            Category::Instructions => path_str.starts_with("instructions/"),
            Category::Skills => path_str.starts_with("skills/"),
            Category::Hooks => path_str.starts_with("hooks/"),
            Category::Config => {
                path_str == "mcp.json" || path_str == "settings.local.json"
            }
            Category::Docs => {
                path_str == "CLAUDE.md" || path_str == "TODO.md"
            }
            Category::Designs => path_str.starts_with("designs/"),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Category::ALL
            .iter()
            .find(|c| c.as_str() == s)
            .copied()
            .ok_or_else(|| {
                let valid: Vec<&str> = Category::ALL.iter().map(|c| c.as_str()).collect();
                anyhow::anyhow!(
                    "Unknown category: '{}'\nValid categories: {}",
                    s,
                    valid.join(", ")
                )
            })
    }
}

/// Filter for selective template installation
#[derive(Debug, Clone, Default)]
pub struct InstallFilter {
    /// Only install these categories (empty = all)
    pub only: Vec<Category>,
    /// Skip these paths (applied on top of `only`)
    pub skip: Vec<String>,
}

impl InstallFilter {
    pub fn new(only: Vec<Category>, skip: Vec<String>) -> Self {
        Self { only, skip }
    }

    /// Whether `--only` was specified
    pub fn has_only(&self) -> bool {
        !self.only.is_empty()
    }

    /// Check if a category is included in the filter
    pub fn includes_category(&self, cat: Category) -> bool {
        if self.only.is_empty() {
            return true;
        }
        self.only.contains(&cat)
    }

    /// Check if a template path should be installed
    fn should_include(&self, path: &Path) -> bool {
        if should_skip(path, &self.skip) {
            return false;
        }

        if self.only.is_empty() {
            return true;
        }

        self.only.iter().any(|cat| cat.matches_path(path))
    }
}

// ============================================================================
// Template Access
// ============================================================================

/// Get a specific template file content
pub fn get_template(filename: &str) -> Result<&'static str> {
    TEMPLATES
        .get_file(filename)
        .and_then(|f| f.contents_utf8())
        .with_context(|| format!("Template not found: {}", filename))
}

// ============================================================================
// Template Copying (with filter)
// ============================================================================

/// Copy templates to target directory with category filter
pub fn copy_all_templates_filtered(target_dir: &Path, dry_run: bool, filter: &InstallFilter) -> Result<()> {
    copy_dir_recursive(&TEMPLATES, target_dir, dry_run, filter)
}

/// Recursively copy directory structure from embedded templates
fn copy_dir_recursive(dir: &Dir, target_base: &Path, dry_run: bool, filter: &InstallFilter) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(sub_dir) => {
                copy_sub_dir(sub_dir, target_base, dry_run, filter)?;
            }
            include_dir::DirEntry::File(file) => {
                copy_file_entry(file, target_base, dry_run, filter)?;
            }
        }
    }
    Ok(())
}


/// Copy a subdirectory entry from embedded templates
fn copy_sub_dir(sub_dir: &Dir, target_base: &Path, dry_run: bool, filter: &InstallFilter) -> Result<()> {
    let relative_path = sub_dir.path();

    if should_skip(relative_path, &filter.skip) {
        let msg = if dry_run { "Would skip directory:" } else { "Skipped directory:" };
        println!("{} {}", msg.yellow(), relative_path.display());
        return Ok(());
    }

    // When --only is active, skip entire directories that don't match any target category
    if filter.has_only() && !dir_has_matching_files(sub_dir, filter) {
        return Ok(());
    }

    let target_subdir = target_base.join(relative_path);
    if dry_run {
        println!("{} {}", "Would create:".yellow(), target_subdir.display());
    } else {
        utils::ensure_dir(&target_subdir)?;
    }

    copy_dir_recursive(sub_dir, target_base, dry_run, filter)
}

/// Check if any file in this directory (recursively) matches the filter
fn dir_has_matching_files(dir: &Dir, filter: &InstallFilter) -> bool {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                if filter.should_include(file.path()) {
                    return true;
                }
            }
            include_dir::DirEntry::Dir(sub_dir) => {
                if dir_has_matching_files(sub_dir, filter) {
                    return true;
                }
            }
        }
    }
    false
}

/// Copy a file entry from embedded templates
fn copy_file_entry(file: &include_dir::File, target_base: &Path, dry_run: bool, filter: &InstallFilter) -> Result<()> {
    let relative_path = file.path();

    if !filter.should_include(relative_path) {
        if should_skip(relative_path, &filter.skip) {
            let msg = if dry_run { "Would skip file:" } else { "Skipped file:" };
            println!("{} {}", msg.yellow(), relative_path.display());
        }
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

// ============================================================================
// Managed Section (CLAUDE.md @instructions/ references)
// ============================================================================

const MANAGED_START: &str = "<!-- hagi:instructions:start -->";
const MANAGED_END: &str = "<!-- hagi:instructions:end -->";

// general.md sets the overall tone and must be read first by Claude
const PRIORITY_INSTRUCTION: &str = "general.md";

/// Build the managed section content from current instruction templates
fn build_managed_instructions_content() -> String {
    let mut instruction_files: Vec<String> = Vec::new();

    if let Some(dir) = TEMPLATES.get_dir("instructions") {
        for entry in dir.entries() {
            if let include_dir::DirEntry::File(file) = entry {
                if let Some(name) = file.path().file_name() {
                    let name_str = name.to_string_lossy().to_string();
                    if name_str != PRIORITY_INSTRUCTION {
                        instruction_files.push(name_str);
                    }
                }
            }
        }
    }

    instruction_files.sort();
    instruction_files.insert(0, PRIORITY_INSTRUCTION.to_string());

    let entries: Vec<String> = instruction_files
        .iter()
        .map(|name| format!("@instructions/{}", name))
        .collect();

    format!(
        "{}\n{}\n{}",
        MANAGED_START,
        entries.join("\n\n"),
        MANAGED_END,
    )
}

/// Update the managed instructions section in an existing CLAUDE.md
///
/// Returns true if the section was found and updated, false if markers not found.
pub fn update_managed_instructions(claude_md_path: &Path, dry_run: bool) -> Result<bool> {
    if !claude_md_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(claude_md_path)
        .with_context(|| format!("Failed to read {}", claude_md_path.display()))?;

    let start_idx = content.find(MANAGED_START);
    let end_idx = content.find(MANAGED_END);

    let (start, end) = match (start_idx, end_idx) {
        (Some(s), Some(e)) if s < e => (s, e),
        _ => {
            println!(
                "{} CLAUDE.md has no managed section markers. Skipping @instructions/ update.",
                "⚠".yellow()
            );
            return Ok(false);
        }
    };

    let new_section = build_managed_instructions_content();
    let end_of_marker = end + MANAGED_END.len();
    let new_content = format!("{}{}{}", &content[..start], new_section, &content[end_of_marker..]);

    if content == new_content {
        println!("{} CLAUDE.md managed section already up to date", "✓".green());
        return Ok(true);
    }

    if dry_run {
        println!("{} CLAUDE.md managed @instructions/ section", "Would update:".yellow());
        return Ok(true);
    }

    utils::backup_file(claude_md_path)?;
    utils::cleanup_old_backups(claude_md_path, utils::DEFAULT_MAX_BACKUPS)?;

    fs::write(claude_md_path, new_content)
        .with_context(|| format!("Failed to write {}", claude_md_path.display()))?;

    println!("{} CLAUDE.md managed @instructions/ section", "Updated:".green());
    Ok(true)
}

// ============================================================================
// Chat Templates (unchanged)
// ============================================================================

/// Copy chat templates to target directory
pub fn copy_chat_templates(target_dir: &Path, dry_run: bool) -> Result<()> {
    let filter = InstallFilter::default();
    copy_dir_recursive(&CHAT_TEMPLATES, target_dir, dry_run, &filter)
}
