use colored::*;
use manage_command::ManageCommands;
use ni::trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs};
use snm_command::SnmCommands;
use snm_core::utils::get_current_dir::get_current_dir;
use snm_core::{
    model::{
        dispatch_manage::DispatchManage, package_json::PackageManager, trait_manage::ManageTrait,
        PackageJson, SnmError,
    },
    println_success,
};
use snm_node::snm_node::SnmNode;
use snm_npm::snm_npm::SnmNpm;
use snm_pnpm::snm_pnpm::SnmPnpm;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    bump::bump_impl,
    fig::fig_spec_impl,
    manage_command,
    ni::{self, npm_args::NpmArgsTransform, pnpm_args::PnpmArgsTransform},
    snm_command, SnmCli,
};

pub async fn execute_cli(cli: SnmCli) -> Result<(), SnmError> {
    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();
    match cli.command {
        // manage start
        SnmCommands::Pnpm { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmPnpm::new()))
                    .set_default(v)
                    .await?
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmPnpm::new()))
                    .install(v)
                    .await?;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmPnpm::new()))
                    .un_install(v)
                    .await?;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(SnmPnpm::new())).list().await?;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(SnmPnpm::new()))
                    .list_remote(all)
                    .await?;
            }
        },
        SnmCommands::Npm { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNpm::new()))
                    .set_default(v)
                    .await?
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNpm::new()))
                    .install(v)
                    .await?;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNpm::new()))
                    .un_install(v)
                    .await?;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(SnmNpm::new())).list().await?;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(SnmNpm::new()))
                    .list_remote(all)
                    .await?;
            }
        },
        SnmCommands::Node { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .set_default(v)
                    .await?
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .install(v)
                    .await?;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                DispatchManage::new(Box::new(SnmNode::new()))
                    .un_install(v)
                    .await?;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(SnmNode::new())).list().await?;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(SnmNode::new()))
                    .list_remote(all)
                    .await?;
            }
        },
        // manage end

        // snm command start
        SnmCommands::Install(args) => {
            execute_command(|creator| creator.get_install_command(args)).await?;
        }
        SnmCommands::CI(_) => {
            execute_command(|creator| {
                creator.get_install_command(InstallCommandArgs {
                    frozen_lockfile: true,
                })
            })
            .await?;
        }
        SnmCommands::Add(args) => {
            execute_command(|creator| creator.get_add_command(args)).await?;
        }
        SnmCommands::Delete(args) => {
            execute_command(|creator| creator.get_delete_command(args)).await?;
        }
        SnmCommands::Query => todo!(""),
        // snm command end
        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
        SnmCommands::Bump => {
            bump_impl()?;
        }
        SnmCommands::Dlx(args) => {
            execute_command(|creator| creator.get_dlx_command(args)).await?;
        }
        SnmCommands::Exec(args) => {
            execute_command(|creator| creator.get_exec_command(args)).await?;
        }
        SnmCommands::Run(args) => {
            execute_command(|creator| creator.get_run_command(args)).await?;
        }
        SnmCommands::SetCache(args) => {
            execute_command(|creator| creator.get_set_cache_command(args)).await?;
        }
    }
    Ok(())
}

pub async fn get_bin() -> Result<((String, String), PathBuf), SnmError> {
    let dir = get_current_dir()?;
    let package_json_path_buf = dir.join("package.json");
    if package_json_path_buf.exists() {
        let package_json: PackageJson = PackageJson::from_file_path(&package_json_path_buf)?;
        let package_manager = package_json.parse_package_manager()?;
        let manager = get_manage(&package_manager).await?;
        let dispatcher = DispatchManage::new(manager);
        let (_, bin_path_buf) = dispatcher.proxy_process(&package_manager.name).await?;
        return Ok((
            (package_manager.name, package_manager.version),
            bin_path_buf,
        ));
    } else {
        let dispatcher = DispatchManage::new(Box::new(SnmPnpm::new()));
        let (version, bin_path_buf) = dispatcher.proxy_process("pnpm").await?;
        return Ok((("pnpm".to_string(), version), bin_path_buf));
    }
}

async fn execute_command<F>(get_command_args: F) -> Result<(), SnmError>
where
    F: FnOnce(&dyn CommandArgsCreatorTrait) -> Result<Vec<String>, SnmError>,
{
    let ((name, version), bin_path_buf) = get_bin().await?;

    let command_args_creator: Box<dyn CommandArgsCreatorTrait> = match name.as_str() {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        _ => panic!("Unsupported package manager"),
    };

    let args = get_command_args(command_args_creator.as_ref())?;

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
        return Err(SnmError::Error("spawn error".to_string()));
    }

    Ok(())
}

pub async fn get_manage(
    package_manager: &PackageManager,
) -> Result<Box<dyn ManageTrait>, SnmError> {
    let manager: Box<dyn ManageTrait> = match package_manager.name.as_str() {
        "npm" => {
            let manager = SnmNpm::new();
            Box::new(manager)
        }
        "pnpm" => Box::new(SnmPnpm::new()),
        _ => {
            return Err(SnmError::UnsupportedPackageManager {
                name: package_manager.name.to_string(),
                version: package_manager.version.to_string(),
            })
        }
    };
    Ok(manager)
}
