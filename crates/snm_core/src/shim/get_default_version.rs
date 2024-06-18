use snm_config::SnmConfig;
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

pub fn get_default_version(
    shim: &dyn AtomTrait,
    snm_config: SnmConfig,
) -> Result<String, SnmError> {
    if snm_config.get_strict() {
        return Err(SnmError::NotFoundValidVersion);
    } else {
        let (_, default_v) = shim.read_runtime_dir_name_vec()?;
        default_v.ok_or(SnmError::NotFoundValidVersion)
    }
}
