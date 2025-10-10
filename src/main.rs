use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

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
            if dry_run {
                println!("{}", "[DRY RUN MODE]".yellow().bold());
            }
            if global {
                println!("{}", "Installing global configuration...".green());
                println!("{}", "Not yet implemented".yellow());
            } else {
                println!("{}", "Installing project configuration...".green());
                println!("{}", "Not yet implemented".yellow());
            }
        }
        Commands::Uninstall { global, yes: _ } => {
            if global {
                println!("{}", "Uninstalling global configuration...".green());
                println!("{}", "Not yet implemented".yellow());
            } else {
                println!("{}", "Uninstalling project configuration...".green());
                println!("{}", "Not yet implemented".yellow());
            }
        }
        Commands::Status => {
            println!("{}", "Checking installation status...".green());
            println!("{}", "Not yet implemented".yellow());
        }
        Commands::Update => {
            println!("{}", "Updating hagi...".green());
            println!("{}", "Not yet implemented".yellow());
        }
        Commands::Mcp { command } => match command {
            McpCommands::List => {
                println!("{}", "Listing MCP servers...".green());
                println!("{}", "Not yet implemented".yellow());
            }
            McpCommands::Info { name } => {
                println!("{} {}", "MCP server info:".green(), name);
                println!("{}", "Not yet implemented".yellow());
            }
            McpCommands::Enable { name } => {
                println!("{} {}", "Enabling MCP server:".green(), name);
                println!("{}", "Not yet implemented".yellow());
            }
            McpCommands::Disable { name } => {
                println!("{} {}", "Disabling MCP server:".green(), name);
                println!("{}", "Not yet implemented".yellow());
            }
        },
        Commands::Config { command } => match command {
            ConfigCommands::Show { config_type } => {
                println!("{} {}", "Showing configuration:".green(), config_type);
                println!("{}", "Not yet implemented".yellow());
            }
            ConfigCommands::Edit { config_type } => {
                println!("{} {}", "Editing configuration:".green(), config_type);
                println!("{}", "Not yet implemented".yellow());
            }
            ConfigCommands::Validate { config_type } => {
                println!("{} {}", "Validating configuration:".green(), config_type);
                println!("{}", "Not yet implemented".yellow());
            }
        },
    }

    Ok(())
}
