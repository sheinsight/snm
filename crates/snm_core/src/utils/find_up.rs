use std::path::PathBuf;

use crate::model::SnmError;

pub fn find_up(file_name: &str, cwd: Option<PathBuf>) -> Result<Option<PathBuf>, SnmError> {
    let mut real_cwd_dir = if let Some(cwd_dir) = cwd {
        cwd_dir
    } else {
        std::env::current_dir()?
    };
    while let Some(_) = real_cwd_dir.parent() {
        let target_path = real_cwd_dir.join(file_name);
        if target_path.is_file() {
            return Ok(Some(target_path));
        }

        if !real_cwd_dir.pop() {
            break;
        }
    }

    return Ok(None);
}
