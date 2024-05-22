use std::{env::current_dir, path::PathBuf};

use crate::model::SnmError;

pub fn get_current_dir() -> Result<PathBuf, SnmError> {
    current_dir().map_err(|_| SnmError::Error("get current dir error".to_string()))
}
