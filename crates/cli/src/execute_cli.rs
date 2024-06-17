use colored::*;
use ni::trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs};
use snm_config::SnmConfig;
use snm_core::traits::manage::ManageTrait;

use snm_core::model::dispatch_manage::DispatchManage;

use snm_node::snm_node::SnmNode;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::exec::exec_cli;
use snm_utils::snm_error::SnmError;

use crate::commands::manage_command::ManageCommands;
use crate::commands::snm_command::SnmCommands;
use crate::{
    fig::fig_spec_impl,
    ni::{self, npm_args::NpmArgsTransform, pnpm_args::PnpmArgsTransform},
    SnmCli,
};

async fn exec_manage_trait(command: ManageCommands, manage: Box<dyn ManageTrait>) {
    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();
    let dispatch = DispatchManage::new(manage);
    match command {
        ManageCommands::Default { version } => {
            dispatch.set_default(trim_version(version).as_str()).await;
        }
        ManageCommands::Install { version } => {
            dispatch.install(trim_version(version).as_str()).await;
        }
        ManageCommands::Uninstall { version } => {
            dispatch.un_install(trim_version(version).as_str()).await;
        }
        ManageCommands::ListOffline => {
            dispatch.list_offline().await;
        }
        ManageCommands::List => {
            dispatch.list().await;
        }
        ManageCommands::ListRemote { all } => {
            dispatch.list_remote(all).await;
        }
    }
}

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> Result<(), SnmError> {
    match cli.command {
        // manage start
        SnmCommands::Pnpm { command } => {
            let pnpm = Box::new(SnmPackageManager::from_prefix("pnpm", snm_config.clone()));
            exec_manage_trait(command, pnpm).await;
        }
        SnmCommands::Npm { command } => {
            let npm = Box::new(SnmPackageManager::from_prefix("npm", snm_config.clone()));
            exec_manage_trait(command, npm).await;
        }
        SnmCommands::Node { command } => {
            let node = Box::new(SnmNode::new(snm_config));
            exec_manage_trait(command, node).await;
        }
        // manage end
        SnmCommands::I(_)
        | SnmCommands::C(_)
        | SnmCommands::A(_)
        | SnmCommands::D(_)
        | SnmCommands::X(_)
        | SnmCommands::E(_)
        | SnmCommands::R(_) => {
            let name = match parse_package_json(&snm_config.get_workspace()?) {
                Some(package_json) => match package_json.package_manager {
                    Some(package_manager) => package_manager.name.unwrap(),
                    None => {
                        panic!("No package manager found in the workspace.")
                    }
                },
                None => {
                    panic!("No package.json found in the workspace.")
                }
            };

            let transform: Box<dyn CommandArgsCreatorTrait> = match name.as_str() {
                "npm" => Box::new(NpmArgsTransform {}),
                "pnpm" => Box::new(PnpmArgsTransform {}),
                _ => panic!("Unsupported package manager"),
            };

            let args = match cli.command {
                // snm command start
                SnmCommands::I(args) => transform.get_install_command(args),
                SnmCommands::C(_) => transform.get_install_command(InstallCommandArgs {
                    frozen_lockfile: true,
                }),
                SnmCommands::A(args) => transform.get_add_command(args),
                SnmCommands::D(args) => transform.get_delete_command(args),
                SnmCommands::X(args) => transform.get_dlx_command(args),
                SnmCommands::E(args) => transform.get_exec_command(args),
                SnmCommands::R(args) => transform.get_run_command(args),
                _ => unreachable!("unreachable"),
            };

            exec_cli(name, args);
        }

        // snm command end
        SnmCommands::FigSpec => {
            fig_spec_impl();
        }
    }

    Ok(())
}
