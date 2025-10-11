use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod templates;
mod utils;

#[derive(Parser)]
#[command(name = "hagi")]
#[command(about = "Claude Code setup tool for Rust/Shell environments", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install hagi configuration (global or project-specific)
    Install {
        /// Install global configuration to ~/.claude/
        #[arg(short, long)]
        global: bool,

        /// Dry run mode (show what would be done without making changes)
        #[arg(long)]
        dry_run: bool,
    },

    /// Uninstall hagi configuration
    Uninstall {
        /// Uninstall global configuration
        #[arg(short, long)]
        global: bool,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Show installation status
    Status,

    /// Update hagi templates and configuration
    Update,

    /// MCP server management commands
    Mcp {
        #[command(subcommand)]
        command: McpCommands,
    },

    /// Configuration management commands
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum McpCommands {
    /// List installed MCP servers
    List,
    /// Show MCP server information
    Info { name: String },
    /// Enable MCP server
    Enable { name: String },
    /// Disable MCP server
    Disable { name: String },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show configuration
    Show {
        /// Configuration type: mcp, global, or hook
        #[arg(value_name = "TYPE")]
        config_type: String,
    },
    /// Edit configuration
    Edit {
        /// Configuration type: mcp, global, or hook
        #[arg(value_name = "TYPE")]
        config_type: String,
    },
    /// Validate configuration
    Validate {
        /// Configuration type: mcp, global, or hook
        #[arg(value_name = "TYPE")]
        config_type: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { global, dry_run } => {
            if global {
                commands::install::install_global(dry_run)?;
            } else {
                commands::install::install_project(dry_run)?;
            }
        }
        Commands::Uninstall { global, yes } => {
            if global {
                commands::uninstall::uninstall_global(yes)?;
            } else {
                commands::uninstall::uninstall_project(yes)?;
            }
        }
        Commands::Status => {
            commands::status::status()?;
        }
        Commands::Update => {
            commands::update::update()?;
        }
        Commands::Mcp { command } => match command {
            McpCommands::List => {
                commands::mcp::list()?;
            }
            McpCommands::Info { name } => {
                commands::mcp::info(&name)?;
            }
            McpCommands::Enable { name } => {
                commands::mcp::enable(&name)?;
            }
            McpCommands::Disable { name } => {
                commands::mcp::disable(&name)?;
            }
        },
        Commands::Config { command } => match command {
            ConfigCommands::Show { config_type } => {
                commands::config::show(&config_type)?;
            }
            ConfigCommands::Edit { config_type } => {
                commands::config::edit(&config_type)?;
            }
            ConfigCommands::Validate { config_type } => {
                commands::config::validate(&config_type)?;
            }
        },
    }

    Ok(())
}
