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

    // Install git hooks
    install_git_hooks(&project_dir, dry_run)?;

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
    let template_str = templates::get_template("mcp.json")
        .context("Failed to get embedded mcp.json template")?;

    let mut template_content: serde_json::Value = serde_json::from_str(template_str)
        .with_context(|| {
            format!(
                "Failed to parse mcp.json template. Template may be corrupted.\nFirst 200 chars: {}",
                if template_str.len() > 200 {
                    &template_str[..200]
                } else {
                    template_str
                }
            )
        })?;

    // Expand environment variables (${HOME}, ${XDG_CACHE_HOME:-...}, etc.)
    utils::expand_json_env_vars(&mut template_content).with_context(|| {
        format!(
            "Failed to expand environment variables in mcp.json template.\n\
             This may be due to missing HOME or other required environment variables.\n\
             Template content: {}",
            serde_json::to_string_pretty(&template_content).unwrap_or_else(|_| String::from("<unable to format>"))
        )
    })?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: embedded mcp.json (with environment variables expanded)");
    } else {
        utils::merge_json_file(&target, &template_content)
            .with_context(|| format!("Failed to install mcp.json to {}", target.display()))?;
    }

    Ok(())
}

/// Install settings configuration from embedded template (rename settings.local.json → settings.json)
fn install_settings(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let target = claude_dir.join("settings.json");
    let template_str = templates::get_template("settings.local.json")
        .context("Failed to get embedded settings.local.json template")?;

    let template_content: serde_json::Value = serde_json::from_str(template_str)
        .with_context(|| {
            format!(
                "Failed to parse settings.local.json template. Template may be corrupted.\nFirst 200 chars: {}",
                if template_str.len() > 200 {
                    &template_str[..200]
                } else {
                    template_str
                }
            )
        })?;

    if dry_run {
        if target.exists() {
            println!("{} {}", "Would merge into:".yellow(), target.display());
        } else {
            println!("{} {}", "Would create:".yellow(), target.display());
        }
        println!("  Template: embedded settings.local.json → settings.json");
    } else {
        utils::merge_json_file(&target, &template_content)
            .with_context(|| format!("Failed to install settings.json to {}", target.display()))?;
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

/// Install git hooks to .git/hooks/
fn install_git_hooks(project_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let git_hooks_dir = project_dir.join(".git").join("hooks");

    // Check if .git/hooks directory exists
    if !git_hooks_dir.exists() {
        if dry_run {
            println!("\n{} .git/hooks directory not found, would skip git hooks installation", "⚠".yellow());
        } else {
            println!("\n{} .git/hooks directory not found, skipping git hooks installation", "⚠".yellow());
        }
        return Ok(());
    }

    println!("\n{}", "Installing git hooks...".green());

    // pre-commit hook
    let pre_commit_template = include_str!("../../templates/git-hooks/pre-commit");
    let pre_commit_path = git_hooks_dir.join("pre-commit");

    // commit-msg hook
    let commit_msg_template = include_str!("../../templates/git-hooks/commit-msg");
    let commit_msg_path = git_hooks_dir.join("commit-msg");

    if dry_run {
        println!("{} {}", "Would install:".yellow(), pre_commit_path.display());
        println!("{} {}", "Would install:".yellow(), commit_msg_path.display());
    } else {
        // Install pre-commit
        std::fs::write(&pre_commit_path, pre_commit_template)
            .context("Failed to write pre-commit hook")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&pre_commit_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&pre_commit_path, perms)?;
        }

        // Install commit-msg
        std::fs::write(&commit_msg_path, commit_msg_template)
            .context("Failed to write commit-msg hook")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&commit_msg_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&commit_msg_path, perms)?;
        }

        println!("  ✅ {}", "Git hooks installed".green());
        println!("     {}", "- pre-commit: Prevents direct commits to main/master".dimmed());
        println!("     {}", "- commit-msg: Blocks Claude Code signatures".dimmed());
    }

    Ok(())
}
