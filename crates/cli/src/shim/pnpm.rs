use std::{env::current_dir, process::Output};

use snm_core::{
    config::init_config,
    exec_child_process,
    model::{snm_error::handle_snm_error, PackageJson, SnmError},
};
use snm_npm::snm_npm::SnmNpmTrait;
use snm_pnpm::snm_pnpm::SnmPnpm;

#[tokio::main]
async fn main() {
    if let Err(error) = execute().await {
        handle_snm_error(error);
    }
}

async fn execute() -> Result<Output, SnmError> {
    init_config()?;

    let workspace = current_dir()?;

    let package_json = PackageJson::from_file_path(Some(workspace))?;

    let package_manager = package_json.parse_package_manager()?;

    let v = package_manager.version;

    let bin_path_buf = SnmPnpm::new().use_bin("pnpm", &v).await?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = exec_child_process!(&bin_path_buf, &args)?;
    Ok(output)
}
