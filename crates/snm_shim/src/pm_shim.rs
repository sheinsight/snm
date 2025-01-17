use std::{
  env::{self, current_dir},
  path::{Path, PathBuf},
};

use snm_config::SnmConfig;
use snm_node::SNode;
use snm_pm::pm::PackageManager;
use snm_utils::exec::exec_cli;

const NPM_COMMANDS: [&str; 2] = ["npm", "npx"];

pub async fn package_manager(actual_bin_name: &str) -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();

  let cwd = current_dir()?;

  let snm_config = SnmConfig::from(&cwd)?;

  let pm_bin_file = get_package_manager_bin(&snm_config, actual_bin_name).await?;

  let node_bin_dir = SNode::try_from(&snm_config)?.get_bin().await?;

  let dir = build_bin_path(&pm_bin_file, &node_bin_dir);

  if let Some(pm_bin_file) = pm_bin_file {
    exec_cli(
      dir,
      vec![
        "node".to_string(),
        pm_bin_file.to_string_lossy().to_string(),
        args.iter().skip(1).map(|s| s.to_string()).collect(),
      ],
    )?;
  } else {
    if !is_npm_command(actual_bin_name) {
      anyhow::bail!("Can't find command {}", actual_bin_name);
    }

    #[cfg(target_os = "windows")]
    {
      let pm_bin_file = node_bin_dir.join(format!("{}.cmd", actual_bin_name));
      exec_cli(
        dir,
        vec![
          pm_bin_file.to_string_lossy().to_string(),
          args.iter().skip(1).map(|s| s.to_string()).collect(),
        ],
      )?;
    }

    #[cfg(not(target_os = "windows"))]
    {
      let pm_bin_file = node_bin_dir.join(actual_bin_name);
      exec_cli(
        dir,
        vec![
          "node".to_string(),
          pm_bin_file.to_string_lossy().to_string(),
          args.iter().skip(1).map(|s| s.to_string()).collect(),
        ],
      )?;
    }

    // println!("pm_bin_file---->: {:?}", &pm_bin_file);

    // let real_pm_path = get_real_path(pm_bin_file.clone())?;

    // println!("real_pm_path---->: {:?}", &real_pm_path);
  }

  Ok(())
}

// fn get_real_path(link_path: PathBuf) -> anyhow::Result<PathBuf> {
//   if link_path.is_symlink() {
//     // 读取符号链接的目标
//     let target = std::fs::read_link(&link_path)?;

//     // 如果目标路径是相对路径，需要基于链接所在目录解析
//     if target.is_relative() {
//       let parent = link_path
//         .parent()
//         .ok_or_else(|| anyhow::anyhow!("Link has no parent directory"))?;
//       Ok(parent.join(target).canonicalize()?)
//     } else {
//       // 如果是绝对路径，直接规范化
//       Ok(target.canonicalize()?)
//     }
//   } else {
//     // 如果不是符号链接，返回规范化的原始路径
//     Ok(link_path.canonicalize()?)
//   }
// }

fn build_bin_path<T: AsRef<Path>>(pm_bin_file: &Option<T>, node_bin_dir: &T) -> Vec<String> {
  if let Some(pm_bin_file) = pm_bin_file {
    vec![
      pm_bin_file.as_ref().to_string_lossy().to_string(),
      node_bin_dir.as_ref().to_string_lossy().to_string(),
    ]
  } else {
    vec![node_bin_dir.as_ref().to_string_lossy().to_string()]
  }
}

fn is_npm_command(command: &str) -> bool {
  NPM_COMMANDS.contains(&command)
}

async fn get_package_manager_bin(
  config: &SnmConfig,
  bin_name: &str,
) -> anyhow::Result<Option<PathBuf>> {
  match PackageManager::try_from_env(config) {
    Ok(pm) => Ok(Some(pm.get_bin(&env::args().collect()).await?)),
    Err(_) => match PackageManager::from_default(bin_name, config) {
      Ok(file) => Ok(Some(file)),
      Err(_) => Ok(None),
    },
  }
}
