use std::path::PathBuf;

pub trait SharedBehaviorTrait {
    fn get_anchor_file_path_buf(&self, v: &str) -> PathBuf;
}
