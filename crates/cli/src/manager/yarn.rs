use clap::Subcommand;
use snm_core::model::{manager::ManagerDispatcher, SnmError};
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
    let dispatcher = ManagerDispatcher::new(Box::new(SnmYarn::new()));
    match command {
        YarnCommands::Default { version } => {
            dispatcher
                .set_default(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        YarnCommands::Install { version } => {
            dispatcher
                .install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        YarnCommands::Uninstall { version } => {
            dispatcher
                .un_install(version.trim_start_matches(['v', 'V']))
                .await?;
        }
        YarnCommands::List => {
            dispatcher.list().await?;
        }
    };
    Ok(())
}
