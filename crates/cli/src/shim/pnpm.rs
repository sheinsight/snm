use snm_package_manager::snm_package_manager::SnmPackageManager;

use crate::shim::launch_shim;

mod shim;
const BIN_NAME: &str = "pnpm";
#[tokio::main]
async fn main() {
    env_logger::init();
    launch_shim(Box::new(SnmPackageManager::from_prefix("pnpm")), BIN_NAME).await;
}
