use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::node_manager::node_manager::{ListArgs, ListRemoteArgs, NodeManager};
use crate::snm_command::SnmCommands;
use crate::SnmCli;
use snm_atom::node_atom::NodeAtom;
use snm_config::SnmConfig;
use snm_ni::{CommandArgsCreatorTrait, NpmArgsTransform, PnpmArgsTransform, YarnArgsTransform};
use snm_package_json::ops::ops::InstallArgs;
use snm_package_json::package_json::PackageJson;
use snm_utils::exec::exec_cli;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
    match cli.command {
        // manage start
        SnmCommands::Node { command } => {
            let node_atom = NodeAtom::new(snm_config);
            let node_manager = NodeManager::new(&node_atom);
            match command {
                ManageCommands::Default { version } => {
                    node_manager.set_default(version.as_str()).await?;
                }
                ManageCommands::Install { version } => {
                    node_manager.install(version.as_str()).await?;
                }
                ManageCommands::Uninstall { version } => {
                    node_manager.un_install(version.as_str()).await?;
                }
                ManageCommands::List { offline } => {
                    node_manager.list(ListArgs { offline }).await?;
                }
                ManageCommands::ListRemote { all } => {
                    node_manager.list_remote(ListRemoteArgs { all }).await?;
                }
            }
        }
        // manage end
        SnmCommands::I(_) | SnmCommands::C(_) | SnmCommands::A(_) => {
            if let Some(package_json) = PackageJson::from(snm_config.workspace) {
                if let Some(pm) = package_json.get_pm() {
                    let command = match cli.command {
                        // SnmCommands::Node { command } => todo!(),
                        SnmCommands::I(iargs) => pm.install(iargs),
                        SnmCommands::C(iargs) => pm.install(InstallArgs {
                            frozen_lockfile: true,
                            ..iargs
                        }),
                        SnmCommands::A(aargs) => pm.add(aargs),
                        // SnmCommands::FigSpec => todo!(),
                        _ => unreachable!("unreachable"),
                    }?;

                    let args = command.iter().skip(1).collect::<Vec<_>>();

                    exec_cli(vec![], pm.name(), args)?;
                }
            }
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
    }

    Ok(())
}
