mod download;
mod ensure_binary_path;

use ensure_binary_path::ensure_binary_path;
use snm_config::parse_snm_config;
use snm_core::traits::atom::AtomTrait;
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::env;

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<String, SnmError> {
    env_logger::init();

    let args_all: Vec<String> = env::args().collect();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    // let package_json = snm_config.get_snm_package_json();

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    // let package_manager = match package_json {
    //     Some(ref package_json) => &package_json.package_manager,
    //     None if snm_config.get_strict() => {
    //         return Err(SnmError::NotFoundPackageJsonError(dir.to_path_buf()));
    //     }
    //     None => &None,
    // };

    if snm_config.get_strict() && snm_config.get_runtime_package_manager_name().is_none() {
        return Err(SnmError::NotFoundPackageJsonError(dir.to_path_buf()));
    }

    let version = match snm_config.get_runtime_package_manager_name() {
        Some(name) if name == bin_name => {
            if let Some(v) = snm_config.get_runtime_package_manager_version().clone() {
                v
            } else {
                return Err(SnmError::NotFoundPackageManagerVersionInEnvError { name: name });
            }
        }
        Some(name) => {
            return Err(SnmError::NotMatchPackageManagerError {
                raw_command: args_all.join(" "),
                expected: name.clone(),
                actual: bin_name.to_string(),
            });
        }
        None => snm_package_manage.get_default_version()?,
    };

    let binary_path_buf = ensure_binary_path(bin_name, snm_package_manage, &version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(version)
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: &dyn AtomTrait = &SnmNode::new(snm_config.clone());

    let version = match snm_config.get_runtime_node_version() {
        Some(node_version) => node_version,
        None => snm_node.get_default_version()?,
    };

    let binary_path_buf = ensure_binary_path(bin_name, snm_node, &version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}
