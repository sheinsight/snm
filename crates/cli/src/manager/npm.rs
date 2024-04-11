use clap::Subcommand;
use snm_core::model::{manager::ManagerTraitDispatcher, SnmError};
use snm_npm::snm_npm::SnmNpm;

#[derive(Subcommand, Debug)]
pub enum NpmCommands {
    /// Set default npm version
    Default {
        #[arg(help = "Need to set the npm version number as the default version.")]
        version: String,
    },
    /// Install npm
    Install {
        #[arg(help = "The version number of npm to be installed")]
        version: String,
    },
    /// Uninstall npm
    Uninstall {
        #[arg(help = "The npm version number to be deleted")]
        version: String,
    },
    /// List installed npm versions
    List,
}

pub async fn handle_npm_commands(command: NpmCommands) -> Result<(), SnmError> {
    let dispatcher = ManagerTraitDispatcher::new(Box::new(SnmNpm::new()));

    match command {
        NpmCommands::Default { version } => {
            dispatcher
                .set_default(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        NpmCommands::Install { version } => {
            dispatcher
                .install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        NpmCommands::Uninstall { version } => {
            dispatcher
                .un_install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        NpmCommands::List => {
            dispatcher.list().await?;
        }
    };
    Ok(())
}
