use std::path::PathBuf;

use async_trait::async_trait;

use super::{trait_shared_behavior::SharedBehaviorTrait, trait_shim::ShimTrait, SnmError};

#[async_trait(?Send)]
pub trait ManageTrait: SharedBehaviorTrait {
    fn get_shim_trait(&self) -> Box<dyn ShimTrait>;

    fn get_download_url(&self, v: &str) -> Result<String, SnmError>;

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError>;

    async fn get_expect_shasum(&self, v: &str) -> Result<String, SnmError>;

    async fn get_actual_shasum(
        &self,
        downloaded_file_path_buf: &PathBuf,
    ) -> Result<String, SnmError>;

    fn get_host(&self) -> Option<String>;

    async fn show_list(&self, dir_tuple: &(Vec<String>, Option<String>)) -> Result<(), SnmError>;

    async fn show_list_remote(
        &self,
        dir_tuple: &(Vec<String>, Option<String>),
        all: bool,
    ) -> Result<(), SnmError>;

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError>;
}
