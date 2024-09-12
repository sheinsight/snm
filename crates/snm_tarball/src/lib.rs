use flate2::read::GzDecoder;
use snm_utils::snm_error::SnmError;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

pub fn decompress(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), SnmError> {
    let input_file = File::open(input_path)?;

    let extension = input_path.extension().unwrap().to_str().unwrap();

    match extension {
        "tgz" => {
            // let new_dir_name = "package";
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
                    stripped.to_path_buf()
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
        "zip" => {
            let mut archive = ZipArchive::new(input_file)?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;

                let outpath = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };

                let final_path = output_path.join(outpath);

                if file.is_dir() {
                    std::fs::create_dir_all(&final_path)?;
                } else {
                    if let Some(parent) = final_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }

                    let mut outfile = File::create(&final_path)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
        }
        "xz" => {
            let decoder = XzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);
            for entry in archive.entries()? {
                let mut entry = entry?;
                let path = entry.path()?.to_owned();
                let new_path = if let Some(first_component) = path.components().next() {
                    let stripped = path.strip_prefix(first_component.as_os_str()).unwrap();
                    stripped.to_path_buf()
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

        _ => {
            // return Err(SnmError::UnknownTarballType);
            todo!("UnknownTarballType")
        }
    }

    Ok(())
}
