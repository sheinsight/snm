use snm_config::SnmConfig;
use snm_ni::trait_transform::IArgs;
use snm_ni::{CommandArgsCreatorTrait, NpmArgsTransform, PnpmArgsTransform, YarnArgsTransform};
use snm_node::snm_node::SnmNode;
use snm_utils::snm_error::SnmError;
use std::process::{Command, Stdio};

use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::snm_command::SnmCommands;
use crate::SnmCli;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> Result<(), SnmError> {
    match cli.command {
        // manage start
        SnmCommands::Node { command } => {
            let snm_node = SnmNode::new(snm_config);
            match command {
                ManageCommands::Default { version } => {
                    snm_node.set_default(version.as_str()).await?;
                }
                ManageCommands::Install { version } => {
                    snm_node.install(version.as_str()).await?;
                }
                ManageCommands::Uninstall { version } => {
                    snm_node.un_install(version.as_str()).await?;
                }
                ManageCommands::ListOffline => {
                    snm_node.show_list_offline().await?;
                }
                ManageCommands::List => {
                    snm_node.show_list().await?;
                }
                ManageCommands::ListRemote { all } => {
                    snm_node.show_list_remote(all).await?;
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
