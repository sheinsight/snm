mod ensure_binary_path;
mod get_default_bin_dir;
mod get_node_bin_dir;

use crate::get_default_bin_dir::get_default_bin_dir;
use crate::get_node_bin_dir::get_node_bin_dir;
use ensure_binary_path::ensure_binary_path;
use snm_atom::package_manager_atom::PackageManagerAtom;
use snm_config::parse_snm_config;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::env::{self, current_dir};
use tracing_subscriber::{self};

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    color_backtrace::install();

    tracing_subscriber::fmt::init();

    let node_dir = get_node_bin_dir().await?;

    let args_all: Vec<String> = env::args().collect();

    let command = &args_all[1];

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_package_manage = PackageManagerAtom::new(prefix, snm_config.clone());

    let restricted_list = vec!["install", "i", "run"];

    let bin_dirs = if let Some(package_manager) = snm_config.get_runtime_package_manager() {
        tracing::trace!(
            "There is a package manager in the entry process that is currently in use."
        );
        if package_manager.name == prefix {
            let version = package_manager.version;
            vec![
                node_dir.clone(),
                ensure_binary_path(&snm_package_manage, &version).await?,
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

pub async fn load_node_shim(bin_name: &str) -> Result<(), SnmError> {
    // env_logger::init();

    color_backtrace::install();

    tracing_subscriber::fmt::init();

    let bin_args: Vec<String> = std::env::args().skip(1).collect();

    let node_dir = get_node_bin_dir().await?;

    exec_cli(vec![node_dir], bin_name, &bin_args)?;

    Ok(())
}
