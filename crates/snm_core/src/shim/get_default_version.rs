use snm_config::SnmConfig;
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

use super::get_runtime_dir_name_vec::read_runtime_dir_name_vec;

pub fn get_default_version(
    shim: &dyn AtomTrait,
    snm_config: SnmConfig,
) -> Result<String, SnmError> {
    if snm_config.get_strict() {
        return Err(SnmError::NotFoundValidVersion);
    } else {
        let (_, default_v) = read_runtime_dir_name_vec(shim)?;
        default_v.ok_or(SnmError::NotFoundValidVersion)
    }
}
