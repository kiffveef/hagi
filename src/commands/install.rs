use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::path::PathBuf;

use crate::utils;

/// Install global configuration to ~/.claude/
pub fn install_global(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }

    println!("{}", "Installing global configuration...".green());

    // Get paths
    let claude_dir = utils::claude_dir()?;
    let template_dir = get_template_dir()?;

    // Create ~/.claude/ directory
    if dry_run {
        println!("{} {}", "Would create:".yellow(), claude_dir.display());
    } else {
        utils::ensure_dir(&claude_dir)?;
    }

    // Install mcp.json
    install_mcp_config(&claude_dir, &template_dir, dry_run)?;

    // Install settings.json (from settings.local.json template)
    install_settings(&claude_dir, &template_dir, dry_run)?;

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
    let template_dir = get_template_dir()?;

    // Create .claude/ directory
    if dry_run {
        println!("{} {}", "Would create:".yellow(), claude_dir.display());
    } else {
        utils::ensure_dir(&claude_dir)?;
    }

    // Copy template files
    copy_project_templates(&claude_dir, &template_dir, dry_run)?;

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

/// Get the template directory path
fn get_template_dir() -> Result<PathBuf> {
    // Try to find templates directory relative to the executable
    let exe_path = env::current_exe().context("Failed to get executable path")?;

    // When running with cargo run, exe is in target/debug/
    // When installed, exe is in ~/.cargo/bin/
    // Templates should be in the repository root

    // For development: check if we're in the hagi repository
    let mut template_dir = exe_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("templates/.claude"))
        .context("Failed to determine template directory")?;

    if !template_dir.exists() {
        // Try relative to current directory (for cargo run)
        template_dir = PathBuf::from("templates/.claude");
    }

    if !template_dir.exists() {
        anyhow::bail!(
            "Template directory not found: {}\n\
            Please ensure hagi is installed correctly or run from the repository root.",
            template_dir.display()
        );
    }

    Ok(template_dir)
}

/// Install MCP configuration
fn install_mcp_config(
    claude_dir: &PathBuf,
    template_dir: &PathBuf,
    dry_run: bool,
) -> Result<()> {
    let target = claude_dir.join("mcp.json");
    let template = template_dir.join("mcp.json");

    if !template.exists() {
        anyhow::bail!("Template not found: {}", template.display());
    }

    let template_content = utils::read_json_file(&template)?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: {}", template.display());
    } else {
        utils::merge_json_file(&target, &template_content)?;
    }

    Ok(())
}

/// Install settings configuration
fn install_settings(
    claude_dir: &PathBuf,
    template_dir: &PathBuf,
    dry_run: bool,
) -> Result<()> {
    let target = claude_dir.join("settings.json");
    let template = template_dir.join("settings.local.json");

    if !template.exists() {
        anyhow::bail!("Template not found: {}", template.display());
    }

    let template_content = utils::read_json_file(&template)?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: {}", template.display());
    } else {
        utils::merge_json_file(&target, &template_content)?;
    }

    Ok(())
}

/// Copy project template files
fn copy_project_templates(
    claude_dir: &PathBuf,
    template_dir: &PathBuf,
    dry_run: bool,
) -> Result<()> {
    // For now, just copy mcp.json and settings.local.json
    // CLAUDE.md and instructions/ will be added in Phase 2c

    let files = vec![
        ("mcp.json", "mcp.json"),
        ("settings.local.json", "settings.local.json"),
    ];

    for (template_name, target_name) in files {
        let template = template_dir.join(template_name);
        let target = claude_dir.join(target_name);

        if !template.exists() {
            println!(
                "{} Template not found: {} (skipping)",
                "⚠".yellow(),
                template.display()
            );
            continue;
        }

        if dry_run {
            if target.exists() {
                println!("{} {}", "Would overwrite:".yellow(), target.display());
            } else {
                println!("{} {}", "Would create:".yellow(), target.display());
            }
            println!("  From: {}", template.display());
        } else {
            if target.exists() {
                utils::backup_file(&target)?;
            }
            utils::copy_file(&template, &target)?;
        }
    }

    Ok(())
}

/// Update .gitignore in project root
fn update_project_gitignore(project_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let entries = vec![
        ".claude/",
        ".serena/",
        "mcp.json",
        "settings.json",
        "settings.local.json",
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
