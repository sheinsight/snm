use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::model::SnmError;

pub fn calc_sha256(tar_file_path: &PathBuf) -> Result<String, SnmError> {
    let file = File::open(tar_file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    let mut buffer = [0; 1024];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
