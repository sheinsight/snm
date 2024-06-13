use std::path::PathBuf;

use snm_utils::snm_error::SnmError;

pub trait SharedBehaviorTrait {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;
}
