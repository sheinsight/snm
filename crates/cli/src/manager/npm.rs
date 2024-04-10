use clap::Subcommand;
use snm_core::model::{manager::ManagerTraitDispatcher, SnmError};
use snm_npm::snm_npm::SnmNextNpm;

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
    let snm_next_npm = SnmNextNpm::new("npm");
    let m = ManagerTraitDispatcher::new(Box::new(snm_next_npm));

    match command {
        NpmCommands::Default { version } => {
            m.set_default(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        NpmCommands::Install { version } => {
            m.install(version.trim_start_matches(['v', 'V'])).await?;
        }
        NpmCommands::Uninstall { version } => {
            m.un_install(version.trim_start_matches(['v', 'V'])).await?;
        }
        NpmCommands::List => {
            m.list().await?;
        }
    };
    Ok(())
}
