use std::env::current_exe;

use anyhow::bail;
use snm_config::SnmConfig;
use snm_pm::factory::{PackageManagerFactory, PackageManagerFactoryCommands};
use snm_pm::ops::ops::InstallArgs;
use snm_pm::package_json::PackageJson;
use snm_pm::pm::PackageManager;
use snm_pm::pm_metadata::PackageManagerMetadata;
use snm_utils::exec::exec_cli;

use crate::fig::fig_spec_impl;
use crate::manage_command::NodeManageCommands;
use crate::snm_command::SnmCommands;
use crate::SnmCli;

async fn handle_package_manager(
  pm_name: &str,
  command: &PackageManagerFactoryCommands,
  config: &SnmConfig,
) -> anyhow::Result<()> {
  let version = match command {
    PackageManagerFactoryCommands::Install(args) => &args.version,
    PackageManagerFactoryCommands::Default(args) => &args.version,
    PackageManagerFactoryCommands::Uninstall(args) => &args.version,
  };

  let metadata = PackageManagerMetadata::from_str(&format!("{}@{}", pm_name, version), config)?;
  let pm = PackageManagerFactory::new(&metadata);

  match command {
    PackageManagerFactoryCommands::Install(_) => pm.install().await,
    PackageManagerFactoryCommands::Default(_) => pm.set_default().await,
    PackageManagerFactoryCommands::Uninstall(_) => pm.uninstall().await,
  }
}

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
  match cli.command {
    // manage start
    SnmCommands::Node { command } => {
      let nm = snm_node::factory::NodeFactory::new(&snm_config);

      match command {
        NodeManageCommands::Default(args) => {
          nm.set_default(args).await?;
        }
        NodeManageCommands::Install(args) => {
          nm.install(args).await?;
        }
        NodeManageCommands::Uninstall(args) => {
          nm.uninstall(args).await?;
        }
        NodeManageCommands::List(args) => {
          nm.list(args).await?;
        }
      }
    }
    SnmCommands::Pnpm { command } => handle_package_manager("pnpm", &command, &snm_config).await?,
    SnmCommands::Yarn { command } => handle_package_manager("yarn", &command, &snm_config).await?,
    SnmCommands::Npm { command } => handle_package_manager("npm", &command, &snm_config).await?,
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

      #[cfg(windows)]
      let source = exe_dir.join("snm-shim.exe");
      #[cfg(not(windows))]
      let source = exe_dir.join("snm-shim");

      // let source = exe_dir.join("snm-shim");
      for target in SHIM_TARGETS {
        // let target = exe_dir.join(target);
        #[cfg(windows)]
        let target = exe_dir.join(format!("{}.exe", target));
        #[cfg(not(windows))]
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
