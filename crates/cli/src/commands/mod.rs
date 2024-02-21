use std::error::Error;

use snm_core::utils::package_manager_parser::automatic_version_parsed;

use self::snm::SnmTrait;

pub mod npm;
pub mod pnpm;
pub mod snm;
pub mod yarn;

pub async fn automatic() -> Result<Box<dyn SnmTrait>, Box<dyn Error>> {
    let version_parsed = automatic_version_parsed(None)?;
    let package_manager = match version_parsed.package_manager.as_str() {
        "npm" => Box::new(npm::Npm::new().await?) as Box<dyn SnmTrait>,
        "yarn" => Box::new(yarn::Yarn::new().await?) as Box<dyn SnmTrait>,
        "pnpm" => Box::new(pnpm::Pnpm::new().await?) as Box<dyn SnmTrait>,
        _ => return Err(From::from("Unsupported package manager")),
    };
    Ok(package_manager)
}
