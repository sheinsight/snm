use flate2::read::GzDecoder;
use std::{fs::File, io::Write as _, path::PathBuf};
use tar::Archive;

use crate::model::SnmError;

pub fn decompress_tgz<F>(
    input_path: &PathBuf,
    output_path: &PathBuf,
    progress: &mut Option<F>,
) -> Result<(), SnmError>
where
    F: FnMut(&PathBuf, &PathBuf),
{
    // 打开 tgz 文件
    let tgz_file = File::open(input_path)?;
    // 使用 GzDecoder 解压 gzip 文件
    let tar = GzDecoder::new(tgz_file);
    // 创建 Archive 对象以便操作 tar 文件
    let mut archive = Archive::new(tar);
    // 从 archive 中解压所有文件到指定路径
    archive.unpack(output_path)?;

    let old_base = output_path.join("package");

    let transform = |f: &PathBuf| -> Result<PathBuf, SnmError> {
        let old_base = output_path.join("package");
        let new_path = f.strip_prefix(&old_base)?;
        Ok(output_path.join(new_path))
    };

    rename(&old_base, &transform, progress)?;

    std::fs::remove_dir_all(&old_base)?;

    Ok(())
}

pub fn rename<T, P>(dir: &PathBuf, transform: &T, progress: &mut Option<P>) -> Result<(), SnmError>
where
    T: Fn(&PathBuf) -> Result<PathBuf, SnmError>,
    P: FnMut(&PathBuf, &PathBuf),
{
    let mut stdout = std::io::stdout();
    let mut entries = std::fs::read_dir(&dir)?;

    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            rename(&path, transform, progress)?;
        } else {
            let from = &path;

            let to = &transform(&from)?;

            if let Some(parent) = to.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }

            stdout.flush()?;

            if let Some(p) = progress {
                p(&from, &to);
            }
            std::fs::rename(&from, &to)?;
        }
    }

    Ok(())
}

pub fn decompress_xz<F>(
    input_path: &PathBuf,
    output_path: &PathBuf,
    progress: &mut Option<F>,
) -> Result<(), SnmError>
where
    F: FnMut(&PathBuf, &PathBuf),
{
    let input_file = File::open(input_path)?;

    let decoder = xz2::read::XzDecoder::new(input_file);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(&output_path)?;

    let dir = input_path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.trim_end_matches(".tar.xz").to_string());

    let old_base = output_path.join(dir.as_ref().unwrap());

    let transform = |f: &PathBuf| -> Result<PathBuf, SnmError> {
        let old_base = output_path.join(dir.as_ref().unwrap());
        let new_path = f.strip_prefix(&old_base)?;
        Ok(output_path.join(new_path))
    };

    rename(&old_base, &transform, progress)?;

    std::fs::remove_dir_all(&old_base)?;

    Ok(())
}
