use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use std::path::PathBuf;

use crate::utils;

/// Get MCP config path based on scope (global or project-local)
fn get_mcp_config_path(global: bool) -> Result<PathBuf> {
    if global {
        let claude_dir = utils::claude_dir()?;
        Ok(claude_dir.join("mcp.json"))
    } else {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        let local_mcp = current_dir.join(".claude").join("mcp.json");

        if !local_mcp.exists() {
            return Err(anyhow::anyhow!(
                "Project-local mcp.json not found at .claude/mcp.json\n\
                 Run 'hagi install' first to set up project configuration."
            ));
        }

        Ok(local_mcp)
    }
}

/// List all MCP servers (project configuration only)
///
/// Note: Global MCP configuration is not managed by hagi.
/// MCP servers are configured per-project via .claude/mcp.json (symlinked to .mcp.json).
pub fn list() -> Result<()> {
    let local_path_result = std::env::current_dir()
        .map(|d| d.join(".claude").join("mcp.json"));

    // Display project configuration
    println!("{}", "═══ Project MCP Configuration (.claude/mcp.json) ═══".cyan().bold());
    println!();

    if let Ok(local_path) = local_path_result {
        if !local_path.exists() {
            println!("{}", "❌ Project mcp.json not found.".yellow());
            println!("Run 'hagi install' to set up project configuration.");
        } else {
            let mcp_config = utils::read_json_file(&local_path)?;
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
            let symlink_path = std::env::current_dir()
                .map(|d| d.join(".mcp.json"))
                .ok();

            println!();
            if let Some(ref symlink) = symlink_path {
                if symlink.is_symlink() {
                    println!("{}", "✓ .mcp.json symlink exists (Claude Code 2.1+ compatible)".green());
                } else if symlink.exists() {
                    println!("{}", "⚠ .mcp.json exists but is not a symlink".yellow());
                    println!("  Run 'hagi install' to create proper symlink.");
                } else {
                    println!("{}", "⚠ .mcp.json symlink missing".yellow());
                    println!("  Run 'hagi install' to create symlink for Claude Code 2.1+.");
                }
            }
        }
    } else {
        println!("{}", "❌ Could not determine current directory.".red());
    }

    Ok(())
}

/// Show detailed information about a specific MCP server
pub fn info(name: &str) -> Result<()> {
    let claude_dir = utils::claude_dir()?;
    let mcp_path = claude_dir.join("mcp.json");

    if !mcp_path.exists() {
        println!("{}", "❌ mcp.json not found.".red());
        println!("Run 'hagi install --global' first.");
        return Ok(());
    }

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
    if let Some(command) = config["command"].as_str() {
        let mut cmd_str = command.to_string();

        // Add args if present
        if let Some(args) = config["args"].as_array() {
            for arg in args {
                if let Some(arg_str) = arg.as_str() {
                    cmd_str.push_str(&format!(" {}", arg_str));
                }
            }
        }

        println!("{} {}", "Command:".green(), cmd_str);
    }

    // Show environment variables (hide values)
    if let Some(env_vars) = config["env"].as_object() {
        println!("{}", "Environment:".green());
        for (key, value) in env_vars {
            if let Some(val_str) = value.as_str() {
                if val_str.contains("<your-") || val_str.contains("<") {
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
    if names.is_empty() {
        println!("{}", "❌ No MCP server names provided.".red());
        println!("Usage: hagi mcp enable <NAME> [NAME...]");
        return Ok(());
    }

    let scope_label = if global { "global" } else { "project-local" };
    println!("{} Enabling MCP servers in {} configuration...", "⚙️".cyan(), scope_label);
    println!();

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut env_warnings = Vec::new();

    for name in names {
        match enable_single(name, global) {
            Ok(needs_env) => {
                success_count += 1;
                if needs_env {
                    env_warnings.push(name.clone());
                }
            }
            Err(e) => {
                println!("{} {} - {}", "❌".red(), name, e);
                failed_count += 1;
            }
        }
    }

    // Summary
    println!();
    if success_count > 0 {
        println!("{} {} server(s) enabled.", "✅".green(), success_count);
    }
    if failed_count > 0 {
        println!("{} {} server(s) failed.", "❌".red(), failed_count);
    }

    if success_count > 0 {
        println!();
        println!("{}", "Note: Restart Claude Code to apply changes.".yellow());
    }

    // Environment variable warnings
    if !env_warnings.is_empty() {
        println!();
        println!("{}", "⚠️ Warning: The following servers require environment variables:".yellow());
        for name in &env_warnings {
            println!("  - {}", name.cyan());
        }
        let config_path = if global { "~/.claude/mcp.json" } else { ".claude/mcp.json" };
        println!("Edit {} and configure required variables.", config_path);
    }

    Ok(())
}

/// Enable a single MCP server (internal function)
fn enable_single(name: &str, global: bool) -> Result<bool> {
    let mcp_path = get_mcp_config_path(global)?;

    if !mcp_path.exists() {
        return Err(anyhow::anyhow!("mcp.json not found. Run 'hagi install --global' first."));
    }

    let mut mcp_config = utils::read_json_file(&mcp_path)?;
    let servers = mcp_config["mcpServers"]
        .as_object_mut()
        .context("Invalid mcp.json structure")?;

    if !servers.contains_key(name) {
        return Err(anyhow::anyhow!("MCP server not found"));
    }

    // Backup before modification (only on first server in batch)
    static BACKUP_DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !BACKUP_DONE.swap(true, std::sync::atomic::Ordering::SeqCst) {
        utils::backup_file(&mcp_path)?;
        utils::cleanup_old_backups(&mcp_path, utils::DEFAULT_MAX_BACKUPS)?;
    }

    // Remove disabled field or set to false
    if let Some(server_config) = servers.get_mut(name) {
        if let Some(obj) = server_config.as_object_mut() {
            obj.remove("disabled");
        }
    }

    utils::write_json_file(&mcp_path, &mcp_config)?;

    println!("{} MCP server '{}' enabled", "✅".green(), name.cyan().bold());

    // Return whether this server needs env setup
    Ok(needs_env_setup(name))
}

/// Disable multiple MCP servers
pub fn disable_multiple(names: &[String], global: bool) -> Result<()> {
    if names.is_empty() {
        println!("{}", "❌ No MCP server names provided.".red());
        println!("Usage: hagi mcp disable <NAME> [NAME...]");
        return Ok(());
    }

    let scope_label = if global { "global" } else { "project-local" };
    println!("{} Disabling MCP servers in {} configuration...", "⚙️".cyan(), scope_label);
    println!();

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut critical_warnings = Vec::new();

    for name in names {
        match disable_single(name, global) {
            Ok(is_critical) => {
                success_count += 1;
                if is_critical {
                    critical_warnings.push(name.clone());
                }
            }
            Err(e) => {
                println!("{} {} - {}", "❌".red(), name, e);
                failed_count += 1;
            }
        }
    }

    // Summary
    println!();
    if success_count > 0 {
        println!("{} {} server(s) disabled.", "✅".green(), success_count);
    }
    if failed_count > 0 {
        println!("{} {} server(s) failed.", "❌".red(), failed_count);
    }

    if success_count > 0 {
        println!();
        println!("{}", "Note: Restart Claude Code to apply changes.".yellow());
    }

    // Critical server warnings
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

/// Disable a single MCP server (internal function)
fn disable_single(name: &str, global: bool) -> Result<bool> {
    let mcp_path = get_mcp_config_path(global)?;

    if !mcp_path.exists() {
        return Err(anyhow::anyhow!("mcp.json not found. Run 'hagi install --global' first."));
    }

    let mut mcp_config = utils::read_json_file(&mcp_path)?;
    let servers = mcp_config["mcpServers"]
        .as_object_mut()
        .context("Invalid mcp.json structure")?;

    if !servers.contains_key(name) {
        return Err(anyhow::anyhow!("MCP server not found"));
    }

    // Backup before modification (only on first server in batch)
    static BACKUP_DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !BACKUP_DONE.swap(true, std::sync::atomic::Ordering::SeqCst) {
        utils::backup_file(&mcp_path)?;
        utils::cleanup_old_backups(&mcp_path, utils::DEFAULT_MAX_BACKUPS)?;
    }

    // Set disabled to true
    if let Some(server_config) = servers.get_mut(name) {
        if let Some(obj) = server_config.as_object_mut() {
            obj.insert("disabled".to_string(), Value::Bool(true));
        }
    }

    utils::write_json_file(&mcp_path, &mcp_config)?;

    println!("{} MCP server '{}' disabled", "✅".green(), name.cyan().bold());

    // Return whether this is a critical server
    Ok(is_critical_server(name))
}

/// Get description for a known MCP server
fn get_server_description(name: &str) -> &'static str {
    match name {
        "sequential-thinking" => "Structured thinking and problem-solving",
        "git" => "Git operations and repository management",
        "context7" => "Library documentation and code examples",
        "serena" => "Code analysis and semantic search (token-optimized)",
        "one-search" => "Web search (DuckDuckGo, SearXNG)",
        "memory" => "Long-term memory (SQLite-vec, local)",
        _ => "Custom MCP server",
    }
}

/// Check if server requires environment variables
fn needs_env_setup(_name: &str) -> bool {
    false
}

/// Check if server is critical for recommended workflow
fn is_critical_server(name: &str) -> bool {
    matches!(name, "sequential-thinking" | "context7")
}
