use clap::Subcommand;
use snm_core::model::{manager::ManagerTraitDispatcher, SnmError};
use snm_npm::snm_npm::SnmNpm;
use snm_pnpm::snm_pnpm::SnmPnpm;

#[derive(Subcommand, Debug)]
pub enum PnpmCommands {
    /// Set default pnpm version
    Default {
        #[arg(help = "Need to set the npm version number as the default version.")]
        version: String,
    },
    /// Install pnpm
    Install {
        #[arg(help = "The version number of npm to be installed")]
        version: String,
    },
    /// Uninstall pnpm
    Uninstall {
        #[arg(help = "The npm version number to be deleted")]
        version: String,
    },
    /// List installed pnpm versions
    List,
}

pub async fn handle_pnpm_commands(command: PnpmCommands) -> Result<(), SnmError> {
    let dispatcher = ManagerTraitDispatcher::new(Box::new(SnmPnpm::new()));

    match command {
        PnpmCommands::Default { version } => {
            dispatcher
                .set_default(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        PnpmCommands::Install { version } => {
            dispatcher
                .install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        PnpmCommands::Uninstall { version } => {
            dispatcher
                .un_install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        PnpmCommands::List => {
            dispatcher.list().await?;
        }
    };
    Ok(())
}
