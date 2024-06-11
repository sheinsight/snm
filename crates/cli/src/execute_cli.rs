use colored::*;
use ni::trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs};
use snm_core::snm_content::SnmContentHandler;
use snm_core::traits::manage::ManageTrait;

use snm_core::{model::dispatch_manage::DispatchManage, println_success};

use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

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

pub async fn execute_cli(cli: SnmCli, snm_content_handler: SnmContentHandler) -> () {
    match cli.command {
        // manage start
        SnmCommands::Pnpm { command } => {
            let pnpm = Box::new(SnmPackageManager::from_prefix(
                "pnpm",
                snm_content_handler.clone(),
            ));
            exec_manage_trait(command, pnpm).await;
        }
        SnmCommands::Npm { command } => {
            let npm = Box::new(SnmPackageManager::from_prefix(
                "npm",
                snm_content_handler.clone(),
            ));
            exec_manage_trait(command, npm).await;
        }
        SnmCommands::Node { command } => {
            let node = Box::new(SnmNode::new());
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
            // execute_snm_command(cli.command, snm_content_handler).await;
            match cli.command {
                // snm command start
                SnmCommands::I(args) => {
                    execute_command(
                        |creator| creator.get_install_command(args),
                        snm_content_handler,
                    )
                    .await;
                }
                SnmCommands::C(_) => {
                    execute_command(
                        |creator| {
                            creator.get_install_command(InstallCommandArgs {
                                frozen_lockfile: true,
                            })
                        },
                        snm_content_handler,
                    )
                    .await;
                }
                SnmCommands::A(args) => {
                    execute_command(|creator| creator.get_add_command(args), snm_content_handler)
                        .await;
                }
                SnmCommands::D(args) => {
                    execute_command(
                        |creator| creator.get_delete_command(args),
                        snm_content_handler,
                    )
                    .await;
                }
                SnmCommands::X(args) => {
                    execute_command(|creator| creator.get_dlx_command(args), snm_content_handler)
                        .await;
                }
                SnmCommands::E(args) => {
                    execute_command(
                        |creator| creator.get_exec_command(args),
                        snm_content_handler,
                    )
                    .await;
                }
                SnmCommands::R(args) => {
                    execute_command(|creator| creator.get_run_command(args), snm_content_handler)
                        .await;
                }
                _ => unreachable!("unreachable"),
            }
        }

        // snm command end
        SnmCommands::FigSpec => {
            fig_spec_impl();
        }
    }
}

pub async fn get_bin(snm_content_handler: SnmContentHandler) -> ((String, String), PathBuf) {
    let dir = match current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("NoCurrentDir"),
    };

    let package_json = match parse_package_json(dir) {
        Some(pkg) => pkg,
        None => panic!("NoPackageManager"),
    };

    let package_manager = match package_json.package_manager {
        Some(pm) => pm,
        None => panic!("NoPackageManager"),
    };

    let name = match package_manager.name {
        Some(n) => n,
        None => panic!("NoPackageManager"),
    };

    let version = match package_manager.version {
        Some(v) => v,
        None => panic!("NoPackageManager"),
    };

    let manager = match name.as_str() {
        "npm" => SnmPackageManager::from_prefix(&name, snm_content_handler.clone()),
        "pnpm" => SnmPackageManager::from_prefix(&name, snm_content_handler.clone()),
        _ => panic!("UnsupportedPackageManager"),
    };

    let dispatcher = DispatchManage::new(Box::new(manager));
    let (_, bin_path_buf) = dispatcher.proxy_process_by_strict(&name).await;
    return ((name, version), bin_path_buf);
}

async fn execute_command<F>(get_command_args: F, snm_content_handler: SnmContentHandler) -> ()
where
    F: FnOnce(&dyn CommandArgsCreatorTrait) -> Vec<String>,
{
    let ((name, version), bin_path_buf) = get_bin(snm_content_handler).await;

    let command_args_creator: Box<dyn CommandArgsCreatorTrait> = match name.as_str() {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        _ => panic!("Unsupported package manager"),
    };

    let args = get_command_args(command_args_creator.as_ref());

    println_success!(
        "Use {}. {}",
        format!("{:<8}", &version).bright_green(),
        format!("by {}", bin_path_buf.display()).bright_black()
    );

    let output = Command::new(bin_path_buf.display().to_string())
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output());

    if let Err(_) = output {
        panic!("spawn error");
    }
}
