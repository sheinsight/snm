use std::path::PathBuf;

use crate::config::snm_config::InstallStrategy;

#[derive(Debug, Clone)]
pub struct SnmContent {
    // strict model
    pub strict: bool,

    // snm work directory
    pub base_dir_path_buf: PathBuf,
    pub download_dir_path_buf: PathBuf,
    pub node_modules_dir_path_buf: PathBuf,

    pub npm_registry: String,

    pub package_manager_install_strategy: InstallStrategy,
}

#[derive(Debug, Clone)]
pub struct SnmContentHandler {
    snm_content: SnmContent,
}

impl SnmContentHandler {
    pub fn new(snm_content: SnmContent) -> Self {
        Self { snm_content }
    }

    pub fn get_base_dir_path_buf(&self) -> PathBuf {
        self.snm_content.base_dir_path_buf.clone()
    }

    pub fn get_download_dir_path_buf(&self) -> PathBuf {
        self.snm_content.download_dir_path_buf.clone()
    }

    pub fn get_node_modules_dir_path_buf(&self) -> PathBuf {
        self.snm_content.node_modules_dir_path_buf.clone()
    }

    pub fn get_npm_registry(&self) -> String {
        self.snm_content.npm_registry.clone()
    }

    pub fn get_strict(&self) -> bool {
        self.snm_content.strict
    }

    pub fn get_package_manager_install_strategy(&self) -> InstallStrategy {
        self.snm_content.package_manager_install_strategy.clone()
    }
}
