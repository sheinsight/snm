use std::{env::current_dir, process::Output};

use colored::*;
use snm_core::{
    config::SnmConfig,
    exec_proxy_child_process,
    model::{snm_error::handle_snm_error, PackageJson, SnmError},
    println_success,
};
use snm_npm::snm_npm::SnmNpmTrait;
use snm_yarn::snm_yarn::SnmYarn;

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

    if package_manager.name != "yarn" {
        return Err(SnmError::NotMatchPackageManager {
            expect: package_manager.name,
            actual: "yarn".to_string(),
        });
    }
    let v = package_manager.version;
    let mut stdout = std::io::stdout();

    let (v, bin_path_buf) = SnmYarn::new().use_bin("yarn", &v).await?;

    println_success!(stdout, "Use Yarn {} .", format!("{}", v.green()));

    Ok(exec_proxy_child_process!(&bin_path_buf)?)
}
