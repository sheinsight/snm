use std::{path::PathBuf, pin::Pin};

use futures_util::Future;

use super::{shared_behavior::SharedBehaviorTrait, shim::ShimTrait};

pub trait ManageTrait: SharedBehaviorTrait {
    fn get_shim_trait(&self) -> Box<dyn ShimTrait>;

    fn get_download_url(&self, v: &str) -> String;

    fn get_downloaded_file_path_buf(&self, v: &str) -> PathBuf;

    fn get_downloaded_dir_path_buf(&self, v: &str) -> PathBuf;

    fn get_runtime_dir_path_buf(&self, v: &str) -> PathBuf;

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> PathBuf;

    fn get_runtime_base_dir_path_buf(&self) -> PathBuf;

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;

    fn get_host(&self) -> Option<String>;

    fn show_list<'a>(
        &'a self,
        dir_tuple: &'a (Vec<String>, Option<String>),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    fn show_list_offline<'a>(
        &'a self,
        dir_tuple: &'a (Vec<String>, Option<String>),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    fn show_list_remote<'a>(
        &'a self,
        dir_tuple: &'a (Vec<String>, Option<String>),
        all: bool,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> ();
}
