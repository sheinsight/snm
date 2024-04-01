use clap::{command, Parser, Subcommand};

use commands::{
    automatic,
    snm::{AddCommandArgs, InstallCommandArgs},
    yarn::Yarn,
};
use snm_core::model::snm_error::handle_snm_error;

use tripartite::{
    node::{handle_node_commands, NodeCommands},
    npm::{handle_npm_commands, NpmCommands},
    yarn::{handle_yarn_commands, YarnCommands},
};

mod commands;
mod tripartite;

#[derive(Parser, Debug)]
struct SnmCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manager node versions
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },
    /// Manager npm versions
    Npm {
        #[command(subcommand)]
        command: NpmCommands,
    },
    /// Manager yarn versions
    Yarn {
        #[command(subcommand)]
        command: YarnCommands,
    },
    /// Manager pnpm versions
    Pnpm {
        // #[command(subcommand)]
        // command: NpmCommands,
    },
    Install(InstallCommandArgs),
    Add(AddCommandArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    snm_core::config::init_config()?;

    let cli = SnmCli::parse();

    match cli.command {
        Commands::Yarn { command } => {
            if let Err(error) = handle_yarn_commands(command).await {
                handle_snm_error(error)
            }
        }
        Commands::Pnpm {} => {
            println!("Pnpm command not implemented yet");
        }
        Commands::Npm { command } => {
            if let Err(error) = handle_npm_commands(command).await {
                handle_snm_error(error);
            }
        }
        Commands::Node { command } => {
            if let Err(error) = handle_node_commands(command).await {
                handle_snm_error(error);
            };
        }
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
