use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;

use crate::utils;

/// List all MCP servers
pub fn list() -> Result<()> {
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

    println!("{}", "MCP Servers:".green().bold());
    println!();

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
pub fn enable_multiple(names: &[String]) -> Result<()> {
    if names.is_empty() {
        println!("{}", "❌ No MCP server names provided.".red());
        println!("Usage: hagi mcp enable <NAME> [NAME...]");
        return Ok(());
    }

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut env_warnings = Vec::new();

    for name in names {
        match enable_single(name) {
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
        println!("Edit ~/.claude/mcp.json and configure required variables.");
    }

    Ok(())
}

/// Enable a single MCP server (internal function)
fn enable_single(name: &str) -> Result<bool> {
    let claude_dir = utils::claude_dir()?;
    let mcp_path = claude_dir.join("mcp.json");

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
pub fn disable_multiple(names: &[String]) -> Result<()> {
    if names.is_empty() {
        println!("{}", "❌ No MCP server names provided.".red());
        println!("Usage: hagi mcp disable <NAME> [NAME...]");
        return Ok(());
    }

    let mut success_count = 0;
    let mut failed_count = 0;
    let mut critical_warnings = Vec::new();

    for name in names {
        match disable_single(name) {
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
fn disable_single(name: &str) -> Result<bool> {
    let claude_dir = utils::claude_dir()?;
    let mcp_path = claude_dir.join("mcp.json");

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
        "github" => "GitHub integration (issues, PRs, releases)",
        "context7" => "Library documentation and code examples",
        "file-search" => "Fast file search and analysis",
        "serena" => "Code analysis and semantic search",
        _ => "Custom MCP server",
    }
}

/// Check if server requires environment variables
fn needs_env_setup(name: &str) -> bool {
    matches!(name, "github")
}

/// Check if server is critical for recommended workflow
fn is_critical_server(name: &str) -> bool {
    matches!(name, "sequential-thinking" | "context7")
}
