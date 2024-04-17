use bump::bump_impl;
use clap::{command, CommandFactory, Parser};
use colored::*;
use fig::fig_spec_impl;
use manage_command::ManageCommands;
use ni::{
    npm_args::NpmArgsTransform,
    pnpm_args::PnpmArgsTransform,
    trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs},
    yarn_args::YarnArgsTransform,
    yarnpkg_args::YarnPkgArgsTransform,
};
use semver::Version;
use snm_command::SnmCommands;
use snm_core::{
    config::SnmConfig,
    model::{
        dispatch_manage::DispatchManage, package_json::PackageManager, snm_error::handle_snm_error,
        trait_manage::ManageTrait, PackageJson, SnmError,
    },
    println_success,
};
use snm_node::snm_node::SnmNode;
use snm_npm::snm_npm::SnmNpm;
use snm_pnpm::snm_pnpm::SnmPnpm;
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};
mod bump;
mod fig;
mod manage_command;
mod ni;
mod snm_command;

#[derive(Parser, Debug)]
struct SnmCli {
    #[command(subcommand)]
    command: SnmCommands,
}

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    SnmConfig::new().init()?;

    if let Err(error) = execute_cli().await {
        handle_snm_error(error);
    }

    Ok(())
}

async fn execute_cli() -> Result<(), SnmError> {
    let cli = SnmCli::parse();
    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();
    match cli.command {
        // manage start
        SnmCommands::Yarn { command } => match command {
            ManageCommands::Default { version } => {
                let v: &String = &trim_version(version);
                let manager: Box<dyn ManageTrait> = if get_is_less_2(&v)? {
                    Box::new(SnmYarn::new())
                } else {
                    Box::new(SnmYarnPkg::new())
                };
                DispatchManage::new(manager).set_default(v).await?;
            }
            ManageCommands::Install { version } => {
                let v: &String = &trim_version(version);
                let manager: Box<dyn ManageTrait> = if get_is_less_2(&v)? {
                    Box::new(SnmYarn::new())
                } else {
                    Box::new(SnmYarnPkg::new())
                };
                DispatchManage::new(manager).install(v).await?;
            }
            ManageCommands::Uninstall { version } => {
                let v: &String = &trim_version(version);
                let manager: Box<dyn ManageTrait> = if get_is_less_2(&v)? {
                    Box::new(SnmYarn::new())
                } else {
                    Box::new(SnmYarnPkg::new())
                };
                DispatchManage::new(manager).un_install(v).await?;
            }
            ManageCommands::List => {
                DispatchManage::new(Box::new(SnmYarn::new())).list().await?;
            }
            ManageCommands::ListRemote { all } => {
                DispatchManage::new(Box::new(SnmYarn::new()))
                    .list_remote(all)
                    .await?;
            }
        },
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
        SnmCommands::Query => todo!(),
        // snm command end
        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
        SnmCommands::Bump => {
            bump_impl()?;
        }
        SnmCommands::Dlx => todo!(),
        SnmCommands::Exec => todo!(),
    }
    Ok(())
}

async fn get_bin() -> Result<(PackageManager, PathBuf), SnmError> {
    let package_manager = PackageJson::from_dir_path(None)?.parse_package_manager()?;
    let manager = get_manage(&package_manager).await?;
    let dispatcher = DispatchManage::new(manager);
    let (_, bin_path_buf) = dispatcher.ensure_strict_package_manager().await?;
    Ok((package_manager, bin_path_buf))
}

async fn execute_command<F>(get_command_args: F) -> Result<(), SnmError>
where
    F: FnOnce(&dyn CommandArgsCreatorTrait) -> Result<Vec<String>, SnmError>,
{
    let (package_manager, bin_path_buf) = get_bin().await?;

    let command_args_creator: Box<dyn CommandArgsCreatorTrait> = match package_manager.name.as_str()
    {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(YarnArgsTransform {})
            } else {
                Box::new(YarnPkgArgsTransform {})
            }
        }
        _ => panic!("Unsupported package manager"),
    };

    let args = get_command_args(command_args_creator.as_ref())?;

    println_success!(
        "Use {}. {}",
        format!("{:<8}", package_manager.version).bright_green(),
        format!("by {}", bin_path_buf.display()).bright_black()
    );

    Command::new(bin_path_buf.display().to_string())
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output())?;
    Ok(())
}

async fn get_manage(package_manager: &PackageManager) -> Result<Box<dyn ManageTrait>, SnmError> {
    let manager: Box<dyn ManageTrait> = match package_manager.name.as_str() {
        "npm" => {
            let manager = SnmNpm::new();
            Box::new(manager)
        }
        "pnpm" => Box::new(SnmPnpm::new()),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(SnmYarn::new())
            } else {
                Box::new(SnmYarnPkg::new())
            }
        }
        _ => {
            return Err(SnmError::UnsupportedPackageManager {
                name: package_manager.name.to_string(),
                version: package_manager.version.to_string(),
            })
        }
    };
    Ok(manager)
}

fn get_is_less_2(v: &str) -> Result<bool, SnmError> {
    let ver = Version::parse(v)?;
    let is_less_2 = ver < Version::parse("2.0.0")?;
    Ok(is_less_2)
}
