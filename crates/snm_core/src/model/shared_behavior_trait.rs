use std::path::PathBuf;

use super::SnmError;

pub trait SharedBehaviorTrait {
    fn get_anchor_file_path_buf(&self, v: &str) -> Result<PathBuf, SnmError>;
}
