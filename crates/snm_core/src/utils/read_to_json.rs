use crate::model::SnmError;
use colored::*;
use std::fs::read_to_string;

pub fn read_to_json<T: serde::de::DeserializeOwned>(
    abs_path: &std::path::PathBuf,
) -> Result<T, SnmError> {
    // if let Ok(contents) = read_to_string(abs_path) {
    //     if let Ok(node_vec) = serde_json::from_str::<T>(&contents) {
    //         Ok(node_vec)
    //     } else {
    //         Err(SnmError::SerdeJsonError(abs_path.display().to_string()))
    //     }
    // } else {
    //     Err(SnmError::ReadFileToStringError(
    //         abs_path.display().to_string(),
    //     ))
    // }
    let parse_to_json = |contents: String| {
        serde_json::from_str::<T>(&contents).map_err(|e| SnmError::SerdeJsonError {
            file_path: abs_path.display().to_string().bright_red().to_string(),
        })
    };
    read_to_string(abs_path)
        .map_err(|_| SnmError::ReadFileToStringError {
            file_path: abs_path.display().to_string(),
        })
        .and_then(parse_to_json)
}
