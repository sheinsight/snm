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

    let restricted_list = vec!["install", "i", "run"];

    let get_default_version = || -> Result<String, SnmError> {
        if snm_config.get_strict() && restricted_list.contains(&command.as_str()) {
            Err(SnmError::NotFoundValidVersion)
        } else {
            let (_, version) = snm_package_manage.read_runtime_dir_name_vec()?;
            version.ok_or(SnmError::NotFoundValidVersion)
        }
    };

    let version = if let Some(package_manager) = snm_config.get_runtime_package_manager() {
        if package_manager.name == prefix {
            Ok(package_manager.version)
        } else {
            if restricted_list.contains(&command.as_str()) {
                Err(SnmError::NotMatchPackageManagerError {
                    raw_command: args_all.join(" "),
                    expected: package_manager.name.clone(),
                    actual: prefix.to_string(),
                })
            } else {
                get_default_version()
            }
        }
    } else {
        let default_version = get_default_version();
        snm_config
            .get_snm_package_json()
            .and_then(|item| item.package_manager)
            .map(|item| Ok(item.version))
            .unwrap_or(default_version)
    };

    let node_dir = get_node_bin_dir().await?;

    let mut bin_dirs = vec![node_dir];

    if let Ok(v) = version {
        let binary_dir_string = ensure_binary_path(snm_package_manage, &v).await?;
        bin_dirs.insert(0, binary_dir_string);
    }

    exec_cli(bin_dirs, bin_name, &args)?;

    Ok(())
}

pub async fn get_node_bin_dir() -> Result<String, SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: &dyn AtomTrait = &SnmNode::new(snm_config.clone());

    let get_default_version = || -> Result<String, SnmError> {
        if snm_config.get_strict() {
            Err(SnmError::NotFoundValidVersion)
        } else {
            let (_, version) = snm_node.read_runtime_dir_name_vec()?;
            version.ok_or(SnmError::NotFoundValidVersion)
        }
    };

    let version = if let Some(version) = snm_config.get_runtime_node_version() {
        version
    } else {
        let default_version = get_default_version()?;
        snm_config
            .get_snm_node_version()
            .and_then(|node_version| node_version.get_version())
            .unwrap_or(default_version)
    };

    let binary_dir_string = ensure_binary_path(snm_node, &version).await?;

    Ok(binary_dir_string)
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let bin_args: Vec<String> = std::env::args().skip(1).collect();

    let node_dir = get_node_bin_dir().await?;

    exec_cli(vec![node_dir], bin_name, &bin_args)?;

    Ok(())
}
