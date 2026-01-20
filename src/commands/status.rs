use anyhow::{Context, Result};
use colored::*;
use std::collections::{HashMap, HashSet};
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
            println!("{} ~/.claude/ not found", "✗".red());
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
    let files = [
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

    // Check skills directory (v0.2.0+)
    let skills_dir = claude_dir.join("skills");
    if skills_dir.exists() && skills_dir.is_dir() {
        let count = std::fs::read_dir(&skills_dir)
            .map(|entries| entries.filter_map(Result::ok).count())
            .unwrap_or(0);
        println!("  {} .claude/skills/ ({} files)", "✓".green(), count);
    } else {
        println!("  {} .claude/skills/ - {}", "✗".red(), "not found".dimmed());
    }

    Ok(())
}

/// Check MCP server configuration
fn check_mcp_servers() -> Result<()> {
    println!("{}", "[MCP Servers]".cyan().bold());

    // Read global configuration
    let global_config = if let Ok(claude_dir) = utils::claude_dir() {
        let global_mcp = claude_dir.join("mcp.json");
        if global_mcp.exists() {
            Some(utils::read_json_file(&global_mcp)?)
        } else {
            None
        }
    } else {
        None
    };

    // Read project-local configuration
    let project_dir = env::current_dir().context("Failed to get current directory")?;
    let local_mcp = project_dir.join(".claude/mcp.json");
    let local_config = if local_mcp.exists() {
        Some(utils::read_json_file(&local_mcp)?)
    } else {
        None
    };

    if global_config.is_none() && local_config.is_none() {
        println!("{} No MCP configuration found", "✗".red());
        println!("\nRun {} to install MCP configuration", "hagi install --global".yellow());
        return Ok(());
    }

    // Compare configurations and show differences
    println!();
    println!("{}", "Global vs Local Configuration:".yellow().bold());
    println!();

    // Extract server status from both configs
    let global_servers = extract_server_status(&global_config);
    let local_servers = extract_server_status(&local_config);

    // Get all server names from both configs
    let all_servers: HashSet<String> = global_servers
        .keys()
        .chain(local_servers.keys())
        .cloned()
        .collect();

    let mut differences_found = false;

    for name in all_servers.iter() {
        let global_enabled = global_servers.get(name).copied().unwrap_or(false);
        let local_enabled = local_servers.get(name).copied().unwrap_or(false);

        if global_enabled != local_enabled {
            differences_found = true;
            let global_status = if global_enabled { "enabled".green() } else { "disabled".red() };
            let local_status = if local_enabled { "enabled".green() } else { "disabled".red() };
            println!("  {} {} [global: {}, local: {}]",
                "⚠".yellow(), name.cyan(), global_status, local_status);
        } else {
            let status = if global_enabled { "enabled".green() } else { "disabled".dimmed() };
            println!("  {} {} [{}]", "✓".dimmed(), name.cyan(), status);
        }
    }

    if !differences_found {
        println!();
        println!("{}", "✓ No configuration differences between global and local".green());
    }

    Ok(())
}

/// Extract server names and their enabled status from config
fn extract_server_status(config: &Option<serde_json::Value>) -> HashMap<String, bool> {
    let mut result = HashMap::new();

    if let Some(config) = config
        && let Some(servers) = config.get("mcpServers").and_then(|v| v.as_object())
    {
        for (name, server_config) in servers {
            let is_enabled = !server_config
                .get("disabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            result.insert(name.clone(), is_enabled);
        }
    }

    result
}
