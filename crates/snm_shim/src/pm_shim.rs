use std::path::PathBuf;

use anyhow::bail;
use colored::Colorize;
use snm_config::snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::pm::PackageManager;
use snm_utils::{exec::exec_cli, trace_if};
use tracing::trace;

const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];

pub async fn load_pm(snm_config: &SnmConfig, args: &Vec<String>) -> anyhow::Result<()> {
  let [exe_name, ..] = args.as_slice() else {
    bail!(
      r#"No binary name provided in arguments
args: {:?}"#,
      args
    );
  };

  trace_if!(|| {
    trace!(
      r#"Load pm shim , exe_name: {}, args: {}"#,
      exe_name,
      args.join(" ")
    );
  });

  let node_bin_dir = SNode::try_from(&snm_config)?.get_bin().await?;

  let node_str = String::from("node");
  let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

  let paths = vec![node_bin_dir_str];

  let command_args = args[1..].to_vec();

  if let Ok(pm_bin_file) = get_package_manager_bin(&args, &snm_config).await {
    let args = [
      &[node_str, pm_bin_file.to_string_lossy().into_owned()],
      command_args.as_slice(),
    ]
    .concat();
    exec_cli(&args, &paths, true)?;
  } else {
    if !is_npm_command(exe_name) {
      exec_cli(&args, &paths, true)?;
    } else {
      #[cfg(target_os = "windows")]
      let bin_name = format!("{}.cmd", exe_name);

      #[cfg(not(target_os = "windows"))]
      let bin_name = exe_name.to_string();

      let pm_bin_file = node_bin_dir.join(bin_name);

      trace_if!(|| {
        trace!("Default bin file: {:?}", pm_bin_file);
      });

      let mut exec_args = vec![pm_bin_file.to_string_lossy().to_string()];
      exec_args.extend(command_args);

      exec_cli(&exec_args, &paths, true)?;
    }
  }

  Ok(())
}

// 删除 build_bin_path 函数
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

  let pm = PackageManager::try_from_env(config)?;

  if pm.name() != bin_name && config.restricted_list.contains(command) {
    bail!(
      "Package manager mismatch, expect: {}, actual: {} . Restricted list: {}",
      pm.name().green(),
      bin_name.red(),
      config.restricted_list.join(", ").black()
    );
  }

  let metadata = pm.metadata();

  let mut dir = config.node_bin_dir.join(pm.full_name()).join(pm.version());

  let file = dir.join("package.json");

  if !file.try_exists()? {
    dir = snm_pm::downloader::PackageManagerDownloader::new(metadata)
      .download_pm(pm.version())
      .await?;
  }

  let json = snm_pm::package_json::PackageJson::from(dir)?;

  let file = json.get_bin_with_name(bin_name)?;

  Ok(file)
}
