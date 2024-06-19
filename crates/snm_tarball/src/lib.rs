use flate2::read::GzDecoder;
use snm_utils::snm_error::SnmError;
use std::{fs::File, path::PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;

// 压缩包类型

pub enum TarballType {
    Tgz,
    Xz,
}

pub fn decompress(
    input_path: &PathBuf,
    output_path: &PathBuf,
    tarball_type: TarballType,
) -> Result<(), SnmError> {
    let input_file = File::open(input_path)?;

    match tarball_type {
        TarballType::Tgz => {
            let decoder = GzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);
            archive.unpack(output_path)?;
        }
        TarballType::Xz => {
            let decoder = XzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);
            archive.unpack(output_path)?;
        }
    }

    Ok(())
}
