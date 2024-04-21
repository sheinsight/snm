use std::path::PathBuf;

use super::{trait_shared_behavior::SharedBehaviorTrait, SnmError};

pub trait ShimTrait: SharedBehaviorTrait {
    fn get_strict_shim_binary_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError>;

    fn get_strict_shim_version(&self) -> Result<String, SnmError>;

    fn download_condition(&self, version: &str) -> Result<bool, SnmError>;

    fn get_runtime_binary_file_path_buf(
        &self,
        bin_name: &str,
        version: &str,
    ) -> Result<PathBuf, SnmError>;

    fn check_satisfy_strict_mode(&self, bin_name: &str) -> Result<(), SnmError>;

    fn check_default_version(
        &self,
        tuple: &(Vec<String>, Option<String>),
    ) -> Result<String, SnmError>;
}
