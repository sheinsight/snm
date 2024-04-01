use clap::Subcommand;
use snm_core::model::SnmError;
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};

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
    let snm_npm = SnmNpm::new(None);

    match command {
        NpmCommands::Default { version } => {
            snm_npm.default(&version).await?;
        }
        NpmCommands::Install { version } => {
            snm_npm.install(&version).await?;
        }
        NpmCommands::Uninstall { version } => {
            snm_npm.uninstall(&version)?;
        }
        NpmCommands::List => {
            snm_npm.list()?;
        }
    };
    Ok(())
}
