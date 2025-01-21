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
  let pm_bin_file = get_package_manager_bin(&snm_config, exe_name).await?;

  trace_if!(|| {
    trace!("{} bin file: {:?}", exe_name, pm_bin_file);
  });

  let node_bin_dir = SNode::try_from(&snm_config)?.get_bin().await?;

  let paths = vec![node_bin_dir.to_string_lossy().to_string()];

  let command_args = args
    .iter()
    .skip(1)
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

  if let Some(pm_bin_file) = pm_bin_file {
    let mut exec_args = vec![
      "node".to_string(),
      pm_bin_file.to_string_lossy().to_string(),
    ];
    exec_args.extend(command_args.clone());
    exec_cli(paths, exec_args)?;
  } else {
    if !is_npm_command(exe_name) {
      anyhow::bail!("Can't find command {}", exe_name);
    }

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

    exec_cli(paths, exec_args)?;
  }

  Ok(())
}

// 删除 build_bin_path 函数
fn is_npm_command(command: &str) -> bool {
  NPM_COMMANDS.contains(&command)
}

async fn get_package_manager_bin(
  config: &SnmConfig,
  bin_name: &str,
) -> anyhow::Result<Option<PathBuf>> {
  match PackageManager::try_from_env(config) {
    Ok(pm) => Ok(Some(pm.get_bin(&env::args().collect()).await?)),
    Err(_) => match PackageManager::get_default_bin(bin_name, config) {
      Ok(file) => Ok(Some(file)),
      Err(_) => Ok(None),
    },
  }
}
