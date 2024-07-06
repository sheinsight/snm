use std::path::PathBuf;

use crate::snm_error::SnmError;

pub static LOCK_FILE_VEC: [&'static str; 3] = ["package-lock.json", "pnpm-lock.yaml", "yarn.lock"];

pub fn check_duplicate_lock_file(workspace: PathBuf) -> Result<Vec<String>, SnmError> {
    let exists_vec = LOCK_FILE_VEC
        .iter()
        .flat_map(|item| {
            let file_path = workspace.join(item);
            if file_path.exists() {
                Some(item.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if exists_vec.len() > 1 {
        return Err(SnmError::DuplicateLockFileError {
            lock_file_vec: exists_vec,
        });
    }

    Ok(exists_vec)
}
