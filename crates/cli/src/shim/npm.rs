mod shim;

use crate::shim::launch_shim;
use snm_core::{
    config::{snm_config::InstallStrategy, SnmConfig},
    snm_content::{SnmContent, SnmContentHandler},
};
use snm_package_manager::snm_package_manager::SnmPackageManager;

const BIN_NAME: &str = "npm";

#[tokio::main]
async fn main() {
    env_logger::init();

    let snm_content_handler: SnmContentHandler = SnmContentHandler::new(SnmContent {
        strict: SnmConfig::new().get_strict(),
        base_dir_path_buf: SnmConfig::new().get_base_dir_path_buf(),
        download_dir_path_buf: SnmConfig::new().get_download_dir_path_buf(),
        node_modules_dir_path_buf: SnmConfig::new().get_node_modules_dir_path_buf(),
        npm_registry: SnmConfig::new().get_npm_registry_host(),
        package_manager_install_strategy: InstallStrategy::Auto,
    });

    launch_shim(
        Box::new(SnmPackageManager::from_prefix(
            "npm",
            snm_content_handler.clone(),
        )),
        BIN_NAME,
        snm_content_handler.get_strict(),
    )
    .await;
}
