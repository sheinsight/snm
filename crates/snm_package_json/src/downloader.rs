use std::{
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
};

use anyhow::bail;
use flate2::read::GzDecoder;
use sha1::{Digest, Sha1};
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use tar::Archive;

use crate::{package_json::PackageJson, pm_metadata::PackageManagerMetadata};

#[derive(serde::Deserialize)]
struct NpmResponse {
  dist: Dist,
}

#[derive(serde::Deserialize)]
struct Dist {
  shasum: String,
}

pub struct PackageManagerDownloader<'a> {
  metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> PackageManagerDownloader<'a> {
  pub fn new(metadata: &'a PackageManagerMetadata) -> Self {
    Self { metadata }
  }

  pub async fn download_pm(&self, version: &str) -> anyhow::Result<PathBuf> {
    let config = &self.metadata.config;

    let downloaded_file_path_buf = self.download(version).await?;

    self
      .verify_shasum(&downloaded_file_path_buf, version)
      .await?;

    let decompressed_dir_path_buf = config
      .node_modules_dir
      .join(&self.metadata.name)
      .join(version);

    self.decompress_download_file(&downloaded_file_path_buf, &decompressed_dir_path_buf)?;

    Ok(decompressed_dir_path_buf)
  }

  async fn download(&self, version: &str) -> anyhow::Result<PathBuf> {
    let metadata = &self.metadata;

    let download_url = self.get_download_url(version);

    let downloaded_file_path_buf = metadata
      .config
      .download_dir
      .join(&metadata.library_name)
      .join(version)
      .join(format!("{}-{}.tgz", &metadata.library_name, version));

    DownloadBuilder::new()
      .retries(3)
      .timeout(metadata.config.download_timeout_secs)
      .write_strategy(WriteStrategy::WriteAfterDelete)
      .download(&download_url, &downloaded_file_path_buf)
      .await?;

    Ok(downloaded_file_path_buf)
  }

  async fn verify_shasum<T: AsRef<Path>>(&self, file_path: T, version: &str) -> anyhow::Result<()> {
    let expect_shasum = self.get_expect_shasum(version).await?;

    let actual_shasum = self.get_actual_shasum(file_path)?;

    if expect_shasum != actual_shasum {
      bail!("SHASUM mismatch");
    }

    Ok(())
  }

  async fn get_expect_shasum(&self, version: &str) -> anyhow::Result<String> {
    let url = self.get_expect_shasum_url(version);

    let resp = reqwest::get(&url).await?.json::<NpmResponse>().await?;

    Ok(resp.dist.shasum)
  }

  fn get_expect_shasum_url(&self, version: &str) -> String {
    let metadata = &self.metadata;
    // https://registry.npmjs.org/react/0.0.1
    // https://registry.npmjs.org/@yarnpkg/cli-dist/2.4.1
    format!(
      "{host}/{library_name}/{version}",
      host = &metadata.config.npm_registry,
      library_name = &metadata.library_name,
      version = version
    )
  }

  fn get_actual_shasum<T: AsRef<Path>>(
    &self,
    downloaded_file_path_buf: T,
  ) -> anyhow::Result<String> {
    let file = File::open(downloaded_file_path_buf)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    std::io::copy(&mut reader, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
  }

  fn decompress_download_file<P: AsRef<Path>>(
    &self,
    input_file_path_buf: P,
    output_dir_path_buf: P,
  ) -> anyhow::Result<()> {
    let temp_dir = output_dir_path_buf.as_ref().join("temp");
    std::fs::create_dir_all(&temp_dir)?;

    let tar_gz = File::open(input_file_path_buf)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&temp_dir)?;

    // 获取解压后的第一个目录
    let entry = std::fs::read_dir(&temp_dir)?
      .next()
      .ok_or_else(|| anyhow::anyhow!("No files found"))??;

    // 移动文件
    for entry in std::fs::read_dir(entry.path())? {
      let entry = entry?;
      let target = output_dir_path_buf.as_ref().join(entry.file_name());
      std::fs::rename(entry.path(), target)?;
    }

    // 清理临时目录
    std::fs::remove_dir_all(&temp_dir)?;

    let json = PackageJson::from(&output_dir_path_buf)?;

    json.enumerate_bin(|k, v| {
      let dir = v.parent().unwrap();
      let link_file = dir.join(k);
      if !link_file.exists() {
        #[cfg(unix)]
        {
          std::os::unix::fs::symlink(v, link_file).unwrap();
        }
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(v, link_file).unwrap();
      }
    });

    Ok(())
  }

  fn get_download_url(&self, version: &str) -> String {
    let metadata = &self.metadata;
    let npm_registry = &metadata.config.npm_registry;

    // 使用 rsplit_once 直接获取最后一个部分，避免创建 Vec
    let file_name = metadata
      .library_name
      .rsplit_once('/')
      .map_or(metadata.library_name.clone(), |(_, name)| name.to_owned());

    format!(
      "{host}/{name}/-/{file}-{version}.tgz",
      host = npm_registry,
      name = metadata.library_name,
      file = file_name
    )
  }
}
