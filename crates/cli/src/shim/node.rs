mod shim;
use colored::*;
use shim::{get_binary_path_buf_by_strict, get_default_version};
use snm_config::{parse_snm_config, SnmConfig};
use snm_core::{println_error, traits::manage::ManageTrait};
use snm_current_dir::current_dir;
use snm_node::snm_node::SnmNode;
use snm_node_version::parse_node_version;
use snm_utils::{exec::exec_cli, snm_error::SnmError};
use std::{ops::Not, path::PathBuf};
const BIN_NAME: &str = "node";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn ManageTrait> = Box::new(SnmNode::new(snm_config.clone()));

    let mut version = parse_node_version(&snm_config.get_workspace()?)
        .ok()
        .and_then(|node_version| node_version.map(|nv| nv.get_version()))
        .flatten();

    if snm_config.get_strict().not() && version.is_none() {
        version = get_default_version(&snm_node)?;
    }

    let args: Vec<String> = std::env::args().skip(1).collect();

    let binary_path_buf = get_binary_path_buf(BIN_NAME, version, &snm_node).await?;

    exec_cli(binary_path_buf, &args);

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
