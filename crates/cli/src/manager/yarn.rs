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
    let snm_yarn = SnmYarn::new();

    match command {
        YarnCommands::Default { version } => {
            snm_yarn.default(&version).await?;
        }
        YarnCommands::Install { version } => {
            snm_yarn.install(&version).await?;
        }
        YarnCommands::Uninstall { version } => {
            snm_yarn.uninstall(&version)?;
        }
        YarnCommands::List => {
            snm_yarn.list()?;
        }
    };
    Ok(())
}
