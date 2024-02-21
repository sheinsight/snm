use clap::{command, Parser, Subcommand};
use colored::*;
use commands::{
    automatic,
    snm::{AddCommandArgs, InstallCommandArgs},
};
use snm_core::{model::snm_error::handle_snm_error, println_success};
use snm_node::node_mg::{
    install_node, list, list_remote, set_default, snm_node_env, un_install_node,
};
use std::io::stdout;

mod commands;

#[derive(Parser, Debug)]
struct SnmCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Node module management
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },
    Install(InstallCommandArgs),
    Add(AddCommandArgs),
}

#[derive(Subcommand, Debug)]
enum NodeCommands {
    /// List installed node versions
    List,
    /// List available node versions for installation
    ListRemote {
        #[arg(
            short,
            long,
            // default_value = "true",
            help = "List all available versions"
        )]
        all: bool,
    },
    /// Install a specific node version
    Install {
        #[arg(help = "The package spec to install.")]
        package_spec: String,
    },
    /// Uninstall a specific node version
    Uninstall {
        #[arg(help = "The version to uninstall")]
        version: String,
    },
    /// Switch to use a specific node version
    Use,
    /// Create an alias for a node version
    Alias,
    /// Remove an alias for a node version
    Unalias,
    /// Set a node version as default
    Default {
        #[arg(help = "The version to set as default")]
        version: String,
    },
    /// Display the currently used node version
    Current,
    Env,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    snm_core::config::init_config()?;

    let cli = SnmCli::parse();

    let mut stdout = stdout();

    match cli.command {
        Commands::Node { command } => match command {
            NodeCommands::List => {
                if let Err(e) = list().await {
                    handle_snm_error(e);
                }
            }
            NodeCommands::ListRemote { all } => {
                if let Err(e) = list_remote(all).await {
                    handle_snm_error(e);
                }
            }
            NodeCommands::Install { package_spec } => {
                match install_node(package_spec.trim_start_matches('v')).await {
                    Ok(_) => {
                        println_success!(stdout, "{} installed successfully", package_spec.green())
                    }
                    Err(e) => handle_snm_error(e),
                }
            }
            NodeCommands::Default { version } => {
                match set_default(version.trim_start_matches('v')).await {
                    Ok(_) => {
                        println_success!(stdout, "Default Node version set to {}", version.green())
                    }
                    Err(e) => handle_snm_error(e),
                }
            }
            NodeCommands::Env => {
                if let Err(e) = snm_node_env().await {
                    handle_snm_error(e)
                }
            }
            NodeCommands::Uninstall { version } => {
                if let Err(e) = un_install_node(&version).await {
                    handle_snm_error(e)
                }
            }
            NodeCommands::Use => todo!(),
            NodeCommands::Alias => todo!(),
            NodeCommands::Unalias => todo!(),
            NodeCommands::Current => todo!(),
        },
        Commands::Install(args) => {
            let package_manager = automatic().await?;
            package_manager.install(args)?;
        }
        Commands::Add(args) => {
            let package_manager = automatic().await?;
            package_manager.add(args)?;
        }
    }
    Ok(())
}
