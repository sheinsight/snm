use std::env::{self, current_dir};

use anyhow::Context;
use snm_config::SnmConfig;
use snm_node_version::SNode;
use snm_package_json::pm::PackageManager;
use snm_utils::exec::exec_cli;

pub async fn package_manager() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let cwd = current_dir()?;
    let snm_config = SnmConfig::from(&cwd)?;

    let pm_bin_dir = PackageManager::try_from_env(&snm_config)?
        .get_bin(&args)
        .await?;

    let node_bin_dir = SNode::try_from(&snm_config)
        .with_context(|| "Failed to determine Node.js version")?
        .get_bin()
        .await?;

    exec_cli(vec![pm_bin_dir, node_bin_dir], args)?;

    Ok(())
}
