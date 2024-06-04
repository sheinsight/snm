use std::{env::current_dir, path::PathBuf};

pub fn get_current_dir() -> PathBuf {
    current_dir().expect("get current dir error")
}
