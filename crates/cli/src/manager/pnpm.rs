use clap::Subcommand;
use snm_core::model::SnmError;
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};
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
    let snm_pnpm = SnmPnpm::new();

    match command {
        PnpmCommands::Default { version } => {
            snm_pnpm.default(&version).await?;
        }
        PnpmCommands::Install { version } => {
            snm_pnpm.install(&version).await?;
        }
        PnpmCommands::Uninstall { version } => {
            snm_pnpm.uninstall(&version)?;
        }
        PnpmCommands::List => {
            snm_pnpm.list()?;
        }
    };
    Ok(())
}
