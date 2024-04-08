use std::{env::current_dir, ops::Not, path::PathBuf};

use colored::*;
use snm_core::{
    config::SnmConfig,
    exec_proxy_child_process,
    model::{snm_error::handle_snm_error, PackageJson, SnmError},
    println_success,
};
use snm_npm::snm_npm::{SnmNpm, SnmNpmTrait};

const PACKAGE_JSON_FILE: &str = "package.json";

const PREFIX: &str = "npm";

#[tokio::main]
async fn main() {
    env_logger::init();

    match execute().await {
        Ok((v, bin_path_buf)) => {
            println_success!(std::io::stdout(), "Use {} {}. ", PREFIX, v.green());
            exec_proxy_child_process!(&bin_path_buf);
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }
}

fn get_npm_binary_path(npm_package_json_path_buf: &PathBuf) -> Result<PathBuf, SnmError> {
    let npm_package_json = PackageJson::from_file_path(&npm_package_json_path_buf)?;
    let npm_bin_path_buf = npm_package_json
        .bin_to_hashmap()?
        .remove(PREFIX)
        .ok_or(SnmError::UnknownError)?;
    Ok(npm_bin_path_buf)
}

async fn execute() -> Result<(String, PathBuf), SnmError> {
    let snm_config = SnmConfig::new();
    let snm_npm = SnmNpm::new();

    let node_modules_dir_path_buf = snm_config.get_node_modules_dir_path_buf()?;

    let package_json_path_buf = current_dir()?.join(PACKAGE_JSON_FILE);

    if package_json_path_buf.exists().not() {
        if snm_config.get_strict() {
            Err(SnmError::NotFoundPackageJsonFileError {
                package_json_file_path: package_json_path_buf.display().to_string(),
            })?;
        } else {
            let (npm_vec, default_version) = snm_npm.read_bin_dir()?;
            if npm_vec.is_empty() {
                Err(SnmError::EmptyPackageManagerList {
                    name: PREFIX.to_string(),
                })?;
            }

            let version = default_version.ok_or(SnmError::NotFoundDefaultPackageManager {
                name: PREFIX.to_string(),
            })?;

            let npm_package_json_path_buf = node_modules_dir_path_buf
                .join(format!("{}@{}", PREFIX, version))
                .join(PACKAGE_JSON_FILE);

            let npm_bin_path_buf = get_npm_binary_path(&npm_package_json_path_buf)?;

            return Ok((version, npm_bin_path_buf));
        }
    }

    let package_json = PackageJson::from_file_path(&package_json_path_buf)?;

    let package_manager = package_json.parse_package_manager()?;

    if package_manager.name != PREFIX {
        Err(SnmError::NotMatchPackageManager {
            expect: package_manager.name,
            actual: PREFIX.to_string(),
        })?;
    }

    let npm_package_json_path_buf = node_modules_dir_path_buf
        .join(format!("{}@{}", PREFIX, &package_manager.version))
        .join(PACKAGE_JSON_FILE);

    if npm_package_json_path_buf.exists().not() {
        match snm_config.get_package_manager_install_strategy()? {
            snm_core::config::snm_config::InstallStrategy::Ask => {
                if snm_npm.ask_download(&package_manager.version)? {
                    let tar = snm_npm.download(&package_manager.version).await?;
                    snm_npm.decompress(&tar, &package_manager.version)?;
                }
            }
            snm_core::config::snm_config::InstallStrategy::Panic => {
                Err(SnmError::UnsupportedPackageManager {
                    name: "npm".to_string(),
                    version: package_manager.version.clone(),
                })?
            }
            snm_core::config::snm_config::InstallStrategy::Install => {
                let tar = snm_npm.download(&package_manager.version).await?;
                snm_npm.decompress(&tar, &package_manager.version)?;
            }
        }
    }

    let npm_bin_path_buf = get_npm_binary_path(&npm_package_json_path_buf)?;

    Ok((package_manager.version, npm_bin_path_buf))
}
