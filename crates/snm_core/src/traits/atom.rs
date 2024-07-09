use std::{fs, ops::Not as _, path::PathBuf, pin::Pin};

use futures_util::Future;
use snm_config::SnmConfig;
use snm_utils::snm_error::SnmError;
pub trait AtomTrait {
    fn get_download_url(&self, v: &str) -> String;

    fn get_downloaded_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    // fn get_downloaded_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_dir_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_dir_for_default_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_runtime_base_dir_path_buf(&self) -> Result<PathBuf, SnmError>;

    fn get_expect_shasum<'a>(
        &'a self,
        v: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;

    fn get_actual_shasum<'a>(
        &'a self,
        downloaded_file_path_buf: &'a PathBuf,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;

    // fn get_host(&self) -> Option<String>;

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
    ) -> Pin<Box<dyn Future<Output = Result<(), SnmError>> + Send + 'a>>;

    fn decompress_download_file(
        &self,
        input_file_path_buf: &PathBuf,
        output_dir_path_buf: &PathBuf,
    ) -> Result<(), SnmError>;

    // fn get_strict_shim_binary_path_buf(
    //     &self,
    //     bin_name: &str,
    //     version: &str,
    // ) -> Result<PathBuf, SnmError>;

    fn download_condition(&self, version: &str) -> bool;

    fn get_runtime_binary_file_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError>;

    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;

    fn get_snm_config(&self) -> &SnmConfig;

    fn read_runtime_dir_name_vec(&self) -> Result<(Vec<String>, Option<String>), SnmError> {
        let runtime_dir_path_buf = self.get_runtime_base_dir_path_buf()?;

        let mut default_dir = None;

        if runtime_dir_path_buf.exists().not() {
            // TODO here create not suitable , should be find a better way
            fs::create_dir_all(&runtime_dir_path_buf)?;
        }

        let dir_name_vec = runtime_dir_path_buf
            .read_dir()?
            .filter_map(|dir_entry| dir_entry.ok())
            .filter(|dir_entry| dir_entry.path().is_dir())
            .filter_map(|dir_entry| {
                let file_name = dir_entry.file_name().into_string().ok()?;

                if file_name.ends_with("-default") {
                    default_dir = Some(file_name.trim_end_matches("-default").to_string());
                    return None;
                }

                return Some(file_name);
            })
            .collect::<Vec<String>>();

        Ok((dir_name_vec, default_dir))
    }

    fn get_default_version(&self) -> Result<String, SnmError> {
        if self.get_snm_config().get_strict() {
            return Err(SnmError::NotFoundValidVersion);
        } else {
            let (_, default_v) = self.read_runtime_dir_name_vec()?;
            default_v.ok_or(SnmError::NotFoundValidVersion)
        }
    }
}
