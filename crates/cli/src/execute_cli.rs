use colored::*;
use ni::trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs};
use snm_core::snm_content::SnmContentHandler;
use snm_core::traits::manage::ManageTrait;
use snm_core::utils::get_current_dir::get_current_dir;
use snm_core::{
    model::{dispatch_manage::DispatchManage, package_json::PackageManager, PackageJson},
    println_success,
};

use snm_node::snm_node::SnmNode;
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

pub async fn execute_cli(cli: SnmCli, snm_content_handler: SnmContentHandler) -> () {
    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();

    let pnpm = SnmPackageManager::from_prefix("pnpm", snm_content_handler.clone());
    let npm = SnmPackageManager::from_prefix("npm", snm_content_handler.clone());

    match cli.command {
        // manage start
        SnmCommands::Pnpm { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(pnpm)).set_default(v).await;
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(pnpm)).install(v).await;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(pnpm)).un_install(v).await;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(pnpm)).list().await;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(pnpm)).list_remote(all).await;
            }
            ManageCommands::ListOffline => {
                DispatchManage::new(Box::new(pnpm)).list_offline().await;
            }
        },
        SnmCommands::Npm { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(npm)).set_default(v).await;
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(npm)).install(v).await;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(npm)).un_install(v).await;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(npm)).list().await;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(npm)).list_remote(all).await;
            }
            ManageCommands::ListOffline => {
                DispatchManage::new(Box::new(npm)).list_offline().await;
            }
        },
        SnmCommands::Node { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .set_default(v)
                    .await;
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .install(v)
                    .await;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .un_install(v)
                    .await;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(SnmNode::new())).list().await;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(SnmNode::new()))
                    .list_remote(all)
                    .await;
            }
            ManageCommands::ListOffline => {
                DispatchManage::new(Box::new(SnmNode::new()))
                    .list_offline()
                    .await;
            }
        },

        SnmCommands::Semver { command: _ } => todo!(),
        // manage end

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
            execute_command(|creator| creator.get_add_command(args), snm_content_handler).await;
        }
        SnmCommands::D(args) => {
            execute_command(
                |creator| creator.get_delete_command(args),
                snm_content_handler,
            )
            .await;
        }
        SnmCommands::Query => todo!(""),
        SnmCommands::X(args) => {
            execute_command(|creator| creator.get_dlx_command(args), snm_content_handler).await;
        }
        SnmCommands::E(args) => {
            execute_command(
                |creator| creator.get_exec_command(args),
                snm_content_handler,
            )
            .await;
        }
        SnmCommands::R(args) => {
            execute_command(|creator| creator.get_run_command(args), snm_content_handler).await;
        }
        SnmCommands::SetCache(args) => {
            execute_command(
                |creator| creator.get_set_cache_command(args),
                snm_content_handler,
            )
            .await;
        }
        // snm command end
        SnmCommands::FigSpec => {
            fig_spec_impl();
        }
    }
}

pub async fn get_bin(snm_content_handler: SnmContentHandler) -> ((String, String), PathBuf) {
    let dir = get_current_dir();
    let package_json_path_buf = dir.join("package.json");
    if package_json_path_buf.exists() {
        let package_json: PackageJson = PackageJson::from_file_path(&package_json_path_buf);
        let package_manager = package_json.parse_package_manager();
        let manager = get_manage(&package_manager, snm_content_handler.clone()).await;
        let dispatcher = DispatchManage::new(manager);
        let (_, bin_path_buf) = dispatcher
            .proxy_process(&package_manager.name, snm_content_handler.get_strict())
            .await;
        return (
            (package_manager.name, package_manager.version),
            bin_path_buf,
        );
    } else {
        let dispatcher = DispatchManage::new(Box::new(SnmPackageManager::from_prefix(
            "pnpm",
            snm_content_handler.clone(),
        )));
        let (version, bin_path_buf) = dispatcher
            .proxy_process("pnpm", snm_content_handler.get_strict())
            .await;
        return (("pnpm".to_string(), version), bin_path_buf);
    }
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

pub async fn get_manage(
    package_manager: &PackageManager,
    snm_content_handler: SnmContentHandler,
) -> Box<dyn ManageTrait> {
    let manager: Box<dyn ManageTrait> = match package_manager.name.as_str() {
        "npm" => {
            let manager =
                SnmPackageManager::from_prefix(&package_manager.name, snm_content_handler);
            Box::new(manager)
        }
        "pnpm" => Box::new(SnmPackageManager::from_prefix(
            &package_manager.name,
            snm_content_handler,
        )),
        _ => {
            let msg = format!(
                "UnsupportedPackageManager {}@{}",
                package_manager.name, package_manager.version
            );
            panic!("{msg}");
        }
    };
    manager
}
