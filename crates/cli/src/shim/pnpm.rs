use snm_config::parse_snm_config;
use snm_current_dir::current_dir;
use snm_package_manager::snm_package_manager::SnmPackageManager;

use crate::shim::launch_shim;

mod shim;
const BIN_NAME: &str = "pnpm";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    launch_shim(
        Box::new(SnmPackageManager::from_prefix("pnpm", snm_config.clone())),
        BIN_NAME,
        snm_config.get_strict(),
    )
    .await;

    Ok(())
}
