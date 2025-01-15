use std::{
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
// use colored::Colorize;
use flate2::read::GzDecoder;
// use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use snm_config::SnmConfig;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::archive_extension::ArchiveExtension;

pub struct NodeDownloader<'a> {
  config: &'a SnmConfig,
}

impl<'a> NodeDownloader<'a> {
  pub fn new(config: &'a SnmConfig) -> Self {
    Self { config }
  }

  pub async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
    let downloaded_file = self.download_node(&version).await?;

    self.verify_shasum(&downloaded_file, &version).await?;

    let node_dir = self.config.node_bin_dir.join(&version);

    self.decompress(&downloaded_file, &node_dir)?;

    Ok(node_dir)
  }

  fn decompress<T: AsRef<Path>, U: AsRef<Path>>(
    &self,
    downloaded_file_path_buf: T,
    output_dir: U,
  ) -> anyhow::Result<()> {
    let format = ArchiveExtension::from_path(&downloaded_file_path_buf)?;
    let file = File::open(&downloaded_file_path_buf)?;

    let temp_dir = output_dir.as_ref().join("temp");
    std::fs::create_dir_all(&temp_dir)?;
    match format {
      ArchiveExtension::Tgz => {
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        archive.unpack(&temp_dir)?;
      }
      ArchiveExtension::Xz => {
        // 处理 xz
        let xz = XzDecoder::new(file);
        let mut archive = Archive::new(xz);
        archive.unpack(&temp_dir)?;
      }
      ArchiveExtension::Zip => {
        // 处理 zip
        let mut archive = ZipArchive::new(file)?;
        archive.extract(&temp_dir)?;
      }
    }

    // 获取解压后的第一个目录
    let entry = std::fs::read_dir(&temp_dir)?
      .next()
      .ok_or_else(|| anyhow::anyhow!("No files found"))??;

    // 移动文件
    for entry in std::fs::read_dir(entry.path())? {
      let entry = entry?;
      let target = output_dir.as_ref().join(entry.file_name());
      std::fs::rename(entry.path(), target)?;
    }

    // 清理临时目录
    std::fs::remove_dir_all(&temp_dir)?;

    Ok(())
  }

  async fn verify_shasum(&self, file_path: &PathBuf, version: &str) -> anyhow::Result<()> {
    let actual = self.get_actual_shasum(file_path).await?;
    let expected = self.get_expect_shasum(version).await?;

    if actual != expected {
      bail!("Node binary shasum mismatch");
    }
    Ok(())
  }

  async fn get_actual_shasum<T: AsRef<Path>>(
    &self,
    downloaded_file_path_buf: T,
  ) -> anyhow::Result<String> {
    let file = File::open(downloaded_file_path_buf)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    std::io::copy(&mut reader, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
  }

  async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
    let sha256_url = format!(
      "{host}/v{version}/SHASUMS256.txt",
      host = self.config.node_dist_url,
      version = version
    );

    let file_name = self.get_file_name(version);

    let sha256_str = reqwest::get(&sha256_url).await?.text().await?;

    sha256_str
      .lines()
      .find_map(|line| {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next()) {
          (Some(sha256), Some(file)) if file == file_name => Some(sha256.to_owned()),
          _ => None,
        }
      })
      .with_context(|| "Invalid Node SHASUM line format")
  }

  async fn download_node(&self, version: &str) -> anyhow::Result<PathBuf> {
    let download_url = self.get_download_url(version);
    let downloaded_file_path_buf = self.get_downloaded_path_buf(version);

    DownloadBuilder::new()
      .retries(3)
      .timeout(self.config.download_timeout_secs)
      .write_strategy(WriteStrategy::WriteAfterDelete)
      .download(&download_url, &downloaded_file_path_buf)
      .await?;

    Ok(downloaded_file_path_buf)
  }

  fn get_downloaded_path_buf(&self, version: &str) -> PathBuf {
    let file_name = self.get_file_name(version);
    self
      .config
      .download_dir
      .join("node")
      .join(version)
      .join(file_name)
  }

  fn get_file_name(&self, version: &str) -> String {
    format!(
      "node-v{version}-{os}-{arch}.{ext}",
      version = version,
      os = snm_utils::consts::os(),
      arch = snm_utils::consts::arch(),
      ext = snm_utils::consts::ext()
    )
  }

  fn get_download_url(&self, version: &str) -> String {
    format!(
      "{host}/v{version}/node-v{version}-{os}-{arch}.{ext}",
      host = self.config.node_dist_url,
      version = version,
      os = snm_utils::consts::os(),
      arch = snm_utils::consts::arch(),
      ext = snm_utils::consts::ext()
    )
  }
}

// pub const fn get_tarball_ext() -> &'static str {
//     #[cfg(target_os = "windows")]
//     {
//         "zip"
//     }
//     #[cfg(any(target_os = "linux", target_os = "macos"))]
//     {
//         "tar.xz"
//     }
//     #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
//     {
//         "unknown"
//     }
// }

// pub const fn get_arch() -> &'static str {
//     #[cfg(target_arch = "x86")]
//     {
//         "x86"
//     }
//     #[cfg(target_arch = "x86_64")]
//     {
//         "x64"
//     }
//     #[cfg(target_arch = "arm")]
//     {
//         "armv7l"
//     }
//     #[cfg(target_arch = "aarch64")]
//     {
//         "arm64"
//     }
//     #[cfg(target_arch = "powerpc64")]
//     {
//         "ppc64"
//     }
//     #[cfg(target_arch = "powerpc64le")]
//     {
//         "ppc64le"
//     }
//     #[cfg(target_arch = "s390x")]
//     {
//         "s390x"
//     }
//     #[cfg(not(any(
//         target_arch = "x86",
//         target_arch = "x86_64",
//         target_arch = "arm",
//         target_arch = "aarch64",
//         target_arch = "powerpc64",
//         target_arch = "powerpc64le",
//         target_arch = "s390x"
//     )))]
//     {
//         "unknown"
//     }
// }

// pub const fn get_os() -> &'static str {
//     #[cfg(target_os = "macos")]
//     {
//         "darwin"
//     }
//     #[cfg(target_os = "windows")]
//     {
//         "win"
//     }
//     #[cfg(target_os = "linux")]
//     {
//         "linux"
//     }
//     #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
//     {
//         "unknown"
//     }
// }
