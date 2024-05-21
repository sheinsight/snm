use std::{env::current_dir, path::PathBuf};

use crate::model::SnmError;

pub fn get_current_dir() -> Result<PathBuf, SnmError> {
    let dir = current_dir();
    match dir {
        Ok(d) => Ok(d),
        Err(_) => Err(SnmError::Error("get current dir error".to_string())),
    }
}
