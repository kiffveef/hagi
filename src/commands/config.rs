use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::utils;

/// Show configuration file content
pub fn show(config_type: &str) -> Result<()> {
    let config_path = get_config_path(config_type)?;

    if !config_path.exists() {
        println!(
            "{} Configuration file not found: {}",
            "❌".red(),
            config_path.display()
        );
        println!("Run 'hagi install --global' first.");
        return Ok(());
    }

    let content = fs::read_to_string(&config_path).with_context(|| {
        format!("Failed to read configuration file: {}", config_path.display())
    })?;

    println!("{} {}", "Configuration:".green().bold(), config_type.cyan());
    println!("{} {}", "File:".green(), config_path.display());
    println!();
    println!("{}", content);

    Ok(())
}

/// Validate configuration file (JSON syntax check)
pub fn validate(config_type: &str) -> Result<()> {
    let config_path = get_config_path(config_type)?;

    if !config_path.exists() {
        println!(
            "{} Configuration file not found: {}",
            "❌".red(),
            config_path.display()
        );
        println!("Run 'hagi install --global' first.");
        return Ok(());
    }

    match utils::read_json_file(&config_path) {
        Ok(_) => {
            println!(
                "{} Configuration is valid: {}",
                "✅".green(),
                config_path.display()
            );
        }
        Err(e) => {
            println!(
                "{} Configuration is invalid: {}",
                "❌".red(),
                config_path.display()
            );
            println!();
            println!("{} {}", "Error:".red().bold(), e);
            println!();
            println!("{}", "Tip: Use 'jq' to format and validate JSON manually:".yellow());
            println!("  jq . {}", config_path.display());
        }
    }

    Ok(())
}

/// Edit configuration file with $EDITOR
pub fn edit(config_type: &str) -> Result<()> {
    let config_path = get_config_path(config_type)?;

    if !config_path.exists() {
        println!(
            "{} Configuration file not found: {}",
            "❌".red(),
            config_path.display()
        );
        println!("Run 'hagi install --global' first.");
        return Ok(());
    }

    // Get editor from environment (default: vim)
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        println!("{}", "$EDITOR not set, using vim".yellow());
        "vim".to_string()
    });

    // Create backup before editing
    utils::backup_file(&config_path)?;
    utils::cleanup_old_backups(&config_path, utils::DEFAULT_MAX_BACKUPS)?;

    println!("{} {}", "Opening with:".green(), editor);
    println!();

    // Launch editor
    let status = Command::new(&editor)
        .arg(&config_path)
        .status()
        .with_context(|| format!("Failed to launch editor: {}", editor))?;

    if !status.success() {
        println!();
        println!("{}", "⚠️ Editor exited with error".yellow());
        return Ok(());
    }

    println!();
    println!("{}", "✅ Configuration edited successfully!".green());
    println!();
    println!("{}", "Tip: Validate your changes with:".yellow());
    println!("  hagi config validate {}", config_type);
    println!();
    println!("{}", "Note: Restart Claude Code to apply changes.".yellow());

    Ok(())
}

/// Get configuration file path based on type
fn get_config_path(config_type: &str) -> Result<PathBuf> {
    let claude_dir = utils::claude_dir()?;

    let path = match config_type {
        "mcp" => claude_dir.join("mcp.json"),
        "global" => claude_dir.join("settings.json"),
        "hook" => {
            println!("{}", "❌ Hook configuration is not yet supported".red());
            println!("This feature will be added in a future release.");
            return Err(anyhow::anyhow!("Unsupported config type"));
        }
        _ => {
            println!("{} Unknown config type: {}", "❌".red(), config_type);
            println!();
            println!("Available config types:");
            println!("  - {} (MCP server configuration)", "mcp".cyan());
            println!("  - {} (Global Claude settings)", "global".cyan());
            println!("  - {} (Hooks - coming soon)", "hook".dimmed());
            return Err(anyhow::anyhow!("Unknown config type"));
        }
    };

    Ok(path)
}
