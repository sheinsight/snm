use std::env::current_dir;

use snm_atom::{atom::AtomTrait as _, node_atom::NodeAtom};
use snm_config::parse_snm_config;
use snm_utils::snm_error::SnmError;
use tracing::instrument;

use crate::ensure_binary_path;

#[instrument(level = "trace", ret)]
pub async fn get_node_bin_dir() -> Result<String, SnmError> {
    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node = NodeAtom::new(snm_config.clone());

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

    let binary_dir_string = ensure_binary_path(&snm_node, &version).await?;

    Ok(binary_dir_string)
}
