use std::{fs::File, path::PathBuf, time::Duration};

use anyhow::{Context, bail};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use tar::Archive;
use tracing::trace;
use xz2::read::XzDecoder;
use zip::ZipArchive;

#[derive(Debug)]
pub enum ArchiveExtension {
  Tgz(PathBuf),
  Xz(PathBuf),
  Zip(PathBuf),
}

impl ArchiveExtension {
  pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
    let ext = path
      .extension()
      .context(format!("File has no extension: {:?}", &path))?;

    match ext.to_str() {
      Some("tgz") | Some("gz") => Ok(Self::Tgz(path)),
      Some("xz") => Ok(Self::Xz(path)),
      Some("zip") => Ok(Self::Zip(path)),
      Some(ext) => bail!("Unsupported archive format '{}' for file: {:?}", ext, &path),
      None => bail!("Invalid non-UTF8 file extension for file: {:?}", &path),
    }
  }

  fn ensure_parent_dir(&self, path: &PathBuf) -> anyhow::Result<()> {
    // 确保父目录存在
    if let Some(parent) = path.parent() {
      if !parent.try_exists()? {
        std::fs::create_dir_all(parent)?;
      }
    }
    Ok(())
  }

  pub fn decompress(&self, target_dir: &PathBuf) -> anyhow::Result<()> {
    trace!(
      r#"Decompressing archive
source: {:?}
target: {:?}"#,
      self, target_dir
    );

    if target_dir.try_exists()? {
      std::fs::remove_dir_all(target_dir)?;
    } else {
      std::fs::create_dir_all(target_dir)?;
    }

    // 统一的 Spinner 样式设置
    let spinner_style = ProgressStyle::default_spinner()
      .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "-"])
      .template("{spinner:.blue} {msg}")?;

    let source_file_name = match self {
      ArchiveExtension::Tgz(p) | ArchiveExtension::Xz(p) | ArchiveExtension::Zip(p) => {
        p.file_name().unwrap_or_default()
      }
    };

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(spinner_style); // 克隆样式应用到进度条
    pb.set_message(format!("Extracting {:?}...", source_file_name));

    match self {
      ArchiveExtension::Tgz(source_file) => {
        let decoder = GzDecoder::new(File::open(source_file)?);
        let mut archive = Archive::new(decoder);

        for entry in archive.entries()? {
          let mut entry = entry?;
          let path = entry.path()?;

          if path.components().count() <= 1 {
            trace!("Skipping root entry: {:?}", path);
            continue;
          }

          // 去除第一个组件（根目录名）
          let target = path.components().skip(1).collect::<PathBuf>();
          let target = target_dir.join(target);

          if path.is_dir() {
            std::fs::create_dir_all(&target)?;
            continue;
          } else {
            self.ensure_parent_dir(&target)?;
          }

          trace!(r#"Unpacking file: {:?} -> {:?}"#, path, target);

          pb.set_message(format!("Unpacking to {}", target.display().to_string()));

          entry.unpack(&target)?;
        }
      }

      ArchiveExtension::Xz(source_file) => {
        let xz = XzDecoder::new(File::open(source_file)?);
        let mut archive = Archive::new(xz);
        for entry in archive.entries()? {
          let mut entry = entry?;
          let path = entry.path()?;

          if path.components().count() <= 1 {
            trace!("Skipping root entry: {:?}", path);
            continue;
          }

          // 去除第一个组件（根目录名）
          let target = path.components().skip(1).collect::<PathBuf>();
          let target = target_dir.join(target);

          if path.is_dir() {
            std::fs::create_dir_all(&target)?;
            continue;
          } else {
            self.ensure_parent_dir(&target)?;
          }

          trace!(r#"Unpacking file: {:?} -> {:?}"#, path, target);

          pb.set_message(format!("Unpacking to {}", target.display().to_string()));

          entry.unpack(&target)?;
        }
      }
      ArchiveExtension::Zip(source_file) => {
        let mut archive = ZipArchive::new(File::open(source_file)?)?;
        for i in 0..archive.len() {
          let mut file = archive.by_index(i)?;

          let path = file.mangled_name();

          // 跳过根目录
          if path.components().count() <= 1 {
            trace!("Skipping root entry: {:?}", path);
            continue;
          }

          // 去除第一个组件（根目录名）
          let target = path.components().skip(1).collect::<PathBuf>();
          let target = target_dir.join(target);

          if file.is_dir() {
            std::fs::create_dir_all(&target)?;
            continue;
          }

          self.ensure_parent_dir(&target)?;

          trace!(r#"Copying file: {:?} -> {:?}"#, path, target);

          pb.set_message(format!("Unpacking to {}", target.display().to_string()));

          // 只复制文件
          let mut outfile = std::fs::File::create(&target)?;
          std::io::copy(&mut file, &mut outfile)?;
        }
      }
    }

    pb.finish_and_clear();
    Ok(())
  }
}
