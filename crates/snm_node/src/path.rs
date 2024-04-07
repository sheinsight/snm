use snm_core::{
    config::{
        cfg::{get_arch, get_os, get_tarball_ext},
        SnmConfig,
    },
    model::SnmError,
};
use std::path::PathBuf;

pub fn get_node_binary_file_path(node_version: &str) -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir.join(&node_version).join("bin").join("node"))
}

pub fn get_node_binary_base_dir() -> Result<PathBuf, SnmError> {
    let snm_config = SnmConfig::new();
    match snm_config.get_node_bin_dir_path_buf() {
        Ok(bin_dir) => Ok(PathBuf::from(bin_dir)),
        Err(_) => Err(SnmError::NotFoundBinDirConfig),
    }
}

pub fn get_node_dir(node_version: &str) -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir.join(&node_version))
}

pub fn get_default_node_dir(node_version: &str) -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir.join(format!("{}-default", node_version)))
}

pub fn get_default_node_binary_file_path(node_version: &str) -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir
        .join(format!("v{}-default", node_version))
        .join("bin")
        .join("node"))
}

pub fn get_node_tar_file_name(node_version: &str) -> String {
    format!(
        "node-v{}-{}-{}.{}",
        node_version,
        get_os(),
        get_arch(),
        get_tarball_ext()
    )
}

pub fn get_download_dir_path() -> Result<PathBuf, SnmError> {
    match SnmConfig::new().get_download_dir_path_buf() {
        Ok(download_dir) => Ok(PathBuf::from(download_dir)),
        Err(_) => Err(SnmError::NotFoundDownloadDirConfig),
    }
}

pub fn get_node_tar_file_path(node_version: &str) -> Result<PathBuf, SnmError> {
    let download_dir = get_download_dir_path()?;
    Ok(download_dir
        .join(node_version)
        .join(get_node_tar_file_name(node_version)))
}

pub fn get_node_tar_sha256_file_path(node_version: &str) -> Result<PathBuf, SnmError> {
    let download_dir = get_download_dir_path()?;
    Ok(download_dir.join(node_version).join("SHASUMS256.txt"))
}

pub fn get_node_list_json() -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir.join("list.json"))
}

pub fn get_node_schedule_json() -> Result<PathBuf, SnmError> {
    let bin_dir = get_node_binary_base_dir()?;
    Ok(bin_dir.join("schedule.json"))
}
