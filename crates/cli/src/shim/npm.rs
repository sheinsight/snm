use std::{env::current_dir, process::Output};

use snm_core::{
    config::SnmConfig,
    exec_proxy_child_process,
    model::{snm_error::handle_snm_error, PackageJson, SnmError},
};
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};

#[tokio::main]
async fn main() {
    if let Err(error) = execute().await {
        handle_snm_error(error);
    }
}

async fn execute() -> Result<Output, SnmError> {
    SnmConfig::new().init()?;

    let workspace = current_dir()?;

    let package_json = PackageJson::from_dir_path(Some(workspace))?;

    let package_manager = package_json.parse_package_manager()?;

    if package_manager.name != "npm" {
        return Err(SnmError::NotMatchPackageManager {
            expect: package_manager.name,
            actual: "npm".to_string(),
        });
    }

    let v = package_manager.version;

    let (v, bin_path_buf) = SnmNpm::new().use_bin("npm", &v).await?;

    Ok(exec_proxy_child_process!(&bin_path_buf)?)
}
