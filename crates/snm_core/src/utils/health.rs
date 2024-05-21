use crate::model::SnmError;

use super::get_current_dir::get_current_dir;

pub static LOCK_FILE_VEC: [&'static str; 3] = ["package-lock.json", "pnpm-lock.yaml", "yarn.lock"];

pub fn check_multi_lock_file() -> Result<Vec<String>, SnmError> {
    let dir = get_current_dir()?;

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
        return Err(SnmError::Error(format!(
            "Multiple package manager lock files found: {} , Please remove the unnecessary ones.",
            exists_vec.join(", ")
        )));
    }

    Ok(exists_vec)
}
