mod download;
mod ensure_binary_path;

use ensure_binary_path::ensure_binary_path;
use snm_config::parse_snm_config;
use snm_core::traits::atom::AtomTrait;
use snm_node::snm_node::SnmNode;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::{
    env::{self, current_dir},
    ops::Not,
    path::Path,
};
use tracing::{instrument, Level};
use tracing_subscriber::{self, fmt::format::FmtSpan};

#[instrument(level = "trace", ret)]
fn get_default_bin_dir(node_dir: &str, bin_name: &str) -> Result<String, SnmError> {
    let default_bin_dir = Path::new(&node_dir);

    let default_bin = default_bin_dir.join(bin_name);
    if default_bin.exists().not() {
        return Err(SnmError::CannotFindDefaultCommand {
            command: bin_name.to_string(),
        });
    } else {
        Ok(default_bin_dir.display().to_string())
    }
}

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    color_backtrace::install();

    tracing_subscriber::fmt::init();

    let node_dir = get_node_bin_dir().await?;

    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    let restricted_list = vec!["install", "i", "run"];

    let bin_dirs = if let Some(package_manager) = snm_config.get_runtime_package_manager() {
        tracing::trace!(
            "There is a package manager in the entry process that is currently in use."
        );
        if package_manager.name == prefix {
            let version = package_manager.version;
            vec![
                node_dir.clone(),
                ensure_binary_path(snm_package_manage, &version).await?,
            ]
        } else if restricted_list.contains(&command.as_str()) {
            return Err(SnmError::CannotFindDefaultCommand {
                command: bin_name.to_string(),
            });
        } else {
            vec![node_dir.clone(), get_default_bin_dir(&node_dir, bin_name)?]
        }
    } else {
        vec![node_dir.clone(), get_default_bin_dir(&node_dir, bin_name)?]
    };

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
