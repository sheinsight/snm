use snm_core::traits::atom::AtomTrait;
use snm_node_version::NodeVersion;
use snm_utils::snm_error::SnmError;

pub fn get_node_version(
    node_version: Option<NodeVersion>,
    shim: &dyn AtomTrait,
) -> Result<String, SnmError> {
    match node_version {
        Some(node_version) => {
            let version = node_version.get_version();
            match version {
                Some(version) => return Ok(version),
                None => shim.get_default_version(),
            }
        }
        None => shim.get_default_version(),
    }
}
