use std::{fs::File, path::PathBuf};

use anyhow::{bail, Context};
use flate2::read::GzDecoder;
use tar::Archive;
use tracing::trace;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::trace_if;

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
      .and_then(|s| s.to_str())
      .context("Invalid file extension")?;

    match ext {
      "tgz" | "gz" => Ok(Self::Tgz(path)),
      "xz" => Ok(Self::Xz(path)),
      "zip" => Ok(Self::Zip(path)),
      _ => bail!("Unsupported archive format: {}", ext),
    }
  }

  // fn ensure_dir_exists(&self, path: &PathBuf) -> anyhow::Result<()> {
  //   trace_if!(|| {
  //     trace!("Ensure dir exists: {:?}", path);
  //   });

  //   if path.is_file() {
  //     if let Some(parent) = path.parent() {
  //       if !parent.try_exists()? {
  //         std::fs::create_dir_all(parent)?;
  //       }
  //     }
  //   } else {
  //     if !path.try_exists()? {
  //       std::fs::create_dir_all(path)?;
  //     }
  //   }

  //   Ok(())
  // }

  pub fn decompress(&self, target_dir: &PathBuf) -> anyhow::Result<()> {
    if target_dir.try_exists()? {
      std::fs::remove_dir_all(target_dir)?;
    } else {
      std::fs::create_dir_all(target_dir)?;
    }

    match self {
      ArchiveExtension::Tgz(source_file) => {
        let decoder = GzDecoder::new(File::open(source_file)?);
        let mut archive = Archive::new(decoder);
        for entry in archive.entries()? {
          let mut entry = entry?;
          let path = entry.path()?;
          if let Some(first) = path.components().next() {
            let target = path.strip_prefix(first)?;
            let target = target_dir.join(target);
            // self.ensure_dir_exists(&target)?;
            entry.unpack(&target)?;
          }
        }
      }

      ArchiveExtension::Xz(source_file) => {
        let xz = XzDecoder::new(File::open(source_file)?);
        let mut archive = Archive::new(xz);
        for entry in archive.entries()? {
          let mut entry = entry?;
          let path = entry.path()?;
          if let Some(first) = path.components().next() {
            let target = path.strip_prefix(first)?;
            let target = target_dir.join(target);
            // self.ensure_dir_exists(&target)?;
            entry.unpack(&target)?;
          }
        }
      }
      ArchiveExtension::Zip(source_file) => {
        let mut archive = ZipArchive::new(File::open(source_file)?)?;
        for i in 0..archive.len() {
          let mut file = archive.by_index(i)?;
          let path = file.mangled_name();
          if let Some(first) = path.components().next() {
            let target = path.strip_prefix(first)?;
            let target = target_dir.join(target);
            let target = dunce::canonicalize(target)?;
            trace_if!(|| {
              trace!("Decompress file: {}", target.to_string_lossy());
            });
            // self.ensure_dir_exists(&target)?;
            let mut outfile = std::fs::File::create(&target)?;
            std::io::copy(&mut file, &mut outfile)?;
          }
        }
      }
    }

    Ok(())
  }
}
