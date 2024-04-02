use crate::model::SnmError;
use colored::*;
use std::fs::read_to_string;

pub fn read_to_json<T: serde::de::DeserializeOwned>(
    abs_path: &std::path::PathBuf,
) -> Result<T, SnmError> {
    let parse_to_json = |contents: String| {
        serde_json::from_str::<T>(&contents).map_err(|e| {
            return SnmError::SerdeJsonError {
                file_path: abs_path.display().to_string().bright_red().to_string(),
            };
        })
    };
    read_to_string(abs_path)
        .map_err(|_| SnmError::ReadFileToStringError {
            file_path: abs_path.display().to_string(),
        })
        .and_then(parse_to_json)
}
