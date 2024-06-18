use colored::*;
use snm_config::parse_snm_config;
use snm_core::{
    println_error,
    shim::{
        get_binary_path::ensure_binary_path, get_runtime_dir_name_vec::read_runtime_dir_name_vec,
    },
    traits::atom::AtomTrait,
};
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::ops::Not;

pub fn get_default_version(manage: &dyn AtomTrait) -> Result<Option<String>, SnmError> {
    let (_, default_v) = read_runtime_dir_name_vec(manage)?;
    return Ok(default_v);
}

pub async fn load_package_manage_shim(prefix: &str, bin_name: &str) -> Result<(), SnmError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let package_json = parse_package_json(&dir)?;

    // -- start

    let snm_package_manage: &dyn AtomTrait =
        &SnmPackageManager::from_prefix(prefix, snm_config.clone());

    let mut version = None;
    if let Some(package_manager) = package_json.and_then(|x| x.package_manager) {
        let name = package_manager.name.unwrap();
        version = package_manager.version;

        if snm_config.get_strict().not() && version.is_none() {
            version = get_default_version(snm_package_manage)?;
        }

        if name != prefix {
            let msg = format!("you config {} but use {}", name, bin_name);
            panic!("{msg}");
        }
    } else {
        if snm_config.get_strict().not() && version.is_none() {
            version = get_default_version(snm_package_manage)?;
        } else {
            println_error!("No valid package manager found");
            return Ok(());
        }
    }

    let binary_path_buf = match version {
        Some(v) => ensure_binary_path(bin_name, snm_package_manage, v).await,
        None => return Err(SnmError::NotFoundValidNodeVersion),
    }?;

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

    let mut version = node_version.and_then(|node_version| node_version.get_version());

    if snm_config.get_strict().not() && version.is_none() {
        version = get_default_version(snm_node)?;
    };

    let binary_path_buf = match version {
        Some(v) => ensure_binary_path(bin_name, snm_node, v).await,
        None => return Err(SnmError::NotFoundValidNodeVersion),
    }?;

    exec_cli(binary_path_buf, &args);

    Ok(())
}
