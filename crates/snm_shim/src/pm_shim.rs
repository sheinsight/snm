use std::path::PathBuf;

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::{package_json::PJson, pm::SPM};
use snm_utils::{exec::exec_cli, trace_if};
use tracing::trace;

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
        exec_cli(&args, &paths, true)?;
      }
    }
  } else {
    exec_cli(&args, &paths, true)?;
  }

  Ok(())
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
