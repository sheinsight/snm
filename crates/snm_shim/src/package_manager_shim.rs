use std::env::{self, current_dir};

use anyhow::{bail, Context};
use snm_config::SnmConfig;
use snm_node_version::NodeVersionReader;
use snm_package_json::{package_json::PackageJson, pm::PackageManager};
use snm_utils::exec::exec_cli;

pub async fn package_manager(prefix: &str, bin_name: &str) -> anyhow::Result<()> {
    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let cwd = current_dir()?;

    let snm_config = SnmConfig::from(&cwd)?;

    let json = PackageJson::from(&cwd)?;

    let package_manager = match PackageManager::from_env(&snm_config) {
        Ok(pm) => pm,
        Err(_) => match json.package_manager {
            Some(raw) => PackageManager::from_str(&raw, &snm_config).unwrap(),
            None => bail!("No package manager found"),
        },
    };

    let node_version_reader = NodeVersionReader::from_env(&snm_config)
        .or_else(|_| NodeVersionReader::from(&cwd, &snm_config))
        .or_else(|_| NodeVersionReader::from_default(&snm_config))
        .with_context(|| "Failed to determine Node.js version")?;

    let node_bin_dir = node_version_reader.get_bin().await?;

    let version = package_manager.version();

    let bin_dir = package_manager.get_bin(version, prefix, command).await?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    exec_cli(vec![bin_dir, node_bin_dir], bin_name, &args)?;

    Ok(())
}
