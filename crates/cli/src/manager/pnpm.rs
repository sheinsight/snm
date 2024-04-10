use clap::Subcommand;
use snm_core::model::{manager::ManagerTraitDispatcher, SnmError};
use snm_npm::snm_npm::SnmNextNpm;

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
    let snm_next_npm = SnmNextNpm::new("pnpm");
    let m = ManagerTraitDispatcher::new(Box::new(snm_next_npm));

    match command {
        PnpmCommands::Default { version } => {
            m.set_default(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        PnpmCommands::Install { version } => {
            m.install(version.trim_start_matches(['v', 'V'])).await?;
        }
        PnpmCommands::Uninstall { version } => {
            m.un_install(version.trim_start_matches(['v', 'V'])).await?;
        }
        PnpmCommands::List => {
            m.list().await?;
        }
    };
    Ok(())
}
