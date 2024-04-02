use clap::{command, CommandFactory, Parser, Subcommand};

use commands::{
    automatic,
    snm::{AddCommandArgs, InstallCommandArgs},
};
use snm_core::model::{snm_error::handle_snm_error, SnmError};

use manager::{
    node::{handle_node_commands, NodeCommands},
    npm::{handle_npm_commands, NpmCommands},
    pnpm::{handle_pnpm_commands, PnpmCommands},
    yarn::{handle_yarn_commands, YarnCommands},
};

mod commands;
mod manager;

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
        #[command(subcommand)]
        command: PnpmCommands,
    },
    Install(InstallCommandArgs),
    Add(AddCommandArgs),
    FigSpec,
}

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    snm_core::config::init_config()?;

    if let Err(error) = execute_cli().await {
        handle_snm_error(error);
    }

    Ok(())
}

async fn execute_cli() -> Result<(), SnmError> {
    let cli = SnmCli::parse();

    match cli.command {
        Commands::Yarn { command } => {
            handle_yarn_commands(command).await?;
        }
        Commands::Pnpm { command } => {
            handle_pnpm_commands(command).await?;
        }
        Commands::Npm { command } => {
            handle_npm_commands(command).await?;
        }
        Commands::Node { command } => {
            handle_node_commands(command).await?;
        }
        Commands::Install(args) => {
            let package_manager = automatic().await?;
            package_manager.install(args)?;
        }
        Commands::Add(args) => {
            let package_manager = automatic().await?;
            package_manager.add(args)?;
        }
        Commands::FigSpec => clap_complete::generate(
            clap_complete_fig::Fig,
            &mut SnmCli::command(),
            "snm",
            &mut std::io::stdout(),
        ),
    }
    Ok(())
}
