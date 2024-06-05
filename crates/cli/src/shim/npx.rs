mod shim;

use crate::shim::launch_shim;
use snm_package_manager::snm_package_manager::SnmPackageManager;

const BIN_NAME: &str = "npx";

#[tokio::main]
async fn main() {
    env_logger::init();
    launch_shim(Box::new(SnmPackageManager::new()), BIN_NAME).await;
}
