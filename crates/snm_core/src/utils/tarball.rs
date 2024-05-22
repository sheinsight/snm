use flate2::read::GzDecoder;
use std::{fs::File, path::PathBuf};
use tar::Archive;

use crate::model::SnmError;

pub fn decompress_tgz<D>(
    input_path: &PathBuf,
    output_path: &PathBuf,
    get_target_dir: D,
) -> Result<(), SnmError>
where
    D: Fn(&PathBuf) -> PathBuf,
{
    // 打开 tgz 文件
    let tgz_file = File::open(input_path).map_err(|_| {
        SnmError::Error(format!(
            "decompress_tgz File::open error {:?}",
            &input_path.display()
        ))
    })?;
    // 使用 GzDecoder 解压 gzip 文件
    let tar = GzDecoder::new(tgz_file);
    // 创建 Archive 对象以便操作 tar 文件
    let mut archive = Archive::new(tar);

    // 从 archive 中解压所有文件到指定路径
    archive.unpack(output_path).map_err(|_| {
        SnmError::Error(format!(
            "decompress_tgz archive.unpack error {:?}",
            &output_path.display()
        ))
    })?;

    Ok(())
}

pub fn rename<T>(dir: &PathBuf, transform: &T) -> Result<(), SnmError>
where
    T: Fn(&PathBuf) -> Result<PathBuf, SnmError>,
{
    let mut entries = std::fs::read_dir(&dir).map_err(|_| {
        SnmError::Error(format!(
            "rename std::fs::read_dir error {:?}",
            &dir.display()
        ))
    })?;

    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            rename(&path, transform)?;
        } else {
            let from = &path;

            let to = &transform(&from)?;

            if let Some(parent) = to.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent).map_err(|_| {
                        SnmError::Error(format!(
                            "rename std::fs::create_dir_all error {:?}",
                            &parent.display()
                        ))
                    })?;
                }
            }

            std::fs::rename(&from, &to).map_err(|_| {
                SnmError::Error(format!(
                    "rename std::fs::rename error from: {:?} to: {:?}",
                    &from.display(),
                    &to.display()
                ))
            })?;
        }
    }

    Ok(())
}

pub fn decompress_xz(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), SnmError> {
    let input_file = File::open(input_path).map_err(|_| {
        SnmError::Error(format!(
            "decompress_xz File::open error {:?}",
            &input_path.display()
        ))
    })?;

    let decoder = xz2::read::XzDecoder::new(input_file);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&output_path).map_err(|_| {
        SnmError::Error(format!(
            "decompress_xz archive.unpack error {:?}",
            &output_path.display()
        ))
    })?;

    let dir = input_path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.trim_end_matches(".tar.xz").to_string());

    let old_base = output_path.join(dir.as_ref().unwrap());

    let transform = |f: &PathBuf| -> Result<PathBuf, SnmError> {
        let old_base = output_path.join(dir.as_ref().unwrap());
        let new_path = f
            .strip_prefix(&old_base)
            .map_err(|_| SnmError::Error("decompress_xz strip_prefix error".to_string()))?;
        Ok(output_path.join(new_path))
    };

    rename(&old_base, &transform)?;

    std::fs::remove_dir_all(&old_base).map_err(|_| {
        SnmError::Error(format!(
            "decompress_xz remove_dir_all error {:?}",
            &old_base.display()
        ))
    })?;

    Ok(())
}
