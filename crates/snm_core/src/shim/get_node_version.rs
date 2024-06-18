use snm_config::SnmConfig;
use snm_node_version::NodeVersion;
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

use super::get_default_version::get_default_version;

pub fn get_node_version(
    node_version: Option<NodeVersion>,
    shim: &dyn AtomTrait,
    snm_config: SnmConfig,
) -> Result<String, SnmError> {
    match node_version {
        Some(node_version) => {
            let version = node_version.get_version();
            match version {
                Some(version) => return Ok(version),
                None => get_default_version(shim, snm_config),
            }
        }
        None => get_default_version(shim, snm_config),
    }
}
