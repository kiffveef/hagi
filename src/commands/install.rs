use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::{templates, utils};

/// Install global configuration to ~/.claude/
pub fn install_global(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }

    println!("{}", "Installing global configuration...".green());

    // Check dependencies
    let warnings = check_dependencies();
    if !warnings.is_empty() {
        println!("\n{}", "⚠ Warning: Missing dependencies".yellow().bold());
        for warning in &warnings {
            println!("\n{}", warning);
        }
        println!();
    }

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
pub fn install_project(dry_run: bool, skip_paths: &[String]) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }

    println!("{}", "Installing project configuration...".green());

    // Check if current directory is a git repository
    if !is_git_repository() {
        println!("\n{}", "⚠ Not a git repository. Initializing git...".yellow());
        initialize_git_repository(dry_run)?;
        println!();
    }

    // Show skip list if provided
    if !skip_paths.is_empty() {
        println!("\n{}", "Skipping the following paths:".yellow());
        for path in skip_paths {
            println!("  - {}", path);
        }
        println!();
    }

    // Get paths
    let project_dir = env::current_dir().context("Failed to get current directory")?;
    let claude_dir = project_dir.join(".claude");

    // Create .claude/ directory
    if dry_run {
        println!("{} {}", "Would create:".yellow(), claude_dir.display());
    } else {
        utils::ensure_dir(&claude_dir)?;
    }

    // Copy template files with skip list
    templates::copy_all_templates_with_skip(&claude_dir, dry_run, skip_paths)?;

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
    let mut template_content: serde_json::Value = serde_json::from_str(template_str)
        .context("Failed to parse mcp.json template")?;

    // Expand environment variables (${HOME}, ${XDG_CACHE_HOME:-...}, etc.)
    utils::expand_json_env_vars(&mut template_content)
        .context("Failed to expand environment variables in mcp.json")?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: embedded mcp.json (with environment variables expanded)");
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

/// Check if current directory is a git repository
fn is_git_repository() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Initialize git repository in current directory
fn initialize_git_repository(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "  Would run: git init".yellow());
        println!("{}", "  Would run: git commit --allow-empty -m \"🌱 init\"".yellow());
        return Ok(());
    }

    // Initialize git repository
    let status = Command::new("git")
        .arg("init")
        .status()
        .context("Failed to run git init")?;

    if !status.success() {
        anyhow::bail!("git init failed");
    }

    println!("{}", "  ✅ Git repository initialized".green());

    // Create empty initial commit
    let commit_status = Command::new("git")
        .args(["commit", "--allow-empty", "-m", "🌱 init"])
        .status()
        .context("Failed to run git commit")?;

    if !commit_status.success() {
        anyhow::bail!("git commit failed");
    }

    println!("{}", "  ✅ Initial commit created".green());
    Ok(())
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check dependencies and show warnings
fn check_dependencies() -> Vec<String> {
    let mut warnings = Vec::new();

    // Node.js check
    if !command_exists("node") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -\n    sudo apt-get install -y nodejs",
            "Node.js not found".red(),
            "Required for: context7, sequential-thinking, one-search",
            "Install"
        ));
    }

    // uv check
    if !command_exists("uv") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    curl -LsSf https://astral.sh/uv/install.sh | sh",
            "uv not found".red(),
            "Required for: mcp-memory-service",
            "Install"
        ));
    }

    // Python3 check
    if !command_exists("python3") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    sudo apt-get update\n    sudo apt-get install -y python3 python3-pip",
            "Python3 not found".red(),
            "Required for: mcp-memory-service setup",
            "Install"
        ));
    }

    // Git check
    if !command_exists("git") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    sudo apt-get install -y git",
            "Git not found".red(),
            "Required for: mcp-memory-service clone",
            "Install"
        ));
    }

    warnings
}
