mod download;
mod ensure_binary_path;
mod get_node_version;
mod get_package_manage_version;

use ensure_binary_path::ensure_binary_path;
use get_node_version::get_node_version;
use get_package_manage_version::get_package_manage_version;
use snm_config::parse_snm_config;
use snm_core::traits::atom::AtomTrait;
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let package_json = parse_package_json(&dir)?;

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    let version = get_package_manage_version(package_json, snm_package_manage)?;

    let binary_path_buf = ensure_binary_path(bin_name, snm_package_manage, version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: &dyn AtomTrait = &SnmNode::new(snm_config.clone());

    let node_version = parse_node_version(&snm_config.get_workspace()?)?;

    let version = get_node_version(node_version, snm_node)?;

    let binary_path_buf = ensure_binary_path(bin_name, snm_node, version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}
