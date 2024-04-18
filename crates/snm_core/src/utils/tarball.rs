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
    let tgz_file = File::open(input_path).expect(
        format!(
            "decompress_tgz File::open error {:?}",
            &input_path.display()
        )
        .as_str(),
    );
    // 使用 GzDecoder 解压 gzip 文件
    let tar = GzDecoder::new(tgz_file);
    // 创建 Archive 对象以便操作 tar 文件
    let mut archive = Archive::new(tar);

    // 从 archive 中解压所有文件到指定路径
    archive.unpack(output_path).expect(
        format!(
            "decompress_tgz archive.unpack error {:?}",
            &output_path.display()
        )
        .as_str(),
    );

    let old_base = get_target_dir(output_path);

    let transform = |f: &PathBuf| -> Result<PathBuf, SnmError> {
        let new_path = f
            .strip_prefix(&old_base)
            .expect("decompress_tgz strip_prefix error");
        Ok(output_path.join(new_path))
    };

    rename(&old_base, &transform)?;

    std::fs::remove_dir_all(&old_base).expect(
        format!(
            "decompress_tgz remove_dir_all error {:?}",
            &old_base.display()
        )
        .as_str(),
    );

    Ok(())
}

pub fn rename<T>(dir: &PathBuf, transform: &T) -> Result<(), SnmError>
where
    T: Fn(&PathBuf) -> Result<PathBuf, SnmError>,
{
    let mut entries = std::fs::read_dir(&dir)
        .expect(format!("rename std::fs::read_dir error {:?}", &dir.display()).as_str());

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
                    std::fs::create_dir_all(&parent).expect(
                        format!(
                            "rename std::fs::create_dir_all error {:?}",
                            &parent.display()
                        )
                        .as_str(),
                    );
                }
            }

            std::fs::rename(&from, &to).expect(
                format!(
                    "rename std::fs::rename error from: {:?} to: {:?}",
                    &from.display(),
                    &to.display()
                )
                .as_str(),
            );
        }
    }

    Ok(())
}

pub fn decompress_xz(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), SnmError> {
    let input_file = File::open(input_path)
        .expect(format!("decompress_xz File::open error {:?}", &input_path.display()).as_str());

    let decoder = xz2::read::XzDecoder::new(input_file);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&output_path).expect(
        format!(
            "decompress_xz archive.unpack error {:?}",
            &output_path.display()
        )
        .as_str(),
    );

    let dir = input_path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.trim_end_matches(".tar.xz").to_string());

    let old_base = output_path.join(dir.as_ref().unwrap());

    let transform = |f: &PathBuf| -> Result<PathBuf, SnmError> {
        let old_base = output_path.join(dir.as_ref().unwrap());
        let new_path = f
            .strip_prefix(&old_base)
            .expect("decompress_xz strip_prefix error");
        Ok(output_path.join(new_path))
    };

    rename(&old_base, &transform)?;

    std::fs::remove_dir_all(&old_base).expect(
        format!(
            "decompress_xz remove_dir_all error {:?}",
            &old_base.display()
        )
        .as_str(),
    );

    Ok(())
}
