use std::{
  env::{self},
  path::PathBuf,
};

use snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::pm::PackageManager;
use snm_utils::{exec::exec_cli, trace_if};
use tracing::trace;

const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];

pub async fn load_pm(
  snm_config: &SnmConfig,
  exe_name: &str,
  args: Vec<String>,
) -> anyhow::Result<()> {
  trace_if!(|| {
    trace!(
      r#"Load pm shim , exe_name: {}, args: {}"#,
      exe_name,
      args.join(" ")
    );
  });

  let pm_bin_file = get_package_manager_bin(&snm_config).await?;

  trace_if!(|| {
    trace!("Found pm bin file: {:?}", pm_bin_file);
  });

  let node_bin_dir = SNode::try_from(&snm_config)?.get_bin().await?;

  let node_str = String::from("node");
  let node_bin_dir_str = node_bin_dir.to_string_lossy().into_owned();

  let paths = vec![node_bin_dir_str];

  let command_args = args[1..].to_vec();

  if let Some(pm_bin_file) = pm_bin_file {
    let args = [
      &[node_str, pm_bin_file.to_string_lossy().into_owned()],
      command_args.as_slice(),
    ]
    .concat();
    exec_cli(args, paths, true)?;
  } else {
    if !is_npm_command(exe_name) {
      exec_cli(args, paths, true)?;
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

      exec_cli(exec_args, paths, true)?;
    }
  }

  Ok(())
}

// 删除 build_bin_path 函数
fn is_npm_command(command: &str) -> bool {
  NPM_COMMANDS.contains(&command)
}

async fn get_package_manager_bin(config: &SnmConfig) -> anyhow::Result<Option<PathBuf>> {
  match PackageManager::try_from_env(config) {
    Ok(pm) => Ok(Some(pm.get_bin(&env::args().collect()).await?)),
    Err(_) => Ok(None),
  }
}
