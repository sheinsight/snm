use std::{
    env::{self, current_dir},
    path::PathBuf,
};

use anyhow::Context;
use snm_config::SnmConfig;
use snm_node::SNode;
use snm_package_json::pm::PackageManager;
use snm_utils::exec::exec_cli;

pub async fn package_manager(actual_bin_name: &str) -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(&cwd)?;

    let node_bin_dir = SNode::try_from(&snm_config)?.get_bin().await?;

    let pm_bin_file = match PackageManager::try_from_env(&snm_config) {
        Ok(pm) => pm.get_bin(&args).await?,
        Err(_) => match PackageManager::from_default(actual_bin_name, &snm_config) {
            Ok(file) => file,
            Err(_) => PathBuf::from(actual_bin_name),
        },
    };

    if pm_bin_file == PathBuf::from(actual_bin_name) {
        // 存在死循环
        anyhow::bail!("Can't find command {} ", actual_bin_name);
        // exec_cli(
        //     vec![node_bin_dir.clone()],
        //     vec![
        //         pm_bin_file.to_string_lossy().to_string(),
        //         args.iter().skip(1).map(|s| s.to_string()).collect(),
        //     ],
        // )?;
    } else {
        exec_cli(
            vec![node_bin_dir.clone()],
            vec![
                "node".to_string(),
                pm_bin_file.to_string_lossy().to_string(),
                args.iter().skip(1).map(|s| s.to_string()).collect(),
            ],
        )?;
    }

    Ok(())
}
