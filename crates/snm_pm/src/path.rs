use std::path::PathBuf;

use snm_core::model::SnmError;

pub fn get_node_modules_dir() -> Result<PathBuf, SnmError> {
    match std::env::var(snm_core::config::NODE_MODULES_DIR_KEY) {
        Ok(node_modules_dir) => Ok(PathBuf::from(node_modules_dir)),
        Err(_) => Err(SnmError::NotFoundNodeModulesDirConfig),
    }
}

pub fn get_npm_and_version_dir(npm_name: &str, version: &str) -> Result<PathBuf, SnmError> {
    let node_modules_dir = get_node_modules_dir()?;
    Ok(node_modules_dir.join(format!("{}@{}", npm_name, version)))
}

pub fn get_npm_downloaded_file_path(npm_name: &str, version: &str) -> Result<PathBuf, SnmError> {
    match std::env::var(snm_core::config::DOWNLOAD_DIR_KEY) {
        Ok(download_dir) => {
            Ok(PathBuf::from(download_dir).join(format!("{}@{}.tgz", npm_name, version)))
        }
        Err(_) => Err(SnmError::NotFoundDownloadDirConfig),
    }
}
