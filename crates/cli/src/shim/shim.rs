use std::process::Output;

use snm_core::{exec_child_process, model::SnmError, utils::health::check_multi_lock_file};
use snm_node::node_mg::use_node;
use snm_pm::get_manager_bin_file_path;

pub async fn launch(name: &str) -> Result<Output, SnmError> {
    env_logger::init();

    snm_core::config::init_config()?;

    check_multi_lock_file()?;

    let res = match name {
        "yarn" | "npm" | "pnpm" => launch_package_manager_shim(name).await?,
        "node" => launch_node_shim(name).await?,
        _ => return Err(SnmError::UnknownError),
    };

    Ok(res)
}

async fn launch_package_manager_shim(name: &str) -> Result<Output, SnmError> {
    let bin_file_path = get_manager_bin_file_path(name).await?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    let output = exec_child_process!(bin_file_path, &args)?;

    Ok(output)
}

async fn launch_node_shim(_name: &str) -> Result<Output, SnmError> {
    let node_binary_abs_path = use_node().await?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    let output = exec_child_process!(node_binary_abs_path, &args)?;

    Ok(output)
}
