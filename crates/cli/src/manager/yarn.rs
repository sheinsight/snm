use clap::Subcommand;
use snm_core::model::SnmError;
use snm_npm::snm_npm::SnmNpmTrait;
use snm_yarn::snm_yarn::SnmYarn;

#[derive(Subcommand, Debug)]
pub enum YarnCommands {
    /// Set default yarn version
    Default {
        #[arg(help = "Need to set the npm version number as the default version.")]
        version: String,
    },
    /// Install yarn
    Install {
        #[arg(help = "The version number of npm to be installed")]
        version: String,
    },
    /// Uninstall yarn
    Uninstall {
        #[arg(help = "The npm version number to be deleted")]
        version: String,
    },
    /// List installed yarn versions
    List,
}

pub async fn handle_yarn_commands(command: YarnCommands) -> Result<(), SnmError> {
    let snm_npm = SnmYarn::new();

    match command {
        YarnCommands::Default { version } => {
            snm_npm.default(&version).await?;
        }
        YarnCommands::Install { version } => {
            snm_npm.install(&version).await?;
        }
        YarnCommands::Uninstall { version } => {
            snm_npm.uninstall(&version)?;
        }
        YarnCommands::List => {
            snm_npm.list()?;
        }
    };
    Ok(())
}
