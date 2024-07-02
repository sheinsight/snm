use flate2::read::GzDecoder;
use snm_utils::snm_error::SnmError;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
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
            let new_dir_name = "package";
            let decoder = GzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);

            // yarn decompress dir different versions are different.
            // .
            // ├── 1.22.0
            // │   └── yarn-v1.22.0    👈
            // ├── 1.22.1
            // │   └── package
            // ├── 1.22.10
            // │   └── yarn-v1.22.10   👈
            // ├── 1.22.21
            // │   └── package
            // ├── 1.22.22
            // │   └── package
            // ├── 2.4.1
            // │   └── package
            // ├── 2.4.2
            // │   └── package
            // ├── 4.0.0
            // │   └── package

            for entry in archive.entries()? {
                let mut entry = entry?;
                let path = entry.path()?.to_owned();
                let new_path = if let Some(first_component) = path.components().next() {
                    let stripped = path.strip_prefix(first_component.as_os_str()).unwrap();
                    Path::new(new_dir_name).join(stripped)
                } else {
                    path.to_path_buf()
                };
                let final_path = Path::new(&output_path).join(new_path);
                if let Some(parent) = final_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                entry.unpack(final_path)?;
            }
        }
        TarballType::Xz => {
            let decoder = XzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);
            archive.unpack(output_path)?;
        }
    }

    Ok(())
}
