use snm_package_json::PackageJson;
use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

pub fn get_package_manage_version(
    package_json: Option<PackageJson>,
    shim: &dyn AtomTrait,
) -> Result<String, SnmError> {
    match package_json {
        Some(package_json) => match package_json.package_manager {
            Some(package_manager) => Ok(package_manager.version),
            None => shim.get_default_version(),
        },
        None => shim.get_default_version(),
    }
}