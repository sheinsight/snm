use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::node_manager::node_manager::{ListArgs, ListRemoteArgs, NodeManager};
use crate::snm_command::SnmCommands;
use crate::SnmCli;
use anyhow::bail;
use snm_atom::node_atom::NodeAtom;
use snm_config::SnmConfig;
use snm_package_json::ops::ops::InstallArgs;
use snm_package_json::package_json::PackageJson;
use snm_package_json::pm::PackageManager;
use snm_utils::exec::exec_cli;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
    match cli.command {
        // manage start
        SnmCommands::Node { command } => {
            // let node = SNode::try_from(&snm_config)?;
            let nm = snm_node_version::manager::NodeManager::new(&snm_config);
            let node_atom = NodeAtom::new(snm_config.clone());
            let node_manager = NodeManager::new(&node_atom);
            match command {
                ManageCommands::Default(args) => {
                    // node_manager.set_default(version.as_str()).await?;
                    nm.set_default(args).await?;
                }
                ManageCommands::Install(args) => {
                    // node_manager.install(version.as_str()).await?;
                    nm.install(args).await?;
                }
                ManageCommands::Uninstall(args) => {
                    // node_manager.un_install(version.as_str()).await?;
                    nm.uninstall(args).await?;
                }
                ManageCommands::List(args) => {
                    // node_manager.list(ListArgs { offline }).await?;
                    nm.list(args).await?;
                } // ManageCommands::ListRemote { all } => {
                  //     node_manager.list_remote(ListRemoteArgs { all }).await?;
                  // }
            }
        }
        // manage end
        SnmCommands::I(_) | SnmCommands::C(_) | SnmCommands::A(_) | SnmCommands::D(_) => {
            if let Some(package_json) = PackageJson::from(&snm_config.workspace).ok() {
                if let Some(pm) = package_json.package_manager {
                    let pm = PackageManager::from_str(&pm, &snm_config)?;
                    let command = match cli.command {
                        // SnmCommands::Node { command } => todo!(),
                        SnmCommands::I(args) => pm.install(args),
                        SnmCommands::C(args) => pm.install(InstallArgs {
                            frozen_lockfile: true,
                            ..args
                        }),
                        SnmCommands::A(args) => pm.add(args),
                        SnmCommands::D(args) => pm.remove(args),
                        // SnmCommands::FigSpec => todo!(),
                        _ => unreachable!("unreachable"),
                    }?;

                    let args = command.iter().skip(1).collect::<Vec<_>>();

                    exec_cli(vec![], pm.name(), args)?;
                } else {
                    bail!("No package manager found");
                }
            }
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
    }

    Ok(())
}
