use std::{fs, ops::Not};

use snm_utils::snm_error::SnmError;

use crate::traits::atom::AtomTrait;

pub fn read_runtime_dir_name_vec(
    shim: &dyn AtomTrait,
) -> Result<(Vec<String>, Option<String>), SnmError> {
    let runtime_dir_path_buf = shim.get_runtime_base_dir_path_buf()?;

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
