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
        "zip" => {
            // 创建 zip archive
            let mut archive = ZipArchive::new(input_file)?;

            // 遍历 archive 中的每个文件
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;

                // 获取文件名
                let outpath = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue, // 跳过无效路径
                };

                // 构造完整的输出文件路径
                let final_path = output_path.join(outpath);

                // 判断是文件还是目录
                if file.is_dir() {
                    // 创建目录
                    std::fs::create_dir_all(&final_path)?;
                } else {
                    // 确保父目录存在
                    if let Some(parent) = final_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    // 创建文件并写入内容
                    let mut outfile = File::create(&final_path)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
        }
        "xz" => {
            let decoder = XzDecoder::new(input_file);
            let mut archive = Archive::new(decoder);
            archive.unpack(output_path)?;
        }

        _ => {
            // return Err(SnmError::UnknownTarballType);
            todo!("UnknownTarballType")
        }
    }

    Ok(())
}
