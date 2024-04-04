use std::{env::current_dir, process::Output};

use snm_core::{
    config::init_config::{init_config, SNM_ENABLE_DEFAULT_VERSION},
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

    init_config()?;

    check_multi_lock_file()?;

    let enable_default = std::env::var(SNM_ENABLE_DEFAULT_VERSION)?;
    let pkg_file_path = current_dir()?.join("package.json");
    let node_version_file_path = current_dir()?.join(".node-version");

    if enable_default == "false" {
        if ["npm", "pnpm", "yarn"].contains(&name) && !pkg_file_path.exists() {
            return Err(SnmError::NotFoundPackageJsonFileError {
                package_json_file_path: pkg_file_path.display().to_string(),
            });
        }
        if ["node"].contains(&name) && !node_version_file_path.exists() {
            return Err(SnmError::NotFoundNodeVersionFileError {
                file_path: node_version_file_path.display().to_string(),
            });
        }
    }

    let package_json = PackageJson::from_file_path(None)?;

    // TODO parse_package_manager 可能需要返回 None 不能直接报错  launch
    let package_manager = package_json.parse_package_manager()?;

    let v = package_manager.version;

    let bin_file_path = match name {
        "npm" => SnmNpm::new().use_bin("npm", &v).await,
        "pnpm" => SnmPnpm::new().use_bin("pnpm", &v).await,
        "yarn" => SnmYarn::new().use_bin("yarn", &v).await,
        "node" => use_node().await,
        _ => return Err(SnmError::UnknownError),
    }?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = exec_child_process!(bin_file_path, &args)?;

    if !output.status.success() {}

    Ok(output)
}
