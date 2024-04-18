use std::env::current_dir;

use crate::model::SnmError;

pub static LOCK_FILE_VEC: [&'static str; 3] = ["package-lock.json", "pnpm-lock.yaml", "yarn.lock"];

pub fn check_multi_lock_file() -> Result<Vec<String>, SnmError> {
    let dir = current_dir().expect("get current dir error.");

    let exists_vec = LOCK_FILE_VEC
        .iter()
        .flat_map(|item| {
            let file_path = dir.join(item);
            if file_path.exists() {
                Some(item.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if exists_vec.len() > 1 {
        return Err(SnmError::MultiPackageManagerLockFileError {
            lock_file: exists_vec,
        });
    }

    Ok(exists_vec)
}
