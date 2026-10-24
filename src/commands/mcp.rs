use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use std::path::PathBuf;

use crate::templates;
use crate::utils;

/// Path of the project MCP config (.claude/mcp.json); errors when the project is not set up
fn mcp_config_path() -> Result<PathBuf> {
    let path = utils::project_claude_dir()?.join(templates::MCP_JSON);

    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Project mcp.json not found at {}/{}\n\
             Run 'hagi install' first to set up project configuration.",
            utils::CLAUDE_DIR_NAME,
            templates::MCP_JSON
        ));
    }

    Ok(path)
}

/// List all MCP servers (project configuration only)
///
/// Note: Global MCP configuration is not managed by hagi.
/// MCP servers are configured per-project via .claude/mcp.json (symlinked to .mcp.json).
pub fn list() -> Result<()> {
    println!(
        "{}",
        format!(
            "═══ Project MCP Configuration ({}/{}) ═══",
            utils::CLAUDE_DIR_NAME,
            templates::MCP_JSON
        )
        .cyan()
        .bold()
    );
    println!();

    let Ok(mcp_path) = mcp_config_path() else {
        println!("{}", "❌ Project mcp.json not found.".yellow());
        println!("Run 'hagi install' to set up project configuration.");
        return Ok(());
    };

    let mcp_config = utils::read_json_file(&mcp_path)?;
    if let Some(servers) = mcp_config["mcpServers"].as_object() {
        for (name, config) in servers {
            let disabled = config["disabled"].as_bool().unwrap_or(false);
            let status = if disabled {
                "disabled".red()
            } else {
                "enabled".green()
            };

            let description = get_server_description(name);
            println!("  {} [{}] - {}", name.cyan().bold(), status, description);
        }
    }

    // Check symlink status
    let symlink = std::env::current_dir()
        .context("Failed to get current directory")?
        .join(format!(".{}", templates::MCP_JSON));

    println!();
    if symlink.is_symlink() {
        println!("{}", "✓ .mcp.json symlink exists (Claude Code 2.1+ compatible)".green());
    } else if symlink.exists() {
        println!("{}", "⚠ .mcp.json exists but is not a symlink".yellow());
        println!("  Run 'hagi install' to create proper symlink.");
    } else {
        println!("{}", "⚠ .mcp.json symlink missing".yellow());
        println!("  Run 'hagi install' to create symlink for Claude Code 2.1+.");
    }

    Ok(())
}

/// Show detailed information about a specific MCP server
pub fn info(name: &str) -> Result<()> {
    let Ok(mcp_path) = mcp_config_path() else {
        println!("{}", "❌ Project mcp.json not found.".red());
        println!("Run 'hagi install' first.");
        return Ok(());
    };

    let mcp_config = utils::read_json_file(&mcp_path)?;
    let servers = mcp_config["mcpServers"]
        .as_object()
        .context("Invalid mcp.json structure")?;

    if !servers.contains_key(name) {
        println!("{} {}", "❌ MCP server not found:".red(), name);
        println!();
        println!("Available servers:");
        for server_name in servers.keys() {
            println!("  - {}", server_name.cyan());
        }
        return Ok(());
    }

    let config = &servers[name];
    let disabled = config["disabled"].as_bool().unwrap_or(false);
    let status = if disabled { "disabled" } else { "enabled" };

    println!("{} {}", "MCP Server:".green().bold(), name.cyan().bold());
    println!("{} {}", "Status:".green(), status);

    // Show command
    let Some(command) = config["command"].as_str() else {
        println!("{} {}", "Description:".green(), get_server_description(name));
        return Ok(());
    };

    let mut cmd_str = command.to_string();

    // Add args if present
    if let Some(args) = config["args"].as_array() {
        for arg in args.iter().filter_map(|a| a.as_str()) {
            cmd_str.push_str(&format!(" {}", arg));
        }
    }

    println!("{} {}", "Command:".green(), cmd_str);

    // Show environment variables (hide values)
    if let Some(env_vars) = config["env"].as_object() {
        println!("{}", "Environment:".green());
        for (key, value) in env_vars {
            if let Some(val_str) = value.as_str() {
                if val_str.contains('<') {
                    println!("  {}: {} (not set)", key, val_str.yellow());
                } else {
                    println!("  {}: *** (set)", key);
                }
            }
        }
    }

    println!("{} {}", "Description:".green(), get_server_description(name));

    Ok(())
}

/// Enable multiple MCP servers
pub fn enable_multiple(names: &[String], global: bool) -> Result<()> {
    set_servers(names, global, false)
}

/// Disable multiple MCP servers
pub fn disable_multiple(names: &[String], global: bool) -> Result<()> {
    set_servers(names, global, true)
}

/// Enable or disable servers in the project mcp.json with a single read, backup and write
fn set_servers(names: &[String], global: bool, disabled: bool) -> Result<()> {
    let (verb, progress, past) = if disabled {
        ("disable", "Disabling", "disabled")
    } else {
        ("enable", "Enabling", "enabled")
    };

    if global {
        println!("{}", "❌ Global MCP configuration is not managed by hagi.".red());
        println!(
            "MCP servers are configured per-project via {}/{}.",
            utils::CLAUDE_DIR_NAME,
            templates::MCP_JSON
        );
        println!("Run 'hagi mcp {} <NAME>' without -g flag.", verb);
        return Ok(());
    }

    if names.is_empty() {
        println!("{}", "❌ No MCP server names provided.".red());
        println!("Usage: hagi mcp {} <NAME> [NAME...]", verb);
        return Ok(());
    }

    let mcp_path = mcp_config_path()?;
    let mut mcp_config = utils::read_json_file(&mcp_path)?;
    let servers = mcp_config["mcpServers"]
        .as_object_mut()
        .context("Invalid mcp.json structure")?;

    println!("{} {} MCP servers in project configuration...", "⚙️".cyan(), progress);
    println!();

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut critical_warnings = Vec::new();

    for name in names {
        match set_server(servers, name, disabled) {
            Ok(()) => {
                success_count += 1;
                println!("{} MCP server '{}' {}", "✅".green(), name.cyan().bold(), past);
                if disabled && is_critical_server(name) {
                    critical_warnings.push(name.clone());
                }
            }
            Err(e) => {
                println!("{} {} - {}", "❌".red(), name, e);
                failed_count += 1;
            }
        }
    }

    if success_count > 0 {
        println!();
        utils::backup_file(&mcp_path)?;
        utils::write_json_file(&mcp_path, &mcp_config)?;
    }

    // Summary
    println!();
    if success_count > 0 {
        println!("{} {} server(s) {}.", "✅".green(), success_count, past);
    }
    if failed_count > 0 {
        println!("{} {} server(s) failed.", "❌".red(), failed_count);
    }

    if success_count > 0 {
        println!();
        println!("{}", "Note: Restart Claude Code to apply changes.".yellow());
    }

    if !critical_warnings.is_empty() {
        println!();
        println!("{}", "⚠️ Warning: You disabled recommended server(s):".yellow());
        for name in &critical_warnings {
            println!("  - {}", name.cyan());
        }
        println!("This may affect Claude Code functionality.");
    }

    Ok(())
}

/// Set or clear the `disabled` flag of one server entry
fn set_server(servers: &mut serde_json::Map<String, Value>, name: &str, disabled: bool) -> Result<()> {
    let obj = servers
        .get_mut(name)
        .ok_or_else(|| anyhow::anyhow!("MCP server not found"))?
        .as_object_mut()
        .context("Invalid server entry")?;

    // Enabled is the default, so the flag is removed rather than written as false
    if disabled {
        obj.insert("disabled".to_string(), Value::Bool(true));
    } else {
        obj.remove("disabled");
    }
    Ok(())
}

/// Get description for a known MCP server
fn get_server_description(name: &str) -> &'static str {
    match name {
        "context7" => "Library documentation and code examples",
        "serena" => "Code analysis and semantic search (token-optimized)",
        "memory" => "Long-term memory (SQLite-vec, local)",
        _ => "Custom MCP server",
    }
}

/// Check if server is critical for recommended workflow
fn is_critical_server(name: &str) -> bool {
    matches!(name, "context7")
}
