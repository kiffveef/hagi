use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::path::PathBuf;

use crate::{templates, utils};

/// Install global configuration to ~/.claude/
pub fn install_global(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }

    println!("{}", "Installing global configuration...".green());

    // Get paths
    let claude_dir = utils::claude_dir()?;

    // Create ~/.claude/ directory
    if dry_run {
        println!("{} {}", "Would create:".yellow(), claude_dir.display());
    } else {
        utils::ensure_dir(&claude_dir)?;
    }

    // Install mcp.json
    install_mcp_config(&claude_dir, dry_run)?;

    // Install settings.json (from settings.local.json template, renamed)
    install_settings(&claude_dir, dry_run)?;

    if dry_run {
        println!("{}", "\nDry run completed. No files were modified.".yellow());
    } else {
        println!("{}", "\n✅ Global configuration installed successfully!".green().bold());
        println!("\nNext steps:");
        println!("  1. Restart Claude Code to load the new configuration");
        println!("  2. Run 'hagi install' in your project directory");
    }

    Ok(())
}

/// Install project-specific configuration to .claude/
pub fn install_project(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }

    println!("{}", "Installing project configuration...".green());

    // Get paths
    let project_dir = env::current_dir().context("Failed to get current directory")?;
    let claude_dir = project_dir.join(".claude");

    // Create .claude/ directory
    if dry_run {
        println!("{} {}", "Would create:".yellow(), claude_dir.display());
    } else {
        utils::ensure_dir(&claude_dir)?;
    }

    // Copy all template files (preserving directory structure)
    templates::copy_all_templates(&claude_dir, dry_run)?;

    // Update .gitignore
    update_project_gitignore(&project_dir, dry_run)?;

    if dry_run {
        println!("{}", "\nDry run completed. No files were modified.".yellow());
    } else {
        println!("{}", "\n✅ Project configuration installed successfully!".green().bold());
        println!("\nNext steps:");
        println!("  1. Review .claude/CLAUDE.md for project guidelines");
        println!("  2. Customize .claude/instructions/ as needed");
        println!("  3. Enable additional MCP servers with 'hagi mcp enable <name>'");
    }

    Ok(())
}

/// Install MCP configuration from embedded template
fn install_mcp_config(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let target = claude_dir.join("mcp.json");
    let template_str = templates::get_template("mcp.json")?;
    let template_content: serde_json::Value = serde_json::from_str(template_str)
        .context("Failed to parse mcp.json template")?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: embedded mcp.json");
    } else {
        utils::merge_json_file(&target, &template_content)?;
    }

    Ok(())
}

/// Install settings configuration from embedded template (rename settings.local.json → settings.json)
fn install_settings(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let target = claude_dir.join("settings.json");
    let template_str = templates::get_template("settings.local.json")?;
    let template_content: serde_json::Value = serde_json::from_str(template_str)
        .context("Failed to parse settings.local.json template")?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: embedded settings.local.json → settings.json");
    } else {
        utils::merge_json_file(&target, &template_content)?;
    }

    Ok(())
}

/// Update .gitignore in project root
fn update_project_gitignore(project_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let entries = vec![
        "/.claude/",
        "/.serena/",
    ];

    if dry_run {
        println!("{} {}", "Would update:".yellow(), project_dir.join(".gitignore").display());
        for entry in &entries {
            println!("  {} {}", "Would add:".yellow(), entry);
        }
    } else {
        utils::update_gitignore(project_dir, &entries)?;
    }

    Ok(())
}
