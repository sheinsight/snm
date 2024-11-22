use std::env::{self, current_dir};

use anyhow::Context;
use snm_config::SnmConfig;
use snm_node_version::SNode;
use snm_package_json::{package_json::PackageJson, pm::PackageManager};
use snm_utils::exec::exec_cli;

pub async fn package_manager(prefix: &str, bin_name: &str) -> anyhow::Result<()> {
    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(&cwd)?;

    let bin_dir = match PackageJson::from(&cwd) {
        Ok(json) => match json.package_manager {
            Some(pm_raw) => match PackageManager::try_from_env(&pm_raw, &snm_config) {
                Ok(pm) => pm.get_bin(pm.version(), prefix, command).await?,
                Err(_) => String::new(),
            },
            None => String::new(),
        },
        Err(_) => String::new(),
    };

    let node_version_reader =
        SNode::try_from(&snm_config).with_context(|| "Failed to determine Node.js version")?;

    let node_bin_dir = node_version_reader.get_bin().await?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    exec_cli(vec![bin_dir, node_bin_dir], bin_name, &args)?;

    Ok(())
}
