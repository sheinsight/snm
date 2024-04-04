use std::{env::current_dir, process::Output};

use snm_core::{
    exec_child_process,
    model::{PackageJson, SnmError},
    utils::health::check_multi_lock_file,
};
use snm_node::node_mg::use_node;
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};
use snm_pnpm::snm_pnpm::SnmPnpm;
use snm_yarn::snm_yarn::SnmYarn;

pub async fn launch(name: &str) -> Result<Output, SnmError> {
    env_logger::init();

    snm_core::config::init_config()?;

    check_multi_lock_file()?;

    let pkg_file_path = current_dir()?.join("package.json");

    let package_json = PackageJson::from_file_path(None)?;

    // TODO parse_package_manager 可能需要返回 None 不能直接报错  launch
    let package_manager = package_json.parse_package_manager()?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    let bin_file_path = match name {
        "npm" => SnmNpm::new().use_bin("npm", None).await,
        "pnpm" => SnmPnpm::new().use_bin("pnpm", None).await,
        "yarn" => SnmYarn::new().use_bin("yarn", None).await,
        "node" => use_node().await,
        _ => return Err(SnmError::UnknownError),
    }?;

    let output = exec_child_process!(bin_file_path, &args)?;

    Ok(output)
}
