use std::{env::current_dir, process::Output};

use snm_core::{
    exec_child_process,
    model::{PackageJson, SnmError},
    utils::health::check_multi_lock_file,
};
use snm_node::node_mg::use_node;
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};
use snm_pm::get_manager_bin_file_path;
use snm_pnpm::snm_pnpm::SnmPnpm;
use snm_yarn::snm_yarn::SnmYarn;

pub async fn launch(name: &str) -> Result<Output, SnmError> {
    env_logger::init();

    snm_core::config::init_config()?;

    check_multi_lock_file()?;

    let pkg_file_path = current_dir()?.join("package.json");

    if pkg_file_path.exists() {
        let package_json = PackageJson::from_file_path(None)?;
        package_json.parse_package_manager()?;
    }

    // TODO parse_package_manager 可能需要返回 None 不能直接报错  launch
    let package_manager = package_json.parse_package_manager()?;

    let res = match name {
        "npm" => {
            let snm_npm = SnmNpm::new();
            let bin_file_path = snm_npm.use_bin("npm", None).await?;
            let args: Vec<String> = std::env::args().skip(1).collect();
            let output = exec_child_process!(bin_file_path, &args)?;
            output
        }
        "pnpm" => {
            let snm_npm = SnmPnpm::new();
            let bin_file_path = snm_npm.use_bin("pnpm", None).await?;
            let args: Vec<String> = std::env::args().skip(1).collect();
            let output = exec_child_process!(bin_file_path, &args)?;
            output
        }
        "yarn" => {
            let snm_yarn = SnmYarn::new();
            let bin_file_path = snm_yarn.use_bin("yarn", None).await?;
            let args: Vec<String> = std::env::args().skip(1).collect();
            let output = exec_child_process!(bin_file_path, &args)?;
            output
        }
        "node" => {
            let node_binary_abs_path = use_node().await?;
            let args: Vec<String> = std::env::args().skip(1).collect();
            let output = exec_child_process!(node_binary_abs_path, &args)?;
            output
        }
        _ => return Err(SnmError::UnknownError),
    };

    Ok(res)
}
