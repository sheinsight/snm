use std::env::current_exe;

use crate::fig::fig_spec_impl;
use crate::manage_command::ManageCommands;
use crate::snm_command::SnmCommands;
use crate::SnmCli;
use anyhow::bail;
use snm_config::SnmConfig;
use snm_package_json::ops::ops::InstallArgs;
use snm_package_json::package_json::PackageJson;
use snm_package_json::pm::PackageManager;
use snm_utils::exec::exec_cli;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
    match cli.command {
        // manage start
        SnmCommands::Node { command } => {
            let nm = snm_node_version::manager::NodeManager::new(&snm_config);

            match command {
                ManageCommands::Default(args) => {
                    nm.set_default(args).await?;
                }
                ManageCommands::Install(args) => {
                    nm.install(args).await?;
                }
                ManageCommands::Uninstall(args) => {
                    nm.uninstall(args).await?;
                }
                ManageCommands::List(args) => {
                    nm.list(args).await?;
                }
            }
        }
        SnmCommands::Pnpm { command } => todo!(),
        // manage end
        SnmCommands::I(_) | SnmCommands::C(_) | SnmCommands::A(_) | SnmCommands::D(_) => {
            if let Some(package_json) = PackageJson::from(&snm_config.workspace).ok() {
                if let Some(pm) = package_json.package_manager {
                    let pm = PackageManager::from_str(&pm, &snm_config)?;
                    let args = match cli.command {
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

                    exec_cli(vec![], args)?;
                } else {
                    bail!("No package manager found");
                }
            }
        }

        SnmCommands::FigSpec => {
            fig_spec_impl()?;
        }
        SnmCommands::SetUp => {
            let exe = current_exe()?;
            let exe_dir = exe.parent().unwrap();

            const SHIM_TARGETS: &[&str] = &["npm", "npx", "yarn", "pnpm", "pnpx", "node"];

            let source = exe_dir.join("snm-shim");
            for target in SHIM_TARGETS {
                let target = exe_dir.join(target);
                if target.try_exists()? {
                    std::fs::remove_file(&target)?;
                }
                #[cfg(unix)]
                {
                    std::os::unix::fs::symlink(&source, &target)?;
                }
                #[cfg(windows)]
                {
                    std::os::windows::fs::symlink_dir(&source, &target)?;
                }
            }
        }
    }

    Ok(())
}
