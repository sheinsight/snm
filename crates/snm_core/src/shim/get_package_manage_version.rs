use snm_config::SnmConfig;
use snm_package_json::PackageJson;
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

use super::get_default_version::get_default_version;

pub fn get_package_manage_version(
    package_json: Option<PackageJson>,
    shim: &dyn AtomTrait,
    snm_config: SnmConfig,
) -> Result<String, SnmError> {
    match package_json {
        Some(package_json) => match package_json.package_manager {
            Some(package_manager) => Ok(package_manager.version),
            None => get_default_version(shim, snm_config),
        },
        None => get_default_version(shim, snm_config),
    }
}
