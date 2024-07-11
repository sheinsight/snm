use std::process::{Command, Stdio};

use snm_config::SnmConfig;
use snm_core::traits::atom::AtomTrait;

use snm_core::model::dispatch_manage::DispatchManage;

use snm_ni::trait_transform::IArgs;
use snm_ni::{CommandArgsCreatorTrait, NpmArgsTransform, PnpmArgsTransform, YarnArgsTransform};
use snm_node::snm_node::SnmNode;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::snm_error::SnmError;

use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::snm_command::SnmCommands;
use crate::SnmCli;

async fn exec_manage_trait(
    command: ManageCommands,
    manage: Box<dyn AtomTrait>,
) -> Result<(), SnmError> {
    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();
    let dispatch = DispatchManage::new(manage);
    match command {
        ManageCommands::Default { version } => {
            dispatch.set_default(trim_version(version).as_str()).await?;
        }
        ManageCommands::Install { version } => {
            dispatch.install(trim_version(version).as_str()).await?;
        }
        ManageCommands::Uninstall { version } => {
            dispatch.un_install(trim_version(version).as_str()).await?;
        }
        ManageCommands::ListOffline => {
            dispatch.list_offline().await?;
        }
        ManageCommands::List => {
            dispatch.list().await?;
        }
        ManageCommands::ListRemote { all } => {
            dispatch.list_remote(all).await?;
        }
    }

    Ok(())
}

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> Result<(), SnmError> {
    match cli.command {
        // manage start
        SnmCommands::Pnpm { command } => {
            let pnpm = Box::new(SnmPackageManager::from_prefix("pnpm", snm_config.clone()));
            exec_manage_trait(command, pnpm).await?;
        }
        SnmCommands::Npm { command } => {
            let npm = Box::new(SnmPackageManager::from_prefix("npm", snm_config.clone()));
            exec_manage_trait(command, npm).await?;
        }
        SnmCommands::Yarn { command } => {
            let npm = Box::new(SnmPackageManager::from_prefix("yarn", snm_config.clone()));
            exec_manage_trait(command, npm).await?;
        }
        SnmCommands::Node { command } => {
            let node = Box::new(SnmNode::new(snm_config));
            exec_manage_trait(command, node).await?;
        }
        // manage end
        SnmCommands::I(_)
        | SnmCommands::C(_)
        | SnmCommands::A(_)
        | SnmCommands::X(_)
        | SnmCommands::E(_)
        | SnmCommands::R(_) => {
            let package_manager = snm_config
                    .get_snm_package_json()
                    .and_then(|package_json| package_json.package_manager)
                    .expect("No package manager found in the workspace or no package.json found in the workspace.");

            let transform: Box<dyn CommandArgsCreatorTrait> = match package_manager.name.as_str() {
                "npm" => Box::new(NpmArgsTransform {}),
                "pnpm" => Box::new(PnpmArgsTransform {}),
                "yarn" => Box::new(YarnArgsTransform {
                    version: package_manager.version.to_string(),
                }),
                _ => panic!("Unsupported package manager: {}", &package_manager.name),
            };

            let args = match cli.command {
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
            };

            let output = Command::new(package_manager.name)
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .and_then(|process| process.wait_with_output())?;

            if !output.status.success() {
                return Err(SnmError::SNMBinaryProxyFail {
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                });
            }

            print!("{}", String::from_utf8_lossy(&output.stdout).to_string());
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
    }

    Ok(())
}
