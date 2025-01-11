use std::path::Path;

use anyhow::{bail, Context};

#[derive(Debug)]
pub enum ArchiveExtension {
  Tgz,
  Xz,
  Zip,
}

impl ArchiveExtension {
  pub fn from_path<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
    let ext = path
      .as_ref()
      .extension()
      .and_then(|s| s.to_str())
      .context("Invalid file extension")?;

    match ext {
      "tgz" | "gz" => Ok(Self::Tgz),
      "xz" => Ok(Self::Xz),
      "zip" => Ok(Self::Zip),
      _ => bail!("Unsupported archive format: {}", ext),
    }
  }
}
