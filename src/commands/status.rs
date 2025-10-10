use anyhow::{Context, Result};
use colored::*;
use std::env;

use crate::utils;

/// Show installation status
pub fn status() -> Result<()> {
    println!("{}", "Checking hagi installation status...".green().bold());
    println!();

    // Check global configuration
    check_global_configuration()?;
    println!();

    // Check project configuration
    check_project_configuration()?;
    println!();

    // Check MCP servers
    check_mcp_servers()?;

    Ok(())
}

/// Check global configuration status
fn check_global_configuration() -> Result<()> {
    println!("{}", "[Global Configuration]".cyan().bold());

    let claude_dir = match utils::claude_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("{} {}", "✗".red(), "~/.claude/ not found");
            return Ok(());
        }
    };

    // Check ~/.claude/mcp.json
    let mcp_json = claude_dir.join("mcp.json");
    if mcp_json.exists() {
        println!("{} {} - {}", "✓".green(), "~/.claude/mcp.json".bold(), "installed".dimmed());
    } else {
        println!("{} {} - {}", "✗".red(), "~/.claude/mcp.json".bold(), "not found".dimmed());
    }

    // Check ~/.claude/settings.json
    let settings_json = claude_dir.join("settings.json");
    if settings_json.exists() {
        println!("{} {} - {}", "✓".green(), "~/.claude/settings.json".bold(), "installed".dimmed());
    } else {
        println!("{} {} - {}", "✗".red(), "~/.claude/settings.json".bold(), "not found".dimmed());
    }

    Ok(())
}

/// Check project configuration status
fn check_project_configuration() -> Result<()> {
    println!("{}", "[Project Configuration]".cyan().bold());

    let project_dir = env::current_dir().context("Failed to get current directory")?;
    let claude_dir = project_dir.join(".claude");

    if !claude_dir.exists() {
        println!("{} {} - {}", "✗".red(), ".claude/".bold(), "not found".dimmed());
        println!("\nRun {} to install project configuration", "hagi install".yellow());
        return Ok(());
    }

    println!("{} {} - {}", "✓".green(), ".claude/".bold(), "installed".dimmed());

    // Check key template files
    let files = vec![
        ("CLAUDE.md", claude_dir.join("CLAUDE.md")),
        ("mcp.json", claude_dir.join("mcp.json")),
        ("settings.local.json", claude_dir.join("settings.local.json")),
    ];

    for (name, path) in files {
        if path.exists() {
            println!("  {} .claude/{}", "✓".green(), name);
        } else {
            println!("  {} .claude/{} - {}", "✗".red(), name, "not found".dimmed());
        }
    }

    // Check instructions directory
    let instructions_dir = claude_dir.join("instructions");
    if instructions_dir.exists() && instructions_dir.is_dir() {
        let count = std::fs::read_dir(&instructions_dir)
            .map(|entries| entries.filter_map(Result::ok).count())
            .unwrap_or(0);
        println!("  {} .claude/instructions/ ({} files)", "✓".green(), count);
    } else {
        println!("  {} .claude/instructions/ - {}", "✗".red(), "not found".dimmed());
    }

    // Check commands directory
    let commands_dir = claude_dir.join("commands");
    if commands_dir.exists() && commands_dir.is_dir() {
        let count = std::fs::read_dir(&commands_dir)
            .map(|entries| entries.filter_map(Result::ok).count())
            .unwrap_or(0);
        println!("  {} .claude/commands/ ({} files)", "✓".green(), count);
    } else {
        println!("  {} .claude/commands/ - {}", "✗".red(), "not found".dimmed());
    }

    Ok(())
}

/// Check MCP server configuration
fn check_mcp_servers() -> Result<()> {
    println!("{}", "[MCP Servers]".cyan().bold());

    // Try to read global mcp.json first, then project mcp.json
    let mcp_config_path = if let Ok(claude_dir) = utils::claude_dir() {
        let global_mcp = claude_dir.join("mcp.json");
        if global_mcp.exists() {
            global_mcp
        } else {
            let project_dir = env::current_dir().context("Failed to get current directory")?;
            project_dir.join(".claude/mcp.json")
        }
    } else {
        let project_dir = env::current_dir().context("Failed to get current directory")?;
        project_dir.join(".claude/mcp.json")
    };

    if !mcp_config_path.exists() {
        println!("{} {}", "✗".red(), "No MCP configuration found");
        println!("\nRun {} to install MCP configuration", "hagi install --global".yellow());
        return Ok(());
    }

    // Read and parse MCP configuration
    let mcp_config = utils::read_json_file(&mcp_config_path)?;
    let servers = mcp_config
        .get("mcpServers")
        .and_then(|v| v.as_object())
        .context("Invalid MCP configuration format")?;

    let mut enabled_servers = Vec::new();
    let mut disabled_servers = Vec::new();

    for (name, config) in servers {
        let is_disabled = config
            .get("disabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_disabled {
            disabled_servers.push(name.clone());
        } else {
            enabled_servers.push(name.clone());
        }
    }

    // Display enabled servers
    if !enabled_servers.is_empty() {
        println!("\n{}", "Enabled:".green());
        for name in enabled_servers {
            println!("  {} {}", "✓".green(), name);
        }
    }

    // Display disabled servers
    if !disabled_servers.is_empty() {
        println!("\n{}", "Disabled:".dimmed());
        for name in disabled_servers {
            println!("  {} {}", "○".dimmed(), name);
        }
    }

    if servers.is_empty() {
        println!("{} {}", "○".dimmed(), "No MCP servers configured");
    }

    Ok(())
}
