use snm_core::model::{PackageJson, SnmError};

use self::snm::SnmTrait;

pub mod npm;
pub mod pnpm;
pub mod snm;
pub mod yarn;

pub async fn automatic() -> Result<Box<dyn SnmTrait>, SnmError> {
    let package_json = PackageJson::from_file_path(None)?;
    let package_manager = package_json.parse_package_manager()?;

    let package_manager: Box<dyn SnmTrait> = match package_manager.package_manager.as_str() {
        "npm" => Box::new(npm::Npm::new(&package_manager.version)),
        "yarn" => Box::new(yarn::Yarn::new(&package_manager.version)),
        "pnpm" => Box::new(pnpm::Pnpm::new(&package_manager.version)),
        _ => return Err(SnmError::UnknownError),
    };

    Ok(package_manager)
}
