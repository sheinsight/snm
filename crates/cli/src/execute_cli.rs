use std::env::current_exe;
use std::{fs, ops::Not};

use clap::CommandFactory;
use colored::*;
use snm_config::snm_config::SnmConfig;
use snm_pm::pm::PackageManager;
use snm_utils::exec::exec_cli;
use tracing::trace;

use crate::cli::SnmCli;
use crate::manage_command::NodeManageCommands;
use crate::snm_command::SnmCommands;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
  trace!(
    r#"Start execute cli 
{}"#,
    cli
  );

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
    // manage end
    SnmCommands::Install(_) | SnmCommands::Uninstall(_) | SnmCommands::Run(_) => {
      let pm = PackageManager::from(&snm_config.workspace)?;
      let handler = pm.get_ops();
      let commands = match cli.command {
        SnmCommands::Install(install_args) => handler.install(install_args),
        SnmCommands::Uninstall(remove_args) => handler.remove(remove_args),
        SnmCommands::Run(run_args) => handler.run(run_args),
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
