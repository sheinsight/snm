mod shim;

use std::ops::Not;

use shim::{get_binary_path_buf_by_default, get_binary_path_buf_by_strict};
use snm_config::parse_snm_config;
use snm_core::{println_error, traits::manage::ManageTrait};
use snm_current_dir::current_dir;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::exec::exec_cli;
const BIN_NAME: &str = "npx";
const PREFIX: &str = "npm";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn ManageTrait> =
        Box::new(SnmPackageManager::from_prefix(PREFIX, snm_config.clone()));

    if let Some(package_manager) = parse_package_json(&dir).and_then(|x| x.package_manager) {
        let name = package_manager.name.unwrap();
        if name != PREFIX {
            let msg = format!("you config {} but use {}", name.to_string(), BIN_NAME);
            // panic!("{}", msg);
            panic!("{msg}")
        }

        let args: Vec<String> = std::env::args().skip(1).collect();

        if let Ok(binary_path_buf) =
            get_binary_path_buf_by_strict(&snm_node, BIN_NAME, package_manager.version).await
        {
            exec_cli(binary_path_buf, &args);
            return Ok(());
        }

        if snm_config.get_strict().not() {
            if let Ok(binary_path_buf) = get_binary_path_buf_by_default(&snm_node, BIN_NAME).await {
                exec_cli(binary_path_buf, &args);
                return Ok(());
            }
        }
    } else {
        println_error!("No found valid package manager ")
    }

    Ok(())
}
