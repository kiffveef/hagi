use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::templates;
use crate::utils;

/// Uninstall global configuration from ~/.claude/
pub fn uninstall_global(skip_confirm: bool) -> Result<()> {
    println!("{}", "Uninstalling global configuration...".yellow());

    let claude_dir = match utils::claude_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("{} ~/.claude/ not found", "✗".red());
            return Ok(());
        }
    };

    // Only settings.json is installed globally; MCP config is per-project
    let settings_json = claude_dir.join(templates::SETTINGS_JSON);
    let display_name = format!("~/{}/{}", utils::CLAUDE_DIR_NAME, templates::SETTINGS_JSON);

    if !settings_json.exists() {
        println!("{} No global configuration found", "○".dimmed());
        return Ok(());
    }

    // Show what will be removed
    println!("\n{}", "The following files will be removed:".yellow());
    println!("  - {}", display_name);

    // Confirmation prompt
    if !skip_confirm && !utils::confirm("\nProceed with uninstallation?")? {
        println!("{}", "Aborted.".dimmed());
        return Ok(());
    }

    match fs::remove_file(&settings_json) {
        Ok(_) => println!("{} Removed {}", "✓".green(), display_name),
        Err(e) => println!("{} Failed to remove {}: {}", "✗".red(), display_name, e),
    }

    // Try to remove ~/.claude/ if empty
    if let Ok(entries) = fs::read_dir(&claude_dir)
        && entries.count() == 0
    {
        if let Err(e) = fs::remove_dir(&claude_dir) {
            println!("{} Could not remove empty ~/.claude/: {}", "○".dimmed(), e);
        } else {
            println!("{} Removed empty ~/.claude/", "✓".green());
        }
    }

    println!("{}", "\n✅ Global configuration uninstalled successfully!".green().bold());
    Ok(())
}

/// Uninstall project-specific configuration from .claude/
pub fn uninstall_project(skip_confirm: bool) -> Result<()> {
    println!("{}", "Uninstalling project configuration...".yellow());

    let claude_dir = utils::project_claude_dir()?;
    let project_dir = claude_dir
        .parent()
        .context("Failed to get project directory")?;

    if !claude_dir.exists() {
        println!("{} .claude/ not found", "✗".red());
        return Ok(());
    }

    // Show what will be removed
    println!("\n{}", "The following will be removed:".yellow());
    println!("  - .claude/ directory and all its contents");
    println!("  - hagi-related patterns from .gitignore");

    // Confirmation prompt
    if !skip_confirm && !utils::confirm("\nProceed with uninstallation?")? {
        println!("{}", "Aborted.".dimmed());
        return Ok(());
    }

    // Remove .claude/ directory
    match fs::remove_dir_all(&claude_dir) {
        Ok(_) => println!("{} Removed .claude/", "✓".green()),
        Err(e) => {
            println!("{} Failed to remove .claude/: {}", "✗".red(), e);
            return Err(e.into());
        }
    }

    // Clean up .gitignore
    cleanup_gitignore(project_dir)?;

    println!("{}", "\n✅ Project configuration uninstalled successfully!".green().bold());
    Ok(())
}

/// Remove hagi-related patterns from .gitignore
fn cleanup_gitignore(project_dir: &std::path::Path) -> Result<()> {
    let gitignore_path = project_dir.join(".gitignore");

    if !gitignore_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&gitignore_path)
        .context("Failed to read .gitignore")?;

    let hagi_patterns = [
        "/.claude/",
        "/.serena/",
    ];

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let original_len = lines.len();

    // Remove hagi-related patterns
    lines.retain(|line| {
        let trimmed = line.trim();
        !hagi_patterns.contains(&trimmed)
    });

    if lines.len() < original_len {
        // Write back cleaned .gitignore
        let new_content = lines.join("\n");
        fs::write(&gitignore_path, new_content)
            .context("Failed to write .gitignore")?;
        println!("{} Cleaned up .gitignore", "✓".green());
    }

    Ok(())
}
