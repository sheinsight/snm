use std::path::PathBuf;

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::{exec::exec_cli, trace_if};
use tracing::trace;
const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];
pub async fn load_pm(snm_config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
  let [bin_name, command_args @ ..] = args.as_slice() else {
    bail!(
      r#"No binary name provided in arguments
args: {:?}"#,
      args
    );
  };

  trace_if!(|| {
    trace!(
      r#"Load pm shim , exe_name: {}, args: {}"#,
      bin_name,
      args.join(" ")
    );
  });

  let node_bin_dir = SNode::try_from(&snm_config)?
    .ensure_node_and_return_dir()
    .await?;

  let node_str = String::from("node");
  let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

  let paths = vec![node_bin_dir_str];

  if PJson::exists(&snm_config.workspace) {
    if SPM::exists(&snm_config.workspace)? {
      let f = get_package_manager_bin(args, &snm_config).await?;
      let args = [&[node_str, f.to_string_lossy().into_owned()], command_args].concat();
      exec_cli(&args, &paths, true)?;
    } else {
      if snm_config.strict {
        bail!("You have not correctly configured packageManager in package.json");
      } else {
        // exec_cli(&args, &paths, true)?;
        if !is_npm_command(bin_name) {
          exec_cli(&args, &paths, true)?;
        } else {
          #[cfg(target_os = "windows")]
          let bin_name = format!("{}.cmd", bin_name);

          #[cfg(not(target_os = "windows"))]
          let bin_name = bin_name.to_string();

          let pm_bin_file = node_bin_dir.join(bin_name);

          trace_if!(|| {
            trace!("Default bin file: {:?}", pm_bin_file);
          });

          let mut exec_args = vec![pm_bin_file.to_string_lossy().to_string()];
          exec_args.extend(command_args.iter().map(|s| s.to_string()));

          exec_cli(&exec_args, &paths, true)?;
        }
      }
    }
  } else {
    // exec_cli(&args, &paths, true)?;
    if !is_npm_command(bin_name) {
      exec_cli(&args, &paths, true)?;
    } else {
      #[cfg(target_os = "windows")]
      let bin_name = format!("{}.cmd", bin_name);

      #[cfg(not(target_os = "windows"))]
      let bin_name = bin_name.to_string();

      let pm_bin_file = node_bin_dir.join(bin_name);

      trace_if!(|| {
        trace!("Default bin file: {:?}", pm_bin_file);
      });

      let mut exec_args = vec![pm_bin_file.to_string_lossy().to_string()];
      exec_args.extend(command_args.iter().map(|s| s.to_string()));

      exec_cli(&exec_args, &paths, true)?;
    }
  }

  Ok(())
}
fn is_npm_command(command: &str) -> bool {
  NPM_COMMANDS.contains(&command)
}
async fn get_package_manager_bin(
  args: &Vec<String>,
  config: &SnmConfig,
) -> anyhow::Result<PathBuf> {
  let [bin_name, command, _args @ ..] = args.as_slice() else {
    bail!(
      r#"No binary name provided in arguments
args: {:?}"#,
      args
    );
  };

  let spm = SPM::try_from(&config.workspace, config)?;

  let pm = &spm.pm;

  if pm.name() != bin_name && config.restricted_list.contains(command) {
    bail!(
      "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
      pm.name().green(),
      bin_name.red(),
      config.restricted_list.join(", ").black()
    );
  }

  let dir = spm.ensure_bin_dir().await?;

  let json = PJson::from(dir)?;

  let file = json.get_bin_with_name(bin_name)?;

  Ok(file)
}
