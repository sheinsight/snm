mod download;
mod ensure_binary_path;
mod get_node_version;

use std::env;

use colored::*;
use ensure_binary_path::ensure_binary_path;
use get_node_version::get_node_version;
use snm_config::parse_snm_config;
use snm_core::traits::atom::AtomTrait;
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<String, SnmError> {
    env_logger::init();

    let args_all: Vec<String> = env::args().collect();

    let by = format!("by {}", args_all.join(" "));

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let package_json = parse_package_json(&dir)?;

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    let package_manager = match package_json {
        Some(ref package_json) => &package_json.package_manager,
        None if snm_config.get_strict() => {
            return Err(SnmError::NotFoundPackageJsonError(dir.to_path_buf()));
        }
        None => &None,
    };

    if snm_config.get_strict() && package_manager.is_none() {
        return Err(SnmError::NotFoundPackageJsonError(dir.to_path_buf()));
    }

    let version = match package_manager {
        Some(package_manager) if package_manager.name == bin_name => {
            package_manager.version.clone()
        }
        Some(package_manager) => {
            return Err(SnmError::NotMatchPackageManagerError {
                raw_command: args_all.join(" "),
                expected: package_manager.name.clone(),
                actual: bin_name.to_string(),
            });
        }
        None => snm_package_manage.get_default_version()?,
    };

    println!(
        r##"
        🚀  {} {} Command agent active . 

            {}
    "##,
        bin_name.bold().purple(),
        version.bold().yellow(),
        by.black()
    );

    let binary_path_buf = ensure_binary_path(bin_name, snm_package_manage, &version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(version)
}

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args_all: Vec<String> = env::args().collect();

    let by = format!("by {}", args_all.join(" "));

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: &dyn AtomTrait = &SnmNode::new(snm_config.clone());

    let node_version = parse_node_version(&snm_config.get_workspace()?)?;

    let version = get_node_version(node_version, snm_node)?;

    println!(
        r##"
        🚀  {} {} Command agent active . 

            {}
    "##,
        bin_name.bold().purple(),
        version.bold().yellow(),
        by.black()
    );

    let binary_path_buf = ensure_binary_path(bin_name, snm_node, &version).await?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}
