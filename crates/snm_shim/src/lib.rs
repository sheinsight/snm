mod download;
mod ensure_binary_path;

use ensure_binary_path::ensure_binary_path;
use snm_config::parse_snm_config;
use snm_core::traits::atom::AtomTrait;
use snm_node::snm_node::SnmNode;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::env::{self, current_dir};

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    if snm_config.get_strict() && snm_config.get_runtime_package_manager().is_none() {
        return Err(SnmError::NotFoundPackageJsonError(dir.to_path_buf()));
    }

    let restricted_list = vec!["install", "i", "run"];

    let version = match snm_config.get_runtime_package_manager() {
        Some(package_manager) if package_manager.name == prefix => Some(package_manager.version),
        Some(package_manager) if restricted_list.contains(&command.as_str()) => {
            return Err(SnmError::NotMatchPackageManagerError {
                raw_command: args_all.join(" "),
                expected: package_manager.name.clone(),
                actual: prefix.to_string(),
            });
        }
        _ => snm_package_manage.get_default_version()?,
    };

    let binary_dir_string = match version {
        Some(v) => ensure_binary_path(snm_package_manage, &v).await?,
        None => "".to_string(),
    };

    let node_dir = get_node_bin_dir().await?;

    exec_cli(vec![binary_dir_string, node_dir], bin_name, &args)?;

    Ok(())
}

pub async fn get_node_bin_dir() -> Result<String, SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: &dyn AtomTrait = &SnmNode::new(snm_config.clone());

    let version = match snm_config.get_runtime_node_version() {
        Some(node_version) => Some(node_version),
        None => snm_node.get_default_version()?,
    };

    let binary_dir_string = match version {
        Some(v) => ensure_binary_path(snm_node, &v).await?,
        None => return Err(SnmError::NotFoundValidVersion),
    };

    Ok(binary_dir_string)
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let bin_args: Vec<String> = std::env::args().skip(1).collect();

    let node_dir = get_node_bin_dir().await?;

    exec_cli(vec![node_dir], bin_name, &bin_args)?;

    Ok(())
}
