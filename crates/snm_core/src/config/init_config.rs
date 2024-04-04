use crate::model::SnmError;
use colored::*;
use std::{env, fs::create_dir_all, path::PathBuf};

pub static BIN_DIR_KEY: &str = "SNM_NODE_BIN_DIR";
pub static DOWNLOAD_DIR_KEY: &str = "SNM_DOWNLOAD_DIR";
pub static NODE_MODULES_DIR_KEY: &str = "SNM_NODE_MODULES_DIR";

pub static SNM_NPM_REGISTRY_HOST_KEY: &str = "SNM_NPM_REGISTRY_HOST";
pub static SNM_YARN_REGISTRY_HOST_KEY: &str = "SNM_YARN_REGISTRY_HOST_KEY";
pub static SNM_YARN_REPO_HOST_KEY: &str = "SNM_YARN_REPO_HOST_KEY";

pub static SNM_ENABLE_DEFAULT_VERSION: &str = "SNM_ENABLE_DEFAULT_VERSION";

pub fn init_config() -> Result<(), SnmError> {
    init_dir()?;

    if let Err(_) = env::var(SNM_ENABLE_DEFAULT_VERSION) {
        env::set_var(SNM_ENABLE_DEFAULT_VERSION, false.to_string());
    }

    init_url_config();

    Ok(())
}

fn init_dir() -> Result<(), SnmError> {
    let home_dir = dirs::home_dir().ok_or(SnmError::GetHomeDirError)?;

    let key = "SNM_BASE_DIR";

    let snm_base_dir = env::var(key)
        .map(|val| home_dir.join(val))
        .unwrap_or(home_dir.join(".snm"));

    let snm_node_bin_dir = snm_base_dir.join("bin");
    let snm_download_dir = snm_base_dir.join("download");
    let snm_node_modules_dir = snm_base_dir.join("node_modules");

    create_dir_all_with_snm_error(BIN_DIR_KEY, snm_node_bin_dir)?;
    create_dir_all_with_snm_error(DOWNLOAD_DIR_KEY, snm_download_dir)?;
    create_dir_all_with_snm_error(NODE_MODULES_DIR_KEY, snm_node_modules_dir)?;

    Ok(())
}

fn init_url_config() {
    if let Err(_) = env::var(SNM_NPM_REGISTRY_HOST_KEY) {
        env::set_var(
            SNM_NPM_REGISTRY_HOST_KEY,
            "https://registry.npmjs.org".to_string(),
        );
    }

    if let Err(_) = env::var(SNM_YARN_REGISTRY_HOST_KEY) {
        env::set_var(
            SNM_YARN_REGISTRY_HOST_KEY,
            "https://registry.yarnpkg.com".to_string(),
        )
    }

    if let Err(_) = env::var(SNM_YARN_REPO_HOST_KEY) {
        env::set_var(
            SNM_YARN_REPO_HOST_KEY,
            "https://repo.yarnpkg.com".to_string(),
        )
    }
}

fn create_dir_all_with_snm_error(
    env_key: &str,
    dir_path_buf: PathBuf,
) -> Result<PathBuf, SnmError> {
    let dir_str = dir_path_buf.display().to_string();
    env::set_var(env_key, &dir_str);
    if !dir_path_buf.exists() {
        if create_dir_all(&dir_path_buf).is_err() {
            return Err(SnmError::CreateDirFailed {
                dir_path: dir_str.bright_red().to_string(),
            });
        }
    }
    Ok(dir_path_buf)
}
