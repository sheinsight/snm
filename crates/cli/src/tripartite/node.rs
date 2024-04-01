use clap::Subcommand;
use snm_core::model::SnmError;
use snm_node::node_mg::{
    install_node, list, list_remote, set_default, snm_node_env, un_install_node,
};

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    /// List installed node versions
    List,
    /// List available node versions for installation
    ListRemote {
        #[arg(
            short,
            long,
            // default_value = "true",
            help = "List all available versions"
        )]
        all: bool,
    },
    /// Install a specific node version
    Install {
        #[arg(help = "The package spec to install.")]
        package_spec: String,
    },
    /// Uninstall a specific node version
    Uninstall {
        #[arg(help = "The version to uninstall")]
        version: String,
    },
    /// Switch to use a specific node version
    Use,
    /// Create an alias for a node version
    Alias,
    /// Remove an alias for a node version
    Unalias,
    /// Set a node version as default
    Default {
        #[arg(help = "The version to set as default")]
        version: String,
    },
    /// Display the currently used node version
    Current,
    Env,
}

pub async fn handle_node_commands(command: NodeCommands) -> Result<(), SnmError> {
    match command {
        NodeCommands::List => list().await?,
        NodeCommands::ListRemote { all } => {
            list_remote(all).await?;
        }
        NodeCommands::Install { package_spec } => {
            install_node(package_spec.trim_start_matches('v')).await?;
        }
        NodeCommands::Default { version } => {
            set_default(version.trim_start_matches('v')).await?;
        }
        NodeCommands::Env => snm_node_env().await?,
        NodeCommands::Uninstall { version } => {
            un_install_node(&version).await?;
        }
        NodeCommands::Use => todo!(),
        NodeCommands::Alias => todo!(),
        NodeCommands::Unalias => todo!(),
        NodeCommands::Current => todo!(),
    }
    Ok(())
}
