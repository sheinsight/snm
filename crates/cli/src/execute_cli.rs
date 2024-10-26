use snm_atom::node_atom::NodeAtom;
use snm_config::EnvSnmConfig;
use snm_ni::trait_transform::IArgs;
use snm_ni::{CommandArgsCreatorTrait, NpmArgsTransform, PnpmArgsTransform, YarnArgsTransform};
use snm_utils::exec::exec_cli;
use snm_utils::snm_error::SnmError;

use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::node_manager::node_manager::{ListArgs, ListRemoteArgs, NodeManager};
use crate::snm_command::SnmCommands;
use crate::SnmCli;

fn create_transform(name: &str, version: &str) -> Box<dyn CommandArgsCreatorTrait> {
    match name {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        "yarn" => Box::new(YarnArgsTransform {
            version: version.to_string(),
        }),
        _ => panic!("Unsupported package manager: {}", name),
    }
}

fn handle_command(transform: &dyn CommandArgsCreatorTrait, command: SnmCommands) -> Vec<String> {
    match command {
        // snm command start
        SnmCommands::I(args) => transform.i(args),
        SnmCommands::C(_) => transform.i(IArgs {
            frozen_lockfile: true,
        }),
        SnmCommands::A(args) => transform.a(args),
        SnmCommands::X(args) => transform.x(args),
        SnmCommands::E(args) => transform.e(args),
        SnmCommands::R(args) => transform.r(args),
        _ => unreachable!("unreachable"),
    }
}

pub async fn execute_cli(cli: SnmCli, snm_config: EnvSnmConfig) -> Result<(), SnmError> {
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
        SnmCommands::I(_)
        | SnmCommands::C(_)
        | SnmCommands::A(_)
        | SnmCommands::X(_)
        | SnmCommands::E(_)
        | SnmCommands::R(_) => {
            let package_json = snm_config
                .get_snm_package_json()
                .ok_or(SnmError::NotFoundPackageJsonFileError {})?;

            let package_manager = package_json
                .package_manager
                .ok_or(SnmError::NotFondPackageManagerConfigError {})?;

            let transform = create_transform(&package_manager.name, &package_manager.version);

            let args = handle_command(&*transform.as_ref(), cli.command);

            exec_cli(vec![], package_manager.name, args)?;
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
    }

    Ok(())
}
