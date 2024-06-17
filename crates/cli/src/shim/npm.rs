mod shim;
use colored::*;
use shim::{get_binary_path_buf_by_strict, get_default_version};
use snm_config::parse_snm_config;
use snm_core::{println_error, traits::manage::ManageTrait};
use snm_current_dir::current_dir;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::{ops::Not, path::PathBuf};
const BIN_NAME: &str = "npm";
const PREFIX: &str = "npm";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn ManageTrait> =
        Box::new(SnmPackageManager::from_prefix(PREFIX, snm_config.clone()));

    if let Some(package_manager) = parse_package_json(&dir).and_then(|x| x.package_manager) {
        let name = package_manager.name.unwrap();
        let mut version = package_manager.version;

        if snm_config.get_strict().not() && version.is_none() {
            version = get_default_version(&snm_node)?;
        }

        if name != PREFIX {
            let msg = format!("you config {} but use {}", name.to_string(), BIN_NAME);
            panic!("{msg}")
        }

        let args: Vec<String> = std::env::args().skip(1).collect();

        let binary_path_buf = get_binary_path_buf(BIN_NAME, version, &snm_node).await?;

        exec_cli(binary_path_buf, &args);
    } else {
        println_error!("No found valid package manager ")
    }

    Ok(())
}

pub async fn get_binary_path_buf(
    bin_name: &str,
    version: Option<String>,
    manage: &Box<dyn ManageTrait>,
) -> Result<PathBuf, SnmError> {
    match version {
        Some(v) => get_binary_path_buf_by_strict(manage, bin_name, Some(v)).await,
        None => return Err(SnmError::NotFoundValidNodeVersion),
    }
}
