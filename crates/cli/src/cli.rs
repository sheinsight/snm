use std::{
  env::{current_dir, current_exe},
  fmt::Display,
  fs,
  ops::Not,
};

use anyhow::bail;
use clap::{command, crate_authors, crate_name, crate_version, CommandFactory, Parser};
use colored::Colorize;
use serde::Serialize;
use snm_config::snm_config::SnmConfig;
use snm_package_manager::PackageManagerResolver;
use snm_utils::{consts::SNM_PREFIX, exec::exec_cli};
use tracing::trace;

use crate::{
  manage_command::NodeManageCommands, package_manager::Command, snm_command::SnmCommands,
};

/// SnmCli 是 snm 的命令行工具
/// Example:
/// ```rust
/// use snm::cli::SnmCli;
/// use snm::snm_command;
/// use snm_config::snm_config::SnmConfig;
///
/// #[tokio::test]
/// async fn test_snm_cli() -> anyhow::Result<()> {
///   let snm_config = SnmConfig::from(SNM_PREFIX, current_dir()?).unwrap();
///
///   SnmCli::from(snm_command::SnmCommands::Install(
///     snm_pm::ops::ops::InstallArgs {
///       package_spec: vec![],
///       frozen: true,
///       save_prod: false,
///       save_peer: false,
///       save_dev: false,
///       save_optional: false,
///       save_exact: false,
///     },
///   ))
///   .exec(snm_config.clone())
///   .await?;
/// }
/// ```
#[derive(Parser, Debug, Serialize)]
#[
    command(
        name = crate_name!(),
        author = crate_authors!(),
        version = crate_version!(),
        about = "snm = ni + fnm + corepack",
        disable_version_flag = true,
        disable_help_subcommand = true
    )
]
pub struct SnmCli {
  #[command(subcommand)]
  pub command: SnmCommands,
  #[arg(
        short = 'v',
        long = "version",
        action = clap::ArgAction::Version
    )]
  pub version: Option<bool>,
}

impl Display for SnmCli {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Ok(json) = serde_json::to_string_pretty(self) {
      return write!(f, "{}", json);
    }
    write!(f, "{:?}", self)
  }
}

impl From<SnmCommands> for SnmCli {
  fn from(command: SnmCommands) -> Self {
    Self {
      command,
      version: Some(false),
    }
  }
}

impl SnmCli {
  pub async fn exec(self) -> anyhow::Result<()> {
    let dir = current_dir()?;

    trace!("Get current dir: {:#?}", dir);

    let snm_config = SnmConfig::from(SNM_PREFIX, dir)?;

    trace!("Get snm config: {:#?}", snm_config);

    match self.command {
      SnmCommands::Node { command } => {
        // let nm = snm_node::factory::NodeFactory::new(&snm_config);
        let nm = crate::node::NodeFactory::new(&snm_config);
        match command {
          NodeManageCommands::Default(args) => {
            trace!("Set default node: {:#?}", args);
            nm.set_default(args).await?;
          }
          NodeManageCommands::Install(args) => {
            trace!("Install node: {:#?}", args);
            nm.install(args).await?;
          }
          NodeManageCommands::Uninstall(args) => {
            trace!("Uninstall node: {:#?}", args);
            nm.uninstall(args).await?;
          }
          NodeManageCommands::List(args) => {
            trace!("List node: {:#?}", args);
            nm.list(args).await?;
          }
        }
      }
      SnmCommands::Install(_) | SnmCommands::Uninstall(_) | SnmCommands::Run(_) => {
        let resolver = PackageManagerResolver::from(snm_config);

        let Ok(package_manager) = resolver.find_up_package_manager() else {
          bail!("You have not correctly configured packageManager in package.json");
        };

        let handler: Box<dyn Command> = match package_manager.kind() {
          snm_package_manager::PackageManagerKind::Npm => {
            Box::new(crate::package_manager::NpmCommandLine::new())
          }
          snm_package_manager::PackageManagerKind::Yarn => {
            if snm_utils::ver::ver_gt_1(&package_manager.version())? {
              Box::new(crate::package_manager::YarnBerryCommandLine::new())
            } else {
              Box::new(crate::package_manager::YarnCommandLine::new())
            }
          }
          snm_package_manager::PackageManagerKind::Pnpm => {
            Box::new(crate::package_manager::PnpmCommandLine::new())
          }
        };

        let commands = match self.command {
          SnmCommands::Install(install_args) => {
            trace!("Install command: {:#?}", install_args);
            handler.install(install_args)
          }
          SnmCommands::Uninstall(remove_args) => {
            trace!("Uninstall command: {:#?}", remove_args);
            handler.remove(remove_args)
          }
          SnmCommands::Run(run_args) => {
            trace!("Run command: {:#?}", run_args);
            handler.run(run_args)
          }
          _ => unreachable!(),
        }?;

        exec_cli(&commands, &vec![], false)?;
      }
      SnmCommands::SetUp => {
        setup_fig()?;
        setup_symlink()?;
      }
    }
    Ok(())
  }
}

fn setup_fig() -> anyhow::Result<()> {
  let mut output = Vec::new();
  clap_complete::generate(
    clap_complete_fig::Fig,
    &mut SnmCli::command(),
    "snm",
    &mut output,
  );
  let mut output_string = String::from_utf8(output).unwrap();

  output_string = output_string.replace("const completion: Fig.Spec = {", "const completion = {");

  if let Some(home) = dirs::home_dir() {
    let dir = home.join(".fig").join("autocomplete").join("build");

    if dir.exists().not() {
      fs::create_dir_all(&dir)
        .expect(format!("fig_spec_impl create_dir_all error {:?}", &dir.display()).as_str());
    }

    let spec_path_buf = dir.join("snm.js");

    if spec_path_buf.exists() {
      fs::remove_file(&spec_path_buf).expect(
        format!(
          "fig_spec_impl remove_file error {:?}",
          &spec_path_buf.display()
        )
        .as_str(),
      );
    }

    fs::write(&spec_path_buf, &output_string)?;

    let message = format!(
      r##"
    🎉 Fig spec file create successful. 

    🔔 Now ! Fig rename to {}

                                    {}
            "##,
      "Amazon Q".green().bold(),
      "Powered by snm".bright_black(),
    );

    eprintln!("{message}");
  }

  Ok(())
}

fn setup_symlink() -> anyhow::Result<()> {
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
      std::os::windows::fs::symlink_file(&source, &target)?;
    }
  }

  Ok(())
}
